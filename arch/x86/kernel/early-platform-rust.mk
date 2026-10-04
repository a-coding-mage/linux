# SPDX-License-Identifier: GPL-2.0
# Include after the original ebda.o and platform-quirks.o archive entries.
# Keep both canonical object names and their original archive order.
ifeq ($(CONFIG_RUST_X86_EARLY_PLATFORM),y)
# Keep Rust selected when an unsupported compiler plugin is requested.
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_X86_EARLY_PLATFORM=y with GCC_PLUGIN_LATENT_ENTROPY=y is unsupported: C init-function latent-entropy instrumentation is not implemented)
endif

# The helper-bitcode codegen step uses raw KBUILD_CFLAGS, so it can add stack
# depth instrumentation despite the __init exemption in the original C.
# Reject that unsupported combination; keep Rust ownership and security flags.
ifeq ($(CONFIG_RUST_INLINE_HELPERS)$(CONFIG_KSTACK_ERASE),yy)
$(error RUST_X86_EARLY_PLATFORM=y with RUST_INLINE_HELPERS=y and KSTACK_ERASE=y is unsupported: init-only stack-depth instrumentation parity is not implemented)
endif

x86-early-platform-rust-owners := ebda platform-quirks
x86-early-platform-rust-bindings := $(objtree)/rust/bindings/x86_early_platform_generated.rs
x86-early-platform-rust-targets := $(addprefix $(obj)/,\
    $(foreach owner,$(x86-early-platform-rust-owners),\
        $(owner).o $(owner).s $(owner).ll $(owner).rsi \
        .early-platform-rust-listing/$(owner).o))

RUSTFLAGS_ebda.o += -Zfunction-sections=n
RUSTFLAGS_platform-quirks.o += -Zfunction-sections=n
# All functions are __init, whose Clang __no_kstack_erase disables coverage.
ifeq ($(CONFIG_CC_IS_CLANG),y)
KCOV_INSTRUMENT_ebda.o := n
KCOV_INSTRUMENT_platform-quirks.o := n
endif

$(x86-early-platform-rust-targets): private target-stem = $(basename $(notdir $@))
$(x86-early-platform-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(x86-early-platform-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/$(target-stem).o,$(real-obj-y) $(lib-y)),y)
$(x86-early-platform-rust-targets): private part-of-module :=

# Explicit static pattern rules always select Rust despite adjacent C sources.
$(addprefix $(obj)/,$(addsuffix .o,$(x86-early-platform-rust-owners))): \
    $(obj)/%.o: $(src)/%.rs $(x86-early-platform-rust-bindings) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(addprefix $(obj)/,$(addsuffix .s,$(x86-early-platform-rust-owners))): \
    $(obj)/%.s: $(src)/%.rs $(x86-early-platform-rust-bindings) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(addprefix $(obj)/,$(addsuffix .ll,$(x86-early-platform-rust-owners))): \
    $(obj)/%.ll: $(src)/%.rs $(x86-early-platform-rust-bindings) FORCE
	+$(call if_changed_dep,rustc_ll_rs)

quiet_cmd_early_platform_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_early_platform_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(addprefix $(obj)/,$(addsuffix .rsi,$(x86-early-platform-rust-owners))): \
    $(obj)/%.rsi: $(src)/%.rs $(x86-early-platform-rust-bindings) FORCE
	+$(call if_changed_dep,early_platform_rust_rsi)

# Build listing debug information in private objects; never overwrite the
# production objects, their commands, or their dependencies for a .lst request.
$(addprefix $(obj)/.early-platform-rust-listing/,$(addsuffix .o,$(x86-early-platform-rust-owners))): \
    private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(addprefix $(obj)/.early-platform-rust-listing/,$(addsuffix .o,$(x86-early-platform-rust-owners))): \
    private override KBUILD_CFLAGS += -g
$(addprefix $(obj)/.early-platform-rust-listing/,$(addsuffix .o,$(x86-early-platform-rust-owners))): \
    $(obj)/.early-platform-rust-listing/%.o: $(src)/%.rs $(x86-early-platform-rust-bindings) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_early_platform_rust_lst_elf = LD      $@
      cmd_early_platform_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(addprefix $(obj)/.early-platform-rust-listing-elf/,$(addsuffix .o,$(x86-early-platform-rust-owners))): \
    $(obj)/.early-platform-rust-listing-elf/%.o: $(obj)/.early-platform-rust-listing/%.o FORCE
	$(call if_changed,early_platform_rust_lst_elf)
quiet_cmd_early_platform_rust_lst = MKLST   $@
      cmd_early_platform_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(addprefix $(obj)/,$(addsuffix .lst,$(x86-early-platform-rust-owners))): \
    $(obj)/%.lst: $(obj)/.early-platform-rust-listing-elf/%.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,early_platform_rust_lst)

targets += $(foreach owner,$(x86-early-platform-rust-owners),\
    $(owner).s $(owner).ll $(owner).rsi $(owner).lst \
    .early-platform-rust-listing/$(owner).o .early-platform-rust-listing-elf/$(owner).o)
clean-files += .early-platform-rust-listing/ .early-platform-rust-listing-elf/
endif
