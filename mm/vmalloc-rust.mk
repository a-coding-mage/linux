# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the mmu-$(CONFIG_MMU) object list in mm/Makefile.
ifeq ($(CONFIG_RUST_VMALLOC),y)
mmu-$(CONFIG_MMU) := $(patsubst vmalloc.o,vmalloc.o vmalloc_helpers.o,$(mmu-$(CONFIG_MMU)))

# Original vmalloc.c has no SSP exemption; retain reviewed native strong protection.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_VMALLOC native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_VMALLOC native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_VMALLOC native per-init coverage exemption is not integrated)
endif
ifeq ($(CONFIG_PRINTK_INDEX),y)
$(error RUST_VMALLOC native per-callsite printk index records are not integrated)
endif

RUSTFLAGS_vmalloc.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_vmalloc.o += cfi_encoding
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_vmalloc.o = $(dir $@).$(notdir $@).rustc/

vmalloc-rust-inputs := $(addprefix $(src)/,vmalloc_allocator.rs vmalloc_blocks.rs vmalloc_constants.rs vmalloc_init.rs vmalloc_lifetime.rs vmalloc_percpu.rs vmalloc_primitive_aliases.rs vmalloc_readback.rs) \
    $(objtree)/rust/bindings/vmalloc_generated.rs $(rust-native-policy-inputs)
vmalloc-rust-targets := $(addprefix $(obj)/,vmalloc.o vmalloc.s vmalloc.ll \
    vmalloc.rsi .vmalloc-rust-listing/vmalloc.o)
$(vmalloc-rust-targets): private target-stem := vmalloc
$(vmalloc-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(vmalloc-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/vmalloc.o,$(real-obj-y) $(lib-y)),y)
$(vmalloc-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/vmalloc.o: $(src)/vmalloc.rs $(vmalloc-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/vmalloc.s: $(src)/vmalloc.rs $(vmalloc-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/vmalloc.ll: $(src)/vmalloc.rs $(vmalloc-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_vmalloc_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_vmalloc_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/vmalloc.rsi: $(src)/vmalloc.rs $(vmalloc-rust-inputs) FORCE
	+$(call if_changed_dep,vmalloc_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.vmalloc-rust-listing/vmalloc.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.vmalloc-rust-listing/vmalloc.o: private override KBUILD_CFLAGS += -g
$(obj)/.vmalloc-rust-listing/vmalloc.o: $(src)/vmalloc.rs $(vmalloc-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_vmalloc_rust_lst_elf = LD      $@
      cmd_vmalloc_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.vmalloc-rust-listing-elf/vmalloc.o: $(obj)/.vmalloc-rust-listing/vmalloc.o FORCE
	$(call if_changed,vmalloc_rust_lst_elf)
quiet_cmd_vmalloc_rust_lst = MKLST   $@
      cmd_vmalloc_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/vmalloc.lst: $(obj)/.vmalloc-rust-listing-elf/vmalloc.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,vmalloc_rust_lst)

targets += vmalloc.s vmalloc.ll vmalloc.rsi vmalloc.lst \
    .vmalloc-rust-listing/vmalloc.o .vmalloc-rust-listing-elf/vmalloc.o
endif
clean-files += .vmalloc-rust-listing/ .vmalloc-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.vmalloc.$(ext).rustc/)
