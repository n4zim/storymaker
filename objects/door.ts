import * as StoryMaker from "../core.ts"

export interface Object_Door {
  open: boolean
  locked: boolean
  keys: string[]
}

export default StoryMaker.Object<Object_Door>({
  name: () => ({
    one: { en: "Door", fr: "Porte" },
    many: { en: "Doors", fr: "Portes" },
  }),
  init: () => ({
    open: false,
    locked: false,
    keys: [],
  }),
  actions: (state, person) => ({
    open: {
      description: () => ({ en: "Open", fr: "Ouvrir" }),
      condition: () => {
        if(state.options.locked) {
          return { en: "This door is locked.", fr: "Cette porte est verrouillée." }
        }
        if(state.options.open) {
          return { en: "This door is already open.", fr: "Cette porte est déjà ouverte." }
        }
      },
      execute: () => state.options.open = true,
    },
    close: {
      description: () => ({ en: "Close", fr: "Fermer" }),
      condition: () => {
        if(!state.options.open) {
          return { en: "This door is already closed.", fr: "Cette porte est déjà fermée." }
        }
      },
      execute: () => state.options.open = false,
    },
    lock: {
      description: () => ({ en: "Lock", fr: "Verrouiller" }),
      condition: () => {
        if(state.options.locked) {
          return { en: "This door is already locked.", fr: "Cette porte est déjà verrouillée." }
        }
        if(state.options.keys.length === 0) {
          return { en: "This door cannot be locked.", fr: "Cette porte ne peut pas être verrouillée." }
        }
      },
      execute: () => state.options.locked = true,
    },
    unlock: {
      description: () => ({ en: "Unlock", fr: "Déverrouiller" }),
      condition: () => {
        if(!state.options.locked) {
          return { en: "This door is already unlocked.", fr: "Cette porte est déjà déverrouillée." }
        }
      },
      execute: () => state.options.locked = false,
    },
  }),
})
