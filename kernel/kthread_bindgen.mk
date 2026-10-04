# SPDX-License-Identifier: GPL-2.0-only
# Dedicated, built-in native declarations for the selected kthread provider.
targets += bindings/kthread_generated.rs
always-$(CONFIG_RUST_KTHREAD) += bindings/kthread_generated.rs
$(obj)/bindings/kthread_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/kthread_bindgen_parameters)
$(obj)/bindings/kthread_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/kthread_generated.rs: $(srctree)/kernel/kthread_bindings.h \
    $(srctree)/kernel/kthread_bindgen_parameters \
    $(srctree)/kernel/kthread_layout.h $(srctree)/kernel/kthread_warn_sites.inc FORCE
	$(call if_changed_dep,bindgen)
