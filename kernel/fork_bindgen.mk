# SPDX-License-Identifier: GPL-2.0-only
targets += bindings/fork_generated.rs
always-$(CONFIG_RUST_FORK) += bindings/fork_generated.rs
$(obj)/bindings/fork_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/fork_bindgen_parameters)
$(obj)/bindings/fork_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/fork_generated.rs: $(srctree)/kernel/fork_bindings.h \
    $(srctree)/kernel/fork_includes.h $(srctree)/kernel/fork_storage_types.h \
    $(srctree)/kernel/fork_header_primitives.h $(srctree)/kernel/fork_task_primitives.h \
    $(srctree)/kernel/fork_mm_primitives.h $(srctree)/kernel/fork_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
