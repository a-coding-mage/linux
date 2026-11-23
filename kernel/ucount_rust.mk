# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns ucount.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_UCOUNT),y)
obj-y := $(patsubst ucount.o,ucount.o ucount_native.o,$(obj-y))
RUSTFLAGS_ucount.o += -Zfunction-sections=n
$(obj)/ucount.o: $(src)/ucount.rs $(objtree)/rust/bindings/ucount_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
