# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; no generation or native admission in this phase.
ifeq ($(CONFIG_RUST_SCHED_SUPPORT),y)
$(error Rust scheduler support bindings are not admitted for generation)
endif
targets += bindings/sched_support_generated.rs
always-$(CONFIG_RUST_SCHED_SUPPORT) += bindings/sched_support_generated.rs
$(obj)/bindings/sched_support_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_support_bindgen_parameters)
$(obj)/bindings/sched_support_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_support_generated.rs: $(srctree)/kernel/sched/sched_support_bindings.h \
    $(srctree)/kernel/sched/sched_support_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_SUPPORT),y)
$(obj)/bindings.o: $(obj)/bindings/sched_support_generated.rs
endif
