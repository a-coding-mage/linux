# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_EXT_ARENA),y)
$(error SOURCE ONLY HOLD: Rust sched_ext arena bindings are unqualified)
endif
targets += bindings/sched_ext_arena_generated.rs
always-$(CONFIG_RUST_SCHED_EXT_ARENA) += bindings/sched_ext_arena_generated.rs
$(obj)/bindings/sched_ext_arena_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/ext/sched_ext_arena_bindgen_parameters)
$(obj)/bindings/sched_ext_arena_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_ext_arena_generated.rs: $(srctree)/kernel/sched/ext/sched_ext_arena_bindings.h \
    $(srctree)/kernel/sched/ext/internal.h $(srctree)/kernel/sched/ext/arena.h \
    $(srctree)/kernel/sched/ext/sched_ext_arena_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_EXT_ARENA),y)
$(obj)/bindings.o: $(obj)/bindings/sched_ext_arena_generated.rs
endif
