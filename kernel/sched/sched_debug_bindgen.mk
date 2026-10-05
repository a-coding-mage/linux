# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_DEBUG),y)
$(error SOURCE ONLY HOLD: Rust scheduler debug bindings are unqualified)
endif
targets += bindings/sched_debug_generated.rs
always-$(CONFIG_RUST_SCHED_DEBUG) += bindings/sched_debug_generated.rs
$(obj)/bindings/sched_debug_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_debug_bindgen_parameters)
$(obj)/bindings/sched_debug_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_debug_generated.rs: $(srctree)/kernel/sched/sched_debug_bindings.h \
    $(srctree)/kernel/sched/sched_debug_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_DEBUG),y)
$(obj)/bindings.o: $(obj)/bindings/sched_debug_generated.rs
endif
