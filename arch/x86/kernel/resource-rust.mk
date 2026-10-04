# SPDX-License-Identifier: GPL-2.0
# Preserve the canonical object identity and archive position.
ifeq ($(CONFIG_RUST_X86_RESOURCE),y)
ifneq ($(CONFIG_X86_64),y)
$(error RUST_X86_RESOURCE requires X86_64)
endif
# Unsupported native instrumentation must fail, never switch back to C or
# silently delete native compiler flags. KASAN/KCOV retain shared Rust flags;
# KCSAN remains disabled by the original directory Makefile.
ifneq ($(filter y,$(CONFIG_FUNCTION_TRACER) $(CONFIG_GCOV_KERNEL) $(CONFIG_KMSAN) $(CONFIG_UBSAN) $(CONFIG_KSTACK_ERASE) $(CONFIG_GCC_PLUGIN_LATENT_ENTROPY) $(CONFIG_PROPELLER_CLANG)),)
$(error RUST_X86_RESOURCE: requested native instrumentation parity is not implemented)
endif
ifeq ($(CONFIG_PRINTK_INDEX),y)
$(error RUST_X86_RESOURCE: native per-callsite PRINTK_INDEX metadata is not implemented)
endif

x86-resource-rust-owners := resource
x86-resource-rust-bindings := $(objtree)/rust/bindings/x86_resource_generated.rs
x86-resource-rust-targets := $(addprefix $(obj)/,\
    $(foreach owner,$(x86-resource-rust-owners),\
        $(owner).o $(owner).s $(owner).ll $(owner).rsi \
        .resource-rust-listing/$(owner).o))

RUSTFLAGS_resource.o += -Zfunction-sections=n
# Separate Rust's crate/CGU temporaries for concurrent inspection targets.
# Requires the default-preserving rustc_out_dir interface in Makefile.build.
RUSTC_OUT_DIR_resource.o = $(dir $@).$(notdir $@).rustc/
# Opt in this owner only. rustc's strong strategy does not encode the kernel
# guard location. An empty native TU supplies compiler-generated module policy;
# no C owner or substitute guard is linked. Keep other owners on their existing
# route until their original native objects and per-file policy are reviewed.
ifeq ($(CONFIG_STACKPROTECTOR),y)
ifneq ($(CONFIG_CC_IS_CLANG)$(CONFIG_X86_64)$(CONFIG_SMP)$(CONFIG_STACKPROTECTOR_STRONG),yyyy)
$(error RUST_X86_RESOURCE stack protector currently requires reviewed Clang x86-64 SMP strong policy)
endif
ifeq ($(CONFIG_RUST_INLINE_HELPERS),y)
$(error RUST_X86_RESOURCE stack protector with RUST_INLINE_HELPERS requires separate native-policy review)
endif
ifeq ($(CONFIG_LTO_CLANG),y)
$(error RUST_X86_RESOURCE stack protector with Clang LTO requires separate native artifact review)
endif
RUSTFLAGS_resource.o += -Zstack-protector=strong
x86-resource-native-policy := y
x86-resource-native-source ?= $(src)/resource_native_policy.c
x86-resource-native-inputs := $(x86-resource-native-source) \
	$(srctree)/scripts/check-e820-native-policy.pl \
	$(srctree)/scripts/e820-native-codegen.pl
# Reuse only the generic empty/rust-debug/rust/metadata modes. E820's
# function-specific object/assembly guard counts do not apply to resource.
x86-resource-policy-check = $(PERL) $(srctree)/scripts/check-e820-native-policy.pl

# Derive metadata from the complete effective C flags, including per-file
# additions/removals and architecture guard options. Keep the C dependencies
# alongside the Rust dependencies for fixdep and incremental rebuilds.
resource_rust_native_ir = \
	$(PERL) $(srctree)/scripts/e820-native-codegen.pl --check-policy $(CC) $(c_flags) && \
	$(CC) $(subst $(depfile),$@.native.d,$(c_flags)) -S -emit-llvm \
		-o $@.native.ll $(x86-resource-native-source) && \
	$(x86-resource-policy-check) empty $@.native.ll && \
	resource_rust_debug_flags=$$($(x86-resource-policy-check) rust-debug $@.native.ll) && \
	$(rust_common_cmd) $$resource_rust_debug_flags \
		--emit=llvm-bc=$@.rust.bc,llvm-ir=$@.rust.ll $< && \
	$(x86-resource-policy-check) rust $@.rust.ll && \
	$(LLVM_LINK) -S $@.rust.bc $@.native.ll -o $@.linked.ll && \
	$(x86-resource-policy-check) metadata $@.linked.ll && \
	cat $@.native.d >> $(depfile)
resource_rust_native_cc = $(PERL) $(srctree)/scripts/e820-native-codegen.pl $(CC) $(c_flags)

quiet_cmd_resource_rust_o = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_resource_rust_o = $(resource_rust_native_ir) && \
	$(resource_rust_native_cc) -c $@.linked.ll -o $@ || exit $$? \
	$(cmd_ld_single) $(cmd_objtool)
define rule_resource_rust_o
	$(call cmd_and_fixdep,resource_rust_o)
	$(call cmd,gen_objtooldep)
	$(call cmd,gen_symversions_rs)
endef
quiet_cmd_resource_rust_s = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_resource_rust_s = $(resource_rust_native_ir) && \
	$(resource_rust_native_cc) -S $@.linked.ll -o $@ || exit $$?
quiet_cmd_resource_rust_ll = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_resource_rust_ll = $(resource_rust_native_ir) && \
	$(resource_rust_native_cc) -S -emit-llvm $@.linked.ll -o $@ && \
	$(x86-resource-policy-check) metadata $@ && \
	$(x86-resource-policy-check) rust $@ || exit $$?
endif

$(x86-resource-rust-targets): private target-stem = $(basename $(notdir $@))
$(x86-resource-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(x86-resource-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/$(target-stem).o,$(real-obj-y) $(lib-y)),y)
$(x86-resource-rust-targets): private part-of-module :=

# Explicit static pattern rules always select Rust despite adjacent C sources.
$(addprefix $(obj)/,$(addsuffix .o,$(x86-resource-rust-owners))): \
    $(obj)/%.o: $(src)/%.rs $(x86-resource-rust-bindings) $(x86-resource-native-inputs) FORCE
	+$(call if_changed_rule,$(if $(x86-resource-native-policy),resource_rust_o,rustc_o_rs))
$(addprefix $(obj)/,$(addsuffix .s,$(x86-resource-rust-owners))): \
    $(obj)/%.s: $(src)/%.rs $(x86-resource-rust-bindings) $(x86-resource-native-inputs) FORCE
	+$(call if_changed_dep,$(if $(x86-resource-native-policy),resource_rust_s,rustc_s_rs))
$(addprefix $(obj)/,$(addsuffix .ll,$(x86-resource-rust-owners))): \
    $(obj)/%.ll: $(src)/%.rs $(x86-resource-rust-bindings) $(x86-resource-native-inputs) FORCE
	+$(call if_changed_dep,$(if $(x86-resource-native-policy),resource_rust_ll,rustc_ll_rs))

quiet_cmd_resource_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_resource_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(addprefix $(obj)/,$(addsuffix .rsi,$(x86-resource-rust-owners))): \
    $(obj)/%.rsi: $(src)/%.rs $(x86-resource-rust-bindings) FORCE
	+$(call if_changed_dep,resource_rust_rsi)

# Build listing debug information in private objects; never overwrite the
# production objects, their commands, or their dependencies for a .lst request.
$(addprefix $(obj)/.resource-rust-listing/,$(addsuffix .o,$(x86-resource-rust-owners))): \
    private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(addprefix $(obj)/.resource-rust-listing/,$(addsuffix .o,$(x86-resource-rust-owners))): \
    private override KBUILD_CFLAGS += -g
$(addprefix $(obj)/.resource-rust-listing/,$(addsuffix .o,$(x86-resource-rust-owners))): \
    $(obj)/.resource-rust-listing/%.o: $(src)/%.rs $(x86-resource-rust-bindings) $(x86-resource-native-inputs) FORCE
	+$(call if_changed_rule,$(if $(x86-resource-native-policy),resource_rust_o,rustc_o_rs))
quiet_cmd_resource_rust_lst_elf = LD      $@
      cmd_resource_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(addprefix $(obj)/.resource-rust-listing-elf/,$(addsuffix .o,$(x86-resource-rust-owners))): \
    $(obj)/.resource-rust-listing-elf/%.o: $(obj)/.resource-rust-listing/%.o FORCE
	$(call if_changed,resource_rust_lst_elf)
quiet_cmd_resource_rust_lst = MKLST   $@
      cmd_resource_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(addprefix $(obj)/,$(addsuffix .lst,$(x86-resource-rust-owners))): \
    $(obj)/%.lst: $(obj)/.resource-rust-listing-elf/%.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,resource_rust_lst)

targets += $(foreach owner,$(x86-resource-rust-owners),\
    $(owner).s $(owner).ll $(owner).rsi $(owner).lst \
    .resource-rust-listing/$(owner).o .resource-rust-listing-elf/$(owner).o)
clean-files += .resource-rust-listing/ .resource-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.resource.$(ext).rustc/)
clean-files += $(foreach ext,o s ll,$(addprefix resource.$(ext).,rust.bc rust.ll native.d native.ll linked.ll))
endif
