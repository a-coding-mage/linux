# SPDX-License-Identifier: GPL-2.0
# Proposed rust/Makefile include only. Not wired or executed in SOURCE phase.
ifeq ($(CONFIG_RUST_SCHED_DEADLINE),y)
$(error Rust deadline bindings are source-only; native ABI and policy qualification remain open)
endif
targets += bindings/sched_deadline_generated.rs
always-$(CONFIG_RUST_SCHED_DEADLINE) += bindings/sched_deadline_generated.rs
$(obj)/bindings/sched_deadline_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/sched/deadline_bindgen_parameters)
$(obj)/bindings/sched_deadline_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_deadline_generated.rs: $(srctree)/kernel/sched/deadline_native_bindings.h \
    $(srctree)/kernel/sched/deadline_native_primitives.inc \
    $(srctree)/kernel/sched/deadline_runqueue_primitives.inc \
    $(srctree)/kernel/sched/deadline_migration_primitives.inc \
    $(srctree)/kernel/sched/deadline_lifecycle_primitives.inc \
    $(srctree)/kernel/sched/deadline_warnings.inc \
    $(srctree)/kernel/sched/deadline_runqueue_warnings.inc \
    $(srctree)/kernel/sched/deadline_migration_warnings.inc \
    $(srctree)/kernel/sched/deadline_lifecycle_warnings.inc \
    $(srctree)/kernel/sched/deadline_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)

ifeq ($(CONFIG_RUST_SCHED_DEADLINE),y)
$(obj)/bindings.o: $(obj)/bindings/sched_deadline_generated.rs
endif
