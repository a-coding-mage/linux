# SPDX-License-Identifier: GPL-2.0-or-later
#
# Makefile for Linux SMB3 kernel server
#
obj-$(CONFIG_SMB_SERVER) += ksmbd.o

ksmbd-y :=	unicode.o auth.o vfs.o vfs_cache.o server.o ndr.o \
		misc.o oplock.o connection.o ksmbd_work.o crypto_ctx.o \
		mgmt/ksmbd_ida.o mgmt/user_config.o mgmt/share_config.o \
		mgmt/tree_connect.o mgmt/user_session.o smb_common.o \
		transport_tcp.o transport_ipc.o smbacl.o smb2pdu.o \
		smb2ops.o smb2misc.o ksmbd_spnego_negtokeninit.asn1.o \
		ksmbd_spnego_negtokentarg.asn1.o asn1.o compress.o

$(obj)/asn1.o: $(obj)/ksmbd_spnego_negtokeninit.asn1.h $(obj)/ksmbd_spnego_negtokentarg.asn1.h

$(obj)/ksmbd_spnego_negtokeninit.asn1.o: $(obj)/ksmbd_spnego_negtokeninit.asn1.c $(obj)/ksmbd_spnego_negtokeninit.asn1.h
$(obj)/ksmbd_spnego_negtokentarg.asn1.o: $(obj)/ksmbd_spnego_negtokentarg.asn1.c $(obj)/ksmbd_spnego_negtokentarg.asn1.h

ksmbd-$(CONFIG_SMB_SERVER_SMBDIRECT) += transport_rdma.o
ksmbd-$(CONFIG_PROC_FS) += proc.o

ifeq ($(CONFIG_RUST_KSMBD_VFS_CACHE),y)
# Keep vfs_cache.o in the original ksmbd composite object. C remains untouched.
ksmbd-y += vfs_cache_helpers.o
vfs-cache-rust-deps := $(src)/vfs_cache_close.rs $(src)/vfs_cache_lifecycle.rs \
	$(objtree)/rust/bindings/ksmbd_vfs_cache_generated.rs
# Do not let the canonical expanded-source formatter probe's && list hide a
# missing formatter behind if_changed_dep's appended fixdep invocation.
quiet_cmd_vfs_cache_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_vfs_cache_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/vfs_cache.o: $(src)/vfs_cache.rs $(vfs-cache-rust-deps) \
	$(wildcard $(objtree)/include/config/RUST_KSMBD_VFS_CACHE) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/vfs_cache.s: $(src)/vfs_cache.rs $(vfs-cache-rust-deps) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/vfs_cache.ll: $(src)/vfs_cache.rs $(vfs-cache-rust-deps) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/vfs_cache.rsi: $(src)/vfs_cache.rs $(vfs-cache-rust-deps) FORCE
	+$(call if_changed_dep,vfs_cache_rsi)
endif
