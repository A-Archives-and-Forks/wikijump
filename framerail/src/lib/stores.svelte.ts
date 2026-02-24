import { Layout, PagePane } from "./types"

class ErrorPopupState {
  current = $state<{ state: boolean; message: string | null; data: any | null }>({
    state: false,
    message: null,
    data: null
  })
}

class PageLayoutState {
  current = $state<Layout>(Layout.WIKIJUMP)
}

class PagePaneState {
  current = $state<PagePane>(PagePane.None)
}

export const errorPopupState = new ErrorPopupState()
export const pageLayoutState = new PageLayoutState()
export const pagePaneState = new PagePaneState()
