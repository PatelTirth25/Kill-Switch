import { EventType } from "../communication/event_types";
import { socket } from "../main";
import { LENGTH, SPEED } from "../utils/config";
import { Player } from "./player";

export class You extends Player {

  constructor(id: string, x: number, y: number, weapon: boolean) {
    super(id, x, y, weapon, true)

    const max_x = 42 * LENGTH
    const max_y = 21 * LENGTH

    document.addEventListener("keydown", (event) => {
      if (event.key === "w" && this.sprite.y - SPEED >= 0) {
        socket.send(EventType.PLAYER_MOVE, { id: this.id, ox: this.sprite.x, oy: this.sprite.y, nx: this.sprite.x, ny: this.sprite.y - SPEED })
      }
      else if (event.key === "w") {
        socket.send(EventType.PLAYER_MOVE, { id: this.id, ox: this.sprite.x, oy: this.sprite.y, nx: this.sprite.x, ny: 0 })
      }
      if (event.key === "s" && this.sprite.y + SPEED <= max_y) {
        socket.send(EventType.PLAYER_MOVE, { id: this.id, ox: this.sprite.x, oy: this.sprite.y, nx: this.sprite.x, ny: this.sprite.y + SPEED })
      }
      else if (event.key === "s") {
        socket.send(EventType.PLAYER_MOVE, { id: this.id, ox: this.sprite.x, oy: this.sprite.y, nx: this.sprite.x, ny: max_y })
      }
      if (event.key === "a" && this.sprite.x - SPEED >= 0) {
        socket.send(EventType.PLAYER_MOVE, { id: this.id, ox: this.sprite.x, oy: this.sprite.y, nx: this.sprite.x - SPEED, ny: this.sprite.y })
      }
      else if (event.key === "a") {
        socket.send(EventType.PLAYER_MOVE, { id: this.id, ox: this.sprite.x, oy: this.sprite.y, nx: 0, ny: this.sprite.y })
      }
      if (event.key === "d" && this.sprite.x + SPEED <= max_x) {
        socket.send(EventType.PLAYER_MOVE, { id: this.id, ox: this.sprite.x, oy: this.sprite.y, nx: this.sprite.x + SPEED, ny: this.sprite.y })
      }
      else if (event.key === "d") {
        socket.send(EventType.PLAYER_MOVE, { id: this.id, ox: this.sprite.x, oy: this.sprite.y, nx: max_x, ny: this.sprite.y })
      }
    })

  }

}
