import starcraft from "@node-sc2/proto";

class Bot {

  client = starcraft();

  constructor(port) {
    this.port = port;
  }

  async connect() {
    console.log(this.port, "Connecting to port:", this.port);
    await this.client.connect({ host: "localhost", port: this.port });
  }

  async join() {
    console.log(this.port, "Joining game...");
    await this.client.joinGame({
      race: 1,
      options: { raw: true, score: true },
      serverPorts: { gamePort: 10004, basePort: 10004 },
      clientPorts: [{ gamePort: 10005, basePort: 10005 }],
    });

    const response = await this.client.observation();
    const base = response.observation.rawData.units.find(unit => ((unit.owner < 16) && (unit.radius > 2)));

    this.base = base.tag;

    console.log(this.port, "base:", this.base);

    this.started = Date.now();
  }

  async step() {
    if (this.base) await this.client.action({ actions: [{ actionRaw: { unitCommand: { unitTags: [this.base], abilityId: 524 } } }] });
    await this.client.step({ count: 1 });
  }

  async trace() {
    const response = await this.client.observation();
    const loop = response.observation.gameLoop;
    const time = clock(loop);
    const resources = response.observation.playerCommon;

    console.log(this.port, time, "\tworkers:", resources.foodWorkers, "\tminerals:", resources.minerals, "\ttime:", timerate(this.started, loop));
  }

  async disconnect() {
    console.log(this.port, "Disconnecting from port:", this.port);
    await this.client.close();
  }
}

const LOOPS_PER_SECOND = 22.4;
const LOOPS_PER_MINUTE = LOOPS_PER_SECOND * 60;
const SPEED = Math.floor(1000 / LOOPS_PER_SECOND);

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

async function play() {
  try {
    const bot = new Bot(10001);

    await bot.connect();

    await bot.join();

    let time = Date.now() + 1000;
    while (true) {
      if (Date.now() > time) {
        await bot.trace();

        time = Date.now() + 1000;
      }

      await bot.step();

      // await new Promise(r => setTimeout(r, SPEED));
    }
  } catch (error) {
    console.log(error);
  }
}

play();
