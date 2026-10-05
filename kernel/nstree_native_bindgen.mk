# SPDX-License-Identifier: GPL-2.0-only
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_NSTREE),y)
$(error SOURCE ONLY HOLD: Rust namespace-tree native bindings are unqualified)
endif
targets += bindings/nstree_native_generated.rs
always-$(CONFIG_RUST_NSTREE) += bindings/nstree_native_generated.rs
$(obj)/bindings/nstree_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/nstree_bindgen_parameters)
$(obj)/bindings/nstree_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/nstree_native_generated.rs: $(srctree)/kernel/nstree_native_bindings.h \
    $(srctree)/kernel/nstree_native_primitives.inc \
    $(srctree)/kernel/nstree_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_NSTREE),y)
$(obj)/bindings.o: $(obj)/bindings/nstree_native_generated.rs
endif
