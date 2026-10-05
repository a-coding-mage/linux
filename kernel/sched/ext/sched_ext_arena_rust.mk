# SPDX-License-Identifier: GPL-2.0-only
# No object selection or change to shared ext/build_policy ownership.
ifeq ($(CONFIG_RUST_SCHED_EXT_ARENA),y)
$(error SOURCE ONLY HOLD: Rust sched_ext arena is not admitted)
endif
