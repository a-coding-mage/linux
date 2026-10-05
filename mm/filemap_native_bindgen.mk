# SPDX-License-Identifier: GPL-2.0
targets += bindings/filemap_native_generated.rs
always-$(CONFIG_RUST_FILEMAP) += bindings/filemap_native_generated.rs
$(obj)/bindings/filemap_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/filemap_native_bindgen_parameters)
$(obj)/bindings/filemap_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/filemap_native_generated.rs: $(srctree)/mm/filemap_native_bindings.h \
    $(srctree)/mm/filemap_native_includes.h $(srctree)/mm/filemap_native_primitives.h \
    $(srctree)/mm/filemap_native_alloc_sites.h \
    $(srctree)/mm/filemap_native_diagnostics.h \
    $(srctree)/mm/filemap_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)

ifeq ($(CONFIG_RUST_FILEMAP),y)
$(obj)/bindings.o: $(obj)/bindings/filemap_native_generated.rs
rusttestlib-bindings: $(obj)/bindings/filemap_native_generated.rs
endif
