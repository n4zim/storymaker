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
        if(state.owner !== person) {
          return { en: "This door is not yours.", fr: "Cette porte ne vous appartient pas." }
        }
        if(state.open) {
          return { en: "This door is already open.", fr: "Cette porte est déjà ouverte." }
        }
      },
      execute: () => state.open = true,
    },
    close: {
      description: () => ({ en: "Close", fr: "Fermer" }),
      condition: () => {
        if(state.owner !== person) {
          return { en: "This door is not yours.", fr: "Cette porte ne vous appartient pas." }
        }
        if(!state.open) {
          return { en: "This door is already closed.", fr: "Cette porte est déjà fermée." }
        }
      },
      execute: () => state.open = false,
    },
  }),
})
