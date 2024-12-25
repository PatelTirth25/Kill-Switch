import { EventType } from "./event_types";
import { handleEvent } from "./handle_event";

export class SocketManager {
  private socket: WebSocket;
  private openPromise: Promise<void>;

  constructor(url: string) {
    this.socket = new WebSocket(url);
    this.openPromise = new Promise((resolve, reject) => {
      this.socket.onopen = () => {
        console.log("Socket opened");
        resolve();
      };
      this.socket.onerror = (event) => {
        console.log("Socket error");
        reject(event);
      };
    });
    this.socket.onclose = () => {
      console.log("Socket closed");
    }
    this.socket.onmessage = (event) => {
      handleEvent(event.data);
    }
  }

  public async send(type: EventType, message: any) {
    await this.openPromise;
    this.socket.send(JSON.stringify({ event_type: type, data: message }));
  }
}
