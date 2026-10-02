# SPDX-License-Identifier: GPL-2.0
PHONY += posttest

ifneq ($(findstring 1, $(KBUILD_VERBOSE)),)
  posttest_verbose = -v
else
  posttest_verbose =
endif

ifeq ($(CONFIG_64BIT),y)
  posttest_64bit = -y
else
  posttest_64bit = -n
endif

reformatter = $(srctree)/arch/x86/tools/objdump_reformat.awk
chkobjdump = $(srctree)/arch/x86/tools/chkobjdump.awk

quiet_cmd_posttest = TEST    $@
      cmd_posttest = $(OBJDUMP) -d -j .text $(objtree)/vmlinux | $(AWK) -f $(reformatter) | $(obj)/insn_decoder_test $(posttest_64bit) $(posttest_verbose)

quiet_cmd_sanitytest = TEST    $@
      cmd_sanitytest = $(obj)/insn_sanity $(posttest_64bit) -m 1000000

posttest: $(obj)/insn_decoder_test vmlinux $(obj)/insn_sanity
	$(call cmd,posttest)
	$(call cmd,sanitytest)

hostprogs += insn_decoder_test insn_sanity

# -I needed for generated C source and C source which in the kernel tree.
HOSTCFLAGS_insn_decoder_test.o := -Wall -I$(srctree)/tools/arch/x86/lib/ -I$(srctree)/tools/arch/x86/include/ -I$(objtree)/arch/x86/lib/

HOSTCFLAGS_insn_sanity.o := -Wall -I$(srctree)/tools/arch/x86/lib/ -I$(srctree)/tools/arch/x86/include/ -I$(objtree)/arch/x86/lib/

ifneq ($(HOST_TOOLS_LANG),c)
# Always compile the original C tests. Their embedded decoder includes resolve
# to declarations here; the existing Rust C ABI supplies the implementation.
HOSTCFLAGS_insn_decoder_test.o := -I$(src)/insn-host $(HOSTCFLAGS_insn_decoder_test.o)
HOSTCFLAGS_insn_sanity.o := -I$(src)/insn-host $(HOSTCFLAGS_insn_sanity.o)
HOSTLDLIBS_insn_decoder_test += $(obj)/insn_host.a -ldl -lpthread -lm
HOSTLDLIBS_insn_sanity += $(obj)/insn_host.a -ldl -lpthread -lm

$(obj)/insn_decoder_test: $(src)/insn_decoder_test.c $(obj)/insn_host.a
$(obj)/insn_sanity: $(src)/insn_sanity.c $(obj)/insn_host.a

BINDGEN ?= bindgen
quiet_cmd_insn_bindgen = BINDGEN $@
      cmd_insn_bindgen = $(BINDGEN) $< --output $@ \
	--allowlist-type 'insn|insn_field|insn_mode' --newtype-enum insn_mode \
	--use-core --ctypes-prefix core::ffi --no-layout-tests --no-doc-comments \
	--no-debug '.*' --rust-target 1.85 -- \
	-I$(srctree)/tools/arch/x86/include -I$(srctree)/tools/include \
	-MMD -MF $(depfile) -MT $@

$(obj)/insn_host_bindings.rs: $(srctree)/tools/arch/x86/include/asm/insn.h FORCE
	$(call if_changed_dep,insn_bindgen)

quiet_cmd_insn_host = HOSTRUSTC $@
      cmd_insn_host = INSN_HOST_BINDINGS=$(abspath $(obj)/insn_host_bindings.rs) \
	$(HOSTRUSTC) $(hostrust_flags) --crate-type=staticlib -Cpanic=abort \
	--check-cfg='cfg(CONFIG_X86_64)' $(if $(filter y,$(CONFIG_X86_64)),--cfg CONFIG_X86_64) \
	--emit=link=$@ $<

$(obj)/insn_host.a: $(src)/insn_host.rs $(obj)/insn_host_bindings.rs FORCE
	+$(call if_changed_dep,insn_host)

targets += insn_host.a insn_host_bindings.rs
else
# Dependencies are also needed.
$(obj)/insn_decoder_test.o: $(srctree)/tools/arch/x86/lib/insn.c $(srctree)/tools/arch/x86/lib/inat.c $(srctree)/tools/arch/x86/include/asm/inat_types.h $(srctree)/tools/arch/x86/include/asm/inat.h $(srctree)/tools/arch/x86/include/asm/insn.h $(objtree)/arch/x86/lib/inat-tables.c

$(obj)/insn_sanity.o: $(srctree)/tools/arch/x86/lib/insn.c $(srctree)/tools/arch/x86/lib/inat.c $(srctree)/tools/arch/x86/include/asm/inat_types.h $(srctree)/tools/arch/x86/include/asm/inat.h $(srctree)/tools/arch/x86/include/asm/insn.h $(objtree)/arch/x86/lib/inat-tables.c
endif

HOST_EXTRACFLAGS += -I$(srctree)/tools/include -I$(srctree)/include/uapi \
		    -I$(srctree)/arch/$(SUBARCH)/include/uapi

hostprogs	+= relocs vdso2c
ifneq ($(HOST_TOOLS_LANG),c)
relocs-rust	:= y
HOSTRUSTFLAGS_relocs += --check-cfg='cfg(CONFIG_FW_LOADER)'
vdso2c-rust	:= y
else
relocs-objs	:= relocs_32.o relocs_64.o relocs_common.o
endif

always-y	:= $(hostprogs)

PHONY += $(hostprogs)
$(hostprogs): %: $(obj)/%
	@:
