# SPDX-License-Identifier: GPL-2.0
include ../scripts/Makefile.include

bindir ?= /usr/bin

# This will work when dma is built in tools env. where srctree
# isn't set and when invoked from selftests build, where srctree
# is set to ".". building_out_of_srctree is undefined for in srctree
# builds
ifeq ($(strip $(srctree)),)
srctree := $(abspath $(CURDIR)/../..)
endif
ifndef building_out_of_srctree
srctree := $(patsubst %/,%,$(dir $(CURDIR)))
srctree := $(patsubst %/,%,$(dir $(srctree)))
endif

# Do not use make's built-in rules
# (this improves performance and avoids hard-to-debug behaviour);
MAKEFLAGS += -r

override CFLAGS += -O2 -Wall -g -D_GNU_SOURCE -I$(OUTPUT)include

ALL_TARGETS := dma_map_benchmark
ALL_PROGRAMS := $(patsubst %,$(OUTPUT)%,$(ALL_TARGETS))

all: $(ALL_PROGRAMS)

export srctree OUTPUT CC LD CFLAGS
include $(srctree)/tools/build/Makefile.include
include $(srctree)/tools/build/Build.include

HOST_TOOLS_LANG ?= rust
ifneq ($(HOST_TOOLS_LANG),c)
ifneq ($(HOST_TOOLS_LANG),rust)
$(error HOST_TOOLS_LANG must be 'c' or 'rust')
endif
endif
HOSTRUSTC ?= rustc
BINDGEN ?= bindgen
KBUILD_HOSTRUSTFLAGS ?= --edition=2021 -O

#
# We need the following to be outside of kernel tree
#
$(OUTPUT)include/linux/map_benchmark.h: ../../include/uapi/linux/map_benchmark.h
	mkdir -p $(OUTPUT)include/linux 2>&1 || true
	ln -sf $(CURDIR)/../../include/uapi/linux/map_benchmark.h $@

prepare: $(OUTPUT)include/linux/map_benchmark.h

FORCE:

DMA_MAP_BENCHMARK = $(OUTPUT)dma_map_benchmark
DMA_MAP_BINDINGS = $(OUTPUT)dma_map_benchmark-bindings.rs
ifeq ($(HOST_TOOLS_LANG),rust)
ifneq ($(filter-out clean,$(or $(MAKECMDGOALS),all)),)
ifneq ($(CROSS_COMPILE),)
ifeq ($(RUST_TARGET),)
$(error Set RUST_TARGET to the matching Rust triple for CROSS_COMPILE)
endif
endif
ifneq ($(filter --target --target=%,$(KBUILD_HOSTRUSTFLAGS) $(HOSTRUSTFLAGS)),)
$(error Set RUST_TARGET so bindgen and rustc use the same target)
endif
endif
PHONY += prepare FORCE
DMA_RUST_LINKER = $(OUTPUT)dma_map_benchmark-rust-linker
quiet_cmd_dma_linker = LINKER $@
cmd_dma_linker = mkdir -p $(or $(OUTPUT),.); printf '%s\n' '\#!/bin/sh' \
	'exec $(call escsq,$(CC) $(CFLAGS)) "$$@" $(call escsq,$(LDFLAGS))' > $@; chmod +x $@

$(DMA_RUST_LINKER): FORCE
	$(call if_changed,dma_linker)

quiet_cmd_dma_bindings = BINDGEN $@
cmd_dma_bindings = $(BINDGEN) dma_map_benchmark-bindings.h \
	--allowlist-type '^map_benchmark$$' \
	--allowlist-var '^(DMA_MAP_.*|RUST_DMA_MAP_.*)$$' \
	--no-layout-tests --no-doc-comments --use-core \
	--depfile $(depfile) --output $@ -- $(CFLAGS) $(if $(RUST_TARGET),--target=$(RUST_TARGET))

$(DMA_MAP_BINDINGS): dma_map_benchmark-bindings.h dma_map_benchmark.c \
		$(OUTPUT)include/linux/map_benchmark.h FORCE
	$(call if_changed_dep,dma_bindings)

quiet_cmd_dma_rust = RUSTC $@
cmd_dma_rust = DMA_MAP_BINDINGS=$(abspath $(DMA_MAP_BINDINGS)) \
	$(HOSTRUSTC) $(KBUILD_HOSTRUSTFLAGS) $(HOSTRUSTFLAGS) \
	$(if $(RUST_TARGET),--target=$(RUST_TARGET)) \
	-Clinker=$(abspath $(DMA_RUST_LINKER)) --out-dir $(or $(OUTPUT),.) \
	--emit=dep-info=$(depfile),link=$@ dma_map_benchmark.rs

$(DMA_MAP_BENCHMARK): dma_map_benchmark.rs $(DMA_MAP_BINDINGS) $(DMA_RUST_LINKER) FORCE
	$(call if_changed_dep,dma_rust)

-include $(OUTPUT).dma_map_benchmark.cmd $(OUTPUT).dma_map_benchmark-bindings.rs.cmd \
	$(OUTPUT).dma_map_benchmark-rust-linker.cmd
else
$(DMA_MAP_BENCHMARK): prepare FORCE
	$(CC) $(CFLAGS) dma_map_benchmark.c -o $@
	rm -f $(OUTPUT).dma_map_benchmark.cmd $(OUTPUT).dma_map_benchmark.d
endif

clean:
	rm -f $(ALL_PROGRAMS)
	rm -f $(DMA_MAP_BINDINGS)
	rm -f $(OUTPUT)dma_map_benchmark-rust-linker
	rm -rf $(OUTPUT)include
	find $(or $(OUTPUT),.) -name '*.o' -delete -o -name '\.*.d' -delete -o -name '\.*.cmd' -delete

install: $(ALL_PROGRAMS)
	install -d -m 755 $(DESTDIR)$(bindir);		\
	for program in $(ALL_PROGRAMS); do		\
		install $$program $(DESTDIR)$(bindir);	\
	done

.PHONY: all install clean prepare FORCE
