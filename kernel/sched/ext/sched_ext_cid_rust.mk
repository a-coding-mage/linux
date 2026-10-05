# SPDX-License-Identifier: GPL-2.0-only
# No object selection: CID BTF ID sets depend on the original shared ext TU.
# Future admission must preserve that linkage and original build_policy policy.
ifeq ($(CONFIG_RUST_SCHED_EXT_CID),y)
$(error SOURCE ONLY HOLD: Rust sched_ext CID is not admitted)
endif
