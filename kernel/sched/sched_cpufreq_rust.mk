# SPDX-License-Identifier: GPL-2.0
# No owner replacement, instrumentation exception or object selection is admitted.
ifneq ($(filter y,$(CONFIG_RUST_SCHED_CPUFREQ) $(CONFIG_RUST_SCHED_CPUFREQ_SCHEDUTIL)),)
$(error SOURCE ONLY HOLD: Rust scheduler CPUFreq/schedutil is not admitted)
endif
