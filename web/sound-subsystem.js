import * as utils from './utils.js'
export let gain_getter = ctxt => gg => new Proxy(ctxt.createGain(),{get: (o,k) => k === 'gain' ? new Proxy(o[k],{get: (o2,k2) => k2 === 'value' ? gg() : o2[k2]}) : o[k]})
