export const isLogin = () => {

  if (localStorage.getItem("token")) {
    return true
  }

  return false
}

export const Login = async (username: string, password: string) => {
  console.log("Login: ", username, password)


  const r = await fetch((process.env.HTTP_URL || 'localhost:3010/') + 'login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username, password }),
  })

  const data = await r.json()
  console.log("Data: ", data)

  if (data.success) {
    localStorage.setItem("token", data.id)
    window.location.href = "/room"
  }
  else {
    alert("Login failed!")
  }
}


export const SignUp = async (username: string, password: string, confirm_password: string) => {
  console.log("Login: ", username, password, confirm_password)

  const r = await fetch((process.env.HTTP_URL || 'localhost:3010/') + 'signup', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username, password, confirm_password }),
  })

  const data = await r.json()
  console.log("Data: ", data)

  alert(data.message)
}
