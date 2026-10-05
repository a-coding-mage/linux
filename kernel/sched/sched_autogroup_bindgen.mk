# SPDX-License-Identifier: GPL-2.0
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_AUTOGROUP),y)
$(error SOURCE ONLY HOLD: Rust scheduler autogroup bindings are unqualified)
endif
targets += bindings/sched_autogroup_generated.rs
always-$(CONFIG_RUST_SCHED_AUTOGROUP) += bindings/sched_autogroup_generated.rs
$(obj)/bindings/sched_autogroup_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_autogroup_bindgen_parameters)
$(obj)/bindings/sched_autogroup_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_autogroup_generated.rs: $(srctree)/kernel/sched/sched_autogroup_bindings.h \
    $(srctree)/kernel/sched/sched_autogroup_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_AUTOGROUP),y)
$(obj)/bindings.o: $(obj)/bindings/sched_autogroup_generated.rs
endif
