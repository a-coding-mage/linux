# SPDX-License-Identifier: GPL-2.0-only
# Integration proposal: include after the original x86_init.o archive entry.
# Keep the original x86_init.o identity/position and all inherited Kbuild flags.
clean-files += .x86-platform-callbacks-rust-listing/ .x86-platform-callbacks-rust-listing-elf/
ifeq ($(CONFIG_RUST_X86_PLATFORM_CALLBACKS),y)
# Distinct C callback functions have distinct addresses, even for equal bodies.
# All inspection rules below use target-stem=x86_init and inherit this setting.
RUSTFLAGS_x86_init.o += -Zmerge-functions=disabled

# The explicit Rust rule is necessary while the unchanged x86_init.c is present.
$(obj)/x86_init.o: $(src)/x86_init.rs \
    $(objtree)/rust/bindings/x86_platform_callbacks_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Inspection targets must also select the Rust owner when adjacent C exists.
$(addprefix $(obj)/,x86_init.o x86_init.s x86_init.ll x86_init.rsi .x86-platform-callbacks-rust-listing/x86_init.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,x86_init.s x86_init.ll x86_init.rsi): private target-stem := x86_init
$(addprefix $(obj)/,x86_init.s x86_init.ll x86_init.rsi): private part-of-builtin = $(if $(filter $(obj)/x86_init.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,x86_init.s x86_init.ll x86_init.rsi): private part-of-module :=
$(obj)/x86_init.s: $(src)/x86_init.rs $(objtree)/rust/bindings/x86_platform_callbacks_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/x86_init.ll: $(src)/x86_init.rs $(objtree)/rust/bindings/x86_platform_callbacks_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
quiet_cmd_x86_platform_callbacks_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_x86_platform_callbacks_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/x86_init.rsi: $(src)/x86_init.rs $(objtree)/rust/bindings/x86_platform_callbacks_generated.rs FORCE
	+$(call if_changed_dep,x86_platform_callbacks_rust_rsi)

# Only the unlinked listing object gets debug info; x86_init.o is untouched.
$(obj)/.x86-platform-callbacks-rust-listing/x86_init.o: private target-stem := x86_init
$(obj)/.x86-platform-callbacks-rust-listing/x86_init.o: private part-of-builtin = $(if $(filter $(obj)/x86_init.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.x86-platform-callbacks-rust-listing/x86_init.o: private part-of-module :=
$(obj)/.x86-platform-callbacks-rust-listing/x86_init.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.x86-platform-callbacks-rust-listing/x86_init.o: private override KBUILD_CFLAGS += -g
$(obj)/.x86-platform-callbacks-rust-listing/x86_init.o: $(src)/x86_init.rs $(objtree)/rust/bindings/x86_platform_callbacks_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_x86_platform_callbacks_rust_lst_elf = LD      $@
      cmd_x86_platform_callbacks_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.x86-platform-callbacks-rust-listing-elf/x86_init.o: $(obj)/.x86-platform-callbacks-rust-listing/x86_init.o FORCE
	$(call if_changed,x86_platform_callbacks_rust_lst_elf)
quiet_cmd_x86_platform_callbacks_rust_lst = MKLST   $@
      cmd_x86_platform_callbacks_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/x86_init.lst: $(obj)/.x86-platform-callbacks-rust-listing-elf/x86_init.o $(srctree)/scripts/makelst \
    $(wildcard System.map) FORCE
	$(call if_changed,x86_platform_callbacks_rust_lst)
targets += x86_init.s x86_init.ll x86_init.rsi x86_init.lst \
    .x86-platform-callbacks-rust-listing/x86_init.o .x86-platform-callbacks-rust-listing-elf/x86_init.o
endif
