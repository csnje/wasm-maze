# wasm-maze

## About

An implementation of some [simply connected](https://en.wikipedia.org/wiki/Simply_connected_space) [maze generating](https://en.wikipedia.org/wiki/Maze_generation_algorithm) and [maze solving](https://en.wikipedia.org/wiki/Maze-solving_algorithm) algorithms in **Rust** **WebAssembly**.

![Image of solved maze](./images/output.png)

## Prerequisites

Install [**Rust**](https://www.rust-lang.org/) and [**wasm-pack**](https://github.com/rustwasm/wasm-pack).

## Compile

```bash
wasm-pack build --target web
```
or optimised for release
```bash
wasm-pack build --target web --release
```

## Serve and run

## Serve and run

Some options to serve the application include:
```bash
# Python 3.x
python3 -m http.server
# Python 2.x
python -m SimpleHTTPServer
# JDK 18 or later
jwebserver
```

Access via a web browser at [http://localhost:8000](http://localhost:8000).
