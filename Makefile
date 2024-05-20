test:
	cargo test

dots:
	cargo run -- dots --width 10 --height 10 --source examples/dots/source.txt --target examples/dots/target.tga

lines:
	cargo run -- lines --width 100 --height 100 --source examples/lines/source.txt --target examples/lines/target.tga

triangles:
	cargo run -- triangles --width 30 --height 30 --source examples/triangles/source.txt --target examples/triangles/target.tga

triangles_colored:
	cargo run -- triangles --width 30 --height 30 --source examples/triangles/source.txt --target examples/triangles/target_colored.tga --filled

skeleton:
	cargo run -- skeleton --source examples/skeleton/head.obj --target examples/skeleton/target.tga white

model:
	cargo run -- model --source examples/model/head.obj --target examples/model/target.tga

model_textured:
	cargo run -- model --source examples/model/head.obj --target examples/model/target_textured.tga --texture examples/model/head_diffuse.tga