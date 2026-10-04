# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile after cmd_bindgen is defined.
targets += bindings/x86_early_platform_generated.rs
always-$(CONFIG_RUST_X86_EARLY_PLATFORM) += bindings/x86_early_platform_generated.rs
$(obj)/bindings/x86_early_platform_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/arch/x86/kernel/early-platform_bindgen_parameters)
# These are built-in owners; the common command otherwise adds -DMODULE.
$(obj)/bindings/x86_early_platform_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/x86_early_platform_generated.rs: \
    $(srctree)/arch/x86/kernel/early-platform_bindings.h \
    $(srctree)/arch/x86/kernel/early-platform_bindgen_parameters \
    $(objtree)/include/generated/autoconf.h FORCE
	$(call if_changed_dep,bindgen)
