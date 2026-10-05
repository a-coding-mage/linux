# SPDX-License-Identifier: GPL-2.0-or-later
# Source registration only; select no objects or original-C exclusions.
ifeq ($(CONFIG_RUST_CRED),y)
$(error SOURCE ONLY HOLD: Rust credential ownership and native behavior are unqualified)
endif

# Future admission must retain kernel/cred.o's native compiler/instrumentation
# policy and unique init, callback and export identities. Native primitives are
# explicit remaining runtime work, not the original credential policy fallback.
