# SPDX-License-Identifier: GPL-2.0-only
# Included by rust/Makefile. These are built-in definitions, never MODULE views.
targets += bindings/sched_idle_generated.rs bindings/sched_clock_generated.rs
always-$(CONFIG_RUST_SCHED_IDLE) += bindings/sched_idle_generated.rs
always-$(CONFIG_RUST_SCHED_CLOCK) += bindings/sched_clock_generated.rs
$(obj)/bindings/sched_idle_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/sched/sched_idle_bindgen_parameters)
$(obj)/bindings/sched_idle_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_idle_generated.rs: $(srctree)/kernel/sched/sched_idle_bindings.h \
    $(srctree)/kernel/sched/sched_idle_layout.h \
    $(srctree)/kernel/sched/sched_idle_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
$(obj)/bindings/sched_clock_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/sched/sched_clock_bindgen_parameters)
$(obj)/bindings/sched_clock_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_clock_generated.rs: $(srctree)/kernel/sched/sched_clock_bindings.h \
    $(srctree)/kernel/sched/sched_clock_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
