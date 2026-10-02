# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile, retaining the canonical configured bindgen flags.
targets += bindings/nfs4file_generated.rs
always-$(CONFIG_RUST_NFS4_FILE) += bindings/nfs4file_generated.rs
$(obj)/bindings/nfs4file_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/fs/nfs/nfs4file_bindgen_parameters)
$(obj)/bindings/nfs4file_generated.rs: $(srctree)/fs/nfs/nfs4file_bindings.h \
    $(srctree)/fs/nfs/nfs4file_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
