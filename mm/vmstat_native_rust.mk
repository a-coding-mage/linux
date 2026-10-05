# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y declaration containing vmstat.o in mm/Makefile.
ifeq ($(CONFIG_RUST_VMSTAT),y)
obj-y := $(patsubst vmstat.o,vmstat.o vmstat_native_helpers.o,$(obj-y))
# Original mm/Makefile disables vmstat coverage; retain that exact owner policy
# for the extracted native leaves too. No new coverage exemption is introduced.
KCOV_INSTRUMENT_vmstat_native_helpers.o := n

ifeq ($(CONFIG_RANDSTRUCT),y)
$(error RUST_VMSTAT native randomized layout transport is not integrated)
endif

# Source vmstat.c has no SSP exemption; effective policy awaits the integrated audit.
# Enroll mm/vmstat in scripts/rust-native-policy-owners before selecting.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_CFI),y)
$(error RUST_VMSTAT native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_VMSTAT native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_VMSTAT native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_VMSTAT native per-init coverage exemption is not integrated)
endif

RUSTFLAGS_vmstat.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_vmstat.o += cfi_encoding
# Mirror native __init cold only when compiler_types.h enables __cold.
# Deferred extraction uses the completed current binding prerequisite.
# Header metadata extraction must fail closed; the recipe precheck below
# surfaces parse errors rather than silently losing configured annotations.
RUSTFLAGS_vmstat.o += $(shell $(CONFIG_SHELL) $(src)/vmstat_native_cfg.sh $(objtree)/rust/bindings/vmstat_native_generated.rs $(CONFIG_RUST_VMSTAT) 2>/dev/null)
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_vmstat.o = $(dir $@).$(notdir $@).rustc/

vmstat-rust-inputs := \
    $(objtree)/rust/bindings/vmstat_native_generated.rs $(rust-native-policy-inputs) \
    $(src)/vmstat_native_cfg.sh $(addprefix $(src)/,vmstat_accounting.rs vmstat_work.rs vmstat_fragmentation.rs vmstat_text.rs vmstat_seq.rs vmstat_proc.rs vmstat_debug.rs vmstat_init.rs)
vmstat-rust-targets := $(addprefix $(obj)/,vmstat.o vmstat.s vmstat.ll \
    vmstat.rsi .vmstat-rust-listing/vmstat.o)
$(vmstat-rust-targets): private target-stem := vmstat
$(vmstat-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(vmstat-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/vmstat.o,$(real-obj-y) $(lib-y)),y)
$(vmstat-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/vmstat.o: $(src)/vmstat.rs $(vmstat-rust-inputs) FORCE
	@$(CONFIG_SHELL) $(src)/vmstat_native_cfg.sh $(objtree)/rust/bindings/vmstat_native_generated.rs $(CONFIG_RUST_VMSTAT) >/dev/null
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/vmstat.s: $(src)/vmstat.rs $(vmstat-rust-inputs) FORCE
	@$(CONFIG_SHELL) $(src)/vmstat_native_cfg.sh $(objtree)/rust/bindings/vmstat_native_generated.rs $(CONFIG_RUST_VMSTAT) >/dev/null
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/vmstat.ll: $(src)/vmstat.rs $(vmstat-rust-inputs) FORCE
	@$(CONFIG_SHELL) $(src)/vmstat_native_cfg.sh $(objtree)/rust/bindings/vmstat_native_generated.rs $(CONFIG_RUST_VMSTAT) >/dev/null
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_vmstat_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_vmstat_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/vmstat.rsi: $(src)/vmstat.rs $(vmstat-rust-inputs) FORCE
	@$(CONFIG_SHELL) $(src)/vmstat_native_cfg.sh $(objtree)/rust/bindings/vmstat_native_generated.rs $(CONFIG_RUST_VMSTAT) >/dev/null
	+$(call if_changed_dep,vmstat_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.vmstat-rust-listing/vmstat.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.vmstat-rust-listing/vmstat.o: private override KBUILD_CFLAGS += -g
$(obj)/.vmstat-rust-listing/vmstat.o: $(src)/vmstat.rs $(vmstat-rust-inputs) FORCE
	@$(CONFIG_SHELL) $(src)/vmstat_native_cfg.sh $(objtree)/rust/bindings/vmstat_native_generated.rs $(CONFIG_RUST_VMSTAT) >/dev/null
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_vmstat_rust_lst_elf = LD      $@
      cmd_vmstat_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.vmstat-rust-listing-elf/vmstat.o: $(obj)/.vmstat-rust-listing/vmstat.o FORCE
	$(call if_changed,vmstat_rust_lst_elf)
quiet_cmd_vmstat_rust_lst = MKLST   $@
      cmd_vmstat_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/vmstat.lst: $(obj)/.vmstat-rust-listing-elf/vmstat.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,vmstat_rust_lst)

targets += vmstat.s vmstat.ll vmstat.rsi vmstat.lst \
    .vmstat-rust-listing/vmstat.o .vmstat-rust-listing-elf/vmstat.o
endif
clean-files += .vmstat-rust-listing/ .vmstat-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.vmstat.$(ext).rustc/)
