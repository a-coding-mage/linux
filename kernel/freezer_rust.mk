# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; select no objects or original-C exclusions.
ifeq ($(CONFIG_RUST_FREEZER),y)
$(error SOURCE ONLY HOLD: Rust freezer ownership and native qualification are incomplete)
endif
