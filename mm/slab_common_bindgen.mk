# SPDX-License-Identifier: GPL-2.0
# Proposed include from rust/Makefile. No generated bindings are checked in.
targets += bindings/slab_common_generated.rs
always-$(CONFIG_RUST_SLAB_COMMON) += bindings/slab_common_generated.rs
$(obj)/bindings/slab_common_generated.rs: private bindgen_target_flags = \
    $(shell grep -hEv '^\#|^$$' $(srctree)/mm/slab_common_bindgen_parameters)
$(obj)/bindings/slab_common_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/slab_common_generated.rs: $(srctree)/mm/slab_common_bindings.h \
    $(srctree)/mm/slab_common_rcu_types.h $(srctree)/mm/slab_common_leaves.inc \
    $(srctree)/mm/slab_common_rcu_leaves.inc \
    $(srctree)/mm/slab_common_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
