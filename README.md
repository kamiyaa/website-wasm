# website-wasm
My personal website written in rust via yew/wasm

![Alt text](screenshot.png?raw=true "website-wasm")

## Dependencies
see [here](https://yew.rs/docs/en/getting-started/build-a-sample-app)
 - [python3](https://www.python.org/)
 - [sassc](https://github.com/sass/sassc)

## Building
Build wasm
```
$ ./build.sh
```
Build sass
```
$ cd scss
$ ./gen_css.sh
```

## Running
```
$ ./run.sh
```
navigate to `http://localhost:8080/`
