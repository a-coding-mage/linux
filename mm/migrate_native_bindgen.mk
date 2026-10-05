# SPDX-License-Identifier: GPL-2.0
targets += bindings/migrate_native_generated.rs
always-$(CONFIG_RUST_MIGRATE) += bindings/migrate_native_generated.rs
$(obj)/bindings/migrate_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/migrate_native_bindgen_parameters)
$(obj)/bindings/migrate_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/migrate_native_generated.rs: $(srctree)/mm/migrate_native_bindings.h \
    $(srctree)/mm/migrate_native_types.h $(srctree)/mm/migrate_native_constants.h \
    $(srctree)/mm/migrate_native_primitives.def $(srctree)/mm/migrate_native_diagnostics.def \
    $(srctree)/mm/migrate_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
