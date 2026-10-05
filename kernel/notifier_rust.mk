# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; select no objects or original-C exclusions.
ifeq ($(CONFIG_RUST_NOTIFIER),y)
$(error SOURCE ONLY HOLD: Rust notifier ownership and instrumentation are unqualified)
endif
# Future admission must cover actual Rust bodies and all new native leaves for
# original notrace/NOKPROBE, branch profiling and context-analysis contracts.
