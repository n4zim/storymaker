import StoryMakerObject from "../../core/object.ts"

export default StoryMakerObject({
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
