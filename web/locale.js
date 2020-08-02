export default new Proxy(Object.assign(x => import(x),{en: new Proxy({},{get: (o,k) => k,has: (o,k) => true})}),{get: (o,k) => o[k] || o(`./locale/${k}.js`),has: (o,k) => true})
export let defaultLang = 'en';
export let setDefaultLang = l => defaultLang = l;