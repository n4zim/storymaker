import { useState } from "preact/hooks"
import { Play } from "./play"
import { Start } from "./start"

export function App() {
  const [started, setStarted] = useState(false)
  if(!started) return <Start onStart={() => setStarted(true)} />
  return <Play/>
}
