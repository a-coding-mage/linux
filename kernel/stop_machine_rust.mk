# SPDX-License-Identifier: GPL-2.0-or-later
# Source registration only; select no objects or original-C exclusions.
ifeq ($(CONFIG_RUST_STOP_MACHINE),y)
$(error SOURCE ONLY HOLD: Rust stop-machine ownership and ABI are unqualified)
endif
