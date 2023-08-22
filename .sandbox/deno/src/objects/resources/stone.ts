import StoryMakerObject from "../../core/object.ts"

export default StoryMakerObject({
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
