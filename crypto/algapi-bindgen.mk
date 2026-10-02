# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile to retain the canonical target bindgen flags.
targets += bindings/crypto_algapi_generated.rs
always-$(CONFIG_RUST_CRYPTO_ALGAPI) += bindings/crypto_algapi_generated.rs
$(obj)/bindings/crypto_algapi_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/crypto/algapi-bindgen-parameters)
$(obj)/bindings/crypto_algapi_generated.rs: $(srctree)/crypto/algapi-bindings.h \
    $(srctree)/crypto/algapi-bindgen-parameters FORCE
	$(call if_changed_dep,bindgen)
