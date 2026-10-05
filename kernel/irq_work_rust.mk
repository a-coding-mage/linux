# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; select no objects or original-C exclusions.
ifeq ($(CONFIG_RUST_IRQ_WORK),y)
$(error SOURCE ONLY HOLD: Rust IRQ-work ownership and instrumentation are unqualified)
endif
# Future admission must preserve native irq_work.o's CC_FLAGS_FTRACE removal
# across the complete Rust/native split, not merely its public entry wrappers.
# This source checkpoint grants no new instrumentation exemptions.
