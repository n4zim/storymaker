
watchChanges("objects")

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

function handle(type: string) {
  let output = ""
  const values: string[] = []
  for(const obj of recursiveRead(type)) {
    if(obj === "index") continue
    const value = obj.replace(/\//g, "_")
    output += `import ${value} from "./${obj}.ts"\n`
    values.push(value)
  }
  output += `\nexport default {\n  ${values.join(",\n  ")}\n}\n`
  Deno.writeTextFileSync(`${type}/index.ts`, output)
}

async function watchChanges(type: string) {
  handle(type)
  for await(const event of Deno.watchFs(type)) {
    if(
      event.kind === "modify"
      && !event.paths[0].endsWith("index.ts")
    ) {
      handle(type)
    }
  }
}
