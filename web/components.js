import {bind,wire} from 'https://jspm.dev/hyperhtml'
import * as utils from './utils.js'
let ш = {ас: а => (...фкпы) => а(...фкпы),зжкефд: (удуь,тжву) => bind(удуь)`${тжву}`}
export let basicButton = (child,fn) => wire`<button onclick = ${utils.pipe(evt => Promise.resolve(evt),utils.andThen(evt => fn(evt)))}>${child}</button>`
