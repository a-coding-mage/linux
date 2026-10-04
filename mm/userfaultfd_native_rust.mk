# SPDX-License-Identifier: GPL-2.0-or-later
# Include after obj-$(CONFIG_USERFAULTFD) += userfaultfd.o in mm/Makefile.
ifeq ($(CONFIG_RUST_USERFAULTFD),y)
obj-y := $(patsubst userfaultfd.o,userfaultfd.o userfaultfd_native_helpers.o,$(obj-y))

# Original userfaultfd.c has no per-file SSP exemption; retain the native policy.
# Enroll mm/userfaultfd in scripts/rust-native-policy-owners before selecting.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_CFI),y)
$(error RUST_USERFAULTFD native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_USERFAULTFD native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_USERFAULTFD native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_USERFAULTFD native per-init coverage exemption is not integrated)
endif

RUSTFLAGS_userfaultfd.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_userfaultfd.o += cfi_encoding
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_userfaultfd.o = $(dir $@).$(notdir $@).rustc/

userfaultfd-rust-inputs := \
    $(src)/userfaultfd_fill.rs $(src)/userfaultfd_move.rs \
    $(src)/userfaultfd_context.rs $(src)/userfaultfd_ioctl.rs \
    $(src)/userfaultfd_native_aliases.rs \
    $(objtree)/rust/bindings/userfaultfd_native_generated.rs $(rust-native-policy-inputs)
userfaultfd-rust-targets := $(addprefix $(obj)/,userfaultfd.o userfaultfd.s userfaultfd.ll \
    userfaultfd.rsi .userfaultfd-rust-listing/userfaultfd.o)
$(userfaultfd-rust-targets): private target-stem := userfaultfd
$(userfaultfd-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(userfaultfd-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/userfaultfd.o,$(real-obj-y) $(lib-y)),y)
$(userfaultfd-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/userfaultfd.o: $(src)/userfaultfd.rs $(userfaultfd-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/userfaultfd.s: $(src)/userfaultfd.rs $(userfaultfd-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/userfaultfd.ll: $(src)/userfaultfd.rs $(userfaultfd-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_userfaultfd_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_userfaultfd_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/userfaultfd.rsi: $(src)/userfaultfd.rs $(userfaultfd-rust-inputs) FORCE
	+$(call if_changed_dep,userfaultfd_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.userfaultfd-rust-listing/userfaultfd.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.userfaultfd-rust-listing/userfaultfd.o: private override KBUILD_CFLAGS += -g
$(obj)/.userfaultfd-rust-listing/userfaultfd.o: $(src)/userfaultfd.rs $(userfaultfd-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_userfaultfd_rust_lst_elf = LD      $@
      cmd_userfaultfd_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.userfaultfd-rust-listing-elf/userfaultfd.o: $(obj)/.userfaultfd-rust-listing/userfaultfd.o FORCE
	$(call if_changed,userfaultfd_rust_lst_elf)
quiet_cmd_userfaultfd_rust_lst = MKLST   $@
      cmd_userfaultfd_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/userfaultfd.lst: $(obj)/.userfaultfd-rust-listing-elf/userfaultfd.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,userfaultfd_rust_lst)

targets += userfaultfd.s userfaultfd.ll userfaultfd.rsi userfaultfd.lst \
    .userfaultfd-rust-listing/userfaultfd.o .userfaultfd-rust-listing-elf/userfaultfd.o
endif
clean-files += .userfaultfd-rust-listing/ .userfaultfd-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.userfaultfd.$(ext).rustc/)
