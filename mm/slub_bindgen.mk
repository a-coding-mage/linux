# SPDX-License-Identifier: GPL-2.0
# Proposed include from rust/Makefile; no generated output is checked in.
targets += bindings/slub_generated.rs
always-$(CONFIG_RUST_SLUB) += bindings/slub_generated.rs
$(obj)/bindings/slub_generated.rs: private bindgen_target_flags = \
    $(shell grep -hEv '^\#|^$$' $(srctree)/mm/slub_bindgen_parameters)
$(obj)/bindings/slub_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/slub_generated.rs: $(srctree)/mm/slub_bindings.h \
    $(srctree)/mm/slub_late_types.h $(srctree)/mm/slub_allocation_types.h \
    $(srctree)/mm/slub_leaves.inc $(srctree)/mm/slub_allocation_leaves.inc \
    $(srctree)/mm/slub_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
