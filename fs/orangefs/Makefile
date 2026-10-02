# SPDX-License-Identifier: GPL-2.0
#
# Makefile for the ORANGEFS filesystem.
#

obj-$(CONFIG_ORANGEFS_FS) += orangefs.o

orangefs-objs := acl.o file.o orangefs-cache.o orangefs-utils.o xattr.o \
		 dcache.o inode.o orangefs-sysfs.o orangefs-mod.o super.o \
		 devorangefs-req.o namei.o symlink.o dir.o orangefs-bufmap.o \
		 orangefs-debugfs.o waitqueue.o

# Private listing objects never replace the configured canonical xattr.o.
clean-files += .rust-listing/ .rust-listing-elf/

ifeq ($(CONFIG_RUST_ORANGEFS_XATTR),y)
orangefs-objs += xattr-helpers.o

# Explicit Rust recipes have an empty $*; retain orangefs.o membership.
orangefs-xattr-rust-modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,xattr.o xattr.s xattr.ll xattr.rsi \
	.rust-listing/xattr.o): private modname-multi = $(orangefs-xattr-rust-modname-multi)

$(obj)/xattr.o: $(src)/xattr.rs $(objtree)/rust/bindings/orangefs_xattr_generated.rs \
	$(wildcard $(objtree)/include/config/RUST_ORANGEFS_XATTR) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/xattr.s: $(src)/xattr.rs $(objtree)/rust/bindings/orangefs_xattr_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/xattr.ll: $(src)/xattr.rs $(objtree)/rust/bindings/orangefs_xattr_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/xattr.rsi: $(src)/xattr.rs $(objtree)/rust/bindings/orangefs_xattr_generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Debug information for .lst is confined to the private listing object.
$(obj)/.rust-listing/xattr.o: private target-stem := xattr
$(obj)/.rust-listing/xattr.o: private part-of-builtin = $(if $(filter $(obj)/xattr.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/xattr.o: private part-of-module = $(if $(filter $(obj)/xattr.o,$(real-obj-m)),y)
$(obj)/.rust-listing/xattr.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/xattr.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/xattr.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/xattr.o: $(src)/xattr.rs $(objtree)/rust/bindings/orangefs_xattr_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# A private Rust object may contain bitcode when inline helpers are enabled.
quiet_cmd_orangefs_xattr_rust_lst_elf = LD      $@
      cmd_orangefs_xattr_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/xattr.o: $(obj)/.rust-listing/xattr.o FORCE
	$(call if_changed,orangefs_xattr_rust_lst_elf)
quiet_cmd_orangefs_xattr_rust_lst = MKLST   $@
      cmd_orangefs_xattr_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/xattr.lst: $(obj)/.rust-listing-elf/xattr.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,orangefs_xattr_rust_lst)
targets += xattr.s xattr.ll xattr.rsi xattr.lst \
	.rust-listing/xattr.o .rust-listing-elf/xattr.o
endif
