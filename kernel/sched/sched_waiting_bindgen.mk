# SPDX-License-Identifier: GPL-2.0-only
# Source declaration registration only. No binding generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_WAITING),y)
$(error SOURCE ONLY HOLD: Rust waiting bindings are unqualified)
endif
targets += bindings/sched_waiting_generated.rs
always-$(CONFIG_RUST_SCHED_WAITING) += bindings/sched_waiting_generated.rs
$(obj)/bindings/sched_waiting_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_waiting_bindgen_parameters)
$(obj)/bindings/sched_waiting_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_waiting_generated.rs: $(srctree)/kernel/sched/sched_waiting_bindings.h \
    $(srctree)/kernel/sched/sched_waiting_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_WAITING),y)
$(obj)/bindings.o: $(obj)/bindings/sched_waiting_generated.rs
endif
