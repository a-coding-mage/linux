# SPDX-License-Identifier: GPL-2.0
# Include from rust/Makefile; built-in declarations match the IRQ core owner.
targets += bindings/irq_core_generated.rs
always-$(CONFIG_RUST_IRQ_CORE) += bindings/irq_core_generated.rs
$(obj)/bindings/irq_core_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/irq/irq_core_bindgen_parameters)
$(obj)/bindings/irq_core_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/irq_core_generated.rs: $(srctree)/kernel/irq/irq_core_bindings.h \
    $(srctree)/kernel/irq/irq_core_primitives.inc \
    $(srctree)/kernel/irq/irq_core_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
