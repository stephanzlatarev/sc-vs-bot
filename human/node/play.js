import Client from "./client.js";
import Game from "./game.js";
import Network from "./network.js";

async function play() {
  const game = new Game();
  const client = new Client();

  try {
    Network();

    await game.start();
    await client.connect();

    await client.createGame();
    await client.joinGame(3, "Human");

    while (true) {
      await client.step();
    }
  } catch (error) {
    console.log(error);
  } finally {
    await game.stop();
  }
}

play();
