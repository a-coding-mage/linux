# SPDX-License-Identifier: GPL-2.0
# Source registration only; select no objects or original-C exclusions.
ifeq ($(CONFIG_RUST_GROUPS),y)
$(error SOURCE ONLY HOLD: Rust supplementary-group ownership and ABI are unqualified)
endif

# Future admission must retain the MULTIUSER-dependent groups.o native policy
# and unique export/syscall ownership. Preserve global RUST configuration
# restrictions, diagnostics, CFI, uaccess and allocation instrumentation.
