# SPDX-License-Identifier: GPL-2.0-only
# Source-only admission stop. Original build_utility.c ownership is unchanged.
ifeq ($(CONFIG_RUST_SCHED_ISOLATION),y)
$(error SOURCE ONLY HOLD: Rust scheduler isolation is not admitted)
endif
