# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns user.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_USER_ACCOUNTING),y)
obj-y := $(patsubst user.o,user.o user_accounting_native_metadata.o user_accounting_native_primitives.o user_accounting_native_storage.o,$(obj-y))
RUSTFLAGS_user.o += -Zfunction-sections=n
$(obj)/user.o: $(src)/user.rs $(objtree)/rust/bindings/user_accounting_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
