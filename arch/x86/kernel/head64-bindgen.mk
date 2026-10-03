# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile; cmd_bindgen is provided under CONFIG_RUST.
targets += bindings/head64_generated.rs
always-$(CONFIG_RUST_X86_HEAD64) += bindings/head64_generated.rs
$(obj)/bindings/head64_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/arch/x86/kernel/head64_bindgen_parameters)
# This is a built-in early-boot owner. cmd_bindgen adds -DMODULE by default.
$(obj)/bindings/head64_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/head64_generated.rs: $(srctree)/arch/x86/kernel/head64_bindings.h \
    $(srctree)/arch/x86/kernel/head64_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
