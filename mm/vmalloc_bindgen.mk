# SPDX-License-Identifier: GPL-2.0-only
# Include from rust/Makefile. All native types come from the current configuration.
targets += bindings/vmalloc_generated.rs
always-$(CONFIG_RUST_VMALLOC) += bindings/vmalloc_generated.rs
$(obj)/bindings/vmalloc_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/vmalloc_bindgen_parameters)
$(obj)/bindings/vmalloc_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/vmalloc_generated.rs: $(srctree)/mm/vmalloc_bindings.h \
    $(srctree)/mm/vmalloc_constants.h $(srctree)/mm/vmalloc_private_decls.h \
    $(srctree)/mm/vmalloc_primitives.inc $(srctree)/mm/vmalloc_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
