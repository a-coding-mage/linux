# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; select no objects or original-C exclusions.
ifeq ($(CONFIG_RUST_UMH),y)
$(error SOURCE ONLY HOLD: Rust usermode-helper ownership and ABI are unqualified)
endif

# Future admission must retain kernel/umh.o's native compiler/instrumentation
# policy, unique public/callback/init ownership and configured helper policy.
# Native storage/primitives remain explicit runtime work, not a C algorithm fallback.
