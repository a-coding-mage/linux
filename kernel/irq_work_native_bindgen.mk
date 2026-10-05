# SPDX-License-Identifier: GPL-2.0-only
# Inactive configured-declaration proposal; no generation has been performed.
ifeq ($(CONFIG_RUST_IRQ_WORK),y)
$(error SOURCE ONLY HOLD: Rust IRQ-work native bindings are unqualified)
endif
targets += bindings/irq_work_native_generated.rs
always-$(CONFIG_RUST_IRQ_WORK) += bindings/irq_work_native_generated.rs
$(obj)/bindings/irq_work_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/irq_work_bindgen_parameters)
$(obj)/bindings/irq_work_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/irq_work_native_generated.rs: $(srctree)/kernel/irq_work_bindings.h \
    $(srctree)/kernel/irq_work_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_IRQ_WORK),y)
$(obj)/bindings.o: $(obj)/bindings/irq_work_native_generated.rs
endif
