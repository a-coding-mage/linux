# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_SYSCALLS),y)
$(error SOURCE ONLY HOLD: Rust scheduler syscall bindings are unqualified)
endif
targets += bindings/sched_syscalls_generated.rs
always-$(CONFIG_RUST_SCHED_SYSCALLS) += bindings/sched_syscalls_generated.rs
$(obj)/bindings/sched_syscalls_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_syscalls_bindgen_parameters)
$(obj)/bindings/sched_syscalls_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_syscalls_generated.rs: $(srctree)/kernel/sched/sched_syscalls_bindings.h \
    $(srctree)/kernel/sched/sched_syscalls_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_SYSCALLS),y)
$(obj)/bindings.o: $(obj)/bindings/sched_syscalls_generated.rs
endif
