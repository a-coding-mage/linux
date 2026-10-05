# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; no configured generation or activation is admitted.
ifeq ($(CONFIG_RUST_SCHED_CPUTIME),y)
$(error SOURCE ONLY HOLD: scheduler cputime bindings are not admitted)
endif
targets += bindings/cputime_generated.rs
always-$(CONFIG_RUST_SCHED_CPUTIME) += bindings/cputime_generated.rs
$(obj)/bindings/cputime_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/cputime_bindgen_parameters)
$(obj)/bindings/cputime_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/cputime_generated.rs: $(srctree)/kernel/sched/cputime_bindings.h \
    $(srctree)/kernel/sched/cputime_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_CPUTIME),y)
$(obj)/bindings.o: $(obj)/bindings/cputime_generated.rs
endif
