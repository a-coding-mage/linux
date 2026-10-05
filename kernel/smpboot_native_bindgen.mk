# SPDX-License-Identifier: GPL-2.0-only
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_SMPBOOT),y)
$(error SOURCE ONLY HOLD: Rust smpboot native bindings are unqualified)
endif
targets += bindings/smpboot_native_generated.rs
always-$(CONFIG_RUST_SMPBOOT) += bindings/smpboot_native_generated.rs
$(obj)/bindings/smpboot_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/smpboot_native_bindgen_parameters)
$(obj)/bindings/smpboot_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/smpboot_native_generated.rs: $(srctree)/kernel/smpboot_native_bindings.h \
    $(srctree)/kernel/smpboot_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SMPBOOT),y)
$(obj)/bindings.o: $(obj)/bindings/smpboot_native_generated.rs
endif
