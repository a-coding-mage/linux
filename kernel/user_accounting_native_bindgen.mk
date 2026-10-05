# SPDX-License-Identifier: GPL-2.0-only
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_USER_ACCOUNTING),y)
$(error SOURCE ONLY HOLD: Rust user-cache native bindings are unqualified)
endif
targets += bindings/user_accounting_native_generated.rs
always-$(CONFIG_RUST_USER_ACCOUNTING) += bindings/user_accounting_native_generated.rs
$(obj)/bindings/user_accounting_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/user_accounting_native_bindgen_parameters)
$(obj)/bindings/user_accounting_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/user_accounting_native_generated.rs: $(srctree)/kernel/user_accounting_native_bindings.h \
    $(srctree)/kernel/user_accounting_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_USER_ACCOUNTING),y)
$(obj)/bindings.o: $(obj)/bindings/user_accounting_native_generated.rs
endif
