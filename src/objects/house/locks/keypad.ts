import * as StoryMaker from "../../../core.ts"

export default StoryMaker.Object({
  name: () => ({
    one: {
      en: "Keypad lock",
      fr: "Clavier à code",
    },
    many: {
      en: "Keypad locks",
      fr: "Claviers à code",
    },
  }),
})
