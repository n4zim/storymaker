import * as StoryMaker from "../core.ts"

export enum Object_Door_Type {
  CLASSIC,
  MECHANICAL,
  ELECTRONIC,
}

export interface Object_Door_Options {
  type: Object_Door_Type
  open: boolean
  locked: boolean
  keys: StoryMaker.ObjectId[]
  security: number
}

export default StoryMaker.Object<Object_Door_Options>({

  name: () => ({
    one: {
      en: "Door",
      fr: "Porte",
    },
    many: {
      en: "Doors",
      fr: "Portes",
    },
  }),

  init: options => ({
    open: true,
    locked: false,
    keys: [],
    type: options.type ?? Object_Door_Type.CLASSIC,
    security: options.security ?? 0,
  }),

  actions: (state, actor) => ({
    open: {
      description: () => ({
        en: "Open",
        fr: "Ouvrir",
      }),
      condition: () => {
        if(state.options.open) {
          return {
            en: "This door is already open",
            fr: "Cette porte est déjà ouverte",
          }
        }
        if(state.options.locked) {
          return {
            en: "This door is locked",
            fr: "Cette porte est verrouillée",
          }
        }
      },
      execute: () => state.options.open = true,
    },

    close: {
      description: () => ({
        en: "Close",
        fr: "Fermer",
      }),
      condition: () => {
        if(!state.options.open) {
          return {
            en: "This door is already closed",
            fr: "Cette porte est déjà fermée",
          }
        }
      },
      execute: () => state.options.open = false,
    },

    lock: {
      description: () => ({
        en: "Lock",
        fr: "Verrouiller",
      }),
      condition: () => {
        if(state.options.locked) {
          return {
            en: "This door is already locked",
            fr: "Cette porte est déjà verrouillée",
          }
        }
        if(state.options.open) {
          return {
            en: "This door is open",
            fr: "Cette porte est ouverte",
          }
        }
        if(state.options.keys.length === 0) {
          return {
            en: "This door does not have yet any key to lock it",
            fr: "Cette porte ne possède pas encore de clé pour la verrouiller",
          }
        }
        if(!actor.hasOneOf(state.options.keys)) {
          return {
            en: "You do not have a key to lock this door",
            fr: "Vous ne possédez pas de clé pour verrouiller cette porte",
          }
        }
      },
      execute: () => state.options.locked = true,
    },

    unlock: {
      description: () => ({
        en: "Unlock",
        fr: "Déverrouiller",
      }),
      condition: () => {
        if(!state.options.locked) {
          return {
            en: "This door is already unlocked",
            fr: "Cette porte est déjà déverrouillée",
          }
        }
        if(!actor.hasOneOf(state.options.keys)) {
          return {
            en: "You do not have a key to unlock this door",
            fr: "Vous ne possédez pas de clé pour déverrouiller cette porte",
          }
        }
      },
      execute: () => state.options.locked = false,
    },
  }),

})
