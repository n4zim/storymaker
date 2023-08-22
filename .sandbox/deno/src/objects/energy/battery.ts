import StoryMakerObject from "../../core/object.ts"

export default StoryMakerObject({
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
