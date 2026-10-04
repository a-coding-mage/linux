# SPDX-License-Identifier: GPL-2.0-only
targets += bindings/mremap_native_generated.rs
always-$(CONFIG_RUST_MREMAP) += bindings/mremap_native_generated.rs
$(obj)/bindings/mremap_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/mremap_native_bindgen_parameters)
$(obj)/bindings/mremap_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/mremap_native_generated.rs: $(srctree)/mm/mremap_native_bindings.h \
    $(srctree)/mm/mremap_native_includes.h $(srctree)/mm/mremap_native_primitives.h \
    $(srctree)/mm/mremap_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
