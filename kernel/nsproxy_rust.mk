# SPDX-License-Identifier: GPL-2.0-only
# Select no objects and exclude no original C owner during the source phase.
ifeq ($(CONFIG_RUST_NSPROXY),y)
$(error SOURCE ONLY HOLD: Rust namespace-transition ownership and ABI are unqualified)
endif

# Future admission must retain kernel/nsproxy.o's complete native policy and
# unique syscall, initial-storage and init-entry ownership. Native primitives
# are explicit remaining runtime work, not an old-C transition fallback.
