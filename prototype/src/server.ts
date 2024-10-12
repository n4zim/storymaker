
const ws = new WebSocket("ws://localhost:8000")

export function send(message: any) {
  if(ws.readyState === ws.OPEN) {
    console.log("Sending", message)
    ws.send(JSON.stringify(message))
  } else {
    console.error("WebSocket not open")
  }
}

let lastId = 0
const callbacks: { [type: string]: { [id: number]: (message: any) => void } } = {}

ws.onmessage = (event) => {
  const message = JSON.parse(event.data)
  console.log("Received", message, callbacks)
  if(callbacks[message.type]) {
    for(const id in callbacks[message.type]) {
      callbacks[message.type][id](message.data)
    }
  }
}

export function receive<T>(type: string, callback: (message: T) => void) {
  const id = lastId++
  if(!callbacks[type]) callbacks[type] = {}
  callbacks[type][id] = callback
  return () => {
    delete callbacks[type][id]
  }
}
