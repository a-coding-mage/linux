# SPDX-License-Identifier: GPL-2.0
# Configured declarations for the Rust task-work owner.
targets += bindings/task_work_generated.rs
always-$(CONFIG_RUST_TASK_WORK) += bindings/task_work_generated.rs
$(obj)/bindings/task_work_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/task_work_bindgen_parameters)
$(obj)/bindings/task_work_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/task_work_generated.rs: $(srctree)/kernel/task_work_bindings.h \
    $(srctree)/kernel/task_work_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_TASK_WORK),y)
$(obj)/bindings.o: $(obj)/bindings/task_work_generated.rs
endif
