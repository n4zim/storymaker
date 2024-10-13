import { useEffect, useState } from "preact/hooks"
import { receive, send } from "../server"

type Action = {
  id: string
  name: string
  target?: string
}

export function Actions() {
  const [data, setData] = useState<{
    actions: Action[]
    context?: any
  }>({ actions: [] })

  useEffect(() => {
    return receive<Action[]>("actions", (actions, context) => {
      setData({ actions, context })
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
      minHeight: 41.6,
    }}>
      {data.actions.map((action, index) => (
        <button
          key={index}
          style={{
            border: "1px solid black",
            padding: 10,
          }}
          onClick={() => {
            setData({ actions: [] })
            if(["use", "give", "sell", "throw" ].includes(action.id)) {
              if(typeof data.context !== "object") return
              send({
                type: action.id,
                id: data.context.object,
                target: action.target,
              })
            } else if(action.id === "cancel") {
              send({ type: "cancel" })
            } else {
              send({ type: "action", id: action.id, target: action.target })
            }
          }}
        >
          {action.name}
        </button>
      ))}
    </div>
  )
}
