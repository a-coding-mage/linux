# SPDX-License-Identifier: GPL-2.0
include ../scripts/Makefile.include

prefix = /usr
nosy-dump-version = 0.4

nosy-dump-target := $(OUTPUT)nosy-dump
nosy-dump-objs := $(addprefix $(OUTPUT),nosy-dump.o decode-fcp.o)
nosy-dump-deps := $(nosy-dump-objs:.o=.d)

all : $(nosy-dump-target)

$(nosy-dump-target) : CFLAGS = -Wall -O2 -g
$(nosy-dump-target) : CPPFLAGS = -DVERSION=\"$(nosy-dump-version)\" -I../../drivers/firewire
$(nosy-dump-target) : LDFLAGS = -g
$(nosy-dump-target) : LDLIBS = -lpopt

$(nosy-dump-target) : $(nosy-dump-objs)
	$(CC) $(LDFLAGS) $^ $(LDLIBS) -o $@

$(OUTPUT)%.o : %.c
	mkdir -p $(dir $@)
	$(CC) $(CPPFLAGS) $(CFLAGS) -MMD -MP -c $< -o $@

-include $(nosy-dump-deps)

clean :
	rm -f $(nosy-dump-objs) $(nosy-dump-deps) $(nosy-dump-target)

install : $(nosy-dump-target)
	install $(nosy-dump-target) $(prefix)/bin/nosy-dump

.PHONY: all clean install
