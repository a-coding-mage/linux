# SPDX-License-Identifier: GPL-2.0
# Proposed include after the obj-y assignment containing slab_common.o in mm/Makefile.
ifeq ($(CONFIG_RUST_SLAB_COMMON),y)
obj-y += slab_common_helpers.o

# Original slab_common.c has no SSP exemption; retain reviewed native strong protection.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_SLAB_COMMON native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_SLAB_COMMON native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_SLAB_COMMON native per-init coverage exemption is not integrated)
endif
ifeq ($(CONFIG_PRINTK_INDEX),y)
$(error RUST_SLAB_COMMON native per-callsite printk index records are not integrated)
endif

RUSTFLAGS_slab_common.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_slab_common.o += cfi_encoding
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_slab_common.o = $(dir $@).$(notdir $@).rustc/

slab_common-rust-inputs := $(addprefix $(src)/,slab_common_kmalloc.rs slab_common_info.rs slab_common_rcu.rs) \
    $(objtree)/rust/bindings/slab_common_generated.rs $(rust-native-policy-inputs)
slab_common-rust-targets := $(addprefix $(obj)/,slab_common.o slab_common.s slab_common.ll \
    slab_common.rsi .slab_common-rust-listing/slab_common.o)
$(slab_common-rust-targets): private target-stem := slab_common
$(slab_common-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(slab_common-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/slab_common.o,$(real-obj-y) $(lib-y)),y)
$(slab_common-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/slab_common.o: $(src)/slab_common.rs $(slab_common-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/slab_common.s: $(src)/slab_common.rs $(slab_common-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/slab_common.ll: $(src)/slab_common.rs $(slab_common-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_slab_common_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_slab_common_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/slab_common.rsi: $(src)/slab_common.rs $(slab_common-rust-inputs) FORCE
	+$(call if_changed_dep,slab_common_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.slab_common-rust-listing/slab_common.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.slab_common-rust-listing/slab_common.o: private override KBUILD_CFLAGS += -g
$(obj)/.slab_common-rust-listing/slab_common.o: $(src)/slab_common.rs $(slab_common-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_slab_common_rust_lst_elf = LD      $@
      cmd_slab_common_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.slab_common-rust-listing-elf/slab_common.o: $(obj)/.slab_common-rust-listing/slab_common.o FORCE
	$(call if_changed,slab_common_rust_lst_elf)
quiet_cmd_slab_common_rust_lst = MKLST   $@
      cmd_slab_common_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/slab_common.lst: $(obj)/.slab_common-rust-listing-elf/slab_common.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,slab_common_rust_lst)

targets += slab_common.s slab_common.ll slab_common.rsi slab_common.lst \
    .slab_common-rust-listing/slab_common.o .slab_common-rust-listing-elf/slab_common.o
endif
clean-files += .slab_common-rust-listing/ .slab_common-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.slab_common.$(ext).rustc/)
# Native leaves inherit only the original translation-unit exclusions.
KASAN_SANITIZE_slab_common_helpers.o := $(KASAN_SANITIZE_slab_common.o)
KCSAN_SANITIZE_slab_common_helpers.o := $(KCSAN_SANITIZE_slab_common.o)
KCOV_INSTRUMENT_slab_common_helpers.o := $(KCOV_INSTRUMENT_slab_common.o)
# Header and registration companions are included into this object, not linked
# as standalone C algorithm objects.
$(obj)/slab_common_helpers.o: $(src)/slab_common_bindings.h $(src)/slab_common_rcu_types.h \
    $(src)/slab_common_leaves.inc $(src)/slab_common_rcu_leaves.inc \
    $(src)/slab_common_rcu_storage.inc
# Preserve original module parameter/export namespace on the native carrier.
$(obj)/slab_common_helpers.o: private modname := slab_common
$(obj)/slab_common_helpers.o: private modfile := mm/slab_common
# Mirror compiler_types.h's configured __cold predicate from generated headers.
RUSTFLAGS_slab_common.o += $(shell sed -n 's/^pub const RSC_CFG_\([A-Z0-9_]*\): bool[_]* = true;/--cfg RSC_\1/p' $(objtree)/rust/bindings/slab_common_generated.rs 2>/dev/null)
