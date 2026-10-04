# SPDX-License-Identifier: GPL-2.0
# Included after the common cmd_bindgen definition in rust/Makefile.
ifeq ($(CONFIG_RUST_X86_RESOURCE),y)
targets += bindings/x86_resource_generated.rs
always-y += bindings/x86_resource_generated.rs
$(obj)/bindings/x86_resource_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/arch/x86/kernel/resource_bindgen_parameters)
# These are built-in owners; cancel the common bindgen command's -DMODULE.
$(obj)/bindings/x86_resource_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/x86_resource_generated.rs: \
    $(srctree)/arch/x86/kernel/resource_bindings.h \
    $(srctree)/arch/x86/kernel/resource_bindgen_parameters \
    $(objtree)/include/generated/autoconf.h FORCE
	$(call if_changed_dep,bindgen)
endif
