# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y += memory.o in mm/Makefile.
ifeq ($(CONFIG_RUST_MEMORY),y)
obj-y := $(patsubst memory.o,memory.o memory_native_helpers.o,$(obj-y))

# Original memory.c uses native strong protection with no SSP exemptions.
# Enroll mm/memory in scripts/rust-native-policy-owners before selecting.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_CFI),y)
$(error RUST_MEMORY native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_MEMORY native per-init latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_MEMORY native per-init stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_MEMORY native per-init coverage exemption is not integrated)
endif

RUSTFLAGS_memory.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_memory.o += cfi_encoding
# Header-only architecture predicates are obtained from the current binding
# prerequisite at recipe evaluation time, including first builds.
RUSTFLAGS_memory.o += $(shell sed -n 's/^pub const RUST_MEMORY_CFG_\([A-Z0-9_]*\): bool[_]* = true;/--cfg RUST_MEMORY_\1/p' $(objtree)/rust/bindings/memory_native_generated.rs 2>/dev/null)
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_memory.o = $(dir $@).$(notdir $@).rustc/

memory-rust-inputs := \
    $(objtree)/rust/bindings/memory_native_generated.rs $(rust-native-policy-inputs) \
    $(addprefix $(src)/,memory_copy.rs memory_zap.rs memory_insert.rs memory_faults.rs)
memory-rust-targets := $(addprefix $(obj)/,memory.o memory.s memory.ll \
    memory.rsi .memory-rust-listing/memory.o)
$(memory-rust-targets): private target-stem := memory
$(memory-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(memory-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/memory.o,$(real-obj-y) $(lib-y)),y)
$(memory-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/memory.o: $(src)/memory.rs $(memory-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/memory.s: $(src)/memory.rs $(memory-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/memory.ll: $(src)/memory.rs $(memory-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_memory_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_memory_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/memory.rsi: $(src)/memory.rs $(memory-rust-inputs) FORCE
	+$(call if_changed_dep,memory_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.memory-rust-listing/memory.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.memory-rust-listing/memory.o: private override KBUILD_CFLAGS += -g
$(obj)/.memory-rust-listing/memory.o: $(src)/memory.rs $(memory-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_memory_rust_lst_elf = LD      $@
      cmd_memory_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.memory-rust-listing-elf/memory.o: $(obj)/.memory-rust-listing/memory.o FORCE
	$(call if_changed,memory_rust_lst_elf)
quiet_cmd_memory_rust_lst = MKLST   $@
      cmd_memory_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/memory.lst: $(obj)/.memory-rust-listing-elf/memory.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,memory_rust_lst)

targets += memory.s memory.ll memory.rsi memory.lst \
    .memory-rust-listing/memory.o .memory-rust-listing-elf/memory.o
endif
clean-files += .memory-rust-listing/ .memory-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.memory.$(ext).rustc/)
