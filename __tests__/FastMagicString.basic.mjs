import { MagicString as RustMagicString } from 'fast-magic-string'
import MagicString from 'magic-string'
import { SourceMapConsumer } from 'source-map-js'

const validate = handle => {
  const res = []
  ;[RustMagicString, MagicString].map(Cons => {
    res.push(handle(Cons))
  })
  expect(res[0]).toBe(res[1])
}

describe('MagicString', () => {
  describe('append', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('ABC')
        s.append('D')
        s.append('E')
        s.append('F')
        return s.toString()
      })
    })
  })

  describe('append', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('DEF')
        s.prepend('C')
        s.prepend('B')
        s.prepend('A')
        return s.toString()
      })
    })
  })

  describe('appendLeft', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('AF')
        s.appendLeft(1, 'B')
        s.appendLeft(1, 'C')
        s.appendLeft(1, 'D')
        s.appendLeft(1, 'E')
        return s.toString()
      })
    })
  })

  describe('appendRight', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('AF')
        s.appendRight(1, 'B')
        s.appendRight(1, 'C')
        s.appendRight(1, 'D')
        s.appendRight(1, 'E')
        return s.toString()
      })
    })
    it('appendLeft + appendRight', () => {
      validate(Cons => {
        const s = new Cons('AF')
        s.appendLeft(1, 'B')
        s.appendRight(1, 'B')
        s.appendLeft(1, 'C')
        s.appendRight(1, 'C')
        s.appendLeft(1, 'D')
        s.appendRight(1, 'D')
        s.appendLeft(1, 'E')
        s.appendRight(1, 'E')
        return s.toString()
      })
    })
  })

  describe('prependLeft', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('AF')
        s.prependLeft(1, 'E')
        s.prependLeft(1, 'D')
        s.prependLeft(1, 'C')
        s.prependLeft(1, 'B')
        return s.toString()
      })
    })
  })

  describe('prependRight', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('AZ')
        s.prependRight(1, 'Y')
        s.prependRight(1, 'X')
        s.prependRight(1, 'I')
        s.prependRight(1, 'H')
        s.prependRight(1, 'G')
        s.prependRight(1, 'F')
        s.prependRight(1, 'E')
        return s.toString()
      })
    })
    it('prependLeft + prependRight', () => {
      validate(Cons => {
        const s = new Cons('AZ')
        s.prependLeft(1, 'G')
        s.prependLeft(1, 'F')
        s.prependLeft(1, 'E')
        s.prependRight(1, 'Y')
        s.prependRight(1, 'X')
        s.prependRight(1, 'I')
        s.prependRight(1, 'H')
        return s.toString()
      })
    })
  })

  describe('trim', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('      C   ')
        s.prependLeft(4, '  B  ')
        s.appendLeft(3, 'abc')
        s.trim()
        return s.toString()
      })
    })
    it('trim after overwrite', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.overwrite(0, 3, '   ').overwrite(9, 12, '   ').trim()
        return s.toString()
      })
    })
  })

  describe('trimLines', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('BCDEFGHIGK\r\n')
        s.prepend('\r\nA')
        s.trimLines()
        return s.toString()
      })
    })
  })

  describe('move', () => {
    it('moves content from the start', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.move(0, 3, 6)
        return s.toString()
      })
    })

    it('moves content to the start', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.move(3, 6, 0)
        return s.toString()
      })
    })

    it('moves content from the end', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.move(9, 12, 6)
        return s.toString()
      })
    })

    it('moves content to the end', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')

        s.move(6, 9, 12)
        return s.toString()
      })
    })

    it('ignores redundant move', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')

        s.prependRight(9, 'X')
        s.move(9, 12, 6)
        s.appendLeft(12, 'Y')
        s.move(6, 9, 12) // this is redundant – [6,9] is already after [9,12]
        return s.toString()
      })
    })

    it('moves content to the middle', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.move(3, 6, 9)
        return s.toString()
      })
    })

    it('handles multiple moves of the same snippet', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')

        s.move(0, 3, 6)
        s.move(0, 3, 9)
        return s.toString()
      })
    })

    it('handles moves of adjacent snippets', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')

        s.move(0, 2, 6)
        s.move(2, 4, 6)
        return s.toString()
      })
    })

    it('handles moves to same index', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.move(0, 2, 6).move(3, 5, 6)
        return s.toString()
      })
    })

    it('refuses to move a selection to inside itself', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        expect(() => s.move(3, 6, 3)).toThrow(
          /Cannot move a selection inside itself/
        )
        expect(() => s.move(3, 6, 4)).toThrow(
          /Cannot move a selection inside itself/
        )
        expect(() => s.move(3, 6, 6)).toThrow(
          /Cannot move a selection inside itself/
        )
      })
    })

    it('allows edits of moved content', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.move(3, 6, 9)
        s.overwrite(3, 6, 'DEF')
        return s.toString()
      })
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.move(3, 6, 9)
        s.overwrite(4, 5, 'E')
        return s.toString()
      })
    })

    it('moves content inserted at end of range', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.appendLeft(6, 'X').move(3, 6, 9)
        return s.toString()
      })
    })
  })

  describe('isEmpty', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('  ABCDEFG  ')
        return s.isEmpty()
      })
    })
  })

  describe('overwrite', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('ABCDEFG')
        s.appendLeft(3, '--appendLeft--')
        s.update(2, 5, 'A')
        return s.toString()
      })
    })
  })
  describe('update', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('problems = 99')
        s.update(2, 5, 'A')
        s.update(0, 8, 'answer')
        s.toString() // 'answer = 99'
        s.update(11, 13, '42') // character indices always refer to the original string
        s.toString() // 'answer = 42'
        s.prepend('var ').append(';') // most methods are chainable
        return s.toString() // 'var answer = 42;'
      })
    })
  })

  describe('hasChanged', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('problems = 99')
        s.update(2, 5, 'A')
        return s.hasChanged()
      })
    })
  })

  describe('remove', () => {
    it('normal', () => {
      validate(Cons => {
        const s = new Cons('problems = 99')
        s.remove(2, 5)
        return s.toString()
      })
    })
    it('combo', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(0, 6)
        s.appendLeft(6, 'DEF')
        s.overwrite(6, 9, 'GHI')
        return s.toString()
      })
    })
  })

  describe('insert', () => {
    it('normal', () => {
      const s = new RustMagicString('abcdefghijkl')
      expect(() => s.insert()).toThrow(
        'Deprecated api error: magicString.insert(...) is deprecated. Use prependRight(...) or appendLeft(...)'
      )
    })
  })

  describe('clone', () => {
    it('should clone a magic string', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        const c = s.clone()
        return c.toString()
      })
    })

    it('should clone indentExclusionRanges', () => {
      const array = [3, 6]
      const source = new RustMagicString('abcdefghijkl', {
        filename: 'foo.js',
        indentExclusionRanges: array
      })
      const clone = source.clone()
      expect(source.indentExclusionRanges[0]).toBe(
        clone.indentExclusionRanges[0]
      )
      expect(source.indentExclusionRanges[1]).toBe(
        clone.indentExclusionRanges[1]
      )
    })

    it('should clone complex indentExclusionRanges', () => {
      const array = [
        [3, 6],
        [7, 9]
      ]
      const source = new RustMagicString('abcdefghijkl', {
        filename: 'foo.js',
        indentExclusionRanges: array
      })
      const clone = source.clone()
      expect(source.indentExclusionRanges[0][0]).toBe(
        clone.indentExclusionRanges[0][0]
      )
      expect(source.indentExclusionRanges[1][1]).toBe(
        clone.indentExclusionRanges[1][1]
      )
    })

    it('should clone sourcemapLocations', () => {
      validate(Cons => {
        const source = new Cons('abcdefghijkl', {
          filename: 'foo.js'
        })
        source.addSourcemapLocation(3)
        const c = source.clone()
        return c.generateMap().mappings
      })
    })

    it('should clone intro and outro', () => {
      validate(Cons => {
        const source = new Cons('defghi')
        source.prepend('abc')
        source.append('jkl')
        const clone = source.clone()
        return clone.toString()
      })
    })
  })

  describe('snip', () => {
    it('should return a clone with content outside `start` and `end` removed', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl', {
          filename: 'foo.js'
        })
        s.overwrite(6, 9, 'GHI')
        const snippet = s.snip(3, 9)
        return snippet.toString()
      })
    })

    it('should snip from the start', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        const snippet = s.snip(0, 6)
        snippet.overwrite(6, 9, 'GHI')
        return snippet.toString()
      })
    })

    it('should snip from the end', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        const snippet = s.snip(6, 12)
        snippet.overwrite(6, 9, 'GHI')
        return snippet.toString()
      })
    })

    it('should respect original indices', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        const snippet = s.snip(3, 9)
        snippet.overwrite(6, 9, 'GHI')
        return snippet.toString()
      })
    })
  })

  describe('slice', () => {
    it('should return the generated content between the specified original characters', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        expect(s.slice(3, 9)).toBe('defghi')
        s.overwrite(4, 8, 'XX')
        expect(s.slice(3, 9)).toBe('dXXi')
        s.overwrite(2, 10, 'ZZ')
        expect(s.slice(1, 11)).toBe('bZZk')
        expect(s.slice(2, 10)).toBe('ZZ')
        expect(() => s.slice(3, 9)).toThrow(
          'Cannot use replaced character 3 as slice start anchor.'
        )
      })
    })

    it('defaults `end` to the original string length', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        expect(s.slice(3)).toBe('defghijkl')
      })
    })

    it('allows negative numbers as arguments', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        expect(s.slice(-3)).toBe('jkl')
        expect(s.slice(0, -3)).toBe('abcdefghi')
      })
    })

    it('includes inserted characters, respecting insertion direction', () => {
      validate(Cons => {
        const s = new Cons('abefij')
        s.prependRight(2, 'cd')
        s.appendLeft(4, 'gh')
        expect(s.slice()).toBe('abcdefghij')
        expect(s.slice(1, 5)).toBe('bcdefghi')
        expect(s.slice(2, 4)).toBe('cdefgh')
        expect(s.slice(3, 4)).toBe('fgh')
        expect(s.slice(0, 2)).toBe('ab')
        expect(s.slice(0, 3)).toBe('abcde')
        expect(s.slice(4, 6)).toBe('ij')
        expect(s.slice(3, 6)).toBe('fghij')
      })
    })

    it('supports characters moved outward', () => {
      validate(Cons => {
        const s = new Cons('abcdEFghIJklmn')
        s.move(4, 6, 2)
        s.move(8, 10, 12)

        expect(s.toString()).toBe('abEFcdghklIJmn')
        expect(s.slice(1, -1)).toBe('bEFcdghklIJm')
        expect(s.slice(2, -2)).toBe('cdghkl')
        expect(s.slice(3, -3)).toBe('dghk')
        expect(s.slice(4, -4)).toBe('EFcdghklIJ')
        expect(s.slice(5, -5)).toBe('FcdghklI')
        expect(s.slice(6, -6)).toBe('gh')
      })
    })

    it('supports characters moved opposing', () => {
      validate(Cons => {
        const s = new Cons('abCDefghIJkl')
        s.move(2, 4, 8)
        s.move(8, 10, 4)
        expect(s.slice(1, -1)).toBe('bIJefghCDk')
        expect(s.slice(2, -2)).toBe('')
        expect(s.slice(3, -3)).toBe('')
        expect(s.slice(-3, 3)).toBe('JefghC')
        expect(s.slice(4, -4)).toBe('efgh')
        expect(s.slice(0, 3)).toBe('abIJefghC')
        expect(s.slice(3)).toBe('Dkl')
        expect(s.slice(0, -3)).toBe('abI')
        expect(s.slice(-3)).toBe('JefghCDkl')
      })
    })

    it('errors if replaced characters are used as slice anchors', () => {
      validate(Cons => {
        const s = new Cons('abcdef')
        s.overwrite(2, 4, 'CD')
        expect(() => s.slice(2, 3)).toThrow(/slice end anchor/)
        expect(() => s.slice(3, 4)).toThrow(/slice start anchor/)
        expect(() => s.slice(3, 5)).toThrow(/slice start anchor/)
        expect(s.slice(1, 5)).toBe('bCDe')
      })
    })

    it('does not error if slice is after removed characters', () => {
      validate(Cons => {
        const s = new Cons('abcdef')
        s.remove(0, 2)
        expect(s.slice(2, 4)).toBe('cd')
      })
    })
  })

  describe('reset', () => {
    it('should reset moved characters from the original string', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(1, 5)
        s.reset(2, 4)
        s.reset(4, 5)
        return s.toString()
      })
    })

    it('should reset from the start', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(0, 6)
        s.reset(0, 3)
        return s.toString()
      })
    })

    it('should reset from the end', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(6, 12)
        s.reset(10, 12)
        return s.toString()
      })
    })

    it('should treat zero-length resets as a no-op', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(3, 5)
        s.reset(0, 0).reset(6, 6).reset(9, -3)
        return s.toString()
      })
    })

    it('should treat not modified resets as a no-op', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.reset(3, 5)
        return s.toString()
      })
    })

    it('should reset overlapping ranges 1', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(0, 10)
        s.reset(1, 7).reset(5, 9)
        return s.toString()
      })
    })
    it('should reset overlapping ranges 2', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(0, 10)
        s.reset(3, 7).reset(4, 6)
        return s.toString()
      })
    })

    it('should reset overlapping ranges, redux', () => {
      validate(Cons => {
        const s = new Cons('abccde')
        s.remove(0, 6)
        s.reset(2, 3) // c
        s.reset(1, 3) // bc
        return s.toString()
      })
    })

    it('should reset modified ranges', () => {
      validate(Cons => {
        const s = new Cons('abcdefghi')
        s.overwrite(3, 6, 'DEF')
        s.remove(1, 8) // bcDEFgh
        s.reset(2, 7) // cDEFg
        return s.toString()
      })
    })

    it('should reset modified ranges, redux', () => {
      validate(Cons => {
        const s = new Cons('abcdefghi')
        s.remove(1, 8)
        s.appendLeft(2, 'W')
        s.appendRight(2, 'X')
        s.prependLeft(3, 'Y')
        s.prependRight(5, 'Z')
        s.reset(2, 7)
        return s.toString()
      })
    })

    it('should not reset content inserted after the end of range', () => {
      validate(Cons => {
        const s = new Cons('ab.c;')
        s.prependRight(0, '(')
        s.prependRight(4, ')')
        s.remove(1, 4)
        s.reset(2, 4)
        return s.toString()
      })
    })

    it('should provide a useful error when illegal removals are attempted', () => {
      const s = new RustMagicString('abcdefghijkl')
      s.remove(4, 8)
      s.overwrite(5, 7, 'XX')
      expect(() => s.reset(4, 6)).toThrow(
        'Split chunk error: Cannot split a chunk that has already been edited'
      )
    })

    it('removes across moved content', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl')
        s.remove(5, 8)
        s.move(6, 9, 3)
        s.reset(7, 8)
        return s.toString()
      })
    })
  })

  describe('replace', () => {
    it('rejects when replacerFn', () => {
      expect(() => new RustMagicString('123').replace('1', () => {})).toThrow(
        '`replacement` argument do not supports RegExp replacerFn now'
      )
    })

    it('works with string replace', () => {
      validate(Cons => {
        const code = 'abcdefghijklabcdefghijkl'
        const s = new Cons(code)
        s.replace('c', 'aaa')
        return s.toString()
      })
    })

    it('Should not treat string as regexp', () => {
      validate(Cons => {
        const s = new Cons('1234')
        s.replace('.', '*')
        return s.toString()
      })
    })

    it('Should use substitution directly', () => {
      validate(Cons => {
        const s = new Cons('1234')
        s.replace('1', '$0$1')
        return s.toString()
      })
    })

    it('Should not search back', () => {
      validate(Cons => {
        const s = new Cons('122121')
        s.replace('12', '21')
        return s.toString()
      })
    })

    it('works with global regex replace', () => {
      validate(Cons => {
        const s = new Cons('1 2 3 4 a b c')
        s.replace(/(\d)/g, 'xx$1$10')
        return s.toString()
      })
    })

    it('works with global regex replace $$', () => {
      validate(Cons => {
        const s = new Cons('1 2 3 4 a b c')
        s.replace(/(\d)/g, '$$')
        return s.toString()
      })
    })
  })

  describe('replaceAll', () => {
    it('works with string replace 1', () => {
      validate(Cons => {
        const s = new Cons('1212')
        s.replaceAll('2', '3')
        return s.toString()
      })
    })

    it('Should not treat string as regexp', () => {
      validate(Cons => {
        const s = new Cons('1234')
        s.replaceAll('.', '*')
        return s.toString()
      })
    })

    it('Should use substitution directly', () => {
      validate(Cons => {
        const s = new Cons('11')
        s.replaceAll('1', '$0$1')
        return s.toString()
      })
    })

    it('Should not search back', () => {
      validate(Cons => {
        const s = new Cons('121212')
        s.replaceAll('12', '21')
        return s.toString()
      })
    })

    it('global regex result the same as .replace 1', () => {
      validate(Cons => {
        const s = new Cons('1 2 3 4 a b c')
        s.replaceAll(/(\d)/g, 'xx$1$10')
        return s.toString()
      })
    })
    it('global regex result the same as .replace 2', () => {
      validate(Cons => {
        const s = new Cons('1 2 3 4 a b c')
        s.replaceAll(/(\d)/g, '$$')
        return s.toString()
      })
    })

    it('rejects when non-global regexp', () => {
      expect(() => new RustMagicString('123').replaceAll(/1/, '1')).toThrow(
        'TypeError: replaceAll called with a non-global RegExp argument'
      )
    })

    it('rejects when replacerFn', () => {
      expect(() =>
        new RustMagicString('123').replaceAll('1', () => {})
      ).toThrow(
        'TypeError: `replacement` argument do not supports RegExp replacerFn now'
      )
    })
  })

  describe('generateMap', () => {
    it('should generate a sourcemap', () => {
      validate(Cons => {
        const s = new Cons('abcdefghijkl').remove(3, 9)
        return s.generateMap().mappings
      })
    })

    it('should generate a correct sourcemap for prepend content when hires = false', () => {
      validate(Cons => {
        const s = new Cons('x\nq')

        s.prepend('y\n')

        return s.generateMap().mappings
      })
    })

    it('should generate a correct sourcemap for indented content', () => {
      validate(Cons => {
        const s = new Cons(
          'var answer = 42;\nconsole.log("the answer is %s", answer);'
        )
        s.prepend("'use strict';\n\n")
        s.indent('\t').prepend('(function () {\n')
        // https://github.com/Rich-Harris/magic-string/pull/300
        // .append('\n}).call(global);')

        return s.generateMap().mappings
      })
    })

    it('should generate a sourcemap using specified locations', () => {
      validate(Cons => {
        const s = new Cons(
          'var answer = 42;\nconsole.log("the answer is %s", answer);'
        )
        s.addSourcemapLocation(0)
        s.addSourcemapLocation(3)
        s.addSourcemapLocation(10)
        s.remove(6, 9)

        return s.generateMap().mappings
      })
    })

    it('should correctly map inserted content', () => {
      validate(Cons => {
        const s = new Cons('function Foo () {}')

        s.overwrite(9, 12, 'Bar')
        return s.generateMap().mappings
      })
    })

    it('should yield consistent results between appendLeft and prependRight', () => {
      validate(Cons => {
        const s1 = new Cons('abcdefghijkl')
        s1.appendLeft(6, 'X')

        const s2 = new Cons('abcdefghijkl')
        s2.prependRight(6, 'X')

        return s1.generateMap().mappings + s2.generateMap().mappings
      })
    })

    it('should recover original names', () => {
      const s = new MagicString('function Foo () {}')

      s.overwrite(9, 12, 'Bar', { storeName: true })

      const map = s.generateMap({
        file: 'output.js',
        source: 'input.js',
        includeContent: true
      })

      const smc = new SourceMapConsumer(map)

      const loc = smc.originalPositionFor({ line: 1, column: 9 })
      expect(loc.name).toBe('Foo')
    })

    it('should generate one segment per replacement', () => {
      const s = new MagicString('var answer = 42')
      s.overwrite(4, 10, 'number', { storeName: true })

      const map = s.generateMap({
        file: 'output.js',
        source: 'input.js',
        includeContent: true
      })

      const smc = new SourceMapConsumer(map)

      let numMappings = 0
      smc.eachMapping(() => (numMappings += 1))

      expect(numMappings).toBe(3) // one at 0, one at the edit, one afterwards
    })

    it('should generate a sourcemap that correctly locates moved content', () => {
      const s = new MagicString('abcdefghijkl')
      s.move(3, 6, 9)

      const result = s.toString()
      const map = s.generateMap({
        file: 'output.js',
        source: 'input.js',
        includeContent: true,
        hires: true
      })

      const smc = new SourceMapConsumer(map)

      'abcdefghijkl'.split('').forEach((letter, i) => {
        const column = result.indexOf(letter)
        const loc = smc.originalPositionFor({ line: 1, column })

        expect(loc.line).toBe(1)
        expect(loc.column).toBe(i)
      })
    })

    it('generates a map with trimmed content (#53)', () => {
      const s1 = new MagicString('abcdefghijkl ').trim()
      const map1 = s1.generateMap({
        file: 'output',
        source: 'input',
        includeContent: true,
        hires: true
      })

      const smc1 = new SourceMapConsumer(map1)
      const loc1 = smc1.originalPositionFor({ line: 1, column: 11 })

      expect(loc1.column).toBe(11)

      const s2 = new MagicString(' abcdefghijkl').trim()
      const map2 = s2.generateMap({
        file: 'output',
        source: 'input',
        includeContent: true,
        hires: true
      })

      const smc2 = new SourceMapConsumer(map2)
      const loc2 = smc2.originalPositionFor({ line: 1, column: 1 })

      expect(loc2.column).toBe(2)
    })

    it('skips empty segments at the start', () => {
      const s = new MagicString('abcdefghijkl')
      s.remove(0, 3).remove(3, 6)

      const map = s.generateMap()
      const smc = new SourceMapConsumer(map)
      const loc = smc.originalPositionFor({ line: 1, column: 6 })

      expect(loc.column).toBe(6)
    })

    it('skips indentation at the start', () => {
      const s = new MagicString('abcdefghijkl')
      s.indent('    ')

      const map = s.generateMap()
      expect(map.mappings).toBe('IAAA')
    })

    it('generates x_google_ignoreList', () => {
      const s = new MagicString('function foo(){}', {
        ignoreList: true
      })

      const map = s.generateMap({ source: 'foo.js' })
      expect(map.sources[0]).toBe('foo.js')
      expect(map.x_google_ignoreList[0]).toBe(0)
    })

    it('generates segments per word boundary with hires "boundary"', () => {
      const s = new MagicString('function foo(){ console.log("bar") }')

      // rename bar to hello
      s.overwrite(29, 32, 'hello')

      const map = s.generateMap({
        file: 'output.js',
        source: 'input.js',
        includeContent: true,
        hires: 'boundary'
      })

      expect(map.mappings).toBe(
        'AAAA,QAAQ,CAAC,GAAG,CAAC,CAAC,CAAC,CAAC,OAAO,CAAC,GAAG,CAAC,CAAC,KAAG,CAAC,CAAC,CAAC'
      )

      const smc = new SourceMapConsumer(map)
      let loc

      loc = smc.originalPositionFor({ line: 1, column: 3 })
      expect(loc.line).toBe(1)
      expect(loc.column).toBe(0)

      loc = smc.originalPositionFor({ line: 1, column: 11 })
      expect(loc.line).toBe(1)
      expect(loc.column).toBe(9)

      loc = smc.originalPositionFor({ line: 1, column: 29 })
      expect(loc.line).toBe(1)
      expect(loc.column).toBe(29)

      loc = smc.originalPositionFor({ line: 1, column: 35 })
      expect(loc.line).toBe(1)
      expect(loc.column).toBe(33)
    })

    it('generates a correct source map with update using a content containing a new line', () => {
      const s = new MagicString('foobar')
      s.update(3, 4, '\nbb')
      expect(s.toString()).toBe('foo\nbbar')

      const map = s.generateMap({ hires: true })

      const smc = new SourceMapConsumer(map)
      const loc = smc.originalPositionFor({ line: 1, column: 3 })
      expect(loc.line).toBe(1)
      expect(loc.column).toBe(3)
      const loc2 = smc.originalPositionFor({ line: 2, column: 0 })
      expect(loc2.line).toBe(1)
      expect(loc2.column).toBe(3)
      const loc3 = smc.originalPositionFor({ line: 2, column: 1 })
      expect(loc3.line).toBe(1)
      expect(loc3.column).toBe(3)
      const loc4 = smc.originalPositionFor({ line: 2, column: 2 })
      expect(loc4.line).toBe(1)
      expect(loc4.column).toBe(4)
    })

    it('generates a correct source map with update using content ending with a new line', () => {
      const s = new RustMagicString('foobar')
      s.update(2, 3, 'od\n')
      s.update(4, 5, 'a\nnd\n')
      expect(s.toString()).toBe('food\nba\nnd\nr')
      const map = s.generateMap({ hires: true })
      const smc = new SourceMapConsumer(map)

      // od\n
      const loc = smc.originalPositionFor({ line: 1, column: 3 })
      expect(loc.line).toBe(1)
      expect(loc.column).toBe(2)
      const loc2 = smc.originalPositionFor({ line: 1, column: 4 })
      expect(loc2.line).toBe(1)
      expect(loc2.column).toBe(2)
      const loc3 = smc.originalPositionFor({ line: 2, column: 0 })
      expect(loc3.line).toBe(1)
      expect(loc3.column).toBe(3)
      const loc4 = smc.originalPositionFor({ line: 2, column: 1 })
      expect(loc4.line).toBe(1)
      expect(loc4.column).toBe(4)
      // a\nnd\n
      const loc5 = smc.originalPositionFor({ line: 2, column: 2 })
      expect(loc5.line).toBe(1)
      expect(loc5.column).toBe(4)
      const loc6 = smc.originalPositionFor({ line: 2, column: 3 })
      expect(loc6.line).toBe(1)
      expect(loc6.column).toBe(4)
      const loc7 = smc.originalPositionFor({ line: 3, column: 0 })
      expect(loc7.line).toBe(1)
      expect(loc7.column).toBe(4)
      const loc8 = smc.originalPositionFor({ line: 4, column: 0 })
      expect(loc8.line).toBe(1)
      expect(loc8.column).toBe(5)
    })
  })

  describe('indent', () => {
    it('should indent content with a single tab character by default', () => {
      validate(Cons => {
        const s = new Cons('abc\ndef\nghi\njkl')
        s.indent()
        s.indent()
        return s.toString()
      })
    })

    it('should indent content, using existing indentation as a guide', () => {
      validate(Cons => {
        const s = new Cons('abc\n  def\n    ghi\n  jkl')
        s.indent()
        s.indent()
        return s.toString()
      })
    })

    it('should disregard single-space indentation when auto-indenting', () => {
      validate(Cons => {
        const s = new Cons('abc\n/**\n *comment\n */')
        s.indent()
        return s.toString()
      })
    })

    it('should indent content using the supplied indent string', () => {
      validate(Cons => {
        const s = new Cons('abc\ndef\nghi\njkl')
        s.indent('  ')
        s.indent('>>')
        return s.toString()
      })
    })

    it('should indent content using the empty string if specified (i.e. noop)', () => {
      validate(Cons => {
        const s = new Cons('abc\ndef\nghi\njkl')
        s.indent('')
        return s.toString()
      })
    })

    it('should prevent excluded characters from being indented', () => {
      validate(Cons => {
        const s = new Cons('abc\ndef\nghi\njkl')
        s.indent('  ', { exclude: [[7, 15]] })
        s.indent('>>', { exclude: [[7, 15]] })
        return s.toString()
      })
    })

    it('should not add characters to empty lines', () => {
      validate(Cons => {
        const s = new Cons('\n\nabc\ndef\n\nghi\njkl')
        s.indent()
        s.indent()
        return s.toString()
      })
    })

    it('should not add characters to empty lines, even on Windows', () => {
      validate(Cons => {
        const s = new Cons('\r\n\r\nabc\r\ndef\r\n\r\nghi\r\njkl')
        s.indent()
        s.indent()
        return s.toString()
      })
    })

    it('should indent content with removals', () => {
      validate(Cons => {
        const s = new Cons('/* remove this line */\nvar foo = 1;')
        s.remove(0, 23)
        s.indent()
        return s.toString()
      })
    })

    it('should not indent patches in the middle of a line', () => {
      validate(Cons => {
        const s = new Cons('class Foo extends Bar {}')
        s.overwrite(18, 21, 'Baz')
        s.indent()
        return s.toString()
      })
    })
  })
})
