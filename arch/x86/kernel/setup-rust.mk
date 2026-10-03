# SPDX-License-Identifier: GPL-2.0
# Included immediately after the original setup.o archive entry.
# Preserve the original archive slot and the canonical setup object identity.
clean-files += .setup-rust-listing/ .setup-rust-listing-elf/
ifeq ($(CONFIG_RUST_X86_SETUP),y)
obj-y := $(patsubst setup.o,setup.o setup_primitives.o setup_header_calls.o,$(obj-y))
RUSTFLAGS_setup.o += -Zfunction-sections=n

# Explicit rules prevent the adjacent, unchanged C source from being selected.
$(obj)/setup.o: $(src)/setup.rs $(objtree)/rust/bindings/x86_setup_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(addprefix $(obj)/,setup.o setup.s setup.ll setup.rsi .setup-rust-listing/setup.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,setup.s setup.ll setup.rsi): private target-stem := setup
$(addprefix $(obj)/,setup.s setup.ll setup.rsi): private part-of-builtin = $(if $(filter $(obj)/setup.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,setup.s setup.ll setup.rsi): private part-of-module :=
$(obj)/setup.s: $(src)/setup.rs $(objtree)/rust/bindings/x86_setup_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/setup.ll: $(src)/setup.rs $(objtree)/rust/bindings/x86_setup_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
quiet_cmd_setup_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_setup_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/setup.rsi: $(src)/setup.rs $(objtree)/rust/bindings/x86_setup_generated.rs FORCE
	+$(call if_changed_dep,setup_rust_rsi)

# Only the unlinked listing object receives debug information; setup.o is intact.
$(obj)/.setup-rust-listing/setup.o: private target-stem := setup
$(obj)/.setup-rust-listing/setup.o: private part-of-builtin = $(if $(filter $(obj)/setup.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.setup-rust-listing/setup.o: private part-of-module :=
$(obj)/.setup-rust-listing/setup.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.setup-rust-listing/setup.o: private override KBUILD_CFLAGS += -g
$(obj)/.setup-rust-listing/setup.o: $(src)/setup.rs $(objtree)/rust/bindings/x86_setup_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_setup_rust_lst_elf = LD      $@
      cmd_setup_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.setup-rust-listing-elf/setup.o: $(obj)/.setup-rust-listing/setup.o FORCE
	$(call if_changed,setup_rust_lst_elf)
quiet_cmd_setup_rust_lst = MKLST   $@
      cmd_setup_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/setup.lst: $(obj)/.setup-rust-listing-elf/setup.o $(srctree)/scripts/makelst \
    $(wildcard System.map) FORCE
	$(call if_changed,setup_rust_lst)
targets += setup.s setup.ll setup.rsi setup.lst \
    .setup-rust-listing/setup.o .setup-rust-listing-elf/setup.o

endif
ifeq ($(CONFIG_RUST_X86_SETUP),y)
# Generated, allowlisted canonical header calls; no setup.c body is compiled.
CFLAGS_setup_header_calls.o += -I$(objtree)/rust/bindings -I$(srctree)
$(obj)/setup_header_calls.o: $(objtree)/rust/bindings/x86_setup_generated.c
endif
