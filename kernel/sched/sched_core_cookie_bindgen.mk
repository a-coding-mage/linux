# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_CORE_COOKIE),y)
$(error SOURCE ONLY HOLD: Rust scheduler core-cookie bindings are unqualified)
endif
targets += bindings/sched_core_cookie_generated.rs
always-$(CONFIG_RUST_SCHED_CORE_COOKIE) += bindings/sched_core_cookie_generated.rs
$(obj)/bindings/sched_core_cookie_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_core_cookie_bindgen_parameters)
$(obj)/bindings/sched_core_cookie_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_core_cookie_generated.rs: $(srctree)/kernel/sched/sched_core_cookie_bindings.h \
    $(srctree)/kernel/sched/sched_core_cookie_native_leaves.def \
    $(srctree)/kernel/sched/sched_core_cookie_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_CORE_COOKIE),y)
$(obj)/bindings.o: $(obj)/bindings/sched_core_cookie_generated.rs
endif
