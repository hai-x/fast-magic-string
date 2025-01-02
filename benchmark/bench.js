const { Bench } = require('tinybench')
const { withCodSpeed } = require('@codspeed/tinybench-plugin')
const MagicString = require('magic-string')
const { MagicString: FastMagicString } = require('fast-magic-string')

const BANNER = `/**
 * I'M BANNER
 */
`
const CODE = `  ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789  `

const suit = function (m) {
  m.overwrite(13, 16, BANNER)
  m.prepend(BANNER)
  m.prependLeft(3, BANNER)
  m.prependRight(3, BANNER)
  m.appendLeft(10, BANNER)
  m.appendRight(10, BANNER)
  m.append(BANNER)
  m.move(3, 5, 9)
  m.slice(3, 5)
  m.toString()
  m.indent('\t')
  m.trim()
  m.reset(3, 5)
  m.clone()
  m.replace('XYZ', '__REPLACED__')
  m.isEmpty()
  m.update(10, 13, '__REPLACED__')
  m.prepend(BANNER)
  m.generateMap()
  m.prepend(BANNER)
  m.generateDecodedMap()
}

const ms = new MagicString(CODE)
const fms = new FastMagicString(CODE)

const bench = withCodSpeed(new Bench({ name: 'Bench', time: 100 }))

;(async () => {
  if (!Boolean(process.env.CI))
    bench.add('magic-string', () => {
      suit(ms)
    })
  bench.add('fast-magic-string', () => {
    suit(fms)
  })
  await bench.run()
  console.log(bench.table())
})()
