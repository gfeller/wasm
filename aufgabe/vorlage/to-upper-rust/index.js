import init, {to_upper} from "./pkg/wasm_demo.js";

const module = await init("./pkg/wasm_demo_bg.wasm");

console.log(to_upper(100));

// console.log(to_upper("hello world"));

 