import * as StoryMaker from "../../core.ts"

export default StoryMaker.Object({
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
