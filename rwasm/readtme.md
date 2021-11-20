# rwasm 
The purpose of the project is to examine how to use wasm created from rust and apply it in a html canvas to render some kind of "game"/"simulation" in 2D. 

#TODO - Things i think i need to do
* Create docker setup with rust installed
* Create basic html page
* Expose site via docker
* Create basic rust program that compiles wasm
* Make wasm functions available to site javascript
    * [Loading and running WebAssembly code](https://developer.mozilla.org/en-US/docs/WebAssembly/Loading_and_running)
* Hook into wasm from javascript and call function
    * [Using the WebAssembly JavaScript API](https://developer.mozilla.org/en-US/docs/WebAssembly/Using_the_JavaScript_API)
* Extend to explorer capabilities