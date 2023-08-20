import StoryMakerObject from "../../core/object.ts"

export default StoryMakerObject({
  name: () => ({
    one: {
      en: "Generator",
      fr: "Groupe électrogène",
    },
    many: {
      en: "Generators",
      fr: "Groupes électrogènes",
    },
  }),
})
