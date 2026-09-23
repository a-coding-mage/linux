# SPDX-License-Identifier: GPL-2.0
hostprogs := elf-entry
ifneq ($(HOST_TOOLS_LANG),c)
elf-entry-rust := y
endif
PHONY += elf-entry
elf-entry: $(obj)/elf-entry
	@:

hostprogs += loongson3-llsc-check
ifneq ($(HOST_TOOLS_LANG),c)
loongson3-llsc-check-rust := y
endif
PHONY += loongson3-llsc-check
loongson3-llsc-check: $(obj)/loongson3-llsc-check
	@:
