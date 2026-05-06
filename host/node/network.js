import dgram from "dgram";
import net from "net";

const LOCAL_TCP_PORT = 10004;
const PUBLIC_TCP_PORT = 10044;
const LOCAL_UDP_HOST = "127.0.0.1";
const LOCAL_UDP_PORT = 10005;
const PUBLIC_UDP_PORT = 10055;

const status = {
  connected: false,
  playing: false,
};

let gameTunnel;
let udpTunnel;
let pendingLocalGame;

function trace(error, ...details) {
  if (error) console.log("ERROR:", error);
  if (!error && details && details.length) console.log(...details);
}

function pairSockets(left, right, label) {
  console.log(`${label} paired`);

  left.on("error", trace);
  right.on("error", trace);

  left.on("close", () => right.destroy());
  right.on("close", () => left.destroy());

  left.pipe(right);
  right.pipe(left);
}

function pairGameTunnel() {
  if (!gameTunnel || !pendingLocalGame) return;

  const tunnel = gameTunnel;
  const local = pendingLocalGame;

  gameTunnel = null;
  pendingLocalGame = null;

  pairSockets(local, tunnel, "TCP game tunnel");
}

const udp = dgram.createSocket("udp4");
const localTcp = net.createServer((socket) => {
  console.log("Local game client connected");

  if (pendingLocalGame) pendingLocalGame.destroy();

  pendingLocalGame = socket;
  socket.setKeepAlive(true, 1000);
  socket.setNoDelay(true);
  socket.on("error", trace);
  socket.on("close", () => {
    if (pendingLocalGame === socket) pendingLocalGame = null;
  });

  if (!gameTunnel) {
    console.log("TCP game tunnel unavailable");
    socket.destroy();
    return;
  }

  pairGameTunnel();
});

const publicTcp = net.createServer((socket) => {
  console.log("Human TCP tunnel connected");

  if (gameTunnel) gameTunnel.destroy();

  gameTunnel = socket;
  socket.setKeepAlive(true, 1000);
  socket.setNoDelay(true);
  socket.on("error", trace);
  socket.on("close", () => {
    console.log("Human TCP tunnel disconnected");
    if (gameTunnel === socket) gameTunnel = null;
    if (pendingLocalGame) {
      pendingLocalGame.destroy();
      pendingLocalGame = null;
    }
  });

  pairGameTunnel();
});

const publicUdp = net.createServer((socket) => {
  console.log("Human UDP tunnel connected");

  if (udpTunnel) udpTunnel.destroy();

  udpTunnel = socket;
  status.connected = true;
  status.playing = false;

  socket.setKeepAlive(true, 1000);
  socket.setNoDelay(true);
  socket.on("data", (data) => {
    if (data.length <= 1) return;

    console.log(`(${data.length}) TCP>>UDP`, data);
    udp.send(data, LOCAL_UDP_PORT, LOCAL_UDP_HOST, trace);
  });

  socket.on("error", trace);
  socket.on("close", () => {
    console.log("Human UDP tunnel disconnected");
    if (udpTunnel === socket) udpTunnel = null;
    status.connected = false;
    status.playing = false;
  });
});

udp.on("message", (message) => {
  if (!udpTunnel) return;

  console.log(`(${message.length}) UDP>>TCP`, message);
  udpTunnel.write(Buffer.from(message));
  status.playing = true;
});

udp.on("listening", () => {
  const address = udp.address();
  console.log(`Listening on: ${address.address}:${address.port} UDP responses`);
});
localTcp.on("listening", () => {
  const address = localTcp.address();
  console.log(`Listening on: ${address.address}:${address.port} local game TCP`);
});
publicTcp.on("listening", () => {
  const address = publicTcp.address();
  console.log(`Listening on: ${address.address}:${address.port} public TCP tunnel`);
});
publicUdp.on("listening", () => {
  const address = publicUdp.address();
  console.log(`Listening on: ${address.address}:${address.port} public UDP tunnel`);
});

udp.on("error", trace);
localTcp.on("error", trace);
publicTcp.on("error", trace);
publicUdp.on("error", trace);

udp.bind();
localTcp.listen(LOCAL_TCP_PORT, "127.0.0.1");
publicTcp.listen(PUBLIC_TCP_PORT);
publicUdp.listen(PUBLIC_UDP_PORT);

setInterval(() => {
  if (!udpTunnel) return;

  udpTunnel.write(Buffer.from([0]));
}, 500);

export default status;
