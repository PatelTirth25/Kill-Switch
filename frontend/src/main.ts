import { Application } from "pixi.js";
import { EventType } from "./communication/event_types";
import { SocketManager } from "./communication/ws";
import { appInit } from "./utils/application";
import { getId } from "./utils/id";

export let socket: SocketManager;
export let app: Application

(async () => {

  app = await appInit()
  socket = new SocketManager("ws://localhost:3001");
  socket.send(EventType.PLAYER_JOIN, { id: getId() })

})();

