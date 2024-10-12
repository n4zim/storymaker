
let ws: WebSocket | undefined

let lastId = 0
const callbacks: {
  [type: string]: { [id: number]: (message: any, context?: any) => void }
} = {}

connect()

export function send(message: any) {
  if(
    typeof ws !== "undefined" &&
    ws.readyState === ws.OPEN
  ) {
    //console.log("Sending", message)
    ws.send(JSON.stringify(message))
  } else {
    alert("Disconnected from server, please refresh the page")
  }
}

export function receive<T>(type: string, callback: (message: T, context?: any) => void) {
  const id = lastId++
  if(!callbacks[type]) callbacks[type] = {}
  callbacks[type][id] = callback
  return () => {
    delete callbacks[type][id]
  }
}

function connect () {
  ws = new WebSocket("ws://localhost:8000")
  ws.onmessage = (event) => {
    const message = JSON.parse(event.data)
    //console.log("Received", message)
    if(callbacks[message.type]) {
      for(const id in callbacks[message.type]) {
        callbacks[message.type][id](message.data, message.context)
      }
    }
  }
}
