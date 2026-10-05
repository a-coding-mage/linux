# SPDX-License-Identifier: GPL-2.0-only
# Inactive admission guard only. No object or binding generation rules.
ifeq ($(CONFIG_RUST_SCHED_DEBUG),y)
$(error CONFIG_RUST_SCHED_DEBUG is source-only; native qualification is incomplete)
endif
