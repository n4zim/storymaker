import { Actor, NameByQuantity, State, Text } from "./types.ts"

export default function StoryMakerObject<Options = undefined>(options: {
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
