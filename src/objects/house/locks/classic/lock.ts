import * as StoryMaker from "../../../../core.ts"

export default StoryMaker.Object({
  name: () => ({
    one: {
      en: "Lock",
      fr: "Serrure",
    },
    many: {
      en: "Locks",
      fr: "Serrures",
    },
  }),
})
