import starcraft from "@node-sc2/proto";
import { PATH, PORT } from "./config.js";

const MILLIS_PER_LOOP = 1000 / 22.4;

export default class Proxy {

  client = starcraft();

  loops = 0;
  time = 0;

  async connect() {
    const deadline = Date.now() + 60000;

    while (Date.now() < deadline) {
      try {
        console.log("Connecting to StarCraft II...");
        await this.client.connect({ host: "127.0.0.1", port: PORT });

        console.log("Connected");
        return;
      } catch (e) {
        console.log("Error on attempt to connect to StarCraft II:", e.message || e);
        await new Promise(r => setTimeout(r, 3000));
      }
    }

    console.log("Unable to connect to StarCraft II");
  }

  async create() {
    console.log("Creating game");
    const response = await this.client.createGame({
      localMap: { mapPath: PATH + "\\Maps\\LeyLinesAIE_v3.SC2Map" },
      playerSetup: [{ type: 1 }, { type: 1 }],
      realtime: false,
    });
    console.log("Game created:", response);
  }

  async join(race, name) {
    console.log("Joining game...");
    const joined = await this.client.joinGame({
      playerName: name || "Human",
      race: race || 1,
      options: { raw: true, score: true },
      serverPorts: { gamePort: 10004, basePort: 10004 },
      clientPorts: [{ gamePort: 10005, basePort: 10005 }],
    });
    console.log("Game joined:", joined);
  }

  async step() {
    await this.client.step({ count: 1 });

    loops++;

    const elapsed = (Date.now() - this.time);
    const expected = loops * MILLIS_PER_LOOP;

    if (elapsed < expected) {
      await new Promise(r => setTimeout(r, expected - elapsed));
    }
  }

  async disconnect() {
    console.log("Disconnecting...");
    await this.client.close();
  }
}
