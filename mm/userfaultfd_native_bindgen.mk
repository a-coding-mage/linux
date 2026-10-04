# SPDX-License-Identifier: GPL-2.0-only
targets += bindings/userfaultfd_native_generated.rs
always-$(CONFIG_RUST_USERFAULTFD) += bindings/userfaultfd_native_generated.rs
$(obj)/bindings/userfaultfd_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/userfaultfd_native_bindgen_parameters)
$(obj)/bindings/userfaultfd_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/userfaultfd_native_generated.rs: $(srctree)/mm/userfaultfd_native_bindings.h \
    $(srctree)/mm/userfaultfd_native_includes.h $(srctree)/mm/userfaultfd_native_primitives.h \
    $(srctree)/mm/userfaultfd_native_warnings.h \
    $(srctree)/mm/userfaultfd_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
