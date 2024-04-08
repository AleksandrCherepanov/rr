### RR - rust render

The small educational project which I've made during my Rust language learing.

I used a small but a very comprehensive course of basics of computer graphics:
* [Russian](https://habr.com/ru/articles/248153/)
* [English](https://github.com/ssloy/tinyrenderer/wiki)

You can use it via CLI.

How to use:
1. Builld from source using `cargo build -r`
2. Run binary in `./target/release/rr help`
3. Also, you can use `./target/release/rr help <command>`
4. Files that you can use as examples located in the examples directory

P.S. As it's an educational project a lot of expected things are not available. For example it works nice with default image size and current example for rendering models but if you'd like to use your own model the result will be different and you have to modify some code to make the result better.