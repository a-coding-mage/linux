# SPDX-License-Identifier: GPL-2.0
# Exercise the actual tools bootstrap include and its exported OUTPUT contract.
OUTPUT := $(OUT)/
export OUTPUT srctree
all: $(OUTPUT)sample-in.o
include $(srctree)/tools/build/Makefile.include
$(OUTPUT)sample-in.o: fixdep FORCE
	$(MAKE) $(build)=sample
FORCE:
.PHONY: all FORCE
