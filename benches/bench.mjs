import Benchmark from 'benchmark'
import { withCodSpeed } from '@codspeed/benchmark.js-plugin'
import { MagicString } from 'fast-magic-string'

const BANNER = `/**
 * I'M BANNER
 */
`

const CODE = `  ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789  `

const m = new MagicString(CODE)

let fns = {
  overwrite: () => {
    m.overwrite(13, 16, BANNER)
  },
  prepend: () => {
    m.prepend(BANNER)
  },
  prependLeft: () => {
    m.prependLeft(3, BANNER)
  },
  prependRight: () => {
    m.prependRight(3, BANNER)
  },
  append: () => {
    m.append(BANNER)
  },
  appendLeft: () => {
    m.appendLeft(10, BANNER)
  },
  appendRight: () => {
    m.appendRight(10, BANNER)
  },
  move: () => {
    m.move(3, 5, 9)
  },
  slice: () => {
    m.slice(3, 5)
  },
  toString: () => {
    m.toString()
  },
  indent: () => {
    m.indent('\t')
  },
  trim: () => {
    m.trim()
  },
  reset: () => {
    m.reset(3, 5)
  },
  clone: () => {
    m.clone()
  },
  replace: () => {
    m.replace('XYZ', '__REPLACED__')
  },
  isEmpty: () => {
    m.isEmpty()
  },
  update: () => {
    m.update(10, 13, '__REPLACED__')
  },
  generateMap: () => {
    m.prepend(BANNER)
    m.generateMap()
  },
  generateDecodedMap: () => {
    m.prepend(BANNER)
    m.generateDecodedMap()
  }
}

const suite = withCodSpeed(new Benchmark.Suite())

for (const [k, fn] of Object.entries(fns)) {
  suite.add(`MagicString # ${k}`, fn)
}

suite
  .on('cycle', function (event) {
    console.log(String(event.target))
  })

  .on('error', e => {
    console.error(e.target.error)
  })
  .run({ async: true })
