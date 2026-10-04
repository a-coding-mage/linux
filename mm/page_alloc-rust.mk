# SPDX-License-Identifier: GPL-2.0-only
# Include immediately after page-alloc-y assignment, before shuffle.o.
ifeq ($(CONFIG_RUST_PAGE_ALLOC),y)
page-alloc-y += page_alloc_helpers.o

# Original page_alloc.c uses native strong stack protection. Preserve the
# existing per-object KCSAN and KCOV exclusions and rmqueue KMSAN exemption.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_PAGE_ALLOC native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_PAGE_ALLOC native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_PRINTK_INDEX),y)
$(error RUST_PAGE_ALLOC native per-callsite printk index records are not integrated)
endif

RUSTFLAGS_page_alloc.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_page_alloc.o += cfi_encoding,no_sanitize
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_page_alloc.o = $(dir $@).$(notdir $@).rustc/

page_alloc-rust-inputs := $(src)/page_alloc_diagnostics.rs  $(src)/page_alloc_primitives.rs $(src)/page_alloc_buddy.rs \
    $(src)/page_alloc_fastpaths.rs $(src)/page_alloc_late.rs \
    $(src)/page_alloc_zone_tail.rs \
    $(objtree)/rust/bindings/page_alloc_generated.rs $(rust-native-policy-inputs)
page_alloc-rust-targets := $(addprefix $(obj)/,page_alloc.o page_alloc.s page_alloc.ll \
    page_alloc.rsi .page_alloc-rust-listing/page_alloc.o)
$(page_alloc-rust-targets): private target-stem := page_alloc
$(page_alloc-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(page_alloc-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/page_alloc.o,$(real-obj-y) $(lib-y)),y)
$(page_alloc-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/page_alloc.o: $(src)/page_alloc.rs $(page_alloc-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/page_alloc.s: $(src)/page_alloc.rs $(page_alloc-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/page_alloc.ll: $(src)/page_alloc.rs $(page_alloc-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_page_alloc_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_page_alloc_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/page_alloc.rsi: $(src)/page_alloc.rs $(page_alloc-rust-inputs) FORCE
	+$(call if_changed_dep,page_alloc_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.page_alloc-rust-listing/page_alloc.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.page_alloc-rust-listing/page_alloc.o: private override KBUILD_CFLAGS += -g
$(obj)/.page_alloc-rust-listing/page_alloc.o: $(src)/page_alloc.rs $(page_alloc-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_page_alloc_rust_lst_elf = LD      $@
      cmd_page_alloc_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.page_alloc-rust-listing-elf/page_alloc.o: $(obj)/.page_alloc-rust-listing/page_alloc.o FORCE
	$(call if_changed,page_alloc_rust_lst_elf)
quiet_cmd_page_alloc_rust_lst = MKLST   $@
      cmd_page_alloc_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/page_alloc.lst: $(obj)/.page_alloc-rust-listing-elf/page_alloc.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,page_alloc_rust_lst)

targets += page_alloc.s page_alloc.ll page_alloc.rsi page_alloc.lst \
    .page_alloc-rust-listing/page_alloc.o .page_alloc-rust-listing-elf/page_alloc.o
endif
clean-files += .page_alloc-rust-listing/ .page_alloc-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.page_alloc.$(ext).rustc/)
# Companion compiler/header leaves share the original translation-unit exclusions.
KCSAN_SANITIZE_page_alloc_helpers.o := $(KCSAN_SANITIZE_page_alloc.o)
KCOV_INSTRUMENT_page_alloc_helpers.o := $(KCOV_INSTRUMENT_page_alloc.o)
