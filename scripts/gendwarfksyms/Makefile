# SPDX-License-Identifier: GPL-2.0
hostprogs-always-y += gendwarfksyms

ifneq ($(HOST_TOOLS_LANG),c)
gendwarfksyms-rust := y
# Keep library selection overridable for nonstandard elfutils installations.
HOSTRUSTFLAGS_gendwarfksyms += -Clink-args='$(call escsq,$(HOSTLDLIBS_gendwarfksyms))'
else
gendwarfksyms-objs += gendwarfksyms.o
gendwarfksyms-objs += cache.o
gendwarfksyms-objs += die.o
gendwarfksyms-objs += dwarf.o
gendwarfksyms-objs += kabi.o
gendwarfksyms-objs += symbols.o
gendwarfksyms-objs += types.o
endif

HOSTLDLIBS_gendwarfksyms := -ldw -lelf -lz
