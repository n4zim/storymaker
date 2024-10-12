import { useState } from "preact/hooks"
import { send } from "./server"

export function Start(props: {
  onStart: () => void
}) {
  const [name, setName] = useState(() => {
    const param = new URLSearchParams(location.search).get("name")
    if(param) return param
    return ""
  })
  const [seed, setSeed] = useState(Math.floor(Math.random() * 1000000))
  return (
    <>
      <h1>StoryMaker</h1>

      <form
        style={{ display: "flex", flexDirection: "column", gap: 20, width: "80%" }}
        onSubmit={(e) => {
          e.preventDefault()
          send({ type: "start", name, seed })
          props.onStart()
        }}
      >
        <label style={{ display: "flex", flexDirection: "column" }}>
          Name
          <input
            type="text"
            value={name}
            onInput={(e) => setName(e.currentTarget.value)}
          />
        </label>

        <label style={{ display: "flex", flexDirection: "column" }}>
          Seed
          <input
            type="number"
            value={seed}
            onInput={(e) => setSeed(parseInt(e.currentTarget.value))}
          />
        </label>

        <button type="submit" disabled={name === "" || !seed} style={{marginTop: 20 }}>
          Play
        </button>
      </form>
    </>
  )
}
