# SPDX-License-Identifier: GPL-2.0-only
# Included by rust/Makefile; these are built-in declarations, just like softirq.o.
targets += bindings/softirq_generated.rs
always-$(CONFIG_RUST_SOFTIRQ) += bindings/softirq_generated.rs
$(obj)/bindings/softirq_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/softirq_bindgen_parameters)
$(obj)/bindings/softirq_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/softirq_generated.rs: $(srctree)/kernel/softirq_bindings.h \
    $(srctree)/kernel/softirq_bindgen_parameters $(srctree)/kernel/softirq_layout.h FORCE
	$(call if_changed_dep,bindgen)
