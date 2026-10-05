# SPDX-License-Identifier: GPL-2.0-only
# Original completion/swait/wait/wait_bit object ownership stays unchanged.
ifeq ($(CONFIG_RUST_SCHED_WAITING),y)
$(error SOURCE ONLY HOLD: Rust waiting ownership and native policy are unqualified)
endif
