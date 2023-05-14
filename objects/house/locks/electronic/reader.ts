import * as StoryMaker from "../../../../core.ts"

export default StoryMaker.Object({
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
