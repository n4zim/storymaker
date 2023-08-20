import energy_generator from "./energy/generator.ts"
import energy_solarPanel from "./energy/solarPanel.ts"
import energy_battery from "./energy/battery.ts"
import chemicals_gasoline from "./chemicals/gasoline.ts"
import resources_wood from "./resources/wood.ts"
import resources_water from "./resources/water.ts"
import resources_stone from "./resources/stone.ts"
import medical_bandage from "./medical/bandage.ts"
import house_locks_keypad from "./house/locks/keypad.ts"
import house_locks_electronic_reader from "./house/locks/electronic/reader.ts"
import house_locks_electronic_card from "./house/locks/electronic/card.ts"
import house_locks_classic_key from "./house/locks/classic/key.ts"
import house_locks_classic_lock from "./house/locks/classic/lock.ts"
import house_door from "./house/door.ts"

export default {
  chemicals_gasoline,
  energy_battery,
  energy_generator,
  energy_solarPanel,
  house_door,
  house_locks_classic_key,
  house_locks_classic_lock,
  house_locks_electronic_card,
  house_locks_electronic_reader,
  house_locks_keypad,
  medical_bandage,
  resources_stone,
  resources_water,
  resources_wood
}
