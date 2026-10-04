# SPDX-License-Identifier: GPL-2.0
# Proposed include immediately after obj-y += slub.o in mm/Makefile.
ifeq ($(CONFIG_RUST_SLUB),y)
obj-y += slub_helpers.o

# Original slub.c has no SSP exemption; retain reviewed native strong protection.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_SLUB native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_SLUB native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_SLUB native per-init coverage exemption is not integrated)
endif
ifeq ($(CONFIG_PRINTK_INDEX),y)
$(error RUST_SLUB native per-callsite printk index records are not integrated)
endif

RUSTFLAGS_slub.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_slub.o += cfi_encoding,no_sanitize
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_slub.o = $(dir $@).$(notdir $@).rustc/

slub-rust-inputs := $(addprefix $(src)/,slub_foundation.rs slub_debug.rs slub_debug_tail.rs slub_hooks.rs slub_sheaves.rs slub_slabs.rs slub_allocation.rs) \
    $(objtree)/rust/bindings/slub_generated.rs $(rust-native-policy-inputs)
slub-rust-targets := $(addprefix $(obj)/,slub.o slub.s slub.ll \
    slub.rsi .slub-rust-listing/slub.o)
$(slub-rust-targets): private target-stem := slub
$(slub-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(slub-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/slub.o,$(real-obj-y) $(lib-y)),y)
$(slub-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/slub.o: $(src)/slub.rs $(slub-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/slub.s: $(src)/slub.rs $(slub-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/slub.ll: $(src)/slub.rs $(slub-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_slub_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_slub_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/slub.rsi: $(src)/slub.rs $(slub-rust-inputs) FORCE
	+$(call if_changed_dep,slub_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.slub-rust-listing/slub.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.slub-rust-listing/slub.o: private override KBUILD_CFLAGS += -g
$(obj)/.slub-rust-listing/slub.o: $(src)/slub.rs $(slub-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_slub_rust_lst_elf = LD      $@
      cmd_slub_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.slub-rust-listing-elf/slub.o: $(obj)/.slub-rust-listing/slub.o FORCE
	$(call if_changed,slub_rust_lst_elf)
quiet_cmd_slub_rust_lst = MKLST   $@
      cmd_slub_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/slub.lst: $(obj)/.slub-rust-listing-elf/slub.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,slub_rust_lst)

targets += slub.s slub.ll slub.rsi slub.lst \
    .slub-rust-listing/slub.o .slub-rust-listing-elf/slub.o
endif
clean-files += .slub-rust-listing/ .slub-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.slub.$(ext).rustc/)
# Native leaves inherit only the original translation-unit exclusions.
KASAN_SANITIZE_slub_helpers.o := $(KASAN_SANITIZE_slub.o)
KCSAN_SANITIZE_slub_helpers.o := $(KCSAN_SANITIZE_slub.o)
KCOV_INSTRUMENT_slub_helpers.o := $(KCOV_INSTRUMENT_slub.o)
# Header and registration companions are included into this object, not linked
# as standalone C algorithm objects.
$(obj)/slub_helpers.o: $(src)/slub_bindings.h $(src)/slub_late_types.h \
    $(src)/slub_allocation_types.h $(src)/slub_leaves.inc \
    $(src)/slub_allocation_leaves.inc $(src)/slub_allocation_exports.c
