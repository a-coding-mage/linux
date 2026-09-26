# SPDX-License-Identifier: GPL-2.0
ifeq ($(srctree),)
srctree := $(patsubst %/,%,$(dir $(CURDIR)))
srctree := $(patsubst %/,%,$(dir $(srctree)))
endif

HOST_TOOLS_LANG ?= rust
ifneq ($(HOST_TOOLS_LANG),c)
ifneq ($(HOST_TOOLS_LANG),rust)
$(error HOST_TOOLS_LANG must be 'c' or 'rust')
endif
endif
export HOST_TOOLS_LANG

include $(srctree)/tools//scripts/Makefile.include

HOSTRUSTC ?= rustc
BINDGEN ?= bindgen
PYTHON3 ?= python3
ifeq ($(HOST_TOOLS_LANG),rust)
# Cleaning existing artifacts must work without the build toolchain. Mixed
# clean/build invocations still validate before any recipes change outputs.
ifneq ($(if $(MAKECMDGOALS),$(filter-out clean,$(MAKECMDGOALS)),all),)
ifeq ($(strip $(HOSTRUSTC)),)
$(error HOSTRUSTC must name a Rust compiler)
endif
ifeq ($(shell command -v $(firstword $(HOSTRUSTC)) 2>/dev/null),)
$(error HOSTRUSTC compiler not found: $(HOSTRUSTC))
endif
ifeq ($(strip $(BINDGEN)),)
$(error BINDGEN must name a bindgen executable)
endif
ifeq ($(shell command -v $(firstword $(BINDGEN)) 2>/dev/null),)
$(error BINDGEN executable not found: $(BINDGEN))
endif
endif
endif
KBUILD_HOSTRUSTFLAGS ?= --edition=2021 -O -Zbinary_dep_depinfo=y
# Inherit SIGPIPE as the C executable does, including an inherited SIG_IGN.
# As in the top-level build, host flags and SIGPIPE inheritance need -Z.
export RUSTC_BOOTSTRAP := 1
export HOSTRUSTC KBUILD_HOSTRUSTFLAGS KBUILD_HOSTLDFLAGS
export BINDGEN PYTHON3 KBUILD_HOSTCFLAGS TOOLS_FIXDEP_BINDGEN_FLAGS

define allow-override
  $(if $(or $(findstring environment,$(origin $(1))),\
            $(findstring command line,$(origin $(1)))),,\
    $(eval $(1) = $(2)))
endef

$(call allow-override,CC,$(CROSS_COMPILE)gcc)
$(call allow-override,LD,$(CROSS_COMPILE)ld)

export HOSTCC HOSTLD HOSTAR

export srctree CC LD OUTPUT

MAKEFLAGS := --no-print-directory
build     := -f $(srctree)/tools/build/Makefile.build dir=. obj

all: $(OUTPUT)fixdep

# Make sure there's anything to clean,
# feature contains check for existing OUTPUT
TMP_O := $(if $(OUTPUT),$(OUTPUT)feature/,./)

clean:
	$(call QUIET_CLEAN, fixdep)
	$(Q)find $(or $(OUTPUT),.) -name '*.o' -delete -o -name '\.*.cmd' -delete -o -name '\.*.d' -delete
	$(Q)rm -f $(OUTPUT)fixdep $(OUTPUT)fixdep-libc.rs $(OUTPUT)fixdep-linker \
		$(OUTPUT).fixdep-abi.c $(OUTPUT).fixdep-abi.i $(OUTPUT).fixdep-abi.json
	$(call QUIET_CLEAN, feature-detect)
ifneq ($(wildcard $(TMP_O)),)
	$(Q)$(MAKE) -C feature OUTPUT=$(TMP_O) clean >/dev/null
endif

FIXDEP		:= $(OUTPUT)fixdep
FIXDEP_IN	:= $(OUTPUT)fixdep-in.o

ifeq ($(HOST_TOOLS_LANG),rust)
$(FIXDEP): FORCE
	$(Q)set -e; if [ ! -f $@ ]; then \
		$(MAKE) $(build)=fixdep $@; \
		rm -f $(dir $@).$(notdir $@).cmd; \
	fi
	$(Q)$(MAKE) $(build)=fixdep $@
else

include $(srctree)/tools/build/Build.include
quiet_cmd_fixdep_link = HOSTCC  $@
cmd_fixdep_link = $(HOSTCC) $(FIXDEP_IN) $(KBUILD_HOSTLDFLAGS) -o $@

-include $(OUTPUT).fixdep.cmd

# To track fixdep's dependencies properly, fixdep needs to run on itself.
# Build it twice the first time.
$(FIXDEP_IN): FORCE
	$(Q)set -e; if [ ! -f $(FIXDEP) ]; then				\
		$(MAKE) $(build)=fixdep HOSTCFLAGS="$(KBUILD_HOSTCFLAGS)";	\
		$(HOSTCC) $(FIXDEP_IN) $(KBUILD_HOSTLDFLAGS) -o $(FIXDEP); \
		rm -f $(FIXDEP).o;						\
	fi
	$(Q)$(MAKE) $(build)=fixdep HOSTCFLAGS="$(KBUILD_HOSTCFLAGS)"


$(FIXDEP): $(FIXDEP_IN) FORCE
	$(call if_changed,fixdep_link)
endif

FORCE:

PHONY += FORCE
.PHONY: FORCE
