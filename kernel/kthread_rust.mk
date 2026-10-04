# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns kthread.o; do not add a second provider.
ifeq ($(CONFIG_RUST_KTHREAD),y)
obj-y := $(patsubst kthread.o,kthread.o kthread_helpers.o,$(obj-y))
RUSTFLAGS_kthread.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_kthread.o += cfi_encoding
$(obj)/kthread.o: $(src)/kthread.rs $(objtree)/rust/bindings/kthread_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Explicit targets need the same stem, module ownership and per-object policy.
$(addprefix $(obj)/,kthread.o kthread.s kthread.ll kthread.rsi .rust-listing/kthread.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,kthread.s kthread.ll kthread.rsi): private target-stem := kthread
$(addprefix $(obj)/,kthread.s kthread.ll kthread.rsi): private part-of-builtin = $(if $(filter $(obj)/kthread.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,kthread.s kthread.ll kthread.rsi): private part-of-module = $(if $(filter $(obj)/kthread.o,$(real-obj-m)),y)
$(obj)/kthread.s: $(src)/kthread.rs $(objtree)/rust/bindings/kthread_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/kthread.ll: $(src)/kthread.rs $(objtree)/rust/bindings/kthread_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
quiet_cmd_kthread_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_kthread_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/kthread.rsi: $(src)/kthread.rs $(objtree)/rust/bindings/kthread_generated.rs FORCE
	+$(call if_changed_dep,kthread_rust_rsi)

# Dedicated unlinked object for source-interleaved listing, including LTO.
$(obj)/.rust-listing/kthread.o: private target-stem := kthread
$(obj)/.rust-listing/kthread.o: private part-of-builtin = $(if $(filter $(obj)/kthread.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/kthread.o: private part-of-module = $(if $(filter $(obj)/kthread.o,$(real-obj-m)),y)
$(obj)/.rust-listing/kthread.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/kthread.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/kthread.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/kthread.o: $(src)/kthread.rs $(objtree)/rust/bindings/kthread_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_kthread_rust_lst_elf = LD      $@
      cmd_kthread_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/kthread.o: $(obj)/.rust-listing/kthread.o FORCE
	$(call if_changed,kthread_rust_lst_elf)
quiet_cmd_kthread_rust_lst = MKLST   $@
      cmd_kthread_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/kthread.lst: $(obj)/.rust-listing-elf/kthread.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,kthread_rust_lst)
targets += kthread.s kthread.ll kthread.rsi kthread.lst \
	.rust-listing/kthread.o .rust-listing-elf/kthread.o
endif
