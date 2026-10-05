# SPDX-License-Identifier: GPL-2.0
# Preserve original build_utility selection and all native protection settings.
ifeq ($(CONFIG_RUST_SCHED_TOPOLOGY),y)
$(error SOURCE ONLY HOLD: Rust scheduler topology is not admitted)
endif
