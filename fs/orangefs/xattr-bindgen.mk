# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile; configured C flags and canonical bindgen rule.
targets += bindings/orangefs_xattr_generated.rs
always-$(CONFIG_RUST_ORANGEFS_XATTR) += bindings/orangefs_xattr_generated.rs
$(obj)/bindings/orangefs_xattr_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/fs/orangefs/xattr-bindgen-parameters)
$(obj)/bindings/orangefs_xattr_generated.rs: $(srctree)/fs/orangefs/xattr-bindings.h \
    $(srctree)/fs/orangefs/xattr-bindgen-parameters FORCE
	$(call if_changed_dep,bindgen)
