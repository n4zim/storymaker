import { useEffect, useState } from "preact/hooks"
import { Module } from "./module"
import { receive } from "../../server"

export function Profile() {
  const [data, setData] = useState<{ name: string, seed: number } | undefined>()

  useEffect(() => {
    return receive<{ name: string, seed: number }>("profile", (message) => {
      setData(message)
    })
  }, [])

  if(!data) return null

  return (
    <Module>
      <div>
        Name: {data.name}
        <br />
        Seed: {data.seed}
      </div>
    </Module>
  )
}
