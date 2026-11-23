# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns notifier.o; the Rust owner takes that slot.
ifeq ($(CONFIG_RUST_NOTIFIER),y)
obj-y := $(patsubst notifier.o,notifier.o notifier_native_primitives.o \
	notifier_native_metadata.o notifier_native_signatures.o,$(obj-y))
RUSTFLAGS_notifier.o += -Zfunction-sections=n
$(obj)/notifier.o: $(src)/notifier.rs $(objtree)/rust/bindings/notifier_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
