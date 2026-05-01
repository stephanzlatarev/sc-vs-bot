import { spawn } from "child_process";
import game from "./game.js";

class Bot {

  status = {
    running: false,
  };

  start() {
    this.stop();

    console.log("Starting VeTerran-revived...");

    this.process = spawn("/bots/VeTerran-revived", [
      "--GamePort", "10001",
      "--LadderServer", "127.0.0.1",
      "--StartPort", "10001",
      "--OpponentId", "challenger"
    ], {
      cwd: "/tmp",
      stdio: "inherit",
    });

    this.process.on("error", this.exit.bind(this));
    this.process.on("exit", this.exit.bind(this));

    this.status.running = true;
  }

  exit(details) {
    console.error("VeTerran-revived stopped", details || "");
    this.process = null;
    this.status.running = false;

    game.stop();
  }

  stop() {
    if (this.process) this.process.kill();
    this.status.running = false;
  }

}

export default new Bot();
