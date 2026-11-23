# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns nstree.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_NSTREE),y)
obj-y := $(patsubst nstree.o,nstree.o nstree_native.o,$(obj-y))
RUSTFLAGS_nstree.o += -Zfunction-sections=n
$(obj)/nstree.o: $(src)/nstree.rs $(objtree)/rust/bindings/nstree_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
