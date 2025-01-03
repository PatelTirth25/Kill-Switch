import { Application } from "pixi.js";
import { EventType } from "./communication/event_types";
import { SocketManager } from "./communication/ws";
import { appInit } from "./utils/application";
import { getId } from "./utils/id";
import { isLogin } from "./utils/auth";

export let socket: SocketManager;
export let app: Application

(async () => {

  if (!isLogin()) {
    window.location.href = "/auth"
  }
  if (!localStorage.getItem("roomId")) {
    window.location.href = "/room"
  }
  if (!localStorage.getItem("ws_url")) {
    window.location.href = "/room"
  }

  app = await appInit()
  console.log('URL: ', localStorage.getItem("ws_url"))
  socket = new SocketManager(localStorage.getItem("ws_url") || "ws://localhost:3000");
  socket.send(EventType.PLAYER_JOIN, { id: getId() })

  // on window close or leaving page send socket player_leave
  window.addEventListener("beforeunload", () => {
    socket.send(EventType.PLAYER_LEAVE, { id: getId() })
  })

})();

