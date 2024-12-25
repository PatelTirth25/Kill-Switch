import { Sprite } from "pixi.js";
import { LENGTH } from "../utils/config";
import { app } from "../main";

const map: Sprite[][] = []
let weapon: Sprite

export const mapInit = (m: number[][]) => {
  for (let i = 0; i < m.length; i++) {
    map[i] = []
    for (let j = 0; j < m[i].length; j++) {
      if (m[i][j] === 1) {
        const sprite = Sprite.from('violet')
        sprite.width = LENGTH
        sprite.height = LENGTH
        sprite.x = j * LENGTH
        sprite.y = i * LENGTH
        map[i][j] = sprite
        app.stage.addChild(sprite)
      }
      else if (m[i][j] === 2) {
        weapon = Sprite.from('weapon')
        weapon.width = LENGTH
        weapon.height = LENGTH
        weapon.x = j * LENGTH
        weapon.y = i * LENGTH
        app.stage.addChild(weapon)
      }
    }
  }
}

export const removeMap = () => {
  for (let i = 0; i < map.length; i++) {
    for (let j = 0; j < map[i].length; j++) {
      if (map[i][j]) {
        app.stage.removeChild(map[i][j])
      }
    }
  }
  app.stage.removeChild(weapon)
}

export const updateMap = (m: number[][]) => {
  removeMap()
  mapInit(m)
}
