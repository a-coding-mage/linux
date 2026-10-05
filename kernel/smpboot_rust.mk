# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; select no objects or original-C exclusions.
ifeq ($(CONFIG_RUST_SMPBOOT),y)
$(error SOURCE ONLY HOLD: Rust smpboot ownership and ABI are unqualified)
endif
