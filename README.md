# Lusr (Lua Server Runtime)
The minimalistic Lua runtime, that allows you to do anything you want. From hosting a server to
making your own automation scripts, posibilities are endless.
# Features
- High performance (because Lua is the fastest scripting language)
- An easy-to-understand API, not too extensive at the moment.
- It's memory safe! (as it was written in Rust)
- Minimal resource consumption.
# Running scripts
It's very simple to run a script, just run type `lusr` and right next to it your script name (with its extension), just as you would do it with NodeJS. Here's an example:
```bash
lusr src/main.lua
```
**Output**
```
Hello World
```