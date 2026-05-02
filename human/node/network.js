import dgram from "dgram";
import net from "net";

const REMOTE_HOST = "host.docker.internal";
//const REMOTE_HOST = "129.212.171.186";

// TCP proxy: local 10004 -> remote 10044 (equivalent to socat in client.sh)
const PROXY_LISTEN_PORT = 10004;
const PROXY_REMOTE_PORT = 10044;

// UDP/TCP bridge: TCP remote 10055 <-> UDP local 10005
const TCP_PORT = 10055;
const UDP_HOST = "127.0.0.1";
const UDP_PORT = 10005;

// --- TCP proxy (socat equivalent) ---
const proxy = net.createServer((local) => {
  const remote = net.createConnection({ host: REMOTE_HOST, port: PROXY_REMOTE_PORT }, () => {
    console.log(`Proxy: connection established (local -> ${REMOTE_HOST}:${PROXY_REMOTE_PORT})`);
  });

  local.pipe(remote);
  remote.pipe(local);

  local.on("error", (err) => { console.log("Proxy local error:", err.message); remote.destroy(); });
  remote.on("error", (err) => { console.log("Proxy remote error:", err.message); local.destroy(); });
  local.on("close", () => remote.destroy());
  remote.on("close", () => local.destroy());
});

proxy.listen(PROXY_LISTEN_PORT, "0.0.0.0", () => {
  console.log(`Proxy: listening on 0.0.0.0:${PROXY_LISTEN_PORT} -> ${REMOTE_HOST}:${PROXY_REMOTE_PORT}`);
});

proxy.on("error", (err) => console.log("Proxy server error:", err.message));

// --- UDP/TCP bridge ---
const tcp = net.createConnection({ host: REMOTE_HOST, port: TCP_PORT }, () => {
  console.log("Host connected");
});
const udp = dgram.createSocket("udp4");

tcp.setKeepAlive(true);
tcp.setNoDelay(true);
tcp.on("data", (data) => {
  const buffer = Buffer.from(data);
  console.log(`(${buffer.length}) TCP>>UDP`, buffer);
  udp.send(buffer, UDP_PORT, UDP_HOST, trace);
});

udp.on("message", (message) => {
  const buffer = Buffer.from(message);
  console.log(`(${buffer.length}) TCP<<UDP`, buffer);
  tcp.write(buffer);
});

udp.on("listening", () => {
  const address = udp.address();
  console.log(`Listening on: ${address.address}:${address.port} UDP responses`);
});

udp.on("error", trace);

tcp.on("error", trace);
tcp.on("close", () => {
  console.log("Host disconnected");
});

udp.bind();

// Send pings to detect firewalls
setInterval(() => { tcp.write(Buffer.from([0])); }, 500);

function trace(error, ...details) {
  if (error) console.log("ERROR:", error);
  if (!error && details && details.length) console.log(...details);
}

export default function() {
  console.log("Networking created");
}
