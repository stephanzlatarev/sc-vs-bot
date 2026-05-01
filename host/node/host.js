import express from "express";
import bot from "./bot.js";
import game from "./game.js";
import clientStatus from "./network.js";

const PORT = 8000;
const app = express();

let progressing = false;

app.get("/", (_, response) => {
  response.sendFile("/node/index.html");
});

app.get("/maps/LeyLinesAIE_v3.SC2Map", (_, response) => {
  response.sendFile("/StarCraftII/Maps/LeyLinesAIE_v3.SC2Map");
});

app.get("/status", (_, response) => {
  response.json({
    progressing,
    bot: bot.status,
    client: clientStatus,
    game: game.status,
  });
});

app.post("/start", async (_, response) => {
  try {
    progressing = true;

    await game.start();

    bot.start();

    response.json({ error: 0 });
  } catch (error) {
    console.log(error);
    response.json({ error: 1 });
  } finally {
    progressing = false;
  }
});

app.listen(PORT, () => {
  console.log("Listening on:", PORT);
});
