# SPDX-License-Identifier: GPL-2.0-only
# Integration proposal: include from rust/Makefile after cmd_bindgen is defined.
# CONFIG_RUST_X86_PLATFORM_CALLBACKS must be selected by reviewed owner wiring.
ifeq ($(CONFIG_RUST_X86_PLATFORM_CALLBACKS),y)
targets += bindings/x86_platform_callbacks_generated.rs
always-y += bindings/x86_platform_callbacks_generated.rs
$(obj)/bindings/x86_platform_callbacks_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/arch/x86/kernel/x86-platform-callbacks_bindgen_parameters)
# Built-in owner: cancel cmd_bindgen's default -DMODULE.
$(obj)/bindings/x86_platform_callbacks_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/x86_platform_callbacks_generated.rs: \
    $(srctree)/arch/x86/kernel/x86-platform-callbacks_bindings.h \
    $(srctree)/arch/x86/kernel/x86-platform-callbacks_bindgen_parameters \
    $(objtree)/include/generated/autoconf.h FORCE
	$(call if_changed_dep,bindgen)
endif
