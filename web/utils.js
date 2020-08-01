import * as R from 'https://jspm.dev/ramda'
import * as sinon from 'https://jspm.dev/sinon'
export * from 'https://jspm.dev/ramda'
export let ierrormsg = "".split("::");
export let rusni = s => s.replace('дуе','let').replace("екн","try").replace("сфеср","catch").replace("еркжб","throw")
let куздфсу = (ы,а,е) => ы.replace(а,е)
export let hebes = eval(rusni('ы => куздфсу(ы,"ךקא","let")'))
export let load_jsonp = url => R.pipe(x => import(x).catch(err => ({error: err})),R.andThen(v => v ? v.error ? (r => new Promise((a,b) => {self[`f${r}`] = a;import(url + 'callback=f' + r).then(_ => {delete self[`f${r}`];})}))(Math.random()) : v : v))(url);
export let mock = f => sinon.fake(f);
export let it = eval(rusni(`джп => (еуые_тфьу,ат) => {

джп(еуые_тфьу);
екн{
    ат()
}сфеср(укк){
джп(укк);
джп("");
еркжб укк;
}
}`))