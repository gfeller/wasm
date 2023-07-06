import init from "./pkg/wasm_demo.js";
import {SampleQrCode} from "./pkg/wasm_demo.js"; 


function printQrCode(elem, memory, imagePointer, width, height) {
  let imageData = new ImageData(new Uint8ClampedArray(memory, imagePointer, width * height * 4), width, height);
  elem.width = width;
  elem.height = height
  const ctx = elem.getContext('2d');
 
  ctx.putImageData(imageData, 0, 0);
}

const module = await init("./pkg/wasm_demo_bg.wasm");

const generateQrCode = async (value) => {
  debugger;
  const qrCode = SampleQrCode.new(value || "Hello World"); 
  printQrCode( document.getElementById("qr-code"), module.memory.buffer, qrCode.buffer(), qrCode.width(), qrCode.height());
};



document.getElementById("qr-text").addEventListener("change", (event)=>{
  generateQrCode(event.target.value);
})

generateQrCode();