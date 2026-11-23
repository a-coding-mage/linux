# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns stop_machine.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_STOP_MACHINE),y)
obj-y := $(patsubst stop_machine.o,stop_machine.o stop_machine_native_metadata.o stop_machine_native_primitives.o,$(obj-y))
RUSTFLAGS_stop_machine.o += -Zfunction-sections=n
$(obj)/stop_machine.o: $(src)/stop_machine.rs $(objtree)/rust/bindings/stop_machine_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
