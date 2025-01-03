export const stopRoom = async () => {

  const roomId = localStorage.getItem("roomId")
  if (!roomId) {
    return;
  }
  localStorage.removeItem("roomId")
  localStorage.removeItem("ws_url")

  const r = await fetch((process.env.HTTP_URL || 'localhost:3010/') + 'room/stop', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ id: roomId }),
  })

  const data = await r.json()
  console.log("Data: ", data)

}
