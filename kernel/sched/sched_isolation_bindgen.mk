# SPDX-License-Identifier: GPL-2.0-only
# Proposal only. No configured binding generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_ISOLATION),y)
$(error SOURCE ONLY HOLD: Rust scheduler isolation bindings are unqualified)
endif
targets += bindings/sched_isolation_generated.rs
always-$(CONFIG_RUST_SCHED_ISOLATION) += bindings/sched_isolation_generated.rs
$(obj)/bindings/sched_isolation_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_isolation_bindgen_parameters)
$(obj)/bindings/sched_isolation_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_isolation_generated.rs: $(srctree)/kernel/sched/sched_isolation_bindings.h \
    $(srctree)/kernel/sched/sched_isolation_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_ISOLATION),y)
$(obj)/bindings.o: $(obj)/bindings/sched_isolation_generated.rs
endif
