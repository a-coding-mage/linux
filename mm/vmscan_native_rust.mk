# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y += vmscan.o in mm/Makefile.
ifeq ($(CONFIG_RUST_VMSCAN),y)
obj-y := $(patsubst vmscan.o,vmscan.o vmscan_native_helpers.o vmscan_mglru_native.o,$(obj-y))

# Original vmscan.c uses native strong protection with no SSP exemptions.
# Enroll mm/vmscan in scripts/rust-native-policy-owners before selecting.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_CFI),y)
$(error RUST_VMSCAN native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_VMSCAN native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_VMSCAN native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_VMSCAN native per-init coverage exemption is not integrated)
endif

RUSTFLAGS_vmscan.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_vmscan.o += cfi_encoding
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_vmscan.o = $(dir $@).$(notdir $@).rustc/

vmscan-rust-inputs := \
    $(objtree)/rust/bindings/vmscan_native_generated.rs $(rust-native-policy-inputs) \
    $(addprefix $(src)/,vmscan_classic.rs vmscan_direct.rs vmscan_primitive_aliases.rs vmscan_constants.rs vmscan_mglru.rs vmscan_mglru_aging.rs vmscan_mglru_memcg.rs vmscan_mglru_eviction.rs vmscan_mglru_control.rs)
vmscan-rust-targets := $(addprefix $(obj)/,vmscan.o vmscan.s vmscan.ll \
    vmscan.rsi .vmscan-rust-listing/vmscan.o)
$(vmscan-rust-targets): private target-stem := vmscan
$(vmscan-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(vmscan-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/vmscan.o,$(real-obj-y) $(lib-y)),y)
$(vmscan-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/vmscan.o: $(src)/vmscan.rs $(vmscan-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/vmscan.s: $(src)/vmscan.rs $(vmscan-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/vmscan.ll: $(src)/vmscan.rs $(vmscan-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_vmscan_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_vmscan_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/vmscan.rsi: $(src)/vmscan.rs $(vmscan-rust-inputs) FORCE
	+$(call if_changed_dep,vmscan_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.vmscan-rust-listing/vmscan.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.vmscan-rust-listing/vmscan.o: private override KBUILD_CFLAGS += -g
$(obj)/.vmscan-rust-listing/vmscan.o: $(src)/vmscan.rs $(vmscan-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_vmscan_rust_lst_elf = LD      $@
      cmd_vmscan_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.vmscan-rust-listing-elf/vmscan.o: $(obj)/.vmscan-rust-listing/vmscan.o FORCE
	$(call if_changed,vmscan_rust_lst_elf)
quiet_cmd_vmscan_rust_lst = MKLST   $@
      cmd_vmscan_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/vmscan.lst: $(obj)/.vmscan-rust-listing-elf/vmscan.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,vmscan_rust_lst)

targets += vmscan.s vmscan.ll vmscan.rsi vmscan.lst \
    .vmscan-rust-listing/vmscan.o .vmscan-rust-listing-elf/vmscan.o
endif
clean-files += .vmscan-rust-listing/ .vmscan-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.vmscan.$(ext).rustc/)
