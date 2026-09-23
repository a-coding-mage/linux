# SPDX-License-Identifier: GPL-2.0
CFLAGS_REMOVE_empty.o += $(CC_FLAGS_LTO)

hostprogs-always-y	+= modpost mk_elfconfig
ifneq ($(HOST_TOOLS_LANG),c)
modpost-rust		:= y
mk_elfconfig-rust	:= y
else
always-y		+= empty.o
modpost-objs	:= modpost.o file2alias.o sumversion.o symsearch.o
endif

devicetable-offsets-file := devicetable-offsets.h

# Normal builds embed the target compiler's layout metadata. The standalone
# no-config host-tools target may instead supply this header at runtime.
ifneq ($(HOST_TOOLS_LANG),c)
ifneq ($(MODPOST_NO_OFFSETS),y)
HOSTRUSTFLAGS_modpost += --cfg modpost_target_offsets
$(obj)/modpost: private export MODPOST_DEVICETABLE_OFFSETS := $(abspath $(obj)/$(devicetable-offsets-file))
$(obj)/modpost: $(obj)/$(devicetable-offsets-file)
endif
endif

$(obj)/$(devicetable-offsets-file): $(obj)/devicetable-offsets.s FORCE
	$(call filechk,offsets,__DEVICETABLE_OFFSETS_H__)

targets += $(devicetable-offsets-file) devicetable-offsets.s

# dependencies on generated files need to be listed explicitly

ifeq ($(HOST_TOOLS_LANG),c)
$(obj)/modpost.o $(obj)/file2alias.o $(obj)/sumversion.o $(obj)/symsearch.o: $(obj)/elfconfig.h
$(obj)/file2alias.o: $(obj)/$(devicetable-offsets-file)
endif

quiet_cmd_elfconfig = MKELF   $@
      cmd_elfconfig = $(obj)/mk_elfconfig < $< > $@

$(obj)/elfconfig.h: $(obj)/empty.o $(obj)/mk_elfconfig FORCE
	$(call if_changed,elfconfig)

targets += elfconfig.h
