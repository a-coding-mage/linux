# SPDX-License-Identifier: GPL-2.0-only
# Preserve original aggregate selection and all native protection settings.
ifeq ($(CONFIG_RUST_SCHED_SYSCALLS),y)
$(error SOURCE ONLY HOLD: Rust scheduler syscalls are not admitted)
endif
