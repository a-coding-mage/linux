# SPDX-License-Identifier: GPL-2.0
.DEFAULT_GOAL := all

include ../scripts/Makefile.include

ifneq ($(OUTPUT),)
override OUTPUT := $(abspath $(OUTPUT))/
endif

KERNEL_SRC := $(abspath ../..)
KERNEL_BUILD ?= $(or $(ABSOLUTE_O),$(KBUILD_OUTPUT),$(KERNEL_SRC))
KERNEL_BUILD := $(abspath $(KERNEL_BUILD))
MODULE_OUTPUT := $(abspath $(OUTPUT)vhost_test)
TESTS := virtio_test vringh_test vhost_net_test
PROGRAMS := $(addprefix $(OUTPUT),$(TESTS))
OBJECTS := $(addprefix $(OUTPUT),virtio_ring.o virtio_test.o vringh_test.o vringh.o vhost_net_test.o)
RM ?= rm -f

all: test mod
test: $(PROGRAMS)

ifneq ($(OUTPUT),)
.PHONY: $(TESTS)
$(TESTS): %: $(OUTPUT)%
endif

$(OUTPUT)virtio_test: $(OUTPUT)virtio_ring.o $(OUTPUT)virtio_test.o
$(OUTPUT)vringh_test: $(OUTPUT)vringh_test.o $(OUTPUT)vringh.o $(OUTPUT)virtio_ring.o
$(OUTPUT)vhost_net_test: $(OUTPUT)virtio_ring.o $(OUTPUT)vhost_net_test.o

$(PROGRAMS):
	$(CC) $(CFLAGS) $(LDFLAGS) $^ $(LDLIBS) -o $@

# The kernel invokes tools with -rR, so do not rely on built-in C rules.
$(OUTPUT)%.o: %.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

$(OUTPUT)virtio_ring.o: $(KERNEL_SRC)/drivers/virtio/virtio_ring.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

$(OUTPUT)vringh.o: $(KERNEL_SRC)/drivers/vhost/vringh.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

try-run = $(shell set -e;		\
	if ($(1)) >/dev/null 2>&1;	\
	then echo "$(2)";		\
	else echo "$(3)";		\
	fi)

__cc-option = $(call try-run,\
	$(1) -Werror $(2) -c -x c /dev/null -o /dev/null,$(2),)
cc-option = $(call __cc-option, $(CC),$(1))

override CPPFLAGS += -I. -I../include/ -I$(KERNEL_BUILD)/usr/include/ -U_FORTIFY_SOURCE -include $(KERNEL_SRC)/include/linux/kconfig.h
override CFLAGS += -g -O2 -Werror $(call cc-option,-Wno-maybe-uninitialized) -Wall -Wno-pointer-sign -fno-strict-overflow -fno-strict-aliasing -fno-common -MMD $(call cc-option,-mfunction-return=thunk) $(call cc-option,-fcf-protection=none) $(call cc-option,-mindirect-branch-register)

override CFLAGS += -pthread
override LDFLAGS += -pthread
# Re-enter kbuild so it parses M=/MO= rather than inheriting tools/all's
# internal state. MO= keeps external-module artifacts out of the sources.
BUILD = unset sub_make_done; $(MAKE) -C $(KERNEL_SRC) O=$(KERNEL_BUILD) V=$(V)
mod:
	$(BUILD) M=$(CURDIR)/vhost_test MO=$(MODULE_OUTPUT) modules

#oot: build vhost as an out of tree module for a distro kernel
#no effort is taken to make it actually build or work, but tends to mostly work
#if the distro kernel is very close to upstream
#unsupported! this is a development tool only, don't use the
#resulting modules in production!
OOT_KSRC ?= /lib/modules/$(shell uname -r)/build
OOT_VHOST := $(KERNEL_SRC)/drivers/vhost
#Everyone depends on vhost
#Tweak the below to enable more modules
OOT_CONFIGS=\
	CONFIG_VHOST=m \
	CONFIG_VHOST_NET=n \
	CONFIG_VHOST_SCSI=n \
	CONFIG_VHOST_VSOCK=n \
	CONFIG_VHOST_RING=n \
	CONFIG_VHOST_VDPA=n
OOT_BUILD = unset sub_make_done; $(MAKE) -C $(OOT_KSRC) O=$(abspath $(OOT_KSRC)) V=$(V) \
	KCFLAGS="$(KCFLAGS) -include $(CURDIR)/oot-stubs.h -I $(OOT_VHOST)"
oot-build:
	echo "UNSUPPORTED! Don't use the resulting modules in production!"
	$(OOT_BUILD) M=$(CURDIR)/vhost_test MO=$(abspath $(OUTPUT)oot-vhost_test)
	$(OOT_BUILD) M=$(OOT_VHOST) MO=$(abspath $(OUTPUT)oot-vhost) $(OOT_CONFIGS)

oot-clean: oot-build
oot: oot-build
oot-clean: OOT_BUILD+=clean

.PHONY: all test mod clean vhost oot oot-clean oot-build
clean:
	$(RM) $(OBJECTS) $(PROGRAMS) $(OBJECTS:.o=.d) \
		$(MODULE_OUTPUT)/*.o $(MODULE_OUTPUT)/*.ko $(MODULE_OUTPUT)/*.mod \
		$(MODULE_OUTPUT)/*.mod.c $(MODULE_OUTPUT)/.*.cmd \
		$(MODULE_OUTPUT)/Module.symvers $(MODULE_OUTPUT)/modules.order
-include $(OBJECTS:.o=.d)
