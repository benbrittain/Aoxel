tmp=_git_distcheck
aoxel_bin_path=bin
glfw_path=~/workspace/glfw-rs
glfw_lib_path=~/workspace/glfw-rs/build/lib/
gl_lib_path=~/workspace/gl-rs/build/x86_64-unknown-linux-gnu/gl
cgmath_lib_path=~/workspace/cgmath-rs/lib
stb_image_lib_path=lib/rust-stb-image/
link_args="-lglfw"
local_libs=$(shell find src/ -type f -name '*.rs')

libs=-L $(glfw_lib_path) -L $(gl_lib_path) -L $(cgmath_lib_path) -L $(stb_image_lib_path)

all:
	mkdir -p $(aoxel_bin_path)
	rustc src/main.rs --opt-level 3 -o $(aoxel_bin_path)/client $(libs) --link-args=$(link_args)

distcheck:
	rm -rf $(tmp)
	git clone --recursive . $(tmp)
	make -C $(tmp) deps
	make -C $(tmp)
	make -C $(tmp) examples
	rm -rf $(tmp)

deps:
	cd $(glfw_path); cmake .; make lib
	make -C lib/cgmath-rs
	cd lib/gl-rs; rustc --opt-level=3 src/gl/lib.rs
	cd lib/rust-stb-image; ./configure
	make clean -C lib/rust-stb-image
	make -C lib/rust-stb-image
