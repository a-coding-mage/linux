# SPDX-License-Identifier: GPL-2.0
# SOURCE ONLY. This file describes dependencies without admitting execution.
ifeq ($(CONFIG_RUST_SCHED_RT),y)
$(error Rust RT scheduler is source-only; configured ABI/native policy qualification is incomplete)
endif

targets += bindings/sched_rt_generated.rs
always-$(CONFIG_RUST_SCHED_RT) += bindings/sched_rt_generated.rs
$(obj)/bindings/sched_rt_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/sched/rt_bindgen_parameters)
$(obj)/bindings/sched_rt_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_rt_generated.rs: $(srctree)/kernel/sched/rt_native_bindings.h \
    $(srctree)/kernel/sched/rt_native_constants.h \
    $(srctree)/kernel/sched/rt_class_callbacks.h \
    $(srctree)/kernel/sched/rt_native_primitives.inc \
    $(srctree)/kernel/sched/rt_native_diagnostics.inc \
    $(srctree)/kernel/sched/rt_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_RT),y)
$(obj)/bindings.o: $(obj)/bindings/sched_rt_generated.rs
endif
