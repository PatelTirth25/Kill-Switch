import { isLogin } from "./utils/auth"

async function main() {
  if (!isLogin()) {
    window.location.href = "/auth"
  };
}

document.getElementById("create-room")?.addEventListener("click", async () => {
  const roomId = (document.getElementById("room-id") as HTMLInputElement)?.value

  if (roomId !== "") {
    if (localStorage.getItem("roomId")) {
      localStorage.removeItem("roomId")
    }
    localStorage.setItem("roomId", roomId)

    const r = await fetch((process.env.HTTP_URL || 'localhost:3010/') + 'room', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ id: roomId }),
    })

    const data = await r.json()
    console.log("Data: ", data)

    if (data.success) {
      localStorage.setItem("ws_url", data.message)
      window.location.href = "/"
    }
    else {
      alert(data.message)
    }
  }
})

main()
