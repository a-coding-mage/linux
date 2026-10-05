# SPDX-License-Identifier: GPL-2.0-or-later
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_CRED),y)
$(error SOURCE ONLY HOLD: Rust credential bindings and callback ABI are unqualified)
endif
targets += bindings/cred_native_generated.rs
always-$(CONFIG_RUST_CRED) += bindings/cred_native_generated.rs
$(obj)/bindings/cred_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/cred_native_bindgen_parameters)
$(obj)/bindings/cred_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/cred_native_generated.rs: $(srctree)/kernel/cred_native_bindings.h \
    $(srctree)/kernel/cred_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_CRED),y)
$(obj)/bindings.o: $(obj)/bindings/cred_native_generated.rs
endif
