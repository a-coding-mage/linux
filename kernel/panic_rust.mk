# SPDX-License-Identifier: GPL-2.0-only
# Retain the existing panic.o slot and its built-in module/flag ownership.
ifeq ($(CONFIG_RUST_PANIC),y)
include $(srctree)/scripts/Makefile.rust-native-policy
ifneq ($(CONFIG_RUSTC_VERSION),108501)
$(error RUST_PANIC requires the inspected Rust 1.85.1 C-variadic ABI)
endif
obj-y := $(patsubst panic.o,panic.o panic_helpers.o,$(obj-y))
RUSTFLAGS_panic.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_panic.o += c_variadic,link_llvm_intrinsics
ifeq ($(CONFIG_BUG),y)
# Kconfig limits this TU to x86_64; panic_helpers.c also verifies __WARN_FLAGS.
RUSTFLAGS_panic.o += --cfg LUPOS_PANIC_WARN_FLAGS
endif
$(obj)/panic.o: $(src)/panic.rs $(src)/panic_data.rs \
    $(objtree)/rust/bindings/panic_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))

$(addprefix $(obj)/,panic.o panic.s panic.ll panic.rsi .rust-listing/panic.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,panic.s panic.ll panic.rsi): private target-stem := panic
$(addprefix $(obj)/,panic.s panic.ll panic.rsi): private part-of-builtin = $(if $(filter $(obj)/panic.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,panic.s panic.ll panic.rsi): private part-of-module = $(if $(filter $(obj)/panic.o,$(real-obj-m)),y)
$(obj)/panic.s: $(src)/panic.rs $(src)/panic_data.rs $(objtree)/rust/bindings/panic_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/panic.ll: $(src)/panic.rs $(src)/panic_data.rs $(objtree)/rust/bindings/panic_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
quiet_cmd_panic_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_panic_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/panic.rsi: $(src)/panic.rs $(src)/panic_data.rs $(objtree)/rust/bindings/panic_generated.rs FORCE
	+$(call if_changed_dep,panic_rust_rsi)

$(obj)/.rust-listing/panic.o: private target-stem := panic
$(obj)/.rust-listing/panic.o: private part-of-builtin = $(if $(filter $(obj)/panic.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/panic.o: private part-of-module = $(if $(filter $(obj)/panic.o,$(real-obj-m)),y)
$(obj)/.rust-listing/panic.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/panic.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/panic.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/panic.o: $(src)/panic.rs $(src)/panic_data.rs $(objtree)/rust/bindings/panic_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_panic_rust_lst_elf = LD      $@
      cmd_panic_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/panic.o: $(obj)/.rust-listing/panic.o FORCE
	$(call if_changed,panic_rust_lst_elf)
quiet_cmd_panic_rust_lst = MKLST   $@
      cmd_panic_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/panic.lst: $(obj)/.rust-listing-elf/panic.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,panic_rust_lst)
targets += panic.s panic.ll panic.rsi panic.lst \
	.rust-listing/panic.o .rust-listing-elf/panic.o
endif
