# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile to retain the canonical target bindgen flags.
targets += bindings/public_key_generated.rs
always-$(CONFIG_RUST_PUBLIC_KEY) += bindings/public_key_generated.rs
$(obj)/bindings/public_key_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/crypto/asymmetric_keys/public_key_bindgen_parameters)
$(obj)/bindings/public_key_generated.rs: $(srctree)/crypto/asymmetric_keys/public_key_bindings.h \
    $(srctree)/crypto/asymmetric_keys/public_key_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
