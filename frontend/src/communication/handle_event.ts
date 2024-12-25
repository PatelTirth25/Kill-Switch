import { addPlayer, addYou, removePlayer, removeYou, updatePlayer, updateYou } from "../gamestate/game";
import { mapInit, updateMap } from "../gamestate/map";
import { getId } from "../utils/id";
import { EventType } from "./event_types";

export const handleEvent = (event: string) => {
  const e = JSON.parse(event)

  switch (e.event_type as EventType) {
    case EventType.PLAYER_JOIN:
      if (e.data.id === getId()) {
        const u = e.data.players.find((element: any) =>
          element.id === getId()
        )
        addYou(u.id, u.x, u.y, u.weapon)
        mapInit(e.data.map)
        e.data.players.forEach((element: any) => {

          if (element.id !== getId()) {
            addPlayer(element.id, element.x, element.y, element.weapon)
          }

        });
      }
      else {
        e.data.players.forEach((element: any) => {
          if (element.id === e.data.id) {
            addPlayer(element.id, element.x, element.y, element.weapon)
          }
        })
      }
      break;

    case EventType.PLAYER_LEAVE:

      if (e.data.id === getId()) {
        removeYou()
      }
      else {
        removePlayer(e.data.id)
      }

      break;

    case EventType.PLAYER_MOVE:

      if (e.data.id === getId()) {
        updateMap(e.data.map)
        updateYou(e.data.x, e.data.y, e.data.weapon)
      }
      else {
        updatePlayer(e.data.id, e.data.x, e.data.y, e.data.weapon)
      }

      break;

    case EventType.REJECT:

      console.log("Rejected: ", e.error)
      break;

    default:
      console.log('Unknown event type: ' + e.event_type);
      break;
  }
}
