# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_PELT),y)
$(error SOURCE ONLY HOLD: Rust scheduler PELT bindings are unqualified)
endif
targets += bindings/sched_pelt_generated.rs
always-$(CONFIG_RUST_SCHED_PELT) += bindings/sched_pelt_generated.rs
$(obj)/bindings/sched_pelt_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_pelt_bindgen_parameters)
$(obj)/bindings/sched_pelt_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_pelt_generated.rs: $(srctree)/kernel/sched/sched_pelt_bindings.h \
    $(srctree)/kernel/sched/sched_pelt_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_PELT),y)
$(obj)/bindings.o: $(obj)/bindings/sched_pelt_generated.rs
endif
