# SPDX-License-Identifier: GPL-2.0
# No object selection: sub BTF and storage retain shared build_policy ownership.
ifeq ($(CONFIG_RUST_SCHED_EXT_SUB),y)
$(error SOURCE ONLY HOLD: Rust sched_ext sub-scheduler is not admitted)
endif
