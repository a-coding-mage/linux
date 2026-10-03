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

clean-files += .rust-listing/ .rust-listing-elf/

ifeq ($(CONFIG_RUST_KSMBD_VFS_CACHE),y)
ksmbd-y += vfs_cache_helpers.o
vfs_cache-rust-deps := $(src)/vfs_cache_close.rs $(src)/vfs_cache_lifecycle.rs \
	$(objtree)/rust/bindings/ksmbd_vfs_cache_generated.rs
# Explicit Rust rules have no implicit-rule stem; retain composite membership.
vfs_cache-rust-modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,vfs_cache.o vfs_cache.s vfs_cache.ll vfs_cache.rsi \
	.rust-listing/vfs_cache.o): private modname-multi = $(vfs_cache-rust-modname-multi)
$(obj)/vfs_cache.o: $(src)/vfs_cache.rs $(vfs_cache-rust-deps) \
	$(wildcard $(objtree)/include/config/RUST_KSMBD_VFS_CACHE) FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Keep inspection on the selected source, ahead of the generic C recipes.
$(obj)/vfs_cache.s: $(src)/vfs_cache.rs $(vfs_cache-rust-deps) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/vfs_cache.ll: $(src)/vfs_cache.rs $(vfs_cache-rust-deps) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/vfs_cache.rsi: $(src)/vfs_cache.rs $(vfs_cache-rust-deps) FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Listing debug flags and sidecars stay private; never replace the real owner.
$(obj)/.rust-listing/vfs_cache.o: private target-stem := vfs_cache
$(obj)/.rust-listing/vfs_cache.o: private part-of-builtin = $(if $(filter $(obj)/vfs_cache.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/vfs_cache.o: private part-of-module = $(if $(filter $(obj)/vfs_cache.o,$(real-obj-m)),y)
$(obj)/.rust-listing/vfs_cache.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/vfs_cache.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/vfs_cache.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/vfs_cache.o: $(src)/vfs_cache.rs $(vfs_cache-rust-deps) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_vfs_cache_rust_lst_elf = LD      $@
      cmd_vfs_cache_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/vfs_cache.o: $(obj)/.rust-listing/vfs_cache.o FORCE
	$(call if_changed,vfs_cache_rust_lst_elf)
quiet_cmd_vfs_cache_rust_lst = MKLST   $@
      cmd_vfs_cache_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/vfs_cache.lst: $(obj)/.rust-listing-elf/vfs_cache.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,vfs_cache_rust_lst)
targets += vfs_cache.s vfs_cache.ll vfs_cache.rsi vfs_cache.lst \
	.rust-listing/vfs_cache.o .rust-listing-elf/vfs_cache.o
endif
