# SPDX-License-Identifier: GPL-2.0
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_STOP_TASK),y)
$(error SOURCE ONLY HOLD: Rust scheduler stop-task bindings are unqualified)
endif
targets += bindings/sched_stop_task_generated.rs
always-$(CONFIG_RUST_SCHED_STOP_TASK) += bindings/sched_stop_task_generated.rs
$(obj)/bindings/sched_stop_task_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_stop_task_bindgen_parameters)
$(obj)/bindings/sched_stop_task_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_stop_task_generated.rs: $(srctree)/kernel/sched/sched_stop_task_bindings.h \
    $(srctree)/kernel/sched/sched_stop_task_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_STOP_TASK),y)
$(obj)/bindings.o: $(obj)/bindings/sched_stop_task_generated.rs
endif
