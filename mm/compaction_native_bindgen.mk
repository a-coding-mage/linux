# SPDX-License-Identifier: GPL-2.0
targets += bindings/compaction_native_generated.rs
always-$(CONFIG_RUST_COMPACTION) += bindings/compaction_native_generated.rs
$(obj)/bindings/compaction_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/compaction_native_bindgen_parameters)
$(obj)/bindings/compaction_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/compaction_native_generated.rs: $(srctree)/mm/compaction_native_bindings.h \
    $(srctree)/mm/compaction_native_includes.h $(srctree)/mm/compaction_native_primitives.h \
    $(srctree)/mm/compaction_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
