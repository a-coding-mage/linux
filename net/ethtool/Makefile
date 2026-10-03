# SPDX-License-Identifier: GPL-2.0-only

obj-y				+= ioctl.o common.o

obj-$(CONFIG_ETHTOOL_NETLINK)	+= ethtool_nl.o

ethtool_nl-y	:= netlink.o bitset.o strset.o linkinfo.o linkmodes.o rss.o \
		   linkstate.o debug.o wol.o features.o privflags.o rings.o \
		   channels.o coalesce.o pause.o eee.o tsinfo.o cabletest.o \
		   tunnels.o fec.o eeprom.o stats.o phc_vclocks.o mm.o \
		   module.o cmis_fw_update.o cmis_cdb.o pse-pd.o plca.o \
		   phy.o tsconfig.o mse.o

# Proposed reconstruction option; the original C source remains untouched.
clean-files += .rust-listing/ .rust-listing-elf/

ifeq ($(CONFIG_RUST_ETHTOOL_COALESCE),y)
ethtool_nl-y += coalesce_helpers.o
coalesce-rust-deps := $(objtree)/rust/bindings/ethtool_coalesce_generated.rs
# Explicit Rust rules have no implicit-rule stem; retain composite membership.
coalesce-rust-modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,coalesce.o coalesce.s coalesce.ll coalesce.rsi \
	.rust-listing/coalesce.o): private modname-multi = $(coalesce-rust-modname-multi)
$(obj)/coalesce.o: $(src)/coalesce.rs $(coalesce-rust-deps) \
	$(wildcard $(objtree)/include/config/RUST_ETHTOOL_COALESCE) FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Keep inspection on the selected source, ahead of the generic C recipes.
$(obj)/coalesce.s: $(src)/coalesce.rs $(coalesce-rust-deps) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/coalesce.ll: $(src)/coalesce.rs $(coalesce-rust-deps) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/coalesce.rsi: $(src)/coalesce.rs $(coalesce-rust-deps) FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Listing debug flags and sidecars stay private; never replace the real owner.
$(obj)/.rust-listing/coalesce.o: private target-stem := coalesce
$(obj)/.rust-listing/coalesce.o: private part-of-builtin = $(if $(filter $(obj)/coalesce.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/coalesce.o: private part-of-module = $(if $(filter $(obj)/coalesce.o,$(real-obj-m)),y)
$(obj)/.rust-listing/coalesce.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/coalesce.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/coalesce.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/coalesce.o: $(src)/coalesce.rs $(coalesce-rust-deps) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_coalesce_rust_lst_elf = LD      $@
      cmd_coalesce_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/coalesce.o: $(obj)/.rust-listing/coalesce.o FORCE
	$(call if_changed,coalesce_rust_lst_elf)
quiet_cmd_coalesce_rust_lst = MKLST   $@
      cmd_coalesce_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/coalesce.lst: $(obj)/.rust-listing-elf/coalesce.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,coalesce_rust_lst)
targets += coalesce.s coalesce.ll coalesce.rsi coalesce.lst \
	.rust-listing/coalesce.o .rust-listing-elf/coalesce.o
endif
