# SPDX-License-Identifier: GPL-2.0-only
obj-$(CONFIG_INOTIFY_USER)	+= inotify_fsnotify.o inotify_user.o

clean-files += .rust-listing/ .rust-listing-elf/

ifeq ($(CONFIG_RUST_INOTIFY_USER),y)
obj-y += inotify_user_helpers.o
RUSTFLAGS_inotify_user.o += -Zfunction-sections=n
inotify_user-rust-deps := $(objtree)/rust/bindings/inotify_user_generated.rs
# Explicit Rust rules have no implicit-rule stem; retain composite membership.
inotify_user-rust-modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,inotify_user.o inotify_user.s inotify_user.ll inotify_user.rsi \
	.rust-listing/inotify_user.o): private modname-multi = $(inotify_user-rust-modname-multi)
$(obj)/inotify_user.o: $(src)/inotify_user.rs $(inotify_user-rust-deps) \
	$(wildcard $(objtree)/include/config/RUST_INOTIFY_USER) FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Keep inspection on the selected source, ahead of the generic C recipes.
$(obj)/inotify_user.s: $(src)/inotify_user.rs $(inotify_user-rust-deps) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/inotify_user.ll: $(src)/inotify_user.rs $(inotify_user-rust-deps) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/inotify_user.rsi: $(src)/inotify_user.rs $(inotify_user-rust-deps) FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Listing debug flags and sidecars stay private; never replace the real owner.
$(obj)/.rust-listing/inotify_user.o: private target-stem := inotify_user
$(obj)/.rust-listing/inotify_user.o: private part-of-builtin = $(if $(filter $(obj)/inotify_user.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/inotify_user.o: private part-of-module = $(if $(filter $(obj)/inotify_user.o,$(real-obj-m)),y)
$(obj)/.rust-listing/inotify_user.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/inotify_user.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/inotify_user.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/inotify_user.o: $(src)/inotify_user.rs $(inotify_user-rust-deps) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_inotify_user_rust_lst_elf = LD      $@
      cmd_inotify_user_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/inotify_user.o: $(obj)/.rust-listing/inotify_user.o FORCE
	$(call if_changed,inotify_user_rust_lst_elf)
quiet_cmd_inotify_user_rust_lst = MKLST   $@
      cmd_inotify_user_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/inotify_user.lst: $(obj)/.rust-listing-elf/inotify_user.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,inotify_user_rust_lst)
targets += inotify_user.s inotify_user.ll inotify_user.rsi inotify_user.lst \
	.rust-listing/inotify_user.o .rust-listing-elf/inotify_user.o
endif
