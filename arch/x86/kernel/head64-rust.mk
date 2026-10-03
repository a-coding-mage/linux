# SPDX-License-Identifier: GPL-2.0
# Included in place of arch/x86/kernel/Makefile's head$(BITS).o entry.
# Preserve the original archive slot and the canonical head64 object identity.
obj-y += head$(BITS).o
clean-files += .head64-rust-listing/ .head64-rust-listing-elf/
ifeq ($(CONFIG_RUST_X86_HEAD64),y)
obj-y += head64_primitives.o
RUSTFLAGS_head64.o += -Zfunction-sections=n

# Explicit rules prevent the adjacent, unchanged C source from being selected.
$(obj)/head64.o: $(src)/head64.rs $(objtree)/rust/bindings/head64_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(addprefix $(obj)/,head64.o head64.s head64.ll head64.rsi .head64-rust-listing/head64.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,head64.s head64.ll head64.rsi): private target-stem := head64
$(addprefix $(obj)/,head64.s head64.ll head64.rsi): private part-of-builtin = $(if $(filter $(obj)/head64.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,head64.s head64.ll head64.rsi): private part-of-module :=
$(obj)/head64.s: $(src)/head64.rs $(objtree)/rust/bindings/head64_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/head64.ll: $(src)/head64.rs $(objtree)/rust/bindings/head64_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
quiet_cmd_head64_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_head64_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/head64.rsi: $(src)/head64.rs $(objtree)/rust/bindings/head64_generated.rs FORCE
	+$(call if_changed_dep,head64_rust_rsi)

# Only the unlinked listing object receives debug information; head64.o is intact.
$(obj)/.head64-rust-listing/head64.o: private target-stem := head64
$(obj)/.head64-rust-listing/head64.o: private part-of-builtin = $(if $(filter $(obj)/head64.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.head64-rust-listing/head64.o: private part-of-module :=
$(obj)/.head64-rust-listing/head64.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.head64-rust-listing/head64.o: private override KBUILD_CFLAGS += -g
$(obj)/.head64-rust-listing/head64.o: $(src)/head64.rs $(objtree)/rust/bindings/head64_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_head64_rust_lst_elf = LD      $@
      cmd_head64_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.head64-rust-listing-elf/head64.o: $(obj)/.head64-rust-listing/head64.o FORCE
	$(call if_changed,head64_rust_lst_elf)
quiet_cmd_head64_rust_lst = MKLST   $@
      cmd_head64_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/head64.lst: $(obj)/.head64-rust-listing-elf/head64.o $(srctree)/scripts/makelst \
    $(wildcard System.map) FORCE
	$(call if_changed,head64_rust_lst)
targets += head64.s head64.ll head64.rsi head64.lst \
    .head64-rust-listing/head64.o .head64-rust-listing-elf/head64.o

# Carry the original early object's policy to its compiler/architecture bridge.
# target-stem above also applies this policy to the private inspection objects.
CFLAGS_REMOVE_head64.o = -pg
CFLAGS_REMOVE_head64_primitives.o = -pg
CFLAGS_head64.o := -fno-stack-protector
CFLAGS_head64_primitives.o := -fno-stack-protector
KASAN_SANITIZE_head64.o := n
KASAN_SANITIZE_head64_primitives.o := n
KCSAN_SANITIZE_head64.o := n
KCSAN_SANITIZE_head64_primitives.o := n
KMSAN_SANITIZE_head64.o := n
KMSAN_SANITIZE_head64_primitives.o := n
KCOV_INSTRUMENT_head64.o := n
KCOV_INSTRUMENT_head64_primitives.o := n
endif
