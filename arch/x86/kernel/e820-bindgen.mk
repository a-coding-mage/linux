# SPDX-License-Identifier: GPL-2.0-only
# Proposal: include from rust/Makefile after cmd_bindgen is defined.
targets += bindings/x86_e820_generated.rs
always-$(CONFIG_RUST_X86_E820) += bindings/x86_e820_generated.rs
$(obj)/bindings/x86_e820_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/arch/x86/kernel/e820_bindgen_parameters)
$(obj)/bindings/x86_e820_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/x86_e820_generated.rs: \
    $(srctree)/arch/x86/kernel/e820_bindings.h \
    $(srctree)/arch/x86/kernel/e820_bindgen_parameters \
    $(objtree)/include/generated/autoconf.h FORCE
	$(call if_changed_dep,bindgen)
