.PHONY: build release run cargo

# names of assembly files
assembly = bootstrap/boot
# target object files
object_files = build/object_files/$(assembly).o

# the name of the produced rust library
cargo_lib = target/$(debug)/$(shell ls target/$(debug) | grep librustos | head -1)
rustos = build/isofiles/boot/rustos.bin
grub_cfg = build/isofiles/boot/grub/grub.cfg
iso = build/rustos.iso

build: debug = debug
build: cargo_command = cargo build
build: iso

run: build
	@qemu-system-x86_64 -hda $(iso)

release: cargo_command = cargo build --release
release: debug = release
release: iso

clean: 
	@rm -r build
	@rm -r target

# create the rustos iso
iso: $(rustos) $(grub_cfg)
	@grub-mkrescue -o $(iso) build/isofiles

# copy grub config
$(grub_cfg): src/grub/grub.cfg
	@mkdir -p $(shell dirname $@)
	@cp $< $@

# link rustos
$(rustos): src/arch/x86_64/linker.ld cargo $(object_files) 
	@mkdir -p $(shell dirname $(rustos))
	@ld -T $< $(object_files) $(cargo_lib) -o $@

cargo: 
	@$(cargo_command) 

# compile assembly files
$(object_files): src/arch/x86_64/$(assembly).asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@