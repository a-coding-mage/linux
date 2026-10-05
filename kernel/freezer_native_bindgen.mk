# SPDX-License-Identifier: GPL-2.0-only
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_FREEZER),y)
$(error SOURCE ONLY HOLD: Rust freezer native bindings are unqualified)
endif
targets += bindings/freezer_native_generated.rs
always-$(CONFIG_RUST_FREEZER) += bindings/freezer_native_generated.rs
$(obj)/bindings/freezer_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/freezer_native_bindgen_parameters)
$(obj)/bindings/freezer_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/freezer_native_generated.rs: $(srctree)/kernel/freezer_native_bindings.h \
    $(srctree)/kernel/freezer_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_FREEZER),y)
$(obj)/bindings.o: $(obj)/bindings/freezer_native_generated.rs
endif
