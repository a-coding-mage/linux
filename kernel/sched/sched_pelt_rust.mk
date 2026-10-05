# SPDX-License-Identifier: GPL-2.0-only
# Preserve original build_policy selection and native protection settings.
ifeq ($(CONFIG_RUST_SCHED_PELT),y)
$(error SOURCE ONLY HOLD: Rust scheduler PELT is not admitted)
endif
