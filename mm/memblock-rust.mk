# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y += memblock.o in mm/Makefile.
ifeq ($(CONFIG_RUST_MEMBLOCK),y)
obj-y := $(patsubst memblock.o,memblock.o memblock_helpers.o,$(obj-y))

# The source phase must not silently discard native compiler instrumentation.
# The shared native-policy integration can replace these diagnostics after
# inspecting this owner's mixed runtime / __init / __init_memblock functions.
ifeq ($(CONFIG_STACKPROTECTOR),y)
$(error RUST_MEMBLOCK requires reviewed native stack-protector policy integration)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_MEMBLOCK native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_MEMBLOCK native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_MEMBLOCK native per-init coverage exemption is not integrated)
endif
ifeq ($(CONFIG_PRINTK_INDEX),y)
$(error RUST_MEMBLOCK native per-callsite printk index records are not integrated)
endif

RUSTFLAGS_memblock.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_memblock.o += cfi_encoding
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_memblock.o = $(dir $@).$(notdir $@).rustc/

memblock-rust-inputs := $(src)/memblock_reserved.rs \
    $(objtree)/rust/bindings/memblock_generated.rs
memblock-rust-targets := $(addprefix $(obj)/,memblock.o memblock.s memblock.ll \
    memblock.rsi .memblock-rust-listing/memblock.o)
$(memblock-rust-targets): private target-stem := memblock
$(memblock-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(memblock-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/memblock.o,$(real-obj-y) $(lib-y)),y)
$(memblock-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/memblock.o: $(src)/memblock.rs $(memblock-rust-inputs) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/memblock.s: $(src)/memblock.rs $(memblock-rust-inputs) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/memblock.ll: $(src)/memblock.rs $(memblock-rust-inputs) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
quiet_cmd_memblock_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_memblock_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/memblock.rsi: $(src)/memblock.rs $(memblock-rust-inputs) FORCE
	+$(call if_changed_dep,memblock_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.memblock-rust-listing/memblock.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.memblock-rust-listing/memblock.o: private override KBUILD_CFLAGS += -g
$(obj)/.memblock-rust-listing/memblock.o: $(src)/memblock.rs $(memblock-rust-inputs) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_memblock_rust_lst_elf = LD      $@
      cmd_memblock_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.memblock-rust-listing-elf/memblock.o: $(obj)/.memblock-rust-listing/memblock.o FORCE
	$(call if_changed,memblock_rust_lst_elf)
quiet_cmd_memblock_rust_lst = MKLST   $@
      cmd_memblock_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/memblock.lst: $(obj)/.memblock-rust-listing-elf/memblock.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,memblock_rust_lst)

targets += memblock.s memblock.ll memblock.rsi memblock.lst \
    .memblock-rust-listing/memblock.o .memblock-rust-listing-elf/memblock.o
endif
clean-files += .memblock-rust-listing/ .memblock-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.memblock.$(ext).rustc/)
