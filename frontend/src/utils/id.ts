import { v4 } from "uuid";

let id: string;

const generateId = () => {
  if (!localStorage.getItem("id")) {
    id = v4();
    localStorage.setItem("id", id);
  } else {
    id = localStorage.getItem("id") as string;
  }
  return id;
}

generateId()

export const getId = () => id
