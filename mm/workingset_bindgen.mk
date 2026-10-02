# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile to use the canonical, target-configured bindgen rule.
targets += bindings/workingset_generated.rs
always-$(CONFIG_RUST_WORKINGSET) += bindings/workingset_generated.rs
$(obj)/bindings/workingset_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/workingset_bindgen_parameters)
$(obj)/bindings/workingset_generated.rs: $(srctree)/mm/workingset_bindings.h \
    $(srctree)/mm/workingset_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
