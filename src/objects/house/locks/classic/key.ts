import StoryMakerObject from "../../../../core/object.ts"

export default StoryMakerObject({
  name: () => ({
    one: {
      en: "Key",
      fr: "Clé",
    },
    many: {
      en: "Keys",
      fr: "Clés",
    },
  }),
})
