# SPDX-License-Identifier: GPL-2.0
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_GROUPS),y)
$(error SOURCE ONLY HOLD: Rust supplementary-group native bindings are unqualified)
endif
targets += bindings/groups_native_generated.rs
always-$(CONFIG_RUST_GROUPS) += bindings/groups_native_generated.rs
$(obj)/bindings/groups_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/groups_native_bindgen_parameters)
$(obj)/bindings/groups_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/groups_native_generated.rs: $(srctree)/kernel/groups_native_bindings.h \
    $(srctree)/kernel/groups_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_GROUPS),y)
$(obj)/bindings.o: $(obj)/bindings/groups_native_generated.rs
endif
