
type Text = { en: string, fr: string }
type NameByQuantity = { one: Text, many: Text }

type State<S> = S & {
  owner: Person
}

export function Object<S = undefined>(options: {
  name: () => NameByQuantity
  actions?: (state: State<S>, person: Person) => {
    [key: string]: {
      description: () => Text
      condition: () => Text | void
      execute: () => void
    }
  }
} & (S extends undefined ? {} : {
  init: () => S
})) {
  return {
    name: options.name,
    init: (state?: State<S>) => {
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
