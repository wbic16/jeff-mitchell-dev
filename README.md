# Jeff Mitchell's Dev Profile Site

Welcome to my developer profile site, written in Rust, with the Leptos web framework. Styling is done with Tailwind CSS.  This code base is open to all to study, use, refine, and extend.  I only ask that you try to make your own content.

I welcome contributions to this site, especially styling. I prepared this readme file to help those who just want to do that. To get started, here are some instructions:

## Fork

Fork this repo to your own GitHub account. This will allow you to contribute back by creating branches with new additions for which you can send me pull requests. I will evaluate them and incorporate them if I feel they benefit the site.  If you don't know how to fork, Google it, then come back.

## Install Rust

Sorry, you have to.  Instructions are here: [Install Rust](https://rust-lang.org/tools/install). Once Rust is installed on your computer, you'll have access to the cargo build tool.

## Do the Leptos Book

This site is written using the Leptos framework for Rust.  If you've used another declarative library or framework (i.e. React or Svelte), you'll be right at home.  I recommend doing the Leptos book from beginning to end here: [Leptos Book](https://book.leptos.dev/) Trust me, it's excellent and time well spent.

## Install Prerequisites

### Trunk

You need [Trunk](https://trunkrs.dev/), which is a build tool for helping build WebAssembly sites and apps with Rust.  To install it on your computer, at a command prompt type:

```Rust
cargo install trunk
```

### wasm32

To build to WebAssembly, you need a build target installed.  Do this from the command line:

```Rust
rustup target add wasm32-unknown-unknown
```

Yes, I know, the unknown-unknown thing is weird. I stopped worrying about it a long time ago.

## Developing the Site

The front-end content for this site lives in the \site folder.  To work on it, type:

```bash
cd frontend
```

The site is built using the index.html file at the root of the site folder.  Assets live in `\assets` and are linked in via the head tag at the top of index.html. You don't need to touch this though to work on the look and feel of the site.

To start a hot-reload development loop, type:

```bash
trunk serve --open
```

You can then add Tailwind classes and make whatever layout changes you want.  When you save, the site will be reloaded.  Right now, the site consists of a single `<App />` component which renders the site content. This lives within `frontend\src\bin\main.rs`. In the future I'll be adding a page router and splitting the site up into pages and components.



