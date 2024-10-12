import { useEffect, useState } from "preact/hooks"
import { receive } from "../server"

export function Messages() {
  const [data, setData] = useState<{ title: string, content: string }[]>([])
  useEffect(() => {
    return receive<{ title: string, content: string }>("message", (message) => {
      setData((data) => [...data, message ])
    })
  }, [])
  return (
    <div style={{ flex: 1, overflowY: "auto", borderBottom: "3px solid white" }}>
      {data.map((message, index) => (
        <div key={index} style={{ border: "1px solid white", padding: 10, margin: 10 }}>
          <strong>{message.title}</strong>
          <p>{message.content}</p>
        </div>
      ))}
    </div>
  )
}
