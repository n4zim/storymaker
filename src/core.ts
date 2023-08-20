import objects from "./objects/index.ts"

type ObjectId = keyof typeof objects

type Text = { en: string, fr: string }
type NameByQuantity = { one: Text, many: Text }

export type State<Options> = {
  owner: Actor["id"]
  options: Options
  links: { [key: string]: ObjectId }
}

export function Object<Options = undefined>(options: {
  name: () => NameByQuantity
  actions?: (state: State<Options>, actor: Actor) => {
    [key: string]: {
      description: () => Text
      condition: () => Text | void
      execute: () => void
    }
  }
} & (Options extends undefined ? {} : {
  init: (options: Partial<Options>) => Options
})) {
  return {
    name: options.name,
    init: (state?: State<Options>) => {
      if(
        typeof state === "undefined"
        // @ts-ignore-next-line
        && typeof options.init !== "undefined"
      ) {
        // @ts-ignore-next-line
        state = options.init()
      }
    },
  }
}

export type Actor = {
  id: string
  has: (object: string) => boolean
  hasOneOf: (objects: string[]) => boolean
}
