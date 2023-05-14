import * as StoryMaker from "../../core.ts"

export default StoryMaker.Object({
  name: () => ({
    one: {
      en: "Battery",
      fr: "Pile électrique",
    },
    many: {
      en: "Batteries",
      fr: "Piles électriques",
    },
  }),
})
