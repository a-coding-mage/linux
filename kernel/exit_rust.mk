# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns exit.o; do not add a second provider.
ifeq ($(CONFIG_RUST_EXIT),y)
obj-y := $(patsubst exit.o,exit.o exit_helpers.o,$(obj-y))
RUSTFLAGS_exit.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_exit.o += cfi_encoding,linkage
# abort retains native weak and function alignment metadata in its entry shell.
$(obj)/exit.o: $(src)/exit.rs $(filter-out $(src)/exit.rs,$(wildcard $(src)/exit_*.rs)) $(objtree)/rust/bindings/exit_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Explicit targets need the same stem, module ownership and per-object policy.
$(addprefix $(obj)/,exit.o exit.s exit.ll exit.rsi .rust-listing/exit.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,exit.s exit.ll exit.rsi): private target-stem := exit
$(addprefix $(obj)/,exit.s exit.ll exit.rsi): private part-of-builtin = $(if $(filter $(obj)/exit.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,exit.s exit.ll exit.rsi): private part-of-module = $(if $(filter $(obj)/exit.o,$(real-obj-m)),y)
$(obj)/exit.s: $(src)/exit.rs $(filter-out $(src)/exit.rs,$(wildcard $(src)/exit_*.rs)) $(objtree)/rust/bindings/exit_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/exit.ll: $(src)/exit.rs $(filter-out $(src)/exit.rs,$(wildcard $(src)/exit_*.rs)) $(objtree)/rust/bindings/exit_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
quiet_cmd_exit_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_exit_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/exit.rsi: $(src)/exit.rs $(filter-out $(src)/exit.rs,$(wildcard $(src)/exit_*.rs)) $(objtree)/rust/bindings/exit_generated.rs FORCE
	+$(call if_changed_dep,exit_rust_rsi)

# Dedicated unlinked object for source-interleaved listing, including LTO.
$(obj)/.rust-listing/exit.o: private target-stem := exit
$(obj)/.rust-listing/exit.o: private part-of-builtin = $(if $(filter $(obj)/exit.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/exit.o: private part-of-module = $(if $(filter $(obj)/exit.o,$(real-obj-m)),y)
$(obj)/.rust-listing/exit.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/exit.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/exit.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/exit.o: $(src)/exit.rs $(filter-out $(src)/exit.rs,$(wildcard $(src)/exit_*.rs)) $(objtree)/rust/bindings/exit_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_exit_rust_lst_elf = LD      $@
      cmd_exit_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/exit.o: $(obj)/.rust-listing/exit.o FORCE
	$(call if_changed,exit_rust_lst_elf)
quiet_cmd_exit_rust_lst = MKLST   $@
      cmd_exit_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/exit.lst: $(obj)/.rust-listing-elf/exit.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,exit_rust_lst)
targets += exit.s exit.ll exit.rsi exit.lst \
	.rust-listing/exit.o .rust-listing-elf/exit.o
endif
