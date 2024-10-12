import { Profile } from "./profile"

export function HUD() {
  return (
    <div style={{ flexDirection: "row", flexWrap: "wrap", gap: 10, padding: 10 }}>
      <Profile/>
    </div>
  )
}
