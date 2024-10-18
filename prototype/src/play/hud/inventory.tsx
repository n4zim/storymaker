import { useEffect, useState } from "preact/hooks"
import { Module } from "./module"
import { receive, send } from "../../server"

type Item = {
  id: string
  name: string
  usable?: boolean
  giveable?: boolean
  sellable?: boolean
  throwable?: boolean
}

export function Inventory() {
  const [data, setData] = useState<Item[]>([])
  useEffect(() => receive<Item[]>("inventory", setData), [])
  if(!data || data.length === 0) return null
  return (
    <Module>
      <strong>Inventory</strong>
      <div style={{ overflow: "auto", maxHeight: 200 }}>
        <table cellSpacing={5} cellPadding={0} style={{ width: "fit-content" }}>
          <tbody>
            {data.map((item, index) => (
              <tr key={index}>
                <td style={{ padding: 0 }}>
                  {item.name}
                </td>
                <td>
                  <Action enabled={item.usable} name="Use" onClick={() => send({ type: "action", id: "use", item: item.id })} />
                </td>
                <td>
                  <Action enabled={item.giveable} name="Give" onClick={() => send({ type: "action", id: "give", item: item.id })} />
                </td>
                <td>
                  <Action enabled={item.sellable} name="Sell" onClick={() => send({ type: "action", id: "sell", item: item.id })} />
                </td>
                <td>
                  <Action enabled={item.throwable} name="Throw" onClick={() => send({ type: "action", id: "throw", item: item.id })} />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </Module>
  )
}

function Action(props: { enabled?: boolean, name: string, onClick: () => void }) {
  if(!props.enabled) return null
  return (
    <button
      style={{
        padding: "1px 8px",
      }}
      onClick={props.onClick}
    >
      {props.name}
    </button>
  )
}