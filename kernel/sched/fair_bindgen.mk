# SPDX-License-Identifier: GPL-2.0
# Proposed rust/Makefile include; not run during SOURCE phase.
targets += bindings/sched_fair_generated.rs
always-$(CONFIG_RUST_SCHED_FAIR) += bindings/sched_fair_generated.rs
$(obj)/bindings/sched_fair_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/sched/fair_bindgen_parameters)
$(obj)/bindings/sched_fair_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_fair_generated.rs: $(srctree)/kernel/sched/fair_native_bindings.h \
    $(srctree)/kernel/sched/fair_private.h \
    $(srctree)/kernel/sched/fair_constants.h \
    $(srctree)/kernel/sched/fair_native_primitives.inc \
    $(srctree)/kernel/sched/fair_foundation_warnings.inc \
    $(srctree)/kernel/sched/fair_tail_warnings.inc \
    $(srctree)/kernel/sched/fair_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)

# sched_fair_native is emitted only for the admitted owner configuration.
ifeq ($(CONFIG_RUST_SCHED_FAIR),y)
$(obj)/bindings.o: $(obj)/bindings/sched_fair_generated.rs
endif
