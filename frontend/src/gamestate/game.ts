import { Player } from "./player";
import { You } from "./you";
import { WEAPON_VISIBILITY, VISIBILITY, LENGTH } from "../utils/config";

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

export const removePlayer = (id: string) => {
  players = players.filter((player) => {
    if (player.id === id) {
      player.remove()
      return false;
    }
    return true;
  })
}

export const removeYou = () => {
  you.remove()
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
