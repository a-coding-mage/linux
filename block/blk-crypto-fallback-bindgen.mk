# SPDX-License-Identifier: GPL-2.0
# Configured original C ABI; included privately from rust/Makefile.
targets += bindings/blk_crypto_fallback_generated.rs
always-$(CONFIG_RUST_BLK_CRYPTO_FALLBACK) += bindings/blk_crypto_fallback_generated.rs
$(obj)/bindings/blk_crypto_fallback_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/block/blk-crypto-fallback-bindgen-parameters)
$(obj)/bindings/blk_crypto_fallback_generated.rs: $(srctree)/block/blk-crypto-fallback-bindings.h \
    $(srctree)/block/blk-crypto-fallback-bindgen-parameters FORCE
	$(call if_changed_dep,bindgen)
