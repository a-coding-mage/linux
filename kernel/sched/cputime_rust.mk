# SPDX-License-Identifier: GPL-2.0-only
# Original aggregate inclusions and object selection remain untouched.
ifeq ($(CONFIG_RUST_SCHED_CPUTIME),y)
$(error SOURCE ONLY HOLD: scheduler cputime is not admitted)
endif
