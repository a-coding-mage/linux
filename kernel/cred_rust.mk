# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns cred.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_CRED),y)
obj-y := $(patsubst cred.o,cred.o cred_native_primitives.o,$(obj-y))
RUSTFLAGS_cred.o += -Zfunction-sections=n
$(obj)/cred.o: $(src)/cred.rs $(objtree)/rust/bindings/cred_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
