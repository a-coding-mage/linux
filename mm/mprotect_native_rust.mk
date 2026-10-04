# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y += mprotect.o in mm/Makefile.
ifeq ($(CONFIG_RUST_MPROTECT),y)
obj-y := $(patsubst mprotect.o,mprotect.o mprotect_native_helpers.o,$(obj-y))

# Original mprotect.c has no SSP exemptions or per-object sanitizer removals.
# Enrollment is proposed separately and requires the shared native policy review.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_CFI),y)
$(error RUST_MPROTECT native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_MPROTECT native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_MPROTECT native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_MPROTECT native per-init coverage exemption is not integrated)
endif

RUSTFLAGS_mprotect.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_mprotect.o += cfi_encoding
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_mprotect.o = $(dir $@).$(notdir $@).rustc/

mprotect-rust-inputs := \
    $(objtree)/rust/bindings/mprotect_native_generated.rs $(rust-native-policy-inputs)
mprotect-rust-targets := $(addprefix $(obj)/,mprotect.o mprotect.s mprotect.ll \
    mprotect.rsi .mprotect-rust-listing/mprotect.o)
$(mprotect-rust-targets): private target-stem := mprotect
$(mprotect-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(mprotect-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/mprotect.o,$(real-obj-y) $(lib-y)),y)
$(mprotect-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/mprotect.o: $(src)/mprotect.rs $(mprotect-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/mprotect.s: $(src)/mprotect.rs $(mprotect-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/mprotect.ll: $(src)/mprotect.rs $(mprotect-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_mprotect_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_mprotect_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/mprotect.rsi: $(src)/mprotect.rs $(mprotect-rust-inputs) FORCE
	+$(call if_changed_dep,mprotect_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.mprotect-rust-listing/mprotect.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.mprotect-rust-listing/mprotect.o: private override KBUILD_CFLAGS += -g
$(obj)/.mprotect-rust-listing/mprotect.o: $(src)/mprotect.rs $(mprotect-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_mprotect_rust_lst_elf = LD      $@
      cmd_mprotect_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.mprotect-rust-listing-elf/mprotect.o: $(obj)/.mprotect-rust-listing/mprotect.o FORCE
	$(call if_changed,mprotect_rust_lst_elf)
quiet_cmd_mprotect_rust_lst = MKLST   $@
      cmd_mprotect_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/mprotect.lst: $(obj)/.mprotect-rust-listing-elf/mprotect.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,mprotect_rust_lst)

targets += mprotect.s mprotect.ll mprotect.rsi mprotect.lst \
    .mprotect-rust-listing/mprotect.o .mprotect-rust-listing-elf/mprotect.o
endif
clean-files += .mprotect-rust-listing/ .mprotect-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.mprotect.$(ext).rustc/)
