# SPDX-License-Identifier: GPL-2.0
#
# Makefile for s390 specific build tools
#

kapi := arch/$(ARCH)/include/generated/asm
kapi-hdrs-y := $(kapi)/facility-defs.h $(kapi)/dis-defs.h

PHONY += kapi

kapi:	$(kapi-hdrs-y)

hostprogs		    += gen_facilities
hostprogs		    += gen_opcode_table
ifneq ($(HOST_TOOLS_LANG),c)
gen_facilities-rust := y
gen_opcode_table-rust := y

s390-facility-configs := Z196 ZEC12 Z13 Z14 Z15 Z17
HOSTRUSTFLAGS_gen_facilities += $(foreach march,$(s390-facility-configs),\
	--check-cfg='cfg(CONFIG_HAVE_MARCH_$(march)_FEATURES)' \
	$(if $(filter y,$(CONFIG_HAVE_MARCH_$(march)_FEATURES)),\
		--cfg CONFIG_HAVE_MARCH_$(march)_FEATURES))
else
HOSTCFLAGS_gen_facilities.o += $(LINUXINCLUDE)
endif

filechk_facility-defs.h = $(obj)/gen_facilities

filechk_dis-defs.h = \
	$(obj)/gen_opcode_table < $(srctree)/arch/$(ARCH)/tools/opcodes.txt

$(kapi)/facility-defs.h: $(obj)/gen_facilities FORCE
	$(call filechk,facility-defs.h)

$(kapi)/dis-defs.h: $(obj)/gen_opcode_table FORCE
	$(call filechk,dis-defs.h)

hostprogs	+= relocs
ifneq ($(HOST_TOOLS_LANG),c)
relocs-rust := y
endif
PHONY		+= relocs
relocs: $(obj)/relocs
	@:
