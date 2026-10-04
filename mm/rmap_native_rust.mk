# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y += rmap.o in mm/Makefile.
ifeq ($(CONFIG_RUST_RMAP),y)
obj-y := $(patsubst rmap.o,rmap.o rmap_native_helpers.o,$(obj-y))

# Original rmap.c uses native strong protection with no SSP exemptions.
# Enroll mm/rmap in scripts/rust-native-policy-owners before selecting.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_CFI),y)
$(error RUST_RMAP native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_RMAP native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_RMAP native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_RMAP native per-init coverage exemption is not integrated)
endif

RUSTFLAGS_rmap.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_rmap.o += cfi_encoding
# Mirror native __init cold only when compiler_types.h enables __cold.
# Deferred extraction uses the completed current binding prerequisite.
RUSTFLAGS_rmap.o += $(shell sed -n 's/^pub const RUST_RMAP_CFG_\([A-Z0-9_]*\): bool[_]* = true;/--cfg RUST_RMAP_\1/p' $(objtree)/rust/bindings/rmap_native_generated.rs 2>/dev/null)
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_rmap.o = $(dir $@).$(notdir $@).rustc/

rmap-rust-inputs := \
    $(objtree)/rust/bindings/rmap_native_generated.rs $(rust-native-policy-inputs) \
    $(addprefix $(src)/,rmap_anon.rs rmap_walk.rs rmap_account.rs rmap_unmap.rs rmap_migrate.rs)
rmap-rust-targets := $(addprefix $(obj)/,rmap.o rmap.s rmap.ll \
    rmap.rsi .rmap-rust-listing/rmap.o)
$(rmap-rust-targets): private target-stem := rmap
$(rmap-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(rmap-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/rmap.o,$(real-obj-y) $(lib-y)),y)
$(rmap-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/rmap.o: $(src)/rmap.rs $(rmap-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/rmap.s: $(src)/rmap.rs $(rmap-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/rmap.ll: $(src)/rmap.rs $(rmap-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_rmap_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_rmap_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/rmap.rsi: $(src)/rmap.rs $(rmap-rust-inputs) FORCE
	+$(call if_changed_dep,rmap_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.rmap-rust-listing/rmap.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rmap-rust-listing/rmap.o: private override KBUILD_CFLAGS += -g
$(obj)/.rmap-rust-listing/rmap.o: $(src)/rmap.rs $(rmap-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_rmap_rust_lst_elf = LD      $@
      cmd_rmap_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rmap-rust-listing-elf/rmap.o: $(obj)/.rmap-rust-listing/rmap.o FORCE
	$(call if_changed,rmap_rust_lst_elf)
quiet_cmd_rmap_rust_lst = MKLST   $@
      cmd_rmap_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/rmap.lst: $(obj)/.rmap-rust-listing-elf/rmap.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,rmap_rust_lst)

targets += rmap.s rmap.ll rmap.rsi rmap.lst \
    .rmap-rust-listing/rmap.o .rmap-rust-listing-elf/rmap.o
endif
clean-files += .rmap-rust-listing/ .rmap-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.rmap.$(ext).rustc/)
