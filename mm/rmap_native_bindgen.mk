# SPDX-License-Identifier: GPL-2.0
targets += bindings/rmap_native_generated.rs
always-$(CONFIG_RUST_RMAP) += bindings/rmap_native_generated.rs
$(obj)/bindings/rmap_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/rmap_native_bindgen_parameters)
$(obj)/bindings/rmap_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/rmap_native_generated.rs: $(srctree)/mm/rmap_native_bindings.h \
    $(srctree)/mm/rmap_native_includes.h $(srctree)/mm/rmap_native_primitives.h \
    $(srctree)/mm/rmap_native_diagnostics.h \
    $(srctree)/mm/rmap_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
