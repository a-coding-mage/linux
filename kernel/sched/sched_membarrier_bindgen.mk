# SPDX-License-Identifier: GPL-2.0-or-later
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_MEMBARRIER),y)
$(error SOURCE ONLY HOLD: Rust scheduler membarrier bindings are unqualified)
endif
targets += bindings/sched_membarrier_generated.rs
always-$(CONFIG_RUST_SCHED_MEMBARRIER) += bindings/sched_membarrier_generated.rs
$(obj)/bindings/sched_membarrier_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_membarrier_bindgen_parameters)
$(obj)/bindings/sched_membarrier_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_membarrier_generated.rs: $(srctree)/kernel/sched/sched_membarrier_bindings.h \
    $(srctree)/kernel/sched/sched_membarrier_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_MEMBARRIER),y)
$(obj)/bindings.o: $(obj)/bindings/sched_membarrier_generated.rs
endif
