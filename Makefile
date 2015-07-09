.PHONY: build release run cargo

arch := x86_64

# names of assembly files
assembly := $(wildcard src/arch/$(arch)/*.asm) $(wildcard src/arch/$(arch)/*/*.asm)
# target object files
object_files := $(patsubst src/arch/$(arch)/%.asm, build/arch/%.o, $(assembly))

# the name of the produced rust library
cargo_lib = src/main/target/$(debug)/$(shell ls src/main/target/$(debug) | grep librustos | head -1)
rustos := build/isofiles/boot/rustos.bin
rustos-debug := build/isofiles/boot/rustos.debug
grub_cfg := build/isofiles/boot/grub/grub.cfg
iso := build/rustos.iso
qemu_args := -net nic,model=rtl8139 -net user

build: debug = debug
build: cargo_command = cargo rustc -- -C relocation-model=pic
build: iso

run: build
	@qemu-system-$(arch) -s -hda $(iso) $(qemu_args)

debug: build
	@qemu-system-$(arch) -s -S -hda $(iso) $(qemu_args)

release-run: release
	@qemu-system-$(arch) -hda $(iso) $(qemu_args)

release: cargo_command = cargo rustc --release -- -C relocation-model=pic
release: debug = release
release: iso

clean:
	-@rm -r build 2> /dev/null || true
	@cd src/main; cargo clean

# create the rustos iso
iso: $(rustos) $(grub_cfg)
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null

# copy grub config
$(grub_cfg): src/grub/grub.cfg
	@mkdir -p $(shell dirname $@)
	@cp $< $@

# link rustos
$(rustos): src/arch/$(arch)/linker.ld cargo $(object_files)
	@mkdir -p $(shell dirname $(rustos))
	@ld -T $< $(object_files) $(cargo_lib) -o $(rustos-debug)
	@objcopy --strip-debug $(rustos-debug) $@

cargo:
	@cd src/main; $(cargo_command)

# compile assembly files
build/arch/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
