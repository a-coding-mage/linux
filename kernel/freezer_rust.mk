# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns freezer.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_FREEZER),y)
obj-y := $(patsubst freezer.o,freezer.o freezer_native_metadata.o freezer_native_primitives.o,$(obj-y))
RUSTFLAGS_freezer.o += -Zfunction-sections=n
$(obj)/freezer.o: $(src)/freezer.rs $(objtree)/rust/bindings/freezer_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
