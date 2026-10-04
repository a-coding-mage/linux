# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y += mmap.o in mm/Makefile.
ifeq ($(CONFIG_RUST_MMAP),y)
obj-y := $(patsubst mmap.o,mmap.o mmap_native_helpers.o,$(obj-y))

# Original mmap.c uses native strong protection with no SSP exemptions.
# Enroll mm/mmap in scripts/rust-native-policy-owners before selecting.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_CFI),y)
$(error RUST_MMAP native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_MMAP native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_MMAP native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_MMAP native per-init coverage exemption is not integrated)
endif

RUSTFLAGS_mmap.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_mmap.o += cfi_encoding
# Header-only architecture predicates are obtained from the current binding
# prerequisite at recipe evaluation time, including first builds.
RUSTFLAGS_mmap.o += $(shell sed -n 's/^pub const RUST_MMAP_CFG_\([A-Z0-9_]*\): bool[_]* = true;/--cfg RUST_MMAP_\1/p' $(objtree)/rust/bindings/mmap_native_generated.rs 2>/dev/null)
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_mmap.o = $(dir $@).$(notdir $@).rustc/

mmap-rust-inputs := \
    $(objtree)/rust/bindings/mmap_native_generated.rs $(rust-native-policy-inputs)
mmap-rust-targets := $(addprefix $(obj)/,mmap.o mmap.s mmap.ll \
    mmap.rsi .mmap-rust-listing/mmap.o)
$(mmap-rust-targets): private target-stem := mmap
$(mmap-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(mmap-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/mmap.o,$(real-obj-y) $(lib-y)),y)
$(mmap-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/mmap.o: $(src)/mmap.rs $(mmap-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/mmap.s: $(src)/mmap.rs $(mmap-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/mmap.ll: $(src)/mmap.rs $(mmap-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_mmap_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_mmap_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/mmap.rsi: $(src)/mmap.rs $(mmap-rust-inputs) FORCE
	+$(call if_changed_dep,mmap_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.mmap-rust-listing/mmap.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.mmap-rust-listing/mmap.o: private override KBUILD_CFLAGS += -g
$(obj)/.mmap-rust-listing/mmap.o: $(src)/mmap.rs $(mmap-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_mmap_rust_lst_elf = LD      $@
      cmd_mmap_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.mmap-rust-listing-elf/mmap.o: $(obj)/.mmap-rust-listing/mmap.o FORCE
	$(call if_changed,mmap_rust_lst_elf)
quiet_cmd_mmap_rust_lst = MKLST   $@
      cmd_mmap_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/mmap.lst: $(obj)/.mmap-rust-listing-elf/mmap.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,mmap_rust_lst)

targets += mmap.s mmap.ll mmap.rsi mmap.lst \
    .mmap-rust-listing/mmap.o .mmap-rust-listing-elf/mmap.o
endif
clean-files += .mmap-rust-listing/ .mmap-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.mmap.$(ext).rustc/)
