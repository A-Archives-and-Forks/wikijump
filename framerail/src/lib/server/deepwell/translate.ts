import { client } from "$lib/server/deepwell"
import type { TranslateKeys, TranslatedKeys } from "$lib/types"

export async function translate(
  locales: string[],
  keys: TranslateKeys,
  stripKeys: string[] = []
): Promise<TranslatedKeys> {
  return client.request("translate", {
    locales,
    messages: keys,
    strip_message_keys: stripKeys
  })
}
