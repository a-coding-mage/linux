# SPDX-License-Identifier: GPL-2.0-only
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_NSPROXY),y)
$(error SOURCE ONLY HOLD: Rust namespace-transition bindings are unqualified)
endif
targets += bindings/nsproxy_generated.rs
always-$(CONFIG_RUST_NSPROXY) += bindings/nsproxy_generated.rs
$(obj)/bindings/nsproxy_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/nsproxy_bindgen_parameters)
$(obj)/bindings/nsproxy_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/nsproxy_generated.rs: $(srctree)/kernel/nsproxy_native_bindings.h \
    $(srctree)/kernel/nsproxy_native_primitives.inc \
    $(srctree)/kernel/nsproxy_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_NSPROXY),y)
$(obj)/bindings.o: $(obj)/bindings/nsproxy_generated.rs
endif
