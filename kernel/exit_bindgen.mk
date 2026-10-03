# SPDX-License-Identifier: GPL-2.0-only
# Canonical built-in configuration, not a MODULE binding approximation.
targets += bindings/exit_generated.rs
always-$(CONFIG_RUST_EXIT) += bindings/exit_generated.rs
$(obj)/bindings/exit_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/exit_bindgen_parameters)
$(obj)/bindings/exit_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/exit_generated.rs: $(srctree)/kernel/exit_bindings.h \
    $(srctree)/kernel/exit_includes.h $(srctree)/kernel/exit_primitives.inc \
    $(srctree)/kernel/exit_layout.h $(srctree)/kernel/exit_bindgen_parameters \
    $(srctree)/kernel/exit.h FORCE
	$(call if_changed_dep,bindgen)
