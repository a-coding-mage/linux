# SPDX-License-Identifier: GPL-2.0
# scripts/dtc makefile

# *** Also keep .gitignore in sync when changing ***
hostprogs-always-$(CONFIG_DTC)		+= dtc fdtoverlay
hostprogs-always-$(CHECK_DTBS)		+= dtc

ifneq ($(HOST_TOOLS_LANG),c)
dtc-rust	:= y
fdtoverlay-rust	:= y
fdtget-rust	:= y
fdtput-rust	:= y
else
dtc-objs	:= dtc.o flattree.o fstree.o data.o livetree.o treesource.o \
		   srcpos.o checks.o util.o
dtc-objs	+= dtc-lexer.lex.o dtc-parser.tab.o

# The upstream project builds libfdt as a separate library. We directly link
# its object files into each host utility, as in the original C Kbuild rules.
libfdt-objs	:= fdt.o fdt_ro.o fdt_wip.o fdt_sw.o fdt_rw.o fdt_strerror.o fdt_empty_tree.o fdt_addresses.o fdt_overlay.o
libfdt		= $(addprefix libfdt/,$(libfdt-objs))
fdtoverlay-objs	:= $(libfdt) fdtoverlay.o util.o
fdtget-objs	:= $(libfdt) fdtget.o util.o
fdtput-objs	:= $(libfdt) fdtput.o util.o
HOSTCFLAGS_fdtget.o := -include $(src)/fdt_legacy_compat.h
HOSTCFLAGS_fdtput.o := -include $(src)/fdt_legacy_compat.h

# Source files need the userspace libfdt environment rather than kernel types.
HOST_EXTRACFLAGS += -I $(src)/libfdt
HOST_EXTRACFLAGS += -DNO_YAML

# Generated files include headers from the source tree.
HOSTCFLAGS_dtc-lexer.lex.o := -I $(src)
HOSTCFLAGS_dtc-parser.tab.o := -I $(src)
$(obj)/dtc-lexer.lex.o: $(obj)/dtc-parser.tab.h
endif

# The query/edit applets are explicit host targets, not prerequisites of every
# device-tree build. Both implementations share their respective libfdt core.
hostprogs += fdtget fdtput
