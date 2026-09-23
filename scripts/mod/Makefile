# SPDX-License-Identifier: GPL-2.0
CFLAGS_REMOVE_empty.o += $(CC_FLAGS_LTO)

hostprogs-always-y	+= modpost mk_elfconfig
modpost-rust		:= y
mk_elfconfig-rust	:= y

devicetable-offsets-file := devicetable-offsets.h

# Normal builds embed the target compiler's layout metadata. The standalone
# no-config host-tools target may instead supply this header at runtime.
ifneq ($(MODPOST_NO_OFFSETS),y)
HOSTRUSTFLAGS_modpost += --cfg modpost_target_offsets
$(obj)/modpost: private export MODPOST_DEVICETABLE_OFFSETS := $(abspath $(obj)/$(devicetable-offsets-file))
$(obj)/modpost: $(obj)/$(devicetable-offsets-file)
endif

$(obj)/$(devicetable-offsets-file): $(obj)/devicetable-offsets.s FORCE
	$(call filechk,offsets,__DEVICETABLE_OFFSETS_H__)

targets += $(devicetable-offsets-file) devicetable-offsets.s

# dependencies on generated files need to be listed explicitly

quiet_cmd_elfconfig = MKELF   $@
      cmd_elfconfig = $(obj)/mk_elfconfig < $< > $@

$(obj)/elfconfig.h: $(obj)/empty.o $(obj)/mk_elfconfig FORCE
	$(call if_changed,elfconfig)

targets += elfconfig.h
