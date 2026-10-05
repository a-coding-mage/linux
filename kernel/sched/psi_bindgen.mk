# SPDX-License-Identifier: GPL-2.0
# Source-stage declaration registration only; no configured generation admitted.
ifeq ($(CONFIG_RUST_SCHED_PSI),y)
$(error Rust PSI bindings are source-only; native ABI and policy remain unqualified)
endif
targets += bindings/sched_psi_generated.rs
always-$(CONFIG_RUST_SCHED_PSI) += bindings/sched_psi_generated.rs
$(obj)/bindings/sched_psi_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/psi_bindgen_parameters)
$(obj)/bindings/sched_psi_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_psi_generated.rs: $(srctree)/kernel/sched/psi_native_bindings.h \
    $(srctree)/kernel/sched/psi_native_primitives.inc \
    $(srctree)/kernel/sched/psi_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_PSI),y)
$(obj)/bindings.o: $(obj)/bindings/sched_psi_generated.rs
endif
