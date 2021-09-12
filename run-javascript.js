import fs from 'fs'
const code = String(fs.readFileSync('/home/samuel/code/pydantic/pydantic/main.py'))

console.time('load')
import {emphasize} from 'emphasize'
console.timeEnd('load')
console.time('highlight')
const output = emphasize.highlight('python', code).value

console.timeEnd('highlight')
console.log(`output ansi length ${output.length}`)
// console.log(output)
