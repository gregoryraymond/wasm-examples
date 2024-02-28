# WASM Examples

## Router

Router is a cloudflare backend app providing a GET function.

The code it's based off can be generated with: 
```
npm create cloudflare@latest - router
```

It can be launched with: npx wrangler dev

## Tauri App

Tauri app is mostly generated code also, it can be created with the following, and shows a few ways to surface WASM errors.

It also demonstrates how you can call between different sections of the code across WASM / Rust boundaries.

The code it's based off can be generated with:

```
cargo install create-tauri-app --locked
cargo create-tauri-app
```

and choosing leptos for the front-end.

It can be launched with: cargo tauri dev