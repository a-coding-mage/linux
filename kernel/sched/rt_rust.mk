# SPDX-License-Identifier: GPL-2.0
# Deliberate source-stage admission stop; no object replacement or C exclusion.
# Future integration must preserve build_policy.o flags, ordering, CFI, init
# policy, diagnostics, context/barrier instrumentation and every security flag.
ifeq ($(CONFIG_RUST_SCHED_RT),y)
$(error Rust RT scheduler is source-only; aggregate ownership and native policy qualification are incomplete)
endif
