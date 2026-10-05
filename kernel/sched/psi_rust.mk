# SPDX-License-Identifier: GPL-2.0
# No object replacement or original C exclusion in the source-only phase.
ifeq ($(CONFIG_RUST_SCHED_PSI),y)
$(error Rust PSI is source-only; ABI, ownership and native policy remain unqualified)
endif
