# SPDX-License-Identifier: GPL-2.0-only
# Include from rust/Makefile. Do not generate bindings during source phase.
targets += bindings/shmem_native_generated.rs bindings/shmem_native_static.c
always-$(CONFIG_RUST_SHMEM) += bindings/shmem_native_generated.rs
$(obj)/bindings/shmem_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^\#/d; /^$$/d' $(srctree)/mm/shmem_native_bindgen_parameters) \
    --wrap-static-fns-path $(objtree)/rust/bindings/shmem_native_static
$(obj)/bindings/shmem_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/shmem_native_generated.rs: \
    $(srctree)/mm/shmem_native_bindings.h \
    $(srctree)/mm/shmem_native_types.h \
    $(srctree)/mm/shmem_native_includes.h \
    $(srctree)/mm/shmem_native_declarations.h \
    $(srctree)/mm/shmem_native_constants.h \
    $(srctree)/mm/shmem_native_primitives.h \
    $(srctree)/mm/shmem_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
# The .c output is a side effect of the same serialized bindgen invocation.
# Never start a second bindgen process writing the same pair of files.
$(obj)/bindings/shmem_native_static.c: $(obj)/bindings/shmem_native_generated.rs
	@test -f $@ || { echo 'missing shmem bindgen inline output; regenerate binding pair'; exit 1; }

ifeq ($(CONFIG_RUST_SHMEM),y)
# Generate this native module before compiling the existing bindings crate.
$(obj)/bindings.o: $(obj)/bindings/shmem_native_generated.rs
rusttestlib-bindings: $(obj)/bindings/shmem_native_generated.rs
endif
