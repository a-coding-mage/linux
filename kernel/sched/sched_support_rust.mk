# SPDX-License-Identifier: GPL-2.0-only
# Source-stage admission stop. Original aggregate C inclusions stay unchanged.
ifeq ($(CONFIG_RUST_SCHED_SUPPORT),y)
$(error Rust scheduler support is source-only; ABI, ownership and native policy are unqualified)
endif
