# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns groups.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_GROUPS),y)
obj-y := $(patsubst groups.o,groups.o groups_native_metadata.o groups_native_primitives.o,$(obj-y))
RUSTFLAGS_groups.o += -Zfunction-sections=n
$(obj)/groups.o: $(src)/groups.rs $(objtree)/rust/bindings/groups_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
