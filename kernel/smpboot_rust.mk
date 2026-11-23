# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns smpboot.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_SMPBOOT),y)
obj-y := $(patsubst smpboot.o,smpboot.o smpboot_native_metadata.o smpboot_native_primitives.o,$(obj-y))
RUSTFLAGS_smpboot.o += -Zfunction-sections=n
$(obj)/smpboot.o: $(src)/smpboot.rs $(objtree)/rust/bindings/smpboot_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
