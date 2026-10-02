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

# Keep kafs and its C callers unchanged; select only this translation.
ifeq ($(CONFIG_RUST_AFS_ADDR_PREFS),y)
kafs-y += addr_prefs_helpers.o
# An explicit target has no implicit-rule $*; recover composite membership
# from its actual stem, as for the original C object (built-in or module).
$(obj)/addr_prefs.o: private modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(obj)/addr_prefs.o: $(src)/addr_prefs.rs $(objtree)/rust/bindings/afs_addr_prefs_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
