import StoryMakerObject from "../../../../core/object.ts"

export default StoryMakerObject({
  name: () => ({
    one: {
      en: "Access card reader",
      fr: "Lecteur de cartes d'accès",
    },
    many: {
      en: "Access card readers",
      fr: "Lecteurs de cartes d'accès",
    },
  }),
})
