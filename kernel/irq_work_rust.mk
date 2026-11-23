# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns irq_work.o; the Rust owner takes that slot and its
# native companions follow it, keeping the original link and initcall order.
ifeq ($(CONFIG_RUST_IRQ_WORK),y)
obj-y := $(patsubst irq_work.o,irq_work.o irq_work_primitives.o,$(obj-y))
# The original irq_work.o is built without ftrace instrumentation.
CFLAGS_REMOVE_irq_work_primitives.o = $(CC_FLAGS_FTRACE)
RUSTFLAGS_irq_work.o += -Zfunction-sections=n
$(obj)/irq_work.o: $(src)/irq_work.rs $(objtree)/rust/bindings/irq_work_native_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
