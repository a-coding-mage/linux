# SPDX-License-Identifier: GPL-2.0-only
# Included by rust/Makefile; declarations use panic.o's built-in environment.
targets += bindings/panic_generated.rs
always-$(CONFIG_RUST_PANIC) += bindings/panic_generated.rs
$(obj)/bindings/panic_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/panic_bindgen_parameters)
$(obj)/bindings/panic_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/panic_generated.rs: $(srctree)/kernel/panic_bindings.h \
    $(srctree)/kernel/panic_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
