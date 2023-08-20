
watchChanges("src/objects", "Object")

function recursiveRead(
  path: string,
  output: string[] = [],
  prefix: string[] = [],
) {
  for(const file of Deno.readDirSync(path)) {
    if(file.isDirectory) {
      recursiveRead(`${path}/${file.name}`, output, [
        ...prefix,
        file.name,
      ])
    } else {
      output.push([
        ...prefix,
        file.name.slice(0, -3),
      ].join("/"))
    }
  }
  return output
}

function handle(type: string, name: string) {
  let output = ""
  const values: string[] = []
  for(const obj of recursiveRead(type)) {
    if(obj.endsWith("mod")) continue
    const value = obj.replace(/\//g, "_")
    output += `import ${value} from "./${obj}.ts"\n`
    values.push(value)
  }
  output += `\nconst items = {\n  ${values.sort().join(",\n  ")}\n}\n\n`
    + `export default items\n\n`
    + `export type ${name}Id = keyof typeof items\n`
  Deno.writeTextFileSync(`${type}/mod.ts`, output)
}

async function watchChanges(type: string, name: string) {
  handle(type, name)
  for await(const event of Deno.watchFs(type)) {
    if(
      event.kind === "modify"
      && !event.paths[0].endsWith("mod.ts")
    ) {
      handle(type, name)
    }
  }
}
