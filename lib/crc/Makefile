# SPDX-License-Identifier: GPL-2.0-only

# Makefile for the kernel's cyclic redundancy check (CRC) library code

obj-$(CONFIG_CRC4) += crc4.o
obj-$(CONFIG_CRC7) += crc7.o
obj-$(CONFIG_CRC8) += crc8.o
obj-$(CONFIG_CRC16) += crc16.o
ifeq ($(CONFIG_RUST_CRC16),y)
# Retain the original tristate and crc16.ko name while selecting Rust for all
# original-object inspection targets, despite the same-basename C source.
RUST_ALLOWED_FEATURES_crc16.o := $(if $(CONFIG_PPC),asm_experimental_arch)
$(obj)/crc16.o: $(src)/crc16.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)



# Explicit targets have no implicit stem; keep the canonical owner identity.
$(addprefix $(obj)/,crc16.o crc16.s crc16.ll crc16.rsi .rust-listing/crc16.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))

# Inspection uses the selected source and its canonical per-object policy.
$(addprefix $(obj)/,crc16.s crc16.ll crc16.rsi): private target-stem := crc16
$(addprefix $(obj)/,crc16.s crc16.ll crc16.rsi): private part-of-builtin = $(if $(filter $(obj)/crc16.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,crc16.s crc16.ll crc16.rsi): private part-of-module = $(if $(filter $(obj)/crc16.o,$(real-obj-m)),y)
$(obj)/crc16.s: $(src)/crc16.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/crc16.ll: $(src)/crc16.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/crc16.rsi: $(src)/crc16.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Debug information belongs only to this unlinked listing object.
$(obj)/.rust-listing/crc16.o: private target-stem := crc16
$(obj)/.rust-listing/crc16.o: private part-of-builtin = $(if $(filter $(obj)/crc16.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/crc16.o: private part-of-module = $(if $(filter $(obj)/crc16.o,$(real-obj-m)),y)
$(obj)/.rust-listing/crc16.o: private is-single-obj-m = $(and $(part-of-module),$(filter $(obj)/crc16.o,$(obj-m)),y)
$(obj)/.rust-listing/crc16.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/crc16.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/crc16.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/crc16.o: $(src)/crc16.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_crc16_rust_lst_elf = LD      $@
      cmd_crc16_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/crc16.o: $(obj)/.rust-listing/crc16.o FORCE
	$(call if_changed,crc16_rust_lst_elf)
# Keep makelst's real System.map behavior, including creation and deletion.
quiet_cmd_crc16_rust_lst = MKLST   $@
      cmd_crc16_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/crc16.lst: $(obj)/.rust-listing-elf/crc16.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,crc16_rust_lst)
targets += crc16.s crc16.ll crc16.rsi crc16.lst \
	.rust-listing/crc16.o .rust-listing-elf/crc16.o
endif
obj-$(CONFIG_CRC_CCITT) += crc-ccitt.o
obj-$(CONFIG_CRC_ITU_T) += crc-itu-t.o

obj-$(CONFIG_CRC_T10DIF) += crc-t10dif.o
crc-t10dif-y := crc-t10dif-main.o
ifeq ($(CONFIG_CRC_T10DIF_ARCH),y)
CFLAGS_crc-t10dif-main.o += -I$(src)/$(SRCARCH)
crc-t10dif-$(CONFIG_ARM) += arm/crc-t10dif-core.o
crc-t10dif-$(CONFIG_ARM64) += arm64/crc-t10dif-core.o
crc-t10dif-$(CONFIG_PPC) += powerpc/crct10dif-vpmsum_asm.o
crc-t10dif-$(CONFIG_RISCV) += riscv/crc16_msb.o
crc-t10dif-$(CONFIG_X86) += x86/crc16-msb-pclmul.o
endif

obj-$(CONFIG_CRC32) += crc32.o
crc32-y := crc32-main.o
ifeq ($(CONFIG_CRC32_ARCH),y)
CFLAGS_crc32-main.o += -I$(src)/$(SRCARCH)
crc32-$(CONFIG_ARM) += arm/crc32-core.o
crc32-$(CONFIG_ARM64) += arm64/crc32-core.o
crc32-$(CONFIG_PPC) += powerpc/crc32c-vpmsum_asm.o
crc32-$(CONFIG_RISCV) += riscv/crc32_lsb.o riscv/crc32_msb.o
crc32-$(CONFIG_S390) += s390/crc32le-vx.o s390/crc32be-vx.o
crc32-$(CONFIG_SPARC) += sparc/crc32c_asm.o
crc32-$(CONFIG_X86) += x86/crc32-pclmul.o
crc32-$(CONFIG_X86_64) += x86/crc32c-3way.o
endif

obj-$(CONFIG_CRC64) += crc64.o
crc64-y := crc64-main.o
ifeq ($(CONFIG_CRC64_ARCH),y)
CFLAGS_crc64-main.o += -I$(src)/$(SRCARCH)

crc64-cflags-$(CONFIG_ARM) += -march=armv8-a -mfpu=crypto-neon-fp-armv8
crc64-cflags-$(CONFIG_ARM64) += -march=armv8-a+crypto
CFLAGS_REMOVE_crc64-neon.o += $(CC_FLAGS_NO_FPU)
CFLAGS_crc64-neon.o += $(CC_FLAGS_FPU) -I$(src)/$(SRCARCH) $(crc64-cflags-y)
crc64-$(CONFIG_ARM) += crc64-neon.o
crc64-$(CONFIG_ARM64) += crc64-neon.o

crc64-$(CONFIG_RISCV) += riscv/crc64_lsb.o riscv/crc64_msb.o
crc64-$(CONFIG_X86) += x86/crc64-pclmul.o
endif # CONFIG_CRC64_ARCH

obj-y += tests/

hostprogs := gen_crc32table gen_crc64table
ifneq ($(HOST_TOOLS_LANG),c)
gen_crc32table-rust := y
gen_crc64table-rust := y
endif
clean-files := crc32table.h crc64table.h

$(obj)/crc32-main.o: $(obj)/crc32table.h
$(obj)/crc64-main.o: $(obj)/crc64table.h

quiet_cmd_crc32 = GEN     $@
      cmd_crc32 = $< > $@

quiet_cmd_crc64 = GEN     $@
      cmd_crc64 = $< > $@

$(obj)/crc32table.h: $(obj)/gen_crc32table
	$(call cmd,crc32)

$(obj)/crc64table.h: $(obj)/gen_crc64table
	$(call cmd,crc64)

# Private inspection sidecars remain cleanable after deselection.
clean-files += .rust-listing/ .rust-listing-elf/
