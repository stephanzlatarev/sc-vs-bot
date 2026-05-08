import starcraft from "@node-sc2/proto";

export default class Client {

  client = starcraft();

  async connect() {
    const deadline = Date.now() + 60000;

    while (Date.now() < deadline) {
      try {
        console.log("Connecting to StarCraft II...");
        await this.client.connect({ host: "127.0.0.1", port: 10001 });

        console.log("Connected");
        return;
      } catch (e) {
        console.log("Error on attempt to connect to StarCraft II:", e.message || e);
        await new Promise(r => setTimeout(r, 3000));
      }
    }

    console.log("Unable to connect to StarCraft II");
  }

  async createGame() {
    console.log("Creating game");
    const response = await this.client.createGame({
      localMap: { mapPath: "/StarCraftII/Maps/LeyLinesAIE_v3.SC2Map" },
      playerSetup: [{ type: 1 }, { type: 1 }],
      realtime: false,
    });
    console.log("Game created:", response);
  }

  async disconnect() {
    console.log("Disconnecting...");
    await this.client.close();
  }

}
