# SPDX-License-Identifier: GPL-2.0-only
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_UCOUNT),y)
$(error SOURCE ONLY HOLD: Rust user-accounting native bindings are unqualified)
endif
targets += bindings/ucount_native_generated.rs
always-$(CONFIG_RUST_UCOUNT) += bindings/ucount_native_generated.rs
$(obj)/bindings/ucount_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/ucount_native_bindgen_parameters)
$(obj)/bindings/ucount_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/ucount_native_generated.rs: $(srctree)/kernel/ucount_native_bindings.h \
    $(srctree)/kernel/ucount_native_primitives.inc \
    $(srctree)/kernel/ucount_native_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_UCOUNT),y)
$(obj)/bindings.o: $(obj)/bindings/ucount_native_generated.rs
endif
