import express from "express";
import bot from "./bot.js";
import game from "./game.js";
import lobby from "./lobby.js";
import clientStatus from "./network.js";

const PORT = 8000;
const app = express();

let progressing = false;

app.get("/", (_, response) => {
  response.sendFile("/node/index.html");
});

app.get("/exe/sc-vs-bot.exe", (_, response) => {
  response.sendFile("/exe/sc-vs-bot.exe");
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

app.listen(PORT, () => {
  console.log("Listening on:", PORT);
});

lobby();
