# SPDX-License-Identifier: GPL-2.0
# Preserve the canonical object identity and archive position.
ifeq ($(CONFIG_RUST_X86_PHYSADDR),y)
ifneq ($(CONFIG_X86_64),y)
$(error RUST_X86_PHYSADDR requires X86_64)
endif
ifneq ($(CONFIG_SPARSEMEM)$(CONFIG_SPARSEMEM_EXTREME),yy)
$(error RUST_X86_PHYSADDR requires SPARSEMEM_EXTREME; other memory models remain open)
endif
ifeq ($(CONFIG_HAVE_ARCH_PFN_VALID),y)
$(error RUST_X86_PHYSADDR does not implement architecture-specific pfn_valid)
endif
obj-y := $(patsubst physaddr.o,physaddr.o physaddr_primitives.o,$(obj-y))
# Match the original entire-unit coverage and stack-protector exemptions.
KCOV_INSTRUMENT_physaddr_primitives.o := n
CFLAGS_physaddr_primitives.o := -fno-stack-protector
RUSTFLAGS_physaddr.o += -Zstack-protector=none
# Unsupported native instrumentation must fail, never switch back to C or
# silently delete native compiler flags. KASAN/KCOV retain shared Rust flags;
# KCSAN remains disabled by the original directory Makefile.
ifneq ($(filter y,$(CONFIG_FUNCTION_TRACER) $(CONFIG_GCOV_KERNEL) $(CONFIG_KMSAN) $(CONFIG_UBSAN) $(CONFIG_KSTACK_ERASE) $(CONFIG_GCC_PLUGIN_LATENT_ENTROPY) $(CONFIG_PROPELLER_CLANG)),)
$(error RUST_X86_PHYSADDR: requested native instrumentation parity is not implemented)
endif

x86-physaddr-rust-owners := physaddr
x86-physaddr-rust-bindings := $(objtree)/rust/bindings/x86_physaddr_generated.rs
x86-physaddr-rust-targets := $(addprefix $(obj)/,\
    $(foreach owner,$(x86-physaddr-rust-owners),\
        $(owner).o $(owner).s $(owner).ll $(owner).rsi \
        .physaddr-rust-listing/$(owner).o))

RUSTFLAGS_physaddr.o += -Zfunction-sections=n

# The inline-helper backend consumes KBUILD_CFLAGS, not per-object CFLAGS.
# Apply the original physaddr exemption to that compiler stage as well.
$(x86-physaddr-rust-targets): private override KBUILD_CFLAGS += -fno-stack-protector

$(x86-physaddr-rust-targets): private target-stem = $(basename $(notdir $@))
$(x86-physaddr-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(x86-physaddr-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/$(target-stem).o,$(real-obj-y) $(lib-y)),y)
$(x86-physaddr-rust-targets): private part-of-module :=

# Explicit static pattern rules always select Rust despite adjacent C sources.
$(addprefix $(obj)/,$(addsuffix .o,$(x86-physaddr-rust-owners))): \
    $(obj)/%.o: $(src)/%.rs $(x86-physaddr-rust-bindings) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(addprefix $(obj)/,$(addsuffix .s,$(x86-physaddr-rust-owners))): \
    $(obj)/%.s: $(src)/%.rs $(x86-physaddr-rust-bindings) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(addprefix $(obj)/,$(addsuffix .ll,$(x86-physaddr-rust-owners))): \
    $(obj)/%.ll: $(src)/%.rs $(x86-physaddr-rust-bindings) FORCE
	+$(call if_changed_dep,rustc_ll_rs)

quiet_cmd_physaddr_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_physaddr_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(addprefix $(obj)/,$(addsuffix .rsi,$(x86-physaddr-rust-owners))): \
    $(obj)/%.rsi: $(src)/%.rs $(x86-physaddr-rust-bindings) FORCE
	+$(call if_changed_dep,physaddr_rust_rsi)

# Build listing debug information in private objects; never overwrite the
# production objects, their commands, or their dependencies for a .lst request.
$(addprefix $(obj)/.physaddr-rust-listing/,$(addsuffix .o,$(x86-physaddr-rust-owners))): \
    private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(addprefix $(obj)/.physaddr-rust-listing/,$(addsuffix .o,$(x86-physaddr-rust-owners))): \
    private override KBUILD_CFLAGS += -g
$(addprefix $(obj)/.physaddr-rust-listing/,$(addsuffix .o,$(x86-physaddr-rust-owners))): \
    $(obj)/.physaddr-rust-listing/%.o: $(src)/%.rs $(x86-physaddr-rust-bindings) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_physaddr_rust_lst_elf = LD      $@
      cmd_physaddr_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(addprefix $(obj)/.physaddr-rust-listing-elf/,$(addsuffix .o,$(x86-physaddr-rust-owners))): \
    $(obj)/.physaddr-rust-listing-elf/%.o: $(obj)/.physaddr-rust-listing/%.o FORCE
	$(call if_changed,physaddr_rust_lst_elf)
quiet_cmd_physaddr_rust_lst = MKLST   $@
      cmd_physaddr_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(addprefix $(obj)/,$(addsuffix .lst,$(x86-physaddr-rust-owners))): \
    $(obj)/%.lst: $(obj)/.physaddr-rust-listing-elf/%.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,physaddr_rust_lst)

targets += $(foreach owner,$(x86-physaddr-rust-owners),\
    $(owner).s $(owner).ll $(owner).rsi $(owner).lst \
    .physaddr-rust-listing/$(owner).o .physaddr-rust-listing-elf/$(owner).o)
clean-files += .physaddr-rust-listing/ .physaddr-rust-listing-elf/
endif
