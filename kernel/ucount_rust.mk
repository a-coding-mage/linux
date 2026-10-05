# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; select no objects or original-C exclusions.
ifeq ($(CONFIG_RUST_UCOUNT),y)
$(error SOURCE ONLY HOLD: Rust user-accounting ownership and ABI are unqualified)
endif

# Future admission must retain kernel/ucount.o's native compiler/instrumentation
# policy and unique initial storage, sysctl, callback and public entry ownership.
# Native primitives remain explicit runtime work, not an old-C policy fallback.
