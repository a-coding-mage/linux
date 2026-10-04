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

# Opt in this owner only. rustc's strong strategy does not encode the kernel
# guard location. An empty native TU supplies compiler-generated module policy;
# no C owner or substitute guard is linked. Keep other owners on their existing
# route until their original native objects and per-file policy are reviewed.
ifeq ($(CONFIG_STACKPROTECTOR),y)
ifneq ($(CONFIG_CC_IS_CLANG)$(CONFIG_X86_64)$(CONFIG_SMP)$(CONFIG_STACKPROTECTOR_STRONG),yyyy)
$(error RUST_X86_E820 stack protector currently requires reviewed Clang x86-64 SMP strong policy)
endif
ifeq ($(CONFIG_RUST_INLINE_HELPERS),y)
$(error RUST_X86_E820 stack protector with RUST_INLINE_HELPERS requires separate native-policy review)
endif
ifeq ($(CONFIG_LTO_CLANG),y)
$(error RUST_X86_E820 stack protector with Clang LTO requires separate native artifact review)
endif
RUSTFLAGS_e820.o += -Zstack-protector=strong
x86-e820-native-policy := y
x86-e820-native-source ?= $(src)/e820_native_policy.c
x86-e820-native-inputs := $(x86-e820-native-source) \
	$(srctree)/scripts/check-e820-native-policy.pl \
	$(srctree)/scripts/e820-native-codegen.pl
x86-e820-policy-check = $(PERL) $(srctree)/scripts/check-e820-native-policy.pl

# Derive metadata from the complete effective C flags, including per-file
# additions/removals and architecture guard options. Keep the C dependencies
# alongside the Rust dependencies for fixdep and incremental rebuilds.
e820_rust_native_ir = \
	$(PERL) $(srctree)/scripts/e820-native-codegen.pl --check-policy $(CC) $(c_flags) && \
	$(CC) $(subst $(depfile),$@.native.d,$(c_flags)) -S -emit-llvm \
		-o $@.native.ll $(x86-e820-native-source) && \
	$(x86-e820-policy-check) empty $@.native.ll && \
	e820_rust_debug_flags=$$($(x86-e820-policy-check) rust-debug $@.native.ll) && \
	$(rust_common_cmd) $$e820_rust_debug_flags \
		--emit=llvm-bc=$@.rust.bc,llvm-ir=$@.rust.ll $< && \
	$(x86-e820-policy-check) rust $@.rust.ll && \
	$(LLVM_LINK) -S $@.rust.bc $@.native.ll -o $@.linked.ll && \
	$(x86-e820-policy-check) metadata $@.linked.ll && \
	cat $@.native.d >> $(depfile)
e820_rust_native_cc = $(PERL) $(srctree)/scripts/e820-native-codegen.pl $(CC) $(c_flags)

quiet_cmd_e820_rust_o = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_e820_rust_o = $(e820_rust_native_ir) && \
	$(e820_rust_native_cc) -c $@.linked.ll -o $@ && \
	$(x86-e820-policy-check) object $@ $(OBJDUMP) || exit $$? \
	$(cmd_ld_single) $(cmd_objtool)
define rule_e820_rust_o
	$(call cmd_and_fixdep,e820_rust_o)
	$(call cmd,gen_objtooldep)
	$(call cmd,gen_symversions_rs)
endef
quiet_cmd_e820_rust_s = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_e820_rust_s = $(e820_rust_native_ir) && \
	$(e820_rust_native_cc) -S $@.linked.ll -o $@ && \
	$(x86-e820-policy-check) assembly $@ || exit $$?
quiet_cmd_e820_rust_ll = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_e820_rust_ll = $(e820_rust_native_ir) && \
	$(e820_rust_native_cc) -S -emit-llvm $@.linked.ll -o $@ && \
	$(x86-e820-policy-check) metadata $@ && \
	$(x86-e820-policy-check) rust $@ || exit $$?
endif
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
    $(obj)/%.o: $(src)/%.rs $(x86-e820-rust-bindings) $(x86-e820-native-inputs) FORCE
	+$(call if_changed_rule,$(if $(x86-e820-native-policy),e820_rust_o,rustc_o_rs))
$(addprefix $(obj)/,$(addsuffix .s,$(x86-e820-rust-owners))): \
    $(obj)/%.s: $(src)/%.rs $(x86-e820-rust-bindings) $(x86-e820-native-inputs) FORCE
	+$(call if_changed_dep,$(if $(x86-e820-native-policy),e820_rust_s,rustc_s_rs))
$(addprefix $(obj)/,$(addsuffix .ll,$(x86-e820-rust-owners))): \
    $(obj)/%.ll: $(src)/%.rs $(x86-e820-rust-bindings) $(x86-e820-native-inputs) FORCE
	+$(call if_changed_dep,$(if $(x86-e820-native-policy),e820_rust_ll,rustc_ll_rs))

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
    $(obj)/.e820-rust-listing/%.o: $(src)/%.rs $(x86-e820-rust-bindings) $(x86-e820-native-inputs) FORCE
	+$(call if_changed_rule,$(if $(x86-e820-native-policy),e820_rust_o,rustc_o_rs))
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
clean-files += $(foreach ext,o s ll,$(addprefix e820.$(ext).,rust.bc rust.ll native.d native.ll linked.ll))
endif
