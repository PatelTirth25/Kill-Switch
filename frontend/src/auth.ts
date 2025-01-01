import { isLogin, Login, SignUp } from "./utils/auth"

isLogin()

document.getElementById("login")?.addEventListener("click", () => {
  const username = (document.getElementById("l-username") as HTMLInputElement)?.value
  const password = (document.getElementById("l-password") as HTMLInputElement)?.value
  if (username !== "" && password !== "") {
    Login(username as any, password as any)
  }
})


document.getElementById("signup")?.addEventListener("click", () => {
  const username = (document.getElementById("s-username") as HTMLInputElement)?.value
  const password = (document.getElementById("s-password") as HTMLInputElement)?.value
  const confirm_password = (document.getElementById("s-confirm_password") as HTMLInputElement)?.value

  if (username !== "" && password !== "" && confirm_password !== "" && password === confirm_password) {
    SignUp(username as any, password as any, confirm_password as any)
  }
})
