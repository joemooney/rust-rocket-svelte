
# Rust-Svelte-on-Rust

Starter template for [Svelte](https://svelte.dev) frontend apps with Rust [Rocket](https://rocket.rs) backend server. 



## Requirements

* Rust  - [Install](https://www.rust-lang.org/tools/install) 
* Svelte
* Rocket
* NodeJs - [Install](https://nodejs.org/en/download/)

We will use Rust nightly.

## Background
Create a new project based on this example:
(Mozilla Developer Network, Getting Started with Svelte)[https://developer.mozilla.org/en-US/docs/Learn/Tools_and_testing/Client-side_JavaScript_frameworks/Svelte_getting_started]

## Install the dependencies...

```bash
npx degit sveltejs/template moz-todo-svelte
cd svelte-rocket
rustup override set nightly
npm install
```

## Getting Started

Start Rocket server and [Rollup](https://rollupjs.org) in two different terminals 

Terminal 1: (To run the rust server)
```bash
cargo run  
```
Terminal 2: (To build and hot reload svelte components)
```bash
npm run dev  
```

* Navigate to [localhost:8000](http://localhost:8000). You should see your app running. 
* Svelte client code is in `client` directory.
** Upon saving changes live-reloading via rollup will be rendered in the browser.
* Rust server Rocket code is in `src` directory. 
** To rebuild Rust code use cargo run after saving your changes. 
* All static files are served from `public` direcotry. Including the JS code compiled by Svelte Compiler.


## Building and running in production mode

To create an optimised version of the app:

```bash
npm run build
cargo build
```

## Built With
[Rocket](https://rocket.rs/) 

[Svelte](https://svelte.dev/)


## Change Log

v0.0.1: initial version
