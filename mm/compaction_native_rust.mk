# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y += compaction.o in mm/Makefile.
ifeq ($(CONFIG_RUST_COMPACTION),y)
obj-y := $(patsubst compaction.o,compaction.o compaction_native_helpers.o,$(obj-y))

ifeq ($(CONFIG_RANDSTRUCT),y)
$(error RUST_COMPACTION native randomized layout transport is not integrated)
endif

# Original compaction.c uses native strong protection with no SSP exemptions.
# Enroll mm/compaction in scripts/rust-native-policy-owners before selecting.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_CFI),y)
$(error RUST_COMPACTION native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_COMPACTION native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_COMPACTION native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_COMPACTION native per-init coverage exemption is not integrated)
endif

RUSTFLAGS_compaction.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_compaction.o += cfi_encoding
# Mirror native __init cold only when compiler_types.h enables __cold.
# Deferred extraction uses the completed current binding prerequisite.
# Header metadata extraction must fail closed; the recipe precheck below
# surfaces parse errors rather than silently losing configured annotations.
RUSTFLAGS_compaction.o += $(shell $(CONFIG_SHELL) $(src)/compaction_native_cfg.sh $(objtree)/rust/bindings/compaction_native_generated.rs $(CONFIG_COMPACTION) 2>/dev/null)
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_compaction.o = $(dir $@).$(notdir $@).rustc/

compaction-rust-inputs := \
    $(objtree)/rust/bindings/compaction_native_generated.rs $(rust-native-policy-inputs) \
    $(src)/compaction_native_cfg.sh $(addprefix $(src)/,compaction_isolation.rs compaction_scanners.rs compaction_policy.rs compaction_daemon.rs)
compaction-rust-targets := $(addprefix $(obj)/,compaction.o compaction.s compaction.ll \
    compaction.rsi .compaction-rust-listing/compaction.o)
$(compaction-rust-targets): private target-stem := compaction
$(compaction-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(compaction-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/compaction.o,$(real-obj-y) $(lib-y)),y)
$(compaction-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/compaction.o: $(src)/compaction.rs $(compaction-rust-inputs) FORCE
	@$(CONFIG_SHELL) $(src)/compaction_native_cfg.sh $(objtree)/rust/bindings/compaction_native_generated.rs $(CONFIG_COMPACTION) >/dev/null
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/compaction.s: $(src)/compaction.rs $(compaction-rust-inputs) FORCE
	@$(CONFIG_SHELL) $(src)/compaction_native_cfg.sh $(objtree)/rust/bindings/compaction_native_generated.rs $(CONFIG_COMPACTION) >/dev/null
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/compaction.ll: $(src)/compaction.rs $(compaction-rust-inputs) FORCE
	@$(CONFIG_SHELL) $(src)/compaction_native_cfg.sh $(objtree)/rust/bindings/compaction_native_generated.rs $(CONFIG_COMPACTION) >/dev/null
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_compaction_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_compaction_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/compaction.rsi: $(src)/compaction.rs $(compaction-rust-inputs) FORCE
	@$(CONFIG_SHELL) $(src)/compaction_native_cfg.sh $(objtree)/rust/bindings/compaction_native_generated.rs $(CONFIG_COMPACTION) >/dev/null
	+$(call if_changed_dep,compaction_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.compaction-rust-listing/compaction.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.compaction-rust-listing/compaction.o: private override KBUILD_CFLAGS += -g
$(obj)/.compaction-rust-listing/compaction.o: $(src)/compaction.rs $(compaction-rust-inputs) FORCE
	@$(CONFIG_SHELL) $(src)/compaction_native_cfg.sh $(objtree)/rust/bindings/compaction_native_generated.rs $(CONFIG_COMPACTION) >/dev/null
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_compaction_rust_lst_elf = LD      $@
      cmd_compaction_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.compaction-rust-listing-elf/compaction.o: $(obj)/.compaction-rust-listing/compaction.o FORCE
	$(call if_changed,compaction_rust_lst_elf)
quiet_cmd_compaction_rust_lst = MKLST   $@
      cmd_compaction_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/compaction.lst: $(obj)/.compaction-rust-listing-elf/compaction.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,compaction_rust_lst)

targets += compaction.s compaction.ll compaction.rsi compaction.lst \
    .compaction-rust-listing/compaction.o .compaction-rust-listing-elf/compaction.o
endif
clean-files += .compaction-rust-listing/ .compaction-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.compaction.$(ext).rustc/)
