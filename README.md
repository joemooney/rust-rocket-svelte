
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
[Mozilla Developer Network, Getting Started with Svelte](https://developer.mozilla.org/en-US/docs/Learn/Tools_and_testing/Client-side_JavaScript_frameworks/Svelte_getting_started)

## Starting from scratch

Assuming you are not using this repository, to start with a clean slate for Rust/Svelte

```bash
npx degit sveltejs/template moz-todo-svelte
cd moz-todo-svelte
rustup override set nightly
npm install
cargo new myproject
#now move stuff around
```

These are the steps to get to the initial point for starting development.
Then we move the svelte related files into the `client` directory etc.

## Getting Started

Start Rocket server and [Rollup](https://rollupjs.org) in two different terminals

### Terminal #1

To build and hot reload svelte components, this will not launch a http server.

```bash
npm run dev  
```

### Terminal #2

Compile and run the rust rocket http server:

```bash
cargo run  
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

[Svelte Material UI](https://github.com/hperrin/svelte-material-ui)

[Svelte Material Icons](https://github.com/ramiroaisen/svelte-material-icons)

## Change Log

### Step 1

Initial barebones Rust/Rocket/Svelte page working.

### Step 2

Got communication working from Svelete client calling Rust asynchronously.

### Step 3

Added Material UI components to create a first draft user interface.

TODO: I did not keep track of all that I did, need to repeat the process.

```bash
npm i -D @smui/data-table
npm install @material-ui/core   // maybe don't need
npm install @material-ui/icons   // maybe don't need
npm i svelte-material-icons

```
