# SPDX-License-Identifier: GPL-2.0-only
# Included by rust/Makefile after cmd_bindgen is defined.
ifeq ($(CONFIG_RUST_X86_SETUP),y)
targets += bindings/x86_setup_generated.rs bindings/x86_setup_generated.c
always-y += bindings/x86_setup_generated.rs bindings/x86_setup_generated.c
quiet_cmd_x86_setup_bindgen = BINDGEN $@
      cmd_x86_setup_bindgen = \
	$(BINDGEN) $(srctree)/arch/x86/kernel/setup_bindings.h \
		$(shell grep -Ev '^\#|^$$' $(srctree)/arch/x86/kernel/setup_bindgen_parameters) \
		--wrap-static-fns-path $(obj)/bindings/x86_setup_generated \
		--rust-target 1.85 --use-core --with-derive-default --ctypes-prefix ffi \
		--no-layout-tests --no-debug '.*' --enable-function-attribute-detection \
		-o $(obj)/bindings/x86_setup_generated.rs -- $(bindgen_c_flags_final) -DMODULE -UMODULE && \
	$(PERL) $(srctree)/scripts/rust_setup_wrapper_sections.pl $(obj)/bindings/x86_setup_generated.c
# GNU make 4.0 multi-target pattern rule, as used for syncconfig.
$(obj)/bindings/x86_setup_%.rs $(obj)/bindings/x86_setup_%.c: \
    $(srctree)/arch/x86/kernel/setup_bindings.h \
    $(srctree)/arch/x86/kernel/setup_bindgen_parameters \
    $(srctree)/scripts/rust_setup_wrapper_sections.pl $(objtree)/include/generated/autoconf.h FORCE
	$(call if_changed_dep,x86_setup_bindgen)
endif
