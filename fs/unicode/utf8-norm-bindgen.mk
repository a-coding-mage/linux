# SPDX-License-Identifier: GPL-2.0
# Independent of the core selector; use the canonical configured bindgen rule.
targets += bindings/utf8_norm_generated.rs
always-$(CONFIG_RUST_UNICODE_NORM) += bindings/utf8_norm_generated.rs
$(obj)/bindings/utf8_norm_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/fs/unicode/utf8-norm-bindgen-parameters)
$(obj)/bindings/utf8_norm_generated.rs: $(srctree)/fs/unicode/utf8-norm-bindings.h \
    $(srctree)/fs/unicode/utf8-norm-bindgen-parameters FORCE
	$(call if_changed_dep,bindgen)
