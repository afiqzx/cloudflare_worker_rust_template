# Introduction

This template should be ready to use for deploying WASM to Cloudflare Workers.
This is essentially the default template from the wrangler CLI tool, but with Minijinja and Tailwind support added.

# Things to not forget when using this template
 - Install wrangler using npm.
 - Run `cargo update` if needed.
 - Change the project name in `wrangler.toml`
 - Configure the build config. (Maybe you need the build optimization inside `Cargo.toml` or maybe not. Be very careful as switching this may increase build time which is not desirable for debug build.)
