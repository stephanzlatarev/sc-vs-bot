import { spawn } from "child_process";
import starcraft from "@node-sc2/proto";

class Game {

  status = {
    progressing: false,
    running: false,
    cooldown: 0,
  };

  async start() {
    if (this.status.running) return;
    if (this.status.progressing) return;

    this.status.progressing = true;

    if (Date.now() < this.status.cooldown) {
      console.log("Cooling StarCraft II...");
      await new Promise(r => setTimeout(r, this.status.cooldown - Date.now()));
    }

    console.log("Starting StarCraft II...");

    this.process = spawn("/StarCraftII/Versions/Base75689/SC2_x64", [
      "--listen", "127.0.0.1",
      "--port", "10001",
    ], {
      cwd: "/StarCraftII",
      stdio: "inherit",
    });

    this.process.on("error", this.exit.bind(this));
    this.process.on("exit", this.exit.bind(this));

    this.status.running = true;
    this.status.progressing = false;
  }

  exit(details) {
    console.error("StarCraft II stopped", details || "");
    this.process = null;
    this.status.progressing = false;
    this.status.running = false;
    this.status.cooldown = Date.now() + 10000;
  }

  stop() {
    if (this.process) this.process.kill();

    this.status.progressing = false;
    this.status.running = false;
    this.status.cooldown = Date.now() + 10000;
  }

}

export default new Game();
