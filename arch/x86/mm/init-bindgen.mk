# SPDX-License-Identifier: GPL-2.0-only
# Include from rust/Makefile after cmd_bindgen is defined.
targets += bindings/x86_mm_init_generated.rs
always-$(CONFIG_RUST_X86_MM_INIT) += bindings/x86_mm_init_generated.rs
$(obj)/bindings/x86_mm_init_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/arch/x86/mm/init_bindgen_parameters)
$(obj)/bindings/x86_mm_init_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/x86_mm_init_generated.rs: \
    $(srctree)/arch/x86/mm/init_bindings.h \
    $(srctree)/arch/x86/mm/init_primitives.def \
    $(srctree)/arch/x86/mm/init_bindgen_parameters \
    $(objtree)/include/generated/autoconf.h FORCE
	$(call if_changed_dep,bindgen)
