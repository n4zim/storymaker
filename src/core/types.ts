import { ObjectId } from "../objects/mod.ts"

export type Text = { en: string, fr: string }

export type NameByQuantity = { one: Text, many: Text }

export type State<Options> = {
  owner: Actor["id"]
  options: Options
  links: { [key: string]: ObjectId }
}

export type Actor = {
  id: string
  has: (object: string) => boolean
  hasOneOf: (objects: string[]) => boolean
}
