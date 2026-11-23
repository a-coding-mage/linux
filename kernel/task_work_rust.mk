# SPDX-License-Identifier: GPL-2.0
# The existing obj-y entry owns task_work.o; the Rust owner takes that slot.
ifeq ($(CONFIG_RUST_TASK_WORK),y)
obj-y := $(patsubst task_work.o,task_work.o task_work_primitives.o,$(obj-y))

# The native companion's warning and cond_resched sites keep the original
# translation unit's __FILE__, as -fmacro-prefix-map spells it for task_work.c.
CFLAGS_task_work_primitives.o += -DLUPOS_TASK_WORK_ORIGINAL_FILE='"kernel/task_work.c"'

RUSTFLAGS_task_work.o += -Zfunction-sections=n
$(obj)/task_work.o: $(src)/task_work.rs $(objtree)/rust/bindings/task_work_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
