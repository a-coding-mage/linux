# SPDX-License-Identifier: GPL-2.0-or-later
# Use rust/Makefile's canonical bindgen command and configured kernel C flags.
targets += bindings/afs_addr_prefs_generated.rs
always-$(CONFIG_RUST_AFS_ADDR_PREFS) += bindings/afs_addr_prefs_generated.rs
$(obj)/bindings/afs_addr_prefs_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/fs/afs/addr_prefs_bindgen_parameters)
$(obj)/bindings/afs_addr_prefs_generated.rs: $(srctree)/fs/afs/addr_prefs_bindings.h \
    $(srctree)/fs/afs/addr_prefs_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
