# SPDX-License-Identifier: GPL-2.0-only
targets += bindings/memory_native_generated.rs
always-$(CONFIG_RUST_MEMORY) += bindings/memory_native_generated.rs
$(obj)/bindings/memory_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/memory_native_bindgen_parameters)
$(obj)/bindings/memory_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/memory_native_generated.rs: $(srctree)/mm/memory_native_bindings.h \
    $(srctree)/mm/memory_native_includes.h $(srctree)/mm/memory_native_primitives.h \
    $(srctree)/mm/memory_diagnostics_primitives.h \
    $(srctree)/mm/memory_faults_primitives.h \
    $(srctree)/mm/memory_faults_constants.h \
    $(srctree)/mm/memory_faults_diagnostics.h \
    $(srctree)/mm/memory_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
