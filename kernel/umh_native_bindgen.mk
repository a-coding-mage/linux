# SPDX-License-Identifier: GPL-2.0-only
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_UMH),y)
$(error SOURCE ONLY HOLD: Rust usermode-helper native bindings are unqualified)
endif
targets += bindings/umh_native_generated.rs
always-$(CONFIG_RUST_UMH) += bindings/umh_native_generated.rs
$(obj)/bindings/umh_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/umh_native_bindgen_parameters)
$(obj)/bindings/umh_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/umh_native_generated.rs: $(srctree)/kernel/umh_native_bindings.h \
    $(srctree)/kernel/umh_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_UMH),y)
$(obj)/bindings.o: $(obj)/bindings/umh_native_generated.rs
endif
