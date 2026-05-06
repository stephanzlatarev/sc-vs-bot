import Game from "./game.js";
import Network from "./network.js";
import Proxy from "./proxy.js";

async function play() {
  const game = new Game();
  const proxy = new Proxy();

  try {
    Network();

    await game.start(5001);
    await proxy.connect(5001);

    await proxy.create();
    await proxy.join(3, "Human");

    let time = Date.now() + 1000;

    while (true) {
      if (Date.now() > time) {
        await proxy.trace();

        time = Date.now() + 1000;
      }

      await proxy.step();
    }
  } catch (error) {
    console.log(error);
  } finally {
    await game.stop();
  }
}

play();
