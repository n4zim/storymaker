
type Text = { en: string, fr: string }
type NameByQuantity = { one: Text, many: Text }

export type State<Options> = {
  owner: Person
  options: Options
}

export function Object<Options = undefined>(options: {
  name: () => NameByQuantity
  actions?: (state: State<Options>, person: Person) => {
    [key: string]: {
      description: () => Text
      condition: () => Text | void
      execute: () => void
    }
  }
} & (Options extends undefined ? {} : {
  init: () => Options
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

export type Person = string
