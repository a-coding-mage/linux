# SPDX-License-Identifier: GPL-2.0
# Use rust/Makefile's canonical bindgen command and configured C flags.
targets += bindings/utf8_core_generated.rs
always-$(CONFIG_RUST_UNICODE_CORE) += bindings/utf8_core_generated.rs
$(obj)/bindings/utf8_core_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/fs/unicode/utf8-core-bindgen-parameters)
$(obj)/bindings/utf8_core_generated.rs: $(srctree)/fs/unicode/utf8-core-bindings.h \
    $(srctree)/fs/unicode/utf8-core-bindgen-parameters FORCE
	$(call if_changed_dep,bindgen)
