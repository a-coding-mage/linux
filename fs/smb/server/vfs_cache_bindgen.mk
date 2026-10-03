# SPDX-License-Identifier: GPL-2.0-or-later
# Included from rust/Makefile by the separately composed shared integration.
# Reuse the canonical configured C flags and canonical bindgen invocation.
targets += bindings/ksmbd_vfs_cache_generated.rs
always-$(CONFIG_RUST_KSMBD_VFS_CACHE) += bindings/ksmbd_vfs_cache_generated.rs
$(obj)/bindings/ksmbd_vfs_cache_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/fs/smb/server/vfs_cache_bindgen_parameters)
# if_changed_dep appends fixdep and command-recording statements. The explicit
# shell exit propagates failure before that tail can mask the producer status.
quiet_cmd_ksmbd_vfs_cache_bindgen = BINDGEN $@
      cmd_ksmbd_vfs_cache_bindgen = ($(cmd_bindgen)) || exit $$?
$(obj)/bindings/ksmbd_vfs_cache_generated.rs: \
    $(srctree)/fs/smb/server/vfs_cache_bindings.h \
    $(srctree)/fs/smb/server/vfs_cache_bindgen_parameters \
    $(srctree)/fs/smb/server/vfs_cache_bindgen.mk FORCE
	$(call if_changed_dep,ksmbd_vfs_cache_bindgen)
