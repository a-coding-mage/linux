# SPDX-License-Identifier: GPL-2.0

hostprogs	+= relocs
ifneq ($(HOST_TOOLS_LANG),c)
relocs-rust	:= y
else
relocs-objs	+= relocs_32.o
relocs-objs	+= relocs_64.o
relocs-objs	+= relocs_main.o
endif
PHONY += relocs
relocs: $(obj)/relocs
	@:
