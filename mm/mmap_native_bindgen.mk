# SPDX-License-Identifier: GPL-2.0-only
targets += bindings/mmap_native_generated.rs
always-$(CONFIG_RUST_MMAP) += bindings/mmap_native_generated.rs
$(obj)/bindings/mmap_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/mmap_native_bindgen_parameters)
$(obj)/bindings/mmap_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/mmap_native_generated.rs: $(srctree)/mm/mmap_native_bindings.h \
    $(srctree)/mm/mmap_native_includes.h $(srctree)/mm/mmap_native_primitives.h \
    $(srctree)/mm/mmap_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
