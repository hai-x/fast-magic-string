/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface JsIndentOptions {
  exclude?: Array<Array<number>>
  indentStart?: boolean
}
export interface JsMagicStringOptions {
  filename?: string
  indentExclusionRanges?: Array<number> | Array<Array<number>>
  ignoreList?: boolean
}
export interface JsGenerateMapOptions {
  file?: string
  source?: string
  sourceRoot?: string
  includeContent?: boolean
  hires?: boolean
}
export interface JsOverwriteOptions {
  contentOnly?: boolean
  storeName?: boolean
  overwrite?: boolean
}
export interface FmsRegex {
  global?: boolean
  rule: string
}
export interface JsSourceMap {
  version: number
  file?: string
  sourceRoot?: string
  sources: Array<string>
  sourcesContent?: Array<string>
  names: Array<string>
  mappings: string
  xGoogleIgnoreList?: Array<number>
}
export interface JsDecodedMap {
  version: number
  file?: string
  sourceRoot?: string
  sources: Array<string>
  sourcesContent?: Array<string>
  names: Array<string>
  mappings: Array<Array<Array<number>>>
  xGoogleIgnoreList?: Array<number>
}
export type JsMagicString = MagicString
export declare class MagicString {
  indentExclusionRanges?: Array<number> | Array<Array<number>>
  constructor(str: string, options?: JsMagicStringOptions | undefined | null)
  addSourcemapLocation(index: number): this
  append(input: string): this
  appendLeft(index: number, input: string): this
  clone(): MagicString
  generateMap(options?: JsGenerateMapOptions | undefined | null): JsSourceMap
  generateDecodedMap(options?: JsGenerateMapOptions | undefined | null): JsDecodedMap
  indent(indentStr?: string | undefined | null, options?: JsIndentOptions | undefined | null): this
  insert(): void
  insertLeft(index: number, input: string): this
  appendRight(index: number, input: string): this
  prepend(input: string): this
  prependLeft(index: number, input: string): this
  prependRight(index: number, input: string): this
  insertRight(index: number, input: string): this
  trim(charType?: string | undefined | null): this
  trimLines(): this
  trimStart(charType?: string | undefined | null): this
  trimEnd(charType?: string | undefined | null): this
  move(start: number, end: number, index: number): this
  remove(start: number, end: number): this
  overwrite(start: number, end: number, content: string, options?: JsOverwriteOptions | undefined | null): this
  update(start: number, end: number, content: string, options?: JsOverwriteOptions | undefined | null): this
  isEmpty(): boolean
  toString(): string
  hasChanged(): boolean
  snip(start: number, end: number): MagicString
  slice(start?: number | undefined | null, end?: number | undefined | null): string
  reset(start: number, end: number): this
  replace(pattern: RegExp | string, replacement: string | (any)): this
  replaceAll(pattern: RegExp | string, replacement: string | (any)): this
}
