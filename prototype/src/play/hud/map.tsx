import { useEffect, useState } from "preact/hooks"
import { Module } from "./module"
import { receive } from "../../server"

export function Map() {
  const [data, setData] = useState<string[][]>([])
  const [position, setPosition] = useState<{ x: number, y: number } | undefined>()

  useEffect(() => {
    const cancelData = receive<string[][]>("map", setData)
    const cancelPosition = receive<{ x: number, y: number }>("position", setPosition)
    return () => {
      cancelData()
      cancelPosition()
    }
  }, [])

  if(!data || data.length === 0) return null

  return (
    <Module>
      <strong>Map</strong>
      <table cellSpacing={0} cellPadding={0} style={{ width: "fit-content" }}>
        <tbody>
          {data.map((row, rowIndex) => (
            <tr key={rowIndex}>
              {row.map((cell, cellIndex) => {
                const atPosition = typeof position !== "undefined"
                  && position.x === cellIndex
                  && position.y === rowIndex
                return <td
                  key={cellIndex}
                  style={{
                    padding: 1,
                    background: atPosition ? "red" : "none",
                  }}
                >
                  {cell}
                </td>
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </Module>
  )
}
