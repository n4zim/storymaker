import { useState } from "preact/hooks"
import { Play } from "./play"
import { Start } from "./start"

export function App() {
  const [started, setStarted] = useState<0 | 1 | 2>(0)
  if(!started) {
    return <Start
      onStart={() => setStarted(1)}
      onReady={() => setStarted(2)}
    />
  }
  if(started === 1) return null
  return <Play/>
}
