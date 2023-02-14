import * as StoryMaker from "../core.ts"

export default StoryMaker.Object({
  name: () => ({
    one: { en: "Key", fr: "Clé" },
    many: { en: "Keys", fr: "Clés" },
  }),
})
