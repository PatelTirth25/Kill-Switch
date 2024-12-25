import { Application, Assets, Sprite } from "pixi.js";

export const appInit = async () => {

  // Create a new application
  const app = new Application();

  // Initialize the application
  await app.init({ background: "#1099bb", height: 810, width: 1520 });

  // Append the application canvas to the document body
  document.body.appendChild(app.canvas);

  const assets = [
    { alias: 'bunny', src: "/assets/bunny.png" },
    { alias: 'violet', src: "/assets/violet.png" },
    { alias: 'weapon', src: "/assets/weapon.png" },
    { alias: 'space', src: "/assets/space.jpg" }
  ];
  await Assets.load(assets);

  const space = Sprite.from('space')
  space.width = app.screen.width
  space.height = app.screen.height
  app.stage.addChild(space)

  return app

}
