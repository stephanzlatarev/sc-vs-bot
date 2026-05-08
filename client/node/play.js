import Client from "./client.js";
import Network from "./network.js";

async function play() {
  const client = new Client();

  try {
    Network();

    await client.connect();
    await client.createGame();
  } catch (error) {
    console.log(error);
  } finally {
    await client.disconnect();
  }
}

play();
