# SPDX-License-Identifier: GPL-2.0
# Source-only admission gate. No aggregate C inclusion or object rule changes.
ifeq ($(CONFIG_RUST_SCHED_DEADLINE),y)
$(error Rust deadline scheduler is source-only; ABI, ownership and native policy remain unqualified)
endif
