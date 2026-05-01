import dgram from "dgram";
import net from "net";

const TCP_HOST = "host.docker.internal";
//const TCP_HOST = "129.212.171.186";
const TCP_PORT = 10055;

const UDP_HOST = "127.0.0.1";
const UDP_PORT = 10005;

const tcp = net.createConnection({ host: TCP_HOST, port: TCP_PORT }, () => {
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
