# SPDX-License-Identifier: GPL-2.0
targets += bindings/vmstat_native_generated.rs
always-$(CONFIG_RUST_VMSTAT) += bindings/vmstat_native_generated.rs
$(obj)/bindings/vmstat_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/vmstat_native_bindgen_parameters)
$(obj)/bindings/vmstat_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/vmstat_native_generated.rs: $(srctree)/mm/vmstat_native_bindings.h \
    $(srctree)/mm/vmstat_native_includes.h $(srctree)/mm/vmstat_native_primitives.h \
    $(srctree)/mm/vmstat_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_VMSTAT),y)
# This generated native module lives in the existing bindings crate, inheriting
# that crate's existing generated-code policy without relaxing owner warnings.
$(obj)/bindings.o: $(obj)/bindings/vmstat_native_generated.rs
rusttestlib-bindings: $(obj)/bindings/vmstat_native_generated.rs
endif
