# SPDX-License-Identifier: GPL-2.0-only
# Preserve original build_utility selection and native protection policy.
ifeq ($(CONFIG_RUST_SCHED_CORE_COOKIE),y)
$(error SOURCE ONLY HOLD: Rust scheduler core-cookie owner is not admitted)
endif
