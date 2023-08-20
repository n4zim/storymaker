import StoryMakerObject from "../../../core/object.ts"

export default StoryMakerObject({
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
