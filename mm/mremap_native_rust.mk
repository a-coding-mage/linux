# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y += mremap.o in mm/Makefile.
ifeq ($(CONFIG_RUST_MREMAP),y)
obj-y := $(patsubst mremap.o,mremap.o mremap_native_helpers.o,$(obj-y))

# Original mremap.c uses native strong protection with no SSP exemptions.
# Enroll mm/mremap in scripts/rust-native-policy-owners before selecting.
include $(srctree)/scripts/Makefile.rust-native-policy

# Fail closed on native instrumentation combinations not yet audited.
ifeq ($(CONFIG_CFI),y)
$(error RUST_MREMAP native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_MREMAP native mixed-language latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_MREMAP native mixed-language stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_MREMAP native mixed-language coverage policy is not integrated)
endif

RUSTFLAGS_mremap.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_mremap.o += cfi_encoding
# Header-only architecture predicates are obtained from the current binding
# prerequisite at recipe evaluation time, including first builds.
RUSTFLAGS_mremap.o += $(shell sed -n 's/^pub const RUST_MREMAP_CFG_\([A-Z0-9_]*\): bool[_]* = true;/--cfg RUST_MREMAP_\1/p' $(objtree)/rust/bindings/mremap_native_generated.rs 2>/dev/null)
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_mremap.o = $(dir $@).$(notdir $@).rustc/

mremap-rust-inputs := \
    $(objtree)/rust/bindings/mremap_native_generated.rs $(rust-native-policy-inputs)
mremap-rust-targets := $(addprefix $(obj)/,mremap.o mremap.s mremap.ll \
    mremap.rsi .mremap-rust-listing/mremap.o)
$(mremap-rust-targets): private target-stem := mremap
$(mremap-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(mremap-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/mremap.o,$(real-obj-y) $(lib-y)),y)
$(mremap-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/mremap.o: $(src)/mremap.rs $(mremap-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/mremap.s: $(src)/mremap.rs $(mremap-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/mremap.ll: $(src)/mremap.rs $(mremap-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_mremap_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_mremap_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/mremap.rsi: $(src)/mremap.rs $(mremap-rust-inputs) FORCE
	+$(call if_changed_dep,mremap_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.mremap-rust-listing/mremap.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.mremap-rust-listing/mremap.o: private override KBUILD_CFLAGS += -g
$(obj)/.mremap-rust-listing/mremap.o: $(src)/mremap.rs $(mremap-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_mremap_rust_lst_elf = LD      $@
      cmd_mremap_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.mremap-rust-listing-elf/mremap.o: $(obj)/.mremap-rust-listing/mremap.o FORCE
	$(call if_changed,mremap_rust_lst_elf)
quiet_cmd_mremap_rust_lst = MKLST   $@
      cmd_mremap_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/mremap.lst: $(obj)/.mremap-rust-listing-elf/mremap.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,mremap_rust_lst)

targets += mremap.s mremap.ll mremap.rsi mremap.lst \
    .mremap-rust-listing/mremap.o .mremap-rust-listing-elf/mremap.o
endif
clean-files += .mremap-rust-listing/ .mremap-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.mremap.$(ext).rustc/)
