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
ifeq ($(CONFIG_RUST_ETHTOOL_COALESCE),y)
ethtool_nl-y += coalesce_helpers.o
$(obj)/coalesce.o: $(src)/coalesce.rs $(objtree)/rust/bindings/ethtool_coalesce_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
