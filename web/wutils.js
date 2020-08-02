import {bind,wire} from 'https://jspm.dev/hyperhtml'
import * as utils from './utils.js'
export let vueWire = vue => opts => (x => new vue(opts).$mount(x = wire(opts)`<div></div>`) && x)()
