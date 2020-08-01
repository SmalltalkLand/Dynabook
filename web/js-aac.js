import * as components from './components.js'
import * as utils from './utils.js'
let isK = false;
Function.prototype.constructor= (old => function(...args){
    if(isK)return old(this,...args);
    isK = true;
    let v = new Proxy(old(this,...args),{
        apply: (o,t,args) => o.aacRequired ? o.aacFunction(o,t,args) : Reflect.apply(o,t,args),
        set: (o,k,v) => ['aacRequired','aacFunction'].includes(k) && o.aacFunction ? o.aacFunction(o,t,['set',k,v]) : o[k] = v,
    });
    isK = false;
    return v
})(Function.prototype.constructor.call.bind(Function.prototype.constructor))