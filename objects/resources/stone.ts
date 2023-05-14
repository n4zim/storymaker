import * as StoryMaker from "../../core.ts"

export default StoryMaker.Object({
  name: () => ({
    one: {
      en: "Stone",
      fr: "Pierre",
    },
    many: {
      en: "Stones",
      fr: "Pierres",
    },
  }),
})
