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
}`));
hebes = eval(hebes(`ם => ס => ם(ס)`))(hebes);
export let sandbox = sandboxDict => R.partial(new Function('_sd','i','with(sd)return eval(i)'),sandboxDict);
export let koran = /*e*/ text => eval(`function k(c){ //disassembles a Hangul character into parts
	if(c < 0xAC00 || c > 0xD7A3) return "";
	c -= 0xAC00;
	return new Array(Math.floor(c/28/21), Math.floor(c/28) % 21, c % 28);
}`) && eval(`var kc1 = new Array("ㄱ", "ㄲ", "ㄴ", "ㄷ", "ㄸ", "ㄹ", "ㅁ", "ㅂ", "ㅃ", "ㅅ", "ㅆ", "ㅇ", "ㅈ", "ㅉ", "ㅊ", "ㅋ", "ㅌ", "ㅍ", "ㅎ");
var kv = new Array("ㅏ", "ㅐ", "ㅑ", "ㅒ", "ㅓ", "ㅔ", "ㅕ", "ㅖ", "ㅗ", "ㅘ", "ㅙ", "ㅚ", "ㅛ", "ㅜ", "ㅝ", "ㅞ", "ㅟ", "ㅠ", "ㅡ", "ㅢ", "ㅣ");
var kc0 = new Array(" ", "ㄱ", "ㄲ", "ㄳ", "ㄴ", "ㄵ", "ㄶ", "ㄷ", "ㄹ", "ㄺ", "ㄻ", "ㄼ", "ㄽ", "ㄾ", "ㄿ", "ㅀ", "ㅁ", "ㅂ", "ㅄ", "ㅅ", "ㅆ", "ㅇ", "ㅈ", "ㅊ", "ㅋ", "ㅌ", "ㅍ", "ㅎ");
`) && text.split("").map(k).map(a => a.map((o,i) => i === 1 ? kv[o] : eval(`kc${i}`)[o]))