# SPDX-License-Identifier: GPL-2.0-only
targets += bindings/mprotect_native_generated.rs
always-$(CONFIG_RUST_MPROTECT) += bindings/mprotect_native_generated.rs
$(obj)/bindings/mprotect_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/mprotect_native_bindgen_parameters)
$(obj)/bindings/mprotect_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/mprotect_native_generated.rs: $(srctree)/mm/mprotect_native_bindings.h \
    $(srctree)/mm/mprotect_native_includes.h $(srctree)/mm/mprotect_native_primitives.h \
    $(srctree)/mm/mprotect_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
