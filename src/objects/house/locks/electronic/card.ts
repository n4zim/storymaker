import * as StoryMaker from "../../../../core.ts"

export default StoryMaker.Object({
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
