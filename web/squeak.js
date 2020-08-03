import 'https://raw.githubusercontent.com/codefrau/SqueakJS/main/squeak.js'
export let runSqueak = (...args) => new Promise(c => {
    let desc = Object.getOwnPropertyDescriptor(SqueakJS,'vm');
    Object.defineProperty(SqueakJS,'vm',{get: () => null,set: v => {
        c(v);
        Object.defineProperty(Sq,'vm',desc);
    }});
    SqueakJS.runSqueak(...args);
});
export let dynabookPlugin = () => {
    var interpreterProxy,
    primHandler;
let hooks = [];
function setInterpreter(anInterpreterProxy) {
    // Slang interface
    interpreterProxy = new Proxy(anInterpreterProxy,{get: (o,k) => o[k] || o.vm[k].bind(o.vm)});
    // PrimHandler methods for convenience
    primHandler = interpreterProxy.vm.primHandler;
    Object.assign(primHandler,{
        js_executeCallbackAsync: function(block, args, resolve, reject) {
            var squeak = this;
            function again() {squeak.js_executeCallbackAsync(block, args, resolve, reject)}
            if (!this.js_activeCallback) {
                this.js_executeCallback(block, args, resolve, reject);
            } else {
                self.setTimeout(again, 0);
            }
        },
        namedPrimitive: async function(modName, functionName, argCount) {
            let theHooks = hooks.map(h => this.js_fromStBlock(h));
            let unfreeze = interpreterProxy.freeze();
            // duplicated in loadFunctionFrom()
            var mod = modName === "" ? this : this.loadedModules[modName];
            if (mod === undefined) { // null if earlier load failed
                mod = this.loadModule(modName);
                this.loadedModules[modName] = mod;
            }
            var result = false;
            if (mod) {
                this.interpreterProxy.argCount = argCount;
                var primitive = mod[functionName];
                await Promise.all(theHooks.map(h => h(primitive)));
                if (typeof primitive === "function") {
                    result = mod[functionName](argCount);
                } else if (typeof primitive === "string") {
                    // allow late binding for built-ins
                    result = this[primitive](argCount);
                } else {
                    this.vm.warnOnce("missing primitive: " + modName + "." + functionName);
                }
            } else {
                this.vm.warnOnce("missing module: " + modName + " (" + functionName + ")");
            }
            unfreeze();
            if (result === true || result === false) return result;
            return this.success;
        },
    });
    // success
    return true;
};
let primitiveClone = argCount => {
    interpreterProxy.push(Object.create(interpreterProxy.pop()));
    return true;
}
    return {
        setInterpreter,
        primitiveClone
    }
}
