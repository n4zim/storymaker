import { Map } from "./map"
import { Profile } from "./profile"
import { Inventory } from "./inventory"

export function HUD() {
  return (
    <div style={{ flexDirection: "row", flexWrap: "wrap", gap: 10, padding: 10 }}>
      <Profile/>
      <Map/>
      <Inventory/>
    </div>
  )
}
