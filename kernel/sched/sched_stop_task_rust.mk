# SPDX-License-Identifier: GPL-2.0
# Preserve original build_utility selection and native protection policy.
ifeq ($(CONFIG_RUST_SCHED_STOP_TASK),y)
$(error SOURCE ONLY HOLD: Rust scheduler stop-task owner is not admitted)
endif
