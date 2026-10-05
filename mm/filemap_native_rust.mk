# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y += filemap.o in mm/Makefile.
ifeq ($(CONFIG_RUST_FILEMAP),y)
obj-y := $(patsubst filemap.o,filemap.o filemap_native_helpers.o,$(obj-y))

# Original filemap.c uses native strong protection with no SSP exemptions.
# Enroll mm/filemap in scripts/rust-native-policy-owners before selecting.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_CFI),y)
$(error RUST_FILEMAP native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_FILEMAP native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_FILEMAP native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_FILEMAP native per-init coverage exemption is not integrated)
endif

RUSTFLAGS_filemap.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_filemap.o += cfi_encoding
# Mirror native __init cold only when compiler_types.h enables __cold.
# Deferred extraction uses the completed current binding prerequisite.
RUSTFLAGS_filemap.o += $(shell sed -n 's/^pub const RUST_FILEMAP_CFG_\([A-Z0-9_]*\): bool[_]* = true;/--cfg RUST_FILEMAP_\1/p' $(objtree)/rust/bindings/filemap_native_generated.rs 2>/dev/null)
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_filemap.o = $(dir $@).$(notdir $@).rustc/

filemap-rust-inputs := \
    $(objtree)/rust/bindings/filemap_native_generated.rs $(rust-native-policy-inputs) \
    $(addprefix $(src)/,filemap_cache.rs filemap_writeback.rs filemap_wait.rs filemap_lookup.rs filemap_read.rs filemap_fault.rs filemap_nommu.rs filemap_write.rs filemap_cachestat.rs filemap_native_aliases.rs)
filemap-rust-targets := $(addprefix $(obj)/,filemap.o filemap.s filemap.ll \
    filemap.rsi .filemap-rust-listing/filemap.o)
$(filemap-rust-targets): private target-stem := filemap
$(filemap-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(filemap-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/filemap.o,$(real-obj-y) $(lib-y)),y)
$(filemap-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/filemap.o: $(src)/filemap.rs $(filemap-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/filemap.s: $(src)/filemap.rs $(filemap-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/filemap.ll: $(src)/filemap.rs $(filemap-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_filemap_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_filemap_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/filemap.rsi: $(src)/filemap.rs $(filemap-rust-inputs) FORCE
	+$(call if_changed_dep,filemap_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.filemap-rust-listing/filemap.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.filemap-rust-listing/filemap.o: private override KBUILD_CFLAGS += -g
$(obj)/.filemap-rust-listing/filemap.o: $(src)/filemap.rs $(filemap-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_filemap_rust_lst_elf = LD      $@
      cmd_filemap_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.filemap-rust-listing-elf/filemap.o: $(obj)/.filemap-rust-listing/filemap.o FORCE
	$(call if_changed,filemap_rust_lst_elf)
quiet_cmd_filemap_rust_lst = MKLST   $@
      cmd_filemap_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/filemap.lst: $(obj)/.filemap-rust-listing-elf/filemap.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,filemap_rust_lst)

targets += filemap.s filemap.ll filemap.rsi filemap.lst \
    .filemap-rust-listing/filemap.o .filemap-rust-listing-elf/filemap.o
endif
clean-files += .filemap-rust-listing/ .filemap-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.filemap.$(ext).rustc/)
