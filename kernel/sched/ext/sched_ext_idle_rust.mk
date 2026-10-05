# SPDX-License-Identifier: GPL-2.0
# No object selection: idle BTF IDs retain shared ext/build_policy dependencies.
ifeq ($(CONFIG_RUST_SCHED_EXT_IDLE),y)
$(error SOURCE ONLY HOLD: Rust sched_ext idle is not admitted)
endif
