# SPDX-License-Identifier: GPL-2.0
obj-$(CONFIG_SPEAKUP_SYNTH_ACNTSA) += speakup_acntsa.o
obj-$(CONFIG_SPEAKUP_SYNTH_ACNTPC) += speakup_acntpc.o
obj-$(CONFIG_SPEAKUP_SYNTH_APOLLO) += speakup_apollo.o
obj-$(CONFIG_SPEAKUP_SYNTH_AUDPTR) += speakup_audptr.o
obj-$(CONFIG_SPEAKUP_SYNTH_BNS) += speakup_bns.o
obj-$(CONFIG_SPEAKUP_SYNTH_DECTLK) += speakup_dectlk.o
obj-$(CONFIG_SPEAKUP_SYNTH_DECEXT) += speakup_decext.o
obj-$(CONFIG_SPEAKUP_SYNTH_DECPC) += speakup_decpc.o
obj-$(CONFIG_SPEAKUP_SYNTH_DTLK) += speakup_dtlk.o
obj-$(CONFIG_SPEAKUP_SYNTH_KEYPC) += speakup_keypc.o
obj-$(CONFIG_SPEAKUP_SYNTH_LTLK) += speakup_ltlk.o
obj-$(CONFIG_SPEAKUP_SYNTH_SOFT) += speakup_soft.o
obj-$(CONFIG_SPEAKUP_SYNTH_SPKOUT) += speakup_spkout.o
obj-$(CONFIG_SPEAKUP_SYNTH_TXPRT) += speakup_txprt.o
obj-$(CONFIG_SPEAKUP_SYNTH_DUMMY) += speakup_dummy.o

obj-$(CONFIG_SPEAKUP) += speakup.o
speakup-y := \
	buffers.o \
	devsynth.o \
	i18n.o \
	fakekey.o \
	main.o \
	keyhelp.o \
	kobjects.o \
	selection.o \
	spk_ttyio.o \
	synth.o \
	thread.o \
	varhandlers.o
speakup-$(CONFIG_SPEAKUP_SERIALIO) += serialio.o


clean-files := mapdata.h mapdata.bin makemapdata.frontend speakupmap.h


# Generate mapdata.h from headers
hostprogs += makemapdata
ifeq ($(HOST_TOOLS_LANG),c)
makemapdata-objs := makemapdata.o
else
makemapdata-rust := y
$(obj)/makemapdata: $(obj)/makemapdata.frontend
endif

# Validate the retained source and extract only compiler-produced initializer
# data. No C algorithm object or C bridge is linked into either Rust tool.
speakup-python = $(if $(PYTHON3),$(PYTHON3),python3)
speakup-cflags = $(KBUILD_HOSTCFLAGS) $(HOST_EXTRACFLAGS) $(HOSTCFLAGS_$(speakup-tool).o) -I $(obj)
quiet_cmd_spkfrontend = HOSTDATA $@
      cmd_spkfrontend = $(speakup-python) $(src)/mapdata_frontend.py \
	$(if $(filter genmap,$(speakup-tool)),--mapdata $(obj)/mapdata.h) \
	--depfile $(depfile) $(src)/$(speakup-tool).c $@ -- $(HOSTCC) $(speakup-cflags)

$(obj)/makemapdata.frontend: private speakup-tool := makemapdata
$(obj)/makemapdata.frontend: $(src)/makemapdata.c $(src)/utils.h $(src)/mapdata_frontend.py FORCE
	$(call if_changed_dep,spkfrontend)

$(obj)/mapdata.bin: private speakup-tool := genmap
$(obj)/mapdata.bin: $(obj)/mapdata.h $(src)/genmap.c $(src)/utils.h $(src)/mapdata_frontend.py FORCE
	$(call if_changed_dep,spkfrontend)

quiet_cmd_mkmap = MKMAP   $@
      cmd_mkmap = TOPDIR=$(srctree) SPKDIR=$(src) $(obj)/makemapdata > $@

$(obj)/mapdata.h: $(obj)/makemapdata $(srctree)/include/linux/input.h \
		$(srctree)/include/uapi/linux/input-event-codes.h \
		$(src)/spk_priv_keyinfo.h FORCE
	$(call if_changed,mkmap)


# Generate speakupmap.h from mapdata.h
hostprogs += genmap
ifeq ($(HOST_TOOLS_LANG),c)
genmap-objs := genmap.o
$(obj)/genmap.o: $(obj)/mapdata.h
else
genmap-rust := y
$(obj)/genmap: private export SPEAKUP_MAPDATA := $(abspath $(obj)/mapdata.bin)
$(obj)/genmap: $(obj)/mapdata.bin
endif

quiet_cmd_genmap = GENMAP  $@
      cmd_genmap = $(obj)/genmap $< > $@

$(obj)/speakupmap.h: $(src)/speakupmap.map $(obj)/genmap FORCE
	$(call if_changed,genmap)

targets += mapdata.h mapdata.bin makemapdata.frontend speakupmap.h

$(obj)/main.o: $(obj)/speakupmap.h
