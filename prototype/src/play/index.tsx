import { Messages } from "./messages"
import { HUD } from "./hud"
import { Actions } from "./actions"
import { useEffect } from "preact/hooks"
import { send } from "../server"

export function Play() {
  useEffect(() => {
    send({ type: "ready" })
  }, [])
  return (
    <div style={{ flex: 1, height: "100%", width: "100%" }}>
      <h2 style={{ textAlign: "center", margin: 0 }}>
        StoryMaker
      </h2>
      <Messages />
      <Actions/>
      <HUD/>
    </div>
  )
}
