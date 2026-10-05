# SPDX-License-Identifier: GPL-2.0
# Source registration proposal only. No configured generation is admitted.
ifneq ($(filter y,$(CONFIG_RUST_SCHED_CPUFREQ) $(CONFIG_RUST_SCHED_CPUFREQ_SCHEDUTIL)),)
$(error SOURCE ONLY HOLD: Rust scheduler CPUFreq bindings are unqualified)
endif
targets += bindings/sched_cpufreq_generated.rs
always-$(CONFIG_RUST_SCHED_CPUFREQ) += bindings/sched_cpufreq_generated.rs
$(obj)/bindings/sched_cpufreq_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_cpufreq_bindgen_parameters)
$(obj)/bindings/sched_cpufreq_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_cpufreq_generated.rs: $(srctree)/kernel/sched/sched_cpufreq_bindings.h \
    $(srctree)/kernel/sched/sched_cpufreq_private.h \
    $(srctree)/kernel/sched/sched_cpufreq_leaves.inc \
    $(srctree)/kernel/sched/sched_cpufreq_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_CPUFREQ),y)
$(obj)/bindings.o: $(obj)/bindings/sched_cpufreq_generated.rs
endif
