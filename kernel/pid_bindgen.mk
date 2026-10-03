# SPDX-License-Identifier: GPL-2.0-only
# Included by rust/Makefile. Per-TU built-in declarations must match pid.o.
targets += bindings/pid_generated.rs
always-$(CONFIG_RUST_PID) += bindings/pid_generated.rs
$(obj)/bindings/pid_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/pid_bindgen_parameters)
$(obj)/bindings/pid_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/pid_generated.rs: private bindgen_target_extra = && \
    perl $(srctree)/kernel/pid_bindings_adjust.pl $@ > $@.view && \
    mv $@.view $@
$(obj)/bindings/pid_generated.rs: $(srctree)/kernel/pid_bindings.h \
    $(srctree)/kernel/pid_bindgen_parameters $(srctree)/kernel/pid_layout.h $(srctree)/kernel/pid_bindings_adjust.pl FORCE
	$(call if_changed_dep,bindgen)
