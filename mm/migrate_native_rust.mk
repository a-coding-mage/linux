# SPDX-License-Identifier: GPL-2.0-or-later
# Include after the original obj-y += migrate.o in mm/Makefile.
ifeq ($(CONFIG_RUST_MIGRATE),y)
obj-y := $(patsubst migrate.o,migrate.o migrate_native_helpers.o,$(obj-y))

# Original migrate.c retains native strong protection with no SSP exemptions.
# Enroll mm/migrate in scripts/rust-native-policy-owners before selecting.
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve the remaining unimplemented mixed-function instrumentation gates.
ifeq ($(CONFIG_CFI),y)
$(error RUST_MIGRATE native cross-language CFI identities require the combined object audit)
endif
ifeq ($(CONFIG_GCC_PLUGIN_LATENT_ENTROPY),y)
$(error RUST_MIGRATE native mixed-function latent-entropy instrumentation is not integrated)
endif
ifeq ($(CONFIG_KSTACK_ERASE),y)
$(error RUST_MIGRATE native mixed-function stack-depth instrumentation is not integrated)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_MIGRATE native mixed-function coverage exemption is not integrated)
endif

RUSTFLAGS_migrate.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_migrate.o += cfi_encoding
# No owner init/cold annotations occur in immutable migrate.c.
# Supported by the lead's shared rustc output-directory race repair. It is
# target-specific so simultaneous inspection targets have distinct temporaries.
RUSTC_OUT_DIR_migrate.o = $(dir $@).$(notdir $@).rustc/

migrate-rust-inputs := \
    $(objtree)/rust/bindings/migrate_native_generated.rs $(rust-native-policy-inputs) \
    $(addprefix $(src)/,migrate_ptes.rs migrate_mapping.rs migrate_move.rs migrate_batch.rs migrate_syscall.rs migrate_numa.rs migrate_native_aliases.rs)
migrate-rust-targets := $(addprefix $(obj)/,migrate.o migrate.s migrate.ll \
    migrate.rsi .migrate-rust-listing/migrate.o)
$(migrate-rust-targets): private target-stem := migrate
$(migrate-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(migrate-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/migrate.o,$(real-obj-y) $(lib-y)),y)
$(migrate-rust-targets): private part-of-module :=

# Explicit Rust recipes take precedence over the unchanged adjacent C file.
$(obj)/migrate.o: $(src)/migrate.rs $(migrate-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/migrate.s: $(src)/migrate.rs $(migrate-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/migrate.ll: $(src)/migrate.rs $(migrate-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_migrate_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_migrate_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/migrate.rsi: $(src)/migrate.rs $(migrate-rust-inputs) FORCE
	+$(call if_changed_dep,migrate_rust_rsi)

# Listings use a private object, never a C rewrite of the production object.
$(obj)/.migrate-rust-listing/migrate.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.migrate-rust-listing/migrate.o: private override KBUILD_CFLAGS += -g
$(obj)/.migrate-rust-listing/migrate.o: $(src)/migrate.rs $(migrate-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_migrate_rust_lst_elf = LD      $@
      cmd_migrate_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.migrate-rust-listing-elf/migrate.o: $(obj)/.migrate-rust-listing/migrate.o FORCE
	$(call if_changed,migrate_rust_lst_elf)
quiet_cmd_migrate_rust_lst = MKLST   $@
      cmd_migrate_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/migrate.lst: $(obj)/.migrate-rust-listing-elf/migrate.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,migrate_rust_lst)

targets += migrate.s migrate.ll migrate.rsi migrate.lst \
    .migrate-rust-listing/migrate.o .migrate-rust-listing-elf/migrate.o
endif
clean-files += .migrate-rust-listing/ .migrate-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.migrate.$(ext).rustc/)
