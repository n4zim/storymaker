import StoryMakerObject from "../../../../core/object.ts"

export default StoryMakerObject({
  name: () => ({
    one: {
      en: "Access card",
      fr: "Carte d'accès",
    },
    many: {
      en: "Access cards",
      fr: "Cartes d'accès",
    },
  }),
})
