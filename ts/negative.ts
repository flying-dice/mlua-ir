/**
 * What the declarations refuse. Each `@ts-expect-error` must meet an error: if the declarations stop refusing it,
 * the directive is unused and `tsc` fails.
 */
import * as text from 'std:text'
import * as geo from 'std:geo'
import * as log from 'std:log'
import * as fs from 'std:fs'
import * as json from 'lib:json'
import * as collections from 'std:collections'
import * as docker_models from 'vnd:docker_models'

// @ts-expect-error `width` is not optional
text.pad('x')
// @ts-expect-error not a level
log.write('loud', 'x')
// @ts-expect-error a field the record does not have
geo.distance({ x: 0, y: 0, lable: 'a' }, { x: 0, y: 0 })
// @ts-expect-error `std:fs` has no error of kind "missing"
const kind: fs.FsError['kind'] = 'missing'
const [value] = json.decode('{}')
// @ts-expect-error `unknown` must be checked before it is used
value.field
// @ts-expect-error a static, not a method: `Counter.new`, not `new Counter`
new collections.Counter()
const [, err] = fs.read('x')
if (err !== undefined) {
  // @ts-expect-error only a `too_large` error has `limit`
  err.limit
}
// @ts-expect-error bollard's `ContainerSummary` has `Names`, not `Nmaes`
docker_models.sample()[0]?.Nmaes
// @ts-expect-error Docker has no container state "sleeping"
const state: NonNullable<docker_models.ContainerSummary['State']> = 'sleeping'
export { kind, state }
