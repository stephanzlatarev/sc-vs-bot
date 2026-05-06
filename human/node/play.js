import Game from "./game.js";
import Network from "./network.js";
import Proxy from "./proxy.js";

async function play() {
  const game = new Game();
  const proxy = new Proxy();

  try {
    Network();

    await game.start();
    await proxy.connect();

    await proxy.create();
    await proxy.join(3, "Human");

    while (true) {
      await proxy.step();
    }
  } catch (error) {
    console.log(error);
  } finally {
    await game.stop();
  }
}

play();
