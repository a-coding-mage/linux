# SPDX-License-Identifier: GPL-2.0-only
# Proposed include from rust/Makefile. Generated with configured native headers.
targets += bindings/page_alloc_generated.rs
always-$(CONFIG_RUST_PAGE_ALLOC) += bindings/page_alloc_generated.rs
$(obj)/bindings/page_alloc_generated.rs: private bindgen_target_flags = \
    $(shell grep -hEv '^\#|^$$' $(srctree)/mm/page_alloc_bindgen_parameters $(srctree)/mm/page_alloc_late_bindgen_parameters)
$(obj)/bindings/page_alloc_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/page_alloc_generated.rs: $(srctree)/mm/page_alloc_bindings.h \
    $(srctree)/mm/page_alloc_buddy_helpers.h $(srctree)/mm/page_alloc_late_helpers.h \
    $(srctree)/mm/page_alloc_late_primitives.inc $(srctree)/mm/page_alloc_bindgen_parameters \
    $(srctree)/mm/page_alloc_late_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
