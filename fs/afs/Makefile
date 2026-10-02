# SPDX-License-Identifier: GPL-2.0
#
# Makefile for Red Hat Linux AFS client.
#

kafs-y := \
	addr_list.o \
	addr_prefs.o \
	callback.o \
	cell.o \
	cm_security.o \
	cmservice.o \
	dir.o \
	dir_edit.o \
	dir_search.o \
	dir_silly.o \
	dynroot.o \
	file.o \
	flock.o \
	fsclient.o \
	fs_operation.o \
	fs_probe.o \
	inode.o \
	main.o \
	misc.o \
	mntpt.o \
	rotate.o \
	rxrpc.o \
	security.o \
	server.o \
	server_list.o \
	super.o \
	symlink.o \
	validation.o \
	vlclient.o \
	vl_alias.o \
	vl_list.o \
	vl_probe.o \
	vl_rotate.o \
	volume.o \
	write.o \
	xattr.o \
	yfsclient.o

kafs-$(CONFIG_PROC_FS) += proc.o
obj-$(CONFIG_AFS_FS)  := kafs.o

# Private listing products must also be cleaned after disabling the selector.
clean-files += .rust-listing/ .rust-listing-elf/

# Keep kafs and its C callers unchanged; select only this translation.
ifeq ($(CONFIG_RUST_AFS_ADDR_PREFS),y)
kafs-y += addr_prefs_helpers.o

# Explicit rules have no implicit-rule $*; recover the original kafs identity.
afs-addr-prefs-rust-modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,addr_prefs.o addr_prefs.s addr_prefs.ll addr_prefs.rsi \
	.rust-listing/addr_prefs.o): private modname-multi = $(afs-addr-prefs-rust-modname-multi)

$(obj)/addr_prefs.o: $(src)/addr_prefs.rs $(objtree)/rust/bindings/afs_addr_prefs_generated.rs \
	$(wildcard $(objtree)/include/config/RUST_AFS_ADDR_PREFS) FORCE
	+$(call if_changed_rule,rustc_o_rs)
# Select Rust for every inspection target, ahead of the generic C rules.
$(obj)/addr_prefs.s: $(src)/addr_prefs.rs $(objtree)/rust/bindings/afs_addr_prefs_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/addr_prefs.ll: $(src)/addr_prefs.rs $(objtree)/rust/bindings/afs_addr_prefs_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/addr_prefs.rsi: $(src)/addr_prefs.rs $(objtree)/rust/bindings/afs_addr_prefs_generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Confine listing debuginfo and all sidecars to a private object. Restore the
# canonical stem and membership for per-object flags and built-in/module mode.
$(obj)/.rust-listing/addr_prefs.o: private target-stem := addr_prefs
$(obj)/.rust-listing/addr_prefs.o: private part-of-builtin = $(if $(filter $(obj)/addr_prefs.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/addr_prefs.o: private part-of-module = $(if $(filter $(obj)/addr_prefs.o,$(real-obj-m)),y)
$(obj)/.rust-listing/addr_prefs.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/addr_prefs.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/addr_prefs.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/addr_prefs.o: $(src)/addr_prefs.rs $(objtree)/rust/bindings/afs_addr_prefs_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Inline helpers/LTO may leave bitcode. Materialize only the private listing
# object as ELF, without changing the canonical object or objtool policy.
quiet_cmd_afs_addr_prefs_rust_lst_elf = LD      $@
      cmd_afs_addr_prefs_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/addr_prefs.o: $(obj)/.rust-listing/addr_prefs.o FORCE
	$(call if_changed,afs_addr_prefs_rust_lst_elf)

# Track map creation/deletion; let the original makelst own missing-map behavior.
quiet_cmd_afs_addr_prefs_rust_lst = MKLST   $@
      cmd_afs_addr_prefs_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/addr_prefs.lst: $(obj)/.rust-listing-elf/addr_prefs.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,afs_addr_prefs_rust_lst)
targets += addr_prefs.s addr_prefs.ll addr_prefs.rsi addr_prefs.lst \
	.rust-listing/addr_prefs.o .rust-listing-elf/addr_prefs.o
endif
