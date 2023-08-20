import * as StoryMaker from "../../core.ts"

export default StoryMaker.Object({
  name: () => ({
    one: {
      en: "Solar panel",
      fr: "Panneau solaire",
    },
    many: {
      en: "Solar panels",
      fr: "Panneaux solaires",
    },
  }),
})
