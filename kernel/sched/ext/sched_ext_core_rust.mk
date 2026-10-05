# SPDX-License-Identifier: GPL-2.0
# No object selection and no original aggregate-owner exclusion.
ifeq ($(CONFIG_RUST_SCHED_EXT_CORE),y)
$(error SOURCE ONLY HOLD: Rust sched_ext core source owners are incomplete)
endif
