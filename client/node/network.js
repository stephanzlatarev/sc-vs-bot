import dgram from "dgram";
import net from "net";

//const REMOTE_HOST = "host.docker.internal";
const REMOTE_HOST = "209.38.114.125";

const LOCAL_TCP_HOST = "127.0.0.1";
const LOCAL_TCP_PORT = 10004;
const REMOTE_TCP_PORT = 10044;

const REMOTE_UDP_PORT = 10055;
const UDP_PORT = 10005;

let localTcp;
let udpPeer;

function connectLocalGame(remote) {
  if (localTcp) return;

  const local = net.createConnection({ host: LOCAL_TCP_HOST, port: LOCAL_TCP_PORT }, () => {
    console.log(`Local game connected (${LOCAL_TCP_HOST}:${LOCAL_TCP_PORT})`);
    localTcp = local;
    local.pipe(remote);
    remote.pipe(local);
  });

  local.setKeepAlive(true, 1000);
  local.setNoDelay(true);
  local.on("error", (error) => {
    if (localTcp === local) localTcp = null;

    if (error.code === "ECONNREFUSED" || error.code === "ECONNRESET") {
      setTimeout(() => {
        if (!remote.destroyed) connectLocalGame(remote);
      }, 1000);
      return;
    }

    trace(error);
    remote.destroy();
  });
  local.on("close", () => {
    if (localTcp === local) {
      localTcp = null;
      if (!remote.destroyed) remote.destroy();
    }
  });
}

const tcpProxy = net.createConnection({ host: REMOTE_HOST, port: REMOTE_TCP_PORT }, () => {
  console.log(`TCP tunnel connected (${REMOTE_HOST}:${REMOTE_TCP_PORT})`);
  connectLocalGame(tcpProxy);
});
const udp = dgram.createSocket("udp4");

tcpProxy.setKeepAlive(true);
tcpProxy.setNoDelay(true);
tcpProxy.on("error", trace);
tcpProxy.on("close", () => {
  console.log("TCP tunnel disconnected");
  if (localTcp) {
    localTcp.destroy();
    localTcp = null;
  }
});

const tcp = net.createConnection({ host: REMOTE_HOST, port: REMOTE_UDP_PORT }, () => {
  console.log("UDP tunnel connected");
});

tcp.setKeepAlive(true);
tcp.setNoDelay(true);
tcp.on("data", (data) => {
  if (!udpPeer) return;

  if (data.length > 1) {
    console.log(`(${data.length}) TCP>>UDP`, data);
    udp.send(data, udpPeer.port, udpPeer.address, trace);
  }
});

udp.on("message", (message, source) => {
  udpPeer = source;

  console.log(`(${message.length}) UDP>>TCP`, message);
  tcp.write(Buffer.from(message));
});

udp.on("listening", () => {
  const address = udp.address();
  console.log(`Listening on: ${address.address}:${address.port} UDP bridge`);
});

udp.on("error", trace);

tcp.on("error", trace);
tcp.on("close", () => {
  console.log("UDP tunnel disconnected");
});

udp.bind(UDP_PORT);

function trace(error, ...details) {
  if (error) console.log("ERROR:", error);
  if (!error && details && details.length) console.log(...details);
}

export default function() {
  console.log("Networking created");
}
