import './js-aac.js'
import locale, {defaultLang} from './locale.js'
import {bind,wire} from 'https://jspm.dev/hyperhtml'
import * as utils from './utils.js'
export let basicButton = (child,fn) => wire([child,fn])`<button onclick = ${utils.pipe(evt => Promise.resolve(evt),utils.andThen(evt => fn(evt)))}>${child}</button>`
if(window !== window.top)basicButton = (old => (child,fn) => old(child,Object.assign(evt => fn(evt),{aacRequired: true})))(basicButton)