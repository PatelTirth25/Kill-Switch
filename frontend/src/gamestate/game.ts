import { Player } from "./player";
import { You } from "./you";
import { WEAPON_VISIBILITY, VISIBILITY, LENGTH } from "../utils/config";
import { socket } from "../main";
import { EventType } from "../communication/event_types";
import { getId } from "../utils/id";
import { stopRoom } from "../utils/stoproom";

let players: Player[] = []
let you: You;

const checkVis = (x: number, y: number): boolean => {
  let vis;
  if (you.weapon) {
    vis = WEAPON_VISIBILITY * LENGTH
  }
  else {
    vis = VISIBILITY * LENGTH
  }
  // return true if x and y are within the visibility of you.sprite , around visibiliy
  return x >= you.sprite.x - vis && x <= you.sprite.x + vis && y >= you.sprite.y - vis && y <= you.sprite.y + vis
}

export const addPlayer = (id: string, x: number, y: number, weapon: boolean) => {

  players.push(new Player(id, x, y, weapon, checkVis(x, y)))
}

export const addYou = (id: string, x: number, y: number, weapon: boolean) => {
  you = new You(id, x, y, weapon)
}

export const removePlayer = async (id: string) => {
  players = players.filter((player) => {
    if (player.id === id) {
      player.remove()
      return false;
    }
    return true;
  })

  if (players.length === 0) {
    socket.send(EventType.PLAYER_LEAVE, { id: getId() })
    await stopRoom()
    window.location.href = "/win"
  }
}

export const removeYou = () => {
  you.remove()

  window.location.href = "/room"
}

export const updatePlayer = (id: string, x: number, y: number, weapon: boolean) => {
  you.update(you.sprite.x, you.sprite.y, you.weapon, true)
  players.forEach((player) => {
    if (player.id === id) {
      player.update(x, y, weapon, checkVis(x, y))
      return
    }
  })
}

export const updateYou = (x: number, y: number, weapon: boolean) => {
  you.update(x, y, weapon, true)
  players.forEach((player) => {
    player.update(player.sprite.x, player.sprite.y, player.weapon, checkVis(player.sprite.x, player.sprite.y))
  })
}
