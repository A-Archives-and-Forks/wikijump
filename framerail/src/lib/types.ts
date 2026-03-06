import type { Locales } from "../types"
export type Optional<T> = T | undefined
export type Nullable<T> = T | null
export type TranslateKeys = {
  [P in keyof Locales]?: Record<string, string | number>
}
export type TranslatedKeys = Partial<Locales>
export enum Layout {
  WIKIDOT = "wikidot",
  WIKIJUMP = "wikijump"
}
export enum PagePane {
  None = "none",
  File = "file",
  History = "history",
  Layout = "layout",
  Move = "move",
  Parent = "parent",
  Vote = "vote",
  Delete = "delete"
}
export enum UserType {
  Regular = "regular",
  System = "system",
  Site = "site",
  Bot = "bot"
}
