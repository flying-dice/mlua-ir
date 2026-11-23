/**
 * The scenarios again, as a TypeScript backend writes them. `tsc -p ts` checks them against the generated
 * `types.d.ts`; `tstl -p ts` compiles them to Luau, and the Rust tests run the result in the same VM.
 */
import * as text from 'std:text'
import * as math2 from 'std:math2'
import * as geo from 'std:geo'
import * as dir from 'std:dir'
import * as log from 'std:log'
import * as draw from 'std:draw'
import * as labels from 'std:labels'
import * as env from 'std:env'
import * as fs from 'std:fs'
import * as parse from 'std:parse'
import * as collections from 'std:collections'
import * as time from 'std:time'
import * as task from 'std:task'
import * as events from 'std:events'
import * as sort from 'std:sort'
import * as names from 'std:names'
import * as docker_models from 'vnd:docker_models'
import * as json from 'lib:json'
import * as bollard from 'vnd:bollard'

function check(this: void, condition: boolean, what: string): void {
  if (!condition) {
    throw `check failed: ${what}`
  }
}

// 01 a function
check(text.pad('id', 6) === 'id    ', 'pad')

// 02 globals and constants
check(MAX_ARGS === 64 && VOS_VERSION.length > 0 && uptime() >= 0, 'globals')
check(math2.TAU > 6.28 && math2.TAU < 6.29, 'TAU')

// 03 a typed table
const origin: geo.Point = { x: 0, y: 0 }
const mid = geo.midpoint(origin, { x: 4, y: 2, label: 'b' })
check(mid.x === 2 && mid.label === undefined, 'midpoint')

// 04 an options table
check(dir.list('src').length === 1 && dir.list('src', { recursive: true }).length === 3, 'list')

// 05 unions
const level: log.Level = 'warn'
check(log.write(level, 'x') === '[warn] x', 'write')
const figure: draw.Figure = { kind: 'rect', w: 2, h: 3 }
if (figure.kind === 'rect') {
  check(figure.w * figure.h === draw.area(figure), 'area') // narrowed: only a rect has `w`
}

// 06 lists, maps, multiple returns
const [map, parseErr] = labels.parse(['app=web', 'tier=front'])
if (parseErr !== undefined) {
  throw parseErr.message
}
check(map.app === 'web', 'parse') // narrowed by the error check: `map` is the map
const [first, count] = labels.summary(map)
check(first === 'app' && count === 2, 'summary')

// 07 absence
check(env.var('HOME') === '/home/demo' && env.var('NOPE') === undefined, 'var')

// 08 returned errors
const [, readErr] = fs.read('huge.bin')
if (readErr !== undefined && readErr.kind === 'too_large') {
  check(readErr.limit === 1024, 'limit') // only a `too_large` error has `limit`
}

// 09 raised errors
check(parse.number(' 42 ') === 42, 'number')

// 10 a userdata
const counter = collections.Counter.new(41)
check(counter.increment() === 42 && counter.value === 42, 'counter')

// 11 userdata as an argument, one-of arguments
const total = time.Duration.seconds(1).plus(time.Duration.seconds(0.5))
check(time.describe(total) === '1.5s' && time.describe(2) === '2s', 'describe')

// 12 async
check(task.sleep(1) === 1, 'sleep')

// 13 streams
let seen = 0
for (const tick of events.count_to(3).items()) {
  seen += tick.n
}
check(seen === 6, 'stream')

// 14 callbacks
const sorted = sort.sort_by(['pear', 'fig', 'apple'], (left, right) => left.length < right.length)
check(sorted[0] === 'fig', 'sort_by')

// 15 validated text
check(names.describe('web-1') === 'container web-1', 'describe name')

// 16 another crate's types
const [web] = docker_models.sample()
check(web !== undefined && web.State === 'running', 'sample')

// 17 genuinely unknown
const [decoded] = json.decode('{"n": 1}')
check(typeof decoded === 'object', 'decode')

// 18 a whole module
const [docker, connectErr] = bollard.connect({ host: 'unix:///var/run/docker.sock' })
if (connectErr !== undefined) {
  throw connectErr.message
}
const [containers] = docker.list_containers({ all: true })
check(containers !== undefined && containers.length === 2, 'list_containers')
const [, conflict] = docker.create_container('web', { Image: 'nginx:1.27' })
check(conflict !== undefined && conflict.kind === 'conflict', 'conflict')
