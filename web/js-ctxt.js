let isK = false;
Function.stack = [];
Function.prototype.constructor = (old => function(...args){
    if(isK)return old(this,...args);
    isK = true;
    let v = new Proxy(old(this,...args),{
        apply: (o,t,args) => {
            if(t === Function.stack)return Reflect.apply(o,t,args);
            Function.stack.push([o,t,...args]);
            try{
            var v = Reflect.apply(o,t,args);
            }finally{
                Function.stack.pop();
            };
            return v;
        }
    });
    isK = false;
    return v
})(Function.prototype.constructor.call.bind(Function.prototype.constructor))