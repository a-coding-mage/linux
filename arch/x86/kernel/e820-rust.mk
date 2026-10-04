# SPDX-License-Identifier: GPL-2.0-only
# Proposal: include immediately after the existing bootflag.o e820.o entry.
# Preserve e820.o identity and the original archive position.
ifeq ($(CONFIG_RUST_X86_E820),y)
obj-y := $(patsubst e820.o,e820.o e820_primitives.o,$(obj-y))
# Keep Rust selected when an unsupported compiler plugin is requested.
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_X86_E820=y with GCC_PLUGIN_LATENT_ENTROPY=y is unsupported: C init-function latent-entropy instrumentation is not implemented)
endif

# This file mixes runtime and init functions. Direct rustc misses native
# runtime stack-depth instrumentation; the helper-bitcode path may instead
# instrument init functions. Neither route yet has per-function C parity.
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_X86_E820=y with KSTACK_ERASE=y is unsupported: mixed runtime/init stack-depth instrumentation parity is not implemented)
endif

x86-e820-rust-owners := e820
x86-e820-rust-bindings := $(objtree)/rust/bindings/x86_e820_generated.rs
x86-e820-rust-targets := $(addprefix $(obj)/,\
    $(foreach owner,$(x86-e820-rust-owners),\
        $(owner).o $(owner).s $(owner).ll $(owner).rsi \
        .e820-rust-listing/$(owner).o))

RUSTFLAGS_e820.o += -Zfunction-sections=n
# Keep mixed runtime/init instrumentation honest: no whole-file exemption.
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_X86_E820=y with Clang KCOV is unsupported: per-init-function coverage exemption is not implemented)
endif
ifeq ($(CONFIG_PRINTK_INDEX),y)
$(error RUST_X86_E820=y with PRINTK_INDEX=y is unsupported: native per-callsite printk index records are not implemented)
endif

$(x86-e820-rust-targets): private target-stem = $(basename $(notdir $@))
$(x86-e820-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(x86-e820-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/$(target-stem).o,$(real-obj-y) $(lib-y)),y)
$(x86-e820-rust-targets): private part-of-module :=

# Explicit static pattern rules always select Rust despite adjacent C sources.
$(addprefix $(obj)/,$(addsuffix .o,$(x86-e820-rust-owners))): \
    $(obj)/%.o: $(src)/%.rs $(x86-e820-rust-bindings) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(addprefix $(obj)/,$(addsuffix .s,$(x86-e820-rust-owners))): \
    $(obj)/%.s: $(src)/%.rs $(x86-e820-rust-bindings) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(addprefix $(obj)/,$(addsuffix .ll,$(x86-e820-rust-owners))): \
    $(obj)/%.ll: $(src)/%.rs $(x86-e820-rust-bindings) FORCE
	+$(call if_changed_dep,rustc_ll_rs)

quiet_cmd_e820_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_e820_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(addprefix $(obj)/,$(addsuffix .rsi,$(x86-e820-rust-owners))): \
    $(obj)/%.rsi: $(src)/%.rs $(x86-e820-rust-bindings) FORCE
	+$(call if_changed_dep,e820_rust_rsi)

# Build listing debug information in private objects; never overwrite the
# production objects, their commands, or their dependencies for a .lst request.
$(addprefix $(obj)/.e820-rust-listing/,$(addsuffix .o,$(x86-e820-rust-owners))): \
    private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(addprefix $(obj)/.e820-rust-listing/,$(addsuffix .o,$(x86-e820-rust-owners))): \
    private override KBUILD_CFLAGS += -g
$(addprefix $(obj)/.e820-rust-listing/,$(addsuffix .o,$(x86-e820-rust-owners))): \
    $(obj)/.e820-rust-listing/%.o: $(src)/%.rs $(x86-e820-rust-bindings) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_e820_rust_lst_elf = LD      $@
      cmd_e820_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(addprefix $(obj)/.e820-rust-listing-elf/,$(addsuffix .o,$(x86-e820-rust-owners))): \
    $(obj)/.e820-rust-listing-elf/%.o: $(obj)/.e820-rust-listing/%.o FORCE
	$(call if_changed,e820_rust_lst_elf)
quiet_cmd_e820_rust_lst = MKLST   $@
      cmd_e820_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(addprefix $(obj)/,$(addsuffix .lst,$(x86-e820-rust-owners))): \
    $(obj)/%.lst: $(obj)/.e820-rust-listing-elf/%.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,e820_rust_lst)

targets += $(foreach owner,$(x86-e820-rust-owners),\
    $(owner).s $(owner).ll $(owner).rsi $(owner).lst \
    .e820-rust-listing/$(owner).o .e820-rust-listing-elf/$(owner).o)
clean-files += .e820-rust-listing/ .e820-rust-listing-elf/
endif
