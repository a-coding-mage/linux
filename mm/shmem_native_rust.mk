# SPDX-License-Identifier: GPL-2.0-only
# Include after mm/Makefile's original shmem.o object list.
ifeq ($(CONFIG_RUST_SHMEM),y)
obj-y := $(patsubst shmem.o,shmem.o shmem_native_data.o shmem_native_helpers.o shmem_native_inline.o,$(obj-y))
include $(srctree)/scripts/Makefile.rust-native-policy

# Preserve policy and fail closed until the common build verifies unsupported
# native cross-language instrumentation. Never remove security CFLAGS or force
# a configuration off to make this owner compile.
ifneq ($(filter y,$(CONFIG_CFI) $(CONFIG_GCC_PLUGIN_LATENT_ENTROPY) $(CONFIG_KSTACK_ERASE) $(CONFIG_GCC_PLUGIN_RANDSTRUCT) $(CONFIG_RANDSTRUCT_FULL) $(CONFIG_RANDSTRUCT_PERFORMANCE)),)
$(error RUST_SHMEM requires the combined native ABI/instrumentation review for this configuration)
endif
ifeq ($(CONFIG_CC_IS_CLANG)$(CONFIG_KCOV),yy)
$(error RUST_SHMEM native per-init coverage exemption awaits combined policy audit)
endif
RUSTFLAGS_shmem.o += -Zfunction-sections=n
RUSTFLAGS_shmem.o += $(shell sed -n 's/^pub const RUST_SHMEM_CFG_\([A-Z0-9_]*\): bool[_]* = true;/--cfg RUST_SHMEM_\1/p' $(objtree)/rust/bindings/shmem_native_generated.rs 2>/dev/null)
RUSTC_OUT_DIR_shmem.o = $(dir $@).$(notdir $@).rustc/
CFLAGS_shmem_native_inline.o += -I$(objtree)
$(obj)/shmem_native_inline.o: $(objtree)/rust/bindings/shmem_native_static.c

shmem-rust-fragments := shmem_account.rs shmem_cache.rs shmem_common.rs \
    shmem_directory.rs shmem_folio.rs shmem_huge.rs shmem_huge_config.rs \
    shmem_inode.rs shmem_io.rs shmem_mount.rs shmem_super.rs shmem_swap.rs \
    shmem_tiny.rs shmem_truncate.rs shmem_vm.rs shmem_xattr.rs shmem_native_aliases.rs
shmem-rust-inputs := $(objtree)/rust/bindings/shmem_native_generated.rs \
    $(rust-native-policy-inputs) $(addprefix $(src)/,$(shmem-rust-fragments))
shmem-rust-targets := $(addprefix $(obj)/,shmem.o shmem.s shmem.ll shmem.rsi .shmem-rust-listing/shmem.o)
$(shmem-rust-targets): private target-stem := shmem
$(shmem-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(shmem-rust-targets): private part-of-builtin = $(if $(filter $(obj)/shmem.o,$(real-obj-y) $(lib-y)),y)
$(shmem-rust-targets): private part-of-module :=
$(obj)/shmem.o: $(src)/shmem.rs $(shmem-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/shmem.s: $(src)/shmem.rs $(shmem-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/shmem.ll: $(src)/shmem.rs $(shmem-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_shmem_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_shmem_rust_rsi = $(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/shmem.rsi: $(src)/shmem.rs $(shmem-rust-inputs) FORCE
	+$(call if_changed_dep,shmem_rust_rsi)
$(obj)/.shmem-rust-listing/shmem.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.shmem-rust-listing/shmem.o: private override KBUILD_CFLAGS += -g
$(obj)/.shmem-rust-listing/shmem.o: $(src)/shmem.rs $(shmem-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_shmem_rust_lst_elf = LD      $@
      cmd_shmem_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.shmem-rust-listing-elf/shmem.o: $(obj)/.shmem-rust-listing/shmem.o FORCE
	$(call if_changed,shmem_rust_lst_elf)
quiet_cmd_shmem_rust_lst = MKLST $@
      cmd_shmem_rust_lst = $(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/shmem.lst: $(obj)/.shmem-rust-listing-elf/shmem.o $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,shmem_rust_lst)
targets += shmem.s shmem.ll shmem.rsi shmem.lst .shmem-rust-listing/shmem.o .shmem-rust-listing-elf/shmem.o
endif
clean-files += .shmem-rust-listing/ .shmem-rust-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.shmem.$(ext).rustc/)
