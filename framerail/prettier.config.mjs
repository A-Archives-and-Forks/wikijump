/** @type {import("prettier").Config} */
const config = {
  printWidth: 90,
  semi: false,
  singleQuote: false,
  quoteProps: "preserve",
  trailingComma: "none",
  bracketSpacing: true,
  arrowParens: "always",
  endOfLine: "auto",
  plugins: [
    "prettier-plugin-organize-imports",
    "prettier-plugin-jsdoc",
    "prettier-plugin-svelte"
  ],
  jsdocPrintWidth: 75,
  overrides: [
    {
      files: "*.svelte",
      options: {
        parser: "svelte"
      }
    }
  ]
}

export default config
