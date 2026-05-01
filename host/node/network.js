import dgram from "dgram";
import net from "net";

const UDP_PORT = 10005;
const TCP_PORT = 10055;

const status = {
  connected: false, // Client is connected and pings are flowing
  playing: false,   // Status is connected and no joins are sent
  ping: 0,
  join: Infinity,
};

let host;
let client;

function trace(error, ...details) {
  if (error) console.log("ERROR:", error);
  if (!error && details && details.length) console.log(...details);
}

function connect(socket) {
  console.log("Client", socket ? "connected" : "disconnected");

  status.connected = !!socket;
  status.playing = false;
  status.ping = 0;
  status.join = Infinity;

  client = socket;

  if (socket) {
    socket.setKeepAlive(true, 1000);
    socket.setNoDelay(true);
    socket.on("data", (data) => {
      if (!host) return;

      if (data.length > 1) {
        console.log(`(${data.length}) <<<`, data);
        udp.send(data, host.port, host.address, trace);
      } else {
        // This is a ping to detect firewalls
        status.ping = Date.now();
      }
    });

    socket.on("error", trace);
    socket.on("close", connect);
  }
}

const udp = dgram.createSocket("udp4");
const tcp = net.createServer(connect);

udp.on("message", (message, source) => {
  host = source;

  if (client) {
    console.log(`(${message.length}) >>>`, message);
    client.write(Buffer.from(message));
    status.join = Date.now();
  }
});

udp.on("listening", () => {
  const address = udp.address();
  console.log(`Listening on: ${address.address}:${address.port}`);
});
tcp.on("listening", () => {
  const address = tcp.address();
  console.log(`Listening on: ${address.address}:${address.port}`);
});

udp.on("error", trace);
tcp.on("error", trace);

udp.bind(UDP_PORT);
tcp.listen(TCP_PORT);

setInterval(() => {
  if (client && (status.ping >= Date.now() - 3000)) {
    status.connected = true;
    status.playing = (status.join < Date.now() - 3000);
  } else {
    status.connected = false;
    status.playing = false;

    if (client) {
      client.destroy();
      client = null;
    }
  }
}, 3000);

export default status;
