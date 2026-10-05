# SPDX-License-Identifier: GPL-2.0
# Source registration only; select no objects or original-C exclusions.
ifeq ($(CONFIG_RUST_CAPABILITY_BOOT),y)
$(error SOURCE ONLY HOLD: Rust capability boot ownership and ABI are unqualified)
endif
