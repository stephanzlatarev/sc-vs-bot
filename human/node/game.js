import { spawn } from "child_process";
import { PATH, PORT, VERSION } from "./config.js";

export default class Game {

  async start() {
    console.log("Starting StarCraft II...");

    this.process = spawn("..\\Versions\\" + VERSION + "\\SC2_x64.exe", [
      "-displaymode", "0", "-windowx", "0", "-windowy", "0", "-windowwidth", "2500", "-windowheight", "1875",
      "-listen", "127.0.0.1", "-port", String(PORT)
    ], {
      cwd: PATH + "\\Support64",
      stdio: "inherit",
    });

    this.process.on("error", this.exit.bind(this));
    this.process.on("exit", this.exit.bind(this));
  }

  exit(details) {
    console.error("StarCraft II stopped", details || "");
    this.process = null;
  }

  stop() {
    if (this.process) this.process.kill();
  }

}
