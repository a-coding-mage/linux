# SPDX-License-Identifier: GPL-2.0-only
# Include after the canonical obj-y assignment in arch/x86/mm/Makefile.
ifeq ($(CONFIG_RUST_X86_MM_INIT),y)
# Preserve both original archive entry names and positions. The extra object
# holds only header/compiler primitives and native registration metadata.
obj-y := $(patsubst init_64.o,init_64.o init_primitives.o,$(obj-y))

# Both original owners have ordinary native stack protection, with no explicit
# per-function or per-file stack-protector exemptions. The common policy lane
# must enroll arch/x86/mm/init and arch/x86/mm/init_64 before integration.
include $(srctree)/scripts/Makefile.rust-native-policy

x86-mm-init-owners := init init_64
x86-mm-init-bindings := $(objtree)/rust/bindings/x86_mm_init_generated.rs
x86-mm-init-inputs := $(x86-mm-init-bindings) \
    $(src)/init_support.rs $(src)/init_pgtable.rs $(src)/init_identity_support.rs $(src)/init_percpu.rs $(src)/init_cpu_ids.rs $(src)/ident_map.rs $(rust-native-policy-inputs)
x86-mm-init-targets := $(addprefix $(obj)/,\
    $(foreach owner,$(x86-mm-init-owners),\
        $(owner).o $(owner).s $(owner).ll $(owner).rsi \
        .mm-init-rust-listing/$(owner).o))
RUST_ALLOWED_FEATURES_init.o += cfi_encoding
RUST_ALLOWED_FEATURES_init_64.o += cfi_encoding
RUSTFLAGS_init.o += -Zfunction-sections=n
RUSTFLAGS_init_64.o += -Zfunction-sections=n

# The native source combines __init/__meminit and runtime bodies. Reject
# instrumentation that lacks reviewed per-function Rust parity; never fall
# back to compiling the unchanged C sibling.
ifneq ($(filter y,$(CONFIG_GCC_PLUGIN_LATENT_ENTROPY) $(CONFIG_KSTACK_ERASE)),)
$(error RUST_X86_MM_INIT requires per-function latent-entropy/stack-depth instrumentation review)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_X86_MM_INIT requires per-init-function KCOV exemption review)
endif

$(x86-mm-init-targets): private target-stem = $(basename $(notdir $@))
$(x86-mm-init-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(x86-mm-init-targets): private part-of-builtin = \
    $(if $(filter $(obj)/$(target-stem).o,$(real-obj-y) $(lib-y)),y)
$(x86-mm-init-targets): private part-of-module :=

$(addprefix $(obj)/,$(addsuffix .o,$(x86-mm-init-owners))): \
    $(obj)/%.o: $(src)/%.rs $(x86-mm-init-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(addprefix $(obj)/,$(addsuffix .s,$(x86-mm-init-owners))): \
    $(obj)/%.s: $(src)/%.rs $(x86-mm-init-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(addprefix $(obj)/,$(addsuffix .ll,$(x86-mm-init-owners))): \
    $(obj)/%.ll: $(src)/%.rs $(x86-mm-init-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_mm_init_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_mm_init_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(addprefix $(obj)/,$(addsuffix .rsi,$(x86-mm-init-owners))): \
    $(obj)/%.rsi: $(src)/%.rs $(x86-mm-init-inputs) FORCE
	+$(call if_changed_dep,mm_init_rust_rsi)
$(addprefix $(obj)/.mm-init-rust-listing/,$(addsuffix .o,$(x86-mm-init-owners))): \
    private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(addprefix $(obj)/.mm-init-rust-listing/,$(addsuffix .o,$(x86-mm-init-owners))): \
    private override KBUILD_CFLAGS += -g
$(addprefix $(obj)/.mm-init-rust-listing/,$(addsuffix .o,$(x86-mm-init-owners))): \
    $(obj)/.mm-init-rust-listing/%.o: $(src)/%.rs $(x86-mm-init-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_mm_init_rust_lst_elf = LD      $@
      cmd_mm_init_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(addprefix $(obj)/.mm-init-rust-listing-elf/,$(addsuffix .o,$(x86-mm-init-owners))): \
    $(obj)/.mm-init-rust-listing-elf/%.o: $(obj)/.mm-init-rust-listing/%.o FORCE
	$(call if_changed,mm_init_rust_lst_elf)
quiet_cmd_mm_init_rust_lst = MKLST   $@
      cmd_mm_init_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(addprefix $(obj)/,$(addsuffix .lst,$(x86-mm-init-owners))): \
    $(obj)/%.lst: $(obj)/.mm-init-rust-listing-elf/%.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,mm_init_rust_lst)
targets += $(foreach owner,$(x86-mm-init-owners),\
    $(owner).s $(owner).ll $(owner).rsi $(owner).lst \
    .mm-init-rust-listing/$(owner).o .mm-init-rust-listing-elf/$(owner).o)
endif
