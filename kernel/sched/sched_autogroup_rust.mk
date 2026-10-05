# SPDX-License-Identifier: GPL-2.0
# Preserve original build_utility selection and native protection policy.
ifeq ($(CONFIG_RUST_SCHED_AUTOGROUP),y)
$(error SOURCE ONLY HOLD: Rust scheduler autogroup is not admitted)
endif
