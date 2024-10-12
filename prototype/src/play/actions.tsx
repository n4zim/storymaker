import { useEffect, useState } from "preact/hooks"
import { receive, send } from "../server"

export function Actions() {
  const [data, setData] = useState<{ id: string, name: string }[]>([])
  useEffect(() => {
    return receive<{ id: string, name: string }[]>("actions", (message) => {
      setData(message)
    })
  }, [])
  return (
    <div style={{
      flexDirection: "row",
      flexWrap: "wrap",
      borderBottom: "3px solid white",
      gap: 10,
      padding: 10,
      justifyContent: "center",
    }}>
      {data.map((action, index) => (
        <button
          key={index}
          style={{
            border: "1px solid black",
            padding: 10,
          }}
          onClick={() => {
            setData([])
            send({ type: "action", id: action.id })
          }}
        >
          {action.name}
        </button>
      ))}
    </div>
  )
}
