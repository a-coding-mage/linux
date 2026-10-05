# SPDX-License-Identifier: GPL-2.0-only
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_NOTIFIER),y)
$(error SOURCE ONLY HOLD: Rust notifier native bindings are unqualified)
endif
targets += bindings/notifier_native_generated.rs
always-$(CONFIG_RUST_NOTIFIER) += bindings/notifier_native_generated.rs
$(obj)/bindings/notifier_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/notifier_native_bindgen_parameters)
$(obj)/bindings/notifier_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/notifier_native_generated.rs: $(srctree)/kernel/notifier_native_bindings.h \
    $(srctree)/kernel/notifier_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_NOTIFIER),y)
$(obj)/bindings.o: $(obj)/bindings/notifier_native_generated.rs
endif
