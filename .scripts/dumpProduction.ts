import recipes from "https://deno.land/x/storymaker/mod.ts"

console.log("OBJECTS:")
for (const [name, recipe] of Object.entries(recipes.objects)) {
  console.log(`  ${name}: ${recipe.name().one.en}`)
}
