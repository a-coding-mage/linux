# SPDX-License-Identifier: GPL-2.0
# Original owners retain their archive positions and every canonical target.
ifeq ($(CONFIG_RUST_IRQ_CORE),y)
include $(srctree)/scripts/Makefile.rust-native-policy
obj-y := $(patsubst irqdesc.o,irqdesc.o irq_core_native.o,$(obj-y))
irq_core_rust_sources := $(addprefix $(src)/,irq_core.rs irq_core_storage.rs)
# No blanket sanitizer/profiling/coverage exclusions. The only original
# per-function noinstr path is retained explicitly in handle.rs/native glue.
# Section/noinline do not implement all native noinstr exclusions. Refuse
# unreviewed sanitizer/profile/coverage configurations rather than suppressing
# their instrumentation for an entire owner or silently weakening the path.
ifneq ($(filter y,$(CONFIG_KASAN) $(CONFIG_KMSAN) $(CONFIG_KCSAN) \
	$(CONFIG_KCOV) $(CONFIG_FUNCTION_TRACER) $(CONFIG_GCOV_KERNEL) \
	$(CONFIG_AUTOFDO_CLANG) $(CONFIG_PROPELLER_CLANG) \
	$(CONFIG_KSTACK_ERASE) $(CONFIG_GCC_PLUGIN_LATENT_ENTROPY)),)
$(error RUST_IRQ_CORE per-function native instrumentation parity requires separate review)
endif

RUSTFLAGS_irqdesc.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_irqdesc.o += cfi_encoding
$(obj)/irqdesc.o: $(src)/irqdesc.rs $(irq_core_rust_sources) \
    $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(addprefix $(obj)/,irqdesc.o irqdesc.s irqdesc.ll irqdesc.rsi .rust-listing/irqdesc.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,irqdesc.s irqdesc.ll irqdesc.rsi): private target-stem := irqdesc
$(addprefix $(obj)/,irqdesc.s irqdesc.ll irqdesc.rsi): private part-of-builtin = $(if $(filter $(obj)/irqdesc.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,irqdesc.s irqdesc.ll irqdesc.rsi): private part-of-module = $(if $(filter $(obj)/irqdesc.o,$(real-obj-m)),y)
$(obj)/irqdesc.s: $(src)/irqdesc.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/irqdesc.ll: $(src)/irqdesc.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_irqdesc_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_irqdesc_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/irqdesc.rsi: $(src)/irqdesc.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs FORCE
	+$(call if_changed_dep,irqdesc_rust_rsi)
$(obj)/.rust-listing/irqdesc.o: private target-stem := irqdesc
$(obj)/.rust-listing/irqdesc.o: private part-of-builtin = $(if $(filter $(obj)/irqdesc.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/irqdesc.o: private part-of-module = $(if $(filter $(obj)/irqdesc.o,$(real-obj-m)),y)
$(obj)/.rust-listing/irqdesc.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/irqdesc.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/irqdesc.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/irqdesc.o: $(src)/irqdesc.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_irqdesc_rust_lst_elf = LD      $@
      cmd_irqdesc_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/irqdesc.o: $(obj)/.rust-listing/irqdesc.o FORCE
	$(call if_changed,irqdesc_rust_lst_elf)
quiet_cmd_irqdesc_rust_lst = MKLST   $@
      cmd_irqdesc_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/irqdesc.lst: $(obj)/.rust-listing-elf/irqdesc.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,irqdesc_rust_lst)
targets += irqdesc.s irqdesc.ll irqdesc.rsi irqdesc.lst \
    .rust-listing/irqdesc.o .rust-listing-elf/irqdesc.o

RUSTFLAGS_handle.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_handle.o += cfi_encoding
$(obj)/handle.o: $(src)/handle.rs $(irq_core_rust_sources) \
    $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(addprefix $(obj)/,handle.o handle.s handle.ll handle.rsi .rust-listing/handle.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,handle.s handle.ll handle.rsi): private target-stem := handle
$(addprefix $(obj)/,handle.s handle.ll handle.rsi): private part-of-builtin = $(if $(filter $(obj)/handle.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,handle.s handle.ll handle.rsi): private part-of-module = $(if $(filter $(obj)/handle.o,$(real-obj-m)),y)
$(obj)/handle.s: $(src)/handle.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/handle.ll: $(src)/handle.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_handle_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_handle_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/handle.rsi: $(src)/handle.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs FORCE
	+$(call if_changed_dep,handle_rust_rsi)
$(obj)/.rust-listing/handle.o: private target-stem := handle
$(obj)/.rust-listing/handle.o: private part-of-builtin = $(if $(filter $(obj)/handle.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/handle.o: private part-of-module = $(if $(filter $(obj)/handle.o,$(real-obj-m)),y)
$(obj)/.rust-listing/handle.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/handle.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/handle.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/handle.o: $(src)/handle.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_handle_rust_lst_elf = LD      $@
      cmd_handle_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/handle.o: $(obj)/.rust-listing/handle.o FORCE
	$(call if_changed,handle_rust_lst_elf)
quiet_cmd_handle_rust_lst = MKLST   $@
      cmd_handle_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/handle.lst: $(obj)/.rust-listing-elf/handle.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,handle_rust_lst)
targets += handle.s handle.ll handle.rsi handle.lst \
    .rust-listing/handle.o .rust-listing-elf/handle.o

RUSTFLAGS_chip.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_chip.o += cfi_encoding
$(obj)/chip.o: $(src)/chip.rs $(irq_core_rust_sources) \
    $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(addprefix $(obj)/,chip.o chip.s chip.ll chip.rsi .rust-listing/chip.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,chip.s chip.ll chip.rsi): private target-stem := chip
$(addprefix $(obj)/,chip.s chip.ll chip.rsi): private part-of-builtin = $(if $(filter $(obj)/chip.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,chip.s chip.ll chip.rsi): private part-of-module = $(if $(filter $(obj)/chip.o,$(real-obj-m)),y)
$(obj)/chip.s: $(src)/chip.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/chip.ll: $(src)/chip.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_chip_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_chip_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/chip.rsi: $(src)/chip.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs FORCE
	+$(call if_changed_dep,chip_rust_rsi)
$(obj)/.rust-listing/chip.o: private target-stem := chip
$(obj)/.rust-listing/chip.o: private part-of-builtin = $(if $(filter $(obj)/chip.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/chip.o: private part-of-module = $(if $(filter $(obj)/chip.o,$(real-obj-m)),y)
$(obj)/.rust-listing/chip.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/chip.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/chip.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/chip.o: $(src)/chip.rs $(irq_core_rust_sources) $(objtree)/rust/bindings/irq_core_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_chip_rust_lst_elf = LD      $@
      cmd_chip_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/chip.o: $(obj)/.rust-listing/chip.o FORCE
	$(call if_changed,chip_rust_lst_elf)
quiet_cmd_chip_rust_lst = MKLST   $@
      cmd_chip_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/chip.lst: $(obj)/.rust-listing-elf/chip.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,chip_rust_lst)
targets += chip.s chip.ll chip.rsi chip.lst \
    .rust-listing/chip.o .rust-listing-elf/chip.o
endif
