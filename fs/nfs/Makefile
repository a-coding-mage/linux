# SPDX-License-Identifier: GPL-2.0
#
# Makefile for the Linux nfs filesystem routines.
#

obj-$(CONFIG_NFS_FS) += nfs.o

CFLAGS_nfstrace.o += -I$(src)
nfs-y 			:= client.o dir.o file.o getroot.o inode.o super.o \
			   io.o direct.o pagelist.o read.o symlink.o unlink.o \
			   write.o namespace.o mount_clnt.o nfstrace.o \
			   export.o sysfs.o fs_context.o
nfs-$(CONFIG_ROOT_NFS)	+= nfsroot.o
nfs-$(CONFIG_SYSCTL)	+= sysctl.o
nfs-$(CONFIG_NFS_FSCACHE) += fscache.o
nfs-$(CONFIG_NFS_LOCALIO) += localio.o

obj-$(CONFIG_NFS_V2) += nfsv2.o
nfsv2-y := nfs2super.o proc.o nfs2xdr.o

obj-$(CONFIG_NFS_V3) += nfsv3.o
nfsv3-y := nfs3super.o nfs3client.o nfs3proc.o nfs3xdr.o
nfsv3-$(CONFIG_NFS_V3_ACL) += nfs3acl.o

obj-$(CONFIG_NFS_V4) += nfsv4.o
CFLAGS_nfs4trace.o += -I$(src)
nfsv4-y := nfs4proc.o nfs4xdr.o nfs4state.o nfs4renewd.o nfs4super.o nfs4file.o \
	  delegation.o nfs4idmap.o callback.o callback_xdr.o callback_proc.o \
	  nfs4namespace.o nfs4getroot.o nfs4client.o nfs4session.o \
	  dns_resolve.o nfs4trace.o pnfs.o pnfs_dev.o pnfs_nfs.o
nfsv4-$(CONFIG_NFS_USE_LEGACY_DNS) += cache_lib.o
nfsv4-$(CONFIG_SYSCTL)	+= nfs4sysctl.o
nfsv4-$(CONFIG_NFS_V4_0)	+= nfs40client.o nfs40proc.o
nfsv4-$(CONFIG_NFS_V4_2)	+= nfs42proc.o nfs42xattr.o

clean-files += .rust-listing/ .rust-listing-elf/

ifeq ($(CONFIG_RUST_NFS4_FILE),y)
nfsv4-y += nfs4file_helpers.o
nfs4file-rust-deps := $(objtree)/rust/bindings/nfs4file_generated.rs
# Explicit Rust rules have no implicit-rule stem; retain composite membership.
nfs4file-rust-modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,nfs4file.o nfs4file.s nfs4file.ll nfs4file.rsi \
	.rust-listing/nfs4file.o): private modname-multi = $(nfs4file-rust-modname-multi)
$(obj)/nfs4file.o: $(src)/nfs4file.rs $(nfs4file-rust-deps) \
	$(wildcard $(objtree)/include/config/RUST_NFS4_FILE) FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Keep inspection on the selected source, ahead of the generic C recipes.
$(obj)/nfs4file.s: $(src)/nfs4file.rs $(nfs4file-rust-deps) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/nfs4file.ll: $(src)/nfs4file.rs $(nfs4file-rust-deps) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/nfs4file.rsi: $(src)/nfs4file.rs $(nfs4file-rust-deps) FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Listing debug flags and sidecars stay private; never replace the real owner.
$(obj)/.rust-listing/nfs4file.o: private target-stem := nfs4file
$(obj)/.rust-listing/nfs4file.o: private part-of-builtin = $(if $(filter $(obj)/nfs4file.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/nfs4file.o: private part-of-module = $(if $(filter $(obj)/nfs4file.o,$(real-obj-m)),y)
$(obj)/.rust-listing/nfs4file.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/nfs4file.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/nfs4file.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/nfs4file.o: $(src)/nfs4file.rs $(nfs4file-rust-deps) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_nfs4file_rust_lst_elf = LD      $@
      cmd_nfs4file_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/nfs4file.o: $(obj)/.rust-listing/nfs4file.o FORCE
	$(call if_changed,nfs4file_rust_lst_elf)
quiet_cmd_nfs4file_rust_lst = MKLST   $@
      cmd_nfs4file_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/nfs4file.lst: $(obj)/.rust-listing-elf/nfs4file.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,nfs4file_rust_lst)
targets += nfs4file.s nfs4file.ll nfs4file.rsi nfs4file.lst \
	.rust-listing/nfs4file.o .rust-listing-elf/nfs4file.o
endif

obj-$(CONFIG_PNFS_FILE_LAYOUT) += filelayout/
obj-$(CONFIG_PNFS_BLOCK) += blocklayout/
obj-$(CONFIG_PNFS_FLEXFILE_LAYOUT) += flexfilelayout/
