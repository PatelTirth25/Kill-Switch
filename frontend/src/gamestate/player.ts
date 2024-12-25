import { Sprite } from "pixi.js";
import { app } from "../main";
import { LENGTH } from "../utils/config";

export class Player {
  public id: string
  public sprite: Sprite
  public weapon: boolean

  constructor(id: string, x: number, y: number, weapon: boolean, vis: boolean) {
    this.id = id;
    this.sprite = Sprite.from('bunny');
    this.sprite.height = LENGTH
    this.sprite.width = LENGTH
    this.sprite.x = x;
    this.sprite.y = y;
    this.weapon = weapon
    this.sprite.visible = vis
    app.stage.addChild(this.sprite)
  }

  update(x: number, y: number, weapon: boolean, vis: boolean) {
    this.sprite.x = x
    this.sprite.y = y
    this.weapon = weapon
    this.sprite.visible = vis
  }

  remove() {
    app.stage.removeChild(this.sprite)
  }

}
