# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns nsproxy.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_NSPROXY),y)
obj-y := $(patsubst nsproxy.o,nsproxy.o nsproxy_native.o,$(obj-y))
RUSTFLAGS_nsproxy.o += -Zfunction-sections=n
$(obj)/nsproxy.o: $(src)/nsproxy.rs $(objtree)/rust/bindings/nsproxy_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
