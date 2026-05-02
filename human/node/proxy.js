import starcraft from "@node-sc2/proto";
import { PORT } from "./config.js";

const LOOPS_PER_SECOND = 22.4;
const LOOPS_PER_MINUTE = LOOPS_PER_SECOND * 60;
const SPEED = Math.floor(1000 / LOOPS_PER_SECOND);

export default class Proxy {

  client = starcraft();

  async connect() {
    const deadline = Date.now() + 60000;

    while (Date.now() < deadline) {
      try {
        console.log("Connecting to StarCraft II...");
        await this.client.connect({ host: "127.0.0.1", port: PORT });

        console.log("Connected");
        return;
      } catch (_) {
        await new Promise(r => setTimeout(r, 1000));
      }
    }

    console.log("Unable to connecting to StarCraft II");
  }

  async join() {
    console.log("Joining game...");
    await this.client.joinGame({
      race: 1,
      options: { raw: true, score: true },
      serverPorts: { gamePort: 10004, basePort: 10004 },
      clientPorts: [{ gamePort: 10005, basePort: 10005 }],
    });

    const response = await this.client.observation();
    const base = response.observation.rawData.units.find(unit => ((unit.owner < 16) && (unit.radius > 2)));

    this.base = base.tag;

    console.log("Base:", this.base);

    this.started = Date.now();
  }

  async step() {
    await this.client.step({ count: 1 });
    await new Promise(r => setTimeout(r, SPEED));
  }

  async trace() {
    const response = await this.client.observation();
    const loop = response.observation.gameLoop;
    const time = clock(loop);
    const resources = response.observation.playerCommon;

    console.log(time, "\tworkers:", resources.foodWorkers, "\tminerals:", resources.minerals, "\ttime:", timerate(this.started, loop));
  }

  async disconnect() {
    console.log("Disconnecting...");
    await this.client.close();
  }
}

function clock(loop) {
  const minutes = Math.floor(loop / LOOPS_PER_MINUTE);
  const seconds = Math.floor(loop / LOOPS_PER_SECOND) % 60;
  const mm = (minutes >= 10) ? minutes : "0" + minutes;
  const ss = (seconds >= 10) ? seconds : "0" + seconds;

  return `${mm}:${ss}/${loop}`;
}

function timerate(started, loop) {
  const actural = (Date.now() - started);
  const expected = (loop / LOOPS_PER_SECOND) * 1000;
  const rate = (actural * 100 / expected).toFixed(2);

  return `${rate}%`;
}

