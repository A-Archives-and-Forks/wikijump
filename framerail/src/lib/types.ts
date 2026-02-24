export type Optional<T> = T | undefined
export type Nullable<T> = T | null
export type TranslateKeys = Record<string, Record<string, string | number>>
export type TranslatedKeys = Record<string, Optional<string>>
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
