# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns umh.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_UMH),y)
obj-y := $(patsubst umh.o,umh.o umh_native.o umh_abi.o umh_init.o,$(obj-y))
RUSTFLAGS_umh.o += -Zfunction-sections=n
$(obj)/umh.o: $(src)/umh.rs $(objtree)/rust/bindings/umh_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
