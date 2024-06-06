/***
 * Excerpted from "Programming WebAssembly with Rust",
 * published by The Pragmatic Bookshelf.
 * Copyrights apply to this code. It may not be used to create training material,
 * courses, books, articles, and the like. Contact us if you are in doubt.
 * We make no guarantees that this code is fit for any purpose.
 * Visit http://www.pragmaticprogrammer.com/titles/khrust for more book information.
***/
/***
 * Excerpted from "Programming WebAssembly with Rust",
 * published by The Pragmatic Bookshelf.
 * Copyrights apply to this code. It may not be used to create training material,
 * courses, books, articles, and the like. Contact us if you are in doubt.
 * We make no guarantees that this code is fit for any purpose.
 * Visit http://www.pragmaticprogrammer.com/titles/khrust for more book information.
***/

// 动态加载 WebAssembly 模块
async function initWasm() {
    const response = await fetch('./dist/260fd5a3ea529dd3e492.module.wasm');
    const bytes = await response.arrayBuffer();
    const { instance, module } = await WebAssembly.instantiate(bytes, {});
  
    console.log("WebAssembly module loaded...");
    // 你可以在这里使用实例化后的 WebAssembly 模块
    // instance.exports.someExportedFunction();
  }
  
  // 加载 WebAssembly 并初始化
  initWasm().catch(console.error);
  
  // 导入其他 JavaScript 模块
  import('./index').then(module => {
      console.log("JavaScript module loaded...");
  }).catch(console.error);
  