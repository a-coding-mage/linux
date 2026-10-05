# SPDX-License-Identifier: GPL-2.0
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_EXT_IDLE),y)
$(error SOURCE ONLY HOLD: Rust sched_ext idle bindings and BTF are unqualified)
endif
targets += bindings/sched_ext_idle_generated.rs
always-$(CONFIG_RUST_SCHED_EXT_IDLE) += bindings/sched_ext_idle_generated.rs
$(obj)/bindings/sched_ext_idle_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/ext/sched_ext_idle_bindgen_parameters)
$(obj)/bindings/sched_ext_idle_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_ext_idle_generated.rs: $(srctree)/kernel/sched/ext/sched_ext_idle_bindings.h \
    $(srctree)/kernel/sched/ext/internal.h $(srctree)/kernel/sched/ext/idle.h \
    $(srctree)/kernel/sched/ext/cid.h $(srctree)/kernel/sched/ext/sub.h \
    $(srctree)/kernel/sched/ext/sched_ext_idle_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_EXT_IDLE),y)
$(obj)/bindings.o: $(obj)/bindings/sched_ext_idle_generated.rs
endif
