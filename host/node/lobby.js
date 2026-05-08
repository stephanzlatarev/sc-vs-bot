import bot from "./bot.js";
import game from "./game.js";
import network from "./network.js";

async function run() {
  if (network.connectedServerPort && network.connectedClientPort) {
    if (!game.status.running && !game.status.progressing) {
      console.log("Lobby starts game");
      game.start();
    } else if (game.status.running && !bot.status.running && !bot.status.progressing) {
      console.log("Lobby starts bot");
      bot.start();
    }
  } else if (bot.status.running) {
    console.log("Lobby stops bot");
    bot.stop();
  }
}

setInterval(run, 1000);

export default function() {
  console.log("Lobby started");
}
