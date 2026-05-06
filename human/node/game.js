import { spawn } from "child_process";
import { PATH, PORT, VERSION } from "./config.js";

export default class Game {

  async start(port) {
    console.log("Starting StarCraft II...");

    this.process = spawn("..\\Versions\\" + VERSION + "\\SC2_x64.exe", [
      "-dataVersion", "B89B5D6FA7CBF6452E721311BFBC6CB2",
      "-displaymode", "0", "-windowx", "0", "-windowy", "0", "-windowwidth", "800", "-windowheight", "500",
      "-listen", "127.0.0.1", "-port", String(port || PORT)
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
