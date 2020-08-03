import './js-aac.js'
import './js-ctxt.js'
import locale, {defaultLang} from './locale.js'
import {bind,wire} from 'https://jspm.dev/hyperhtml'
import swal from 'https://jspm.dev/sweetalert'
import Keyboard from 'https://jspm.dev/simple-keyboard'
import * as utils from './utils.js'
export let basicButton = (child,fn) => wire([child,fn])`<button onclick = ${utils.pipe(evt => Promise.resolve(evt),utils.andThen(evt => fn(evt)))}>${child}</button>`
if(window !== window.top)basicButton = (old => (child,fn) => old(child,Object.assign(evt => fn(evt),{aacRequired: true})))(basicButton)
export let defaultAAC = async (o,t,args) => await swal("allow?",{buttons: true,dangerMode: true}) && Reflect.apply(o,t,args)
export let keyboard = opts => (elem => [new Keyboard(elem,{...opts,onChange: i => elem.value = i}),elem])(wire(opts)`<input type = "text"></input>`)
export let localizable = text => wire(text)`<span>${locale[defaultLang][text]}</span>`