# SPDX-License-Identifier: GPL-2.0-or-later
# Include from rust/Makefile; uses the kernel's configured clang/bindgen flags.
targets += bindings/memblock_generated.rs
always-$(CONFIG_RUST_MEMBLOCK) += bindings/memblock_generated.rs
$(obj)/bindings/memblock_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/mm/memblock_bindgen_parameters)
$(obj)/bindings/memblock_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/memblock_generated.rs: $(srctree)/mm/memblock_bindings.h \
    $(srctree)/mm/memblock_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
