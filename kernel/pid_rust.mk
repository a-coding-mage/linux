# SPDX-License-Identifier: GPL-2.0-only
# The existing obj-y entry owns pid.o; do not add a second provider.
ifeq ($(CONFIG_RUST_PID),y)
obj-y := $(patsubst pid.o,pid.o pid_helpers.o,$(obj-y))
RUSTFLAGS_pid.o += -Zfunction-sections=n
RUST_ALLOWED_FEATURES_pid.o += cfi_encoding
$(obj)/pid.o: $(src)/pid.rs $(objtree)/rust/bindings/pid_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Explicit targets need the same stem, module ownership and per-object policy.
$(addprefix $(obj)/,pid.o pid.s pid.ll pid.rsi .rust-listing/pid.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,pid.s pid.ll pid.rsi): private target-stem := pid
$(addprefix $(obj)/,pid.s pid.ll pid.rsi): private part-of-builtin = $(if $(filter $(obj)/pid.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,pid.s pid.ll pid.rsi): private part-of-module = $(if $(filter $(obj)/pid.o,$(real-obj-m)),y)
$(obj)/pid.s: $(src)/pid.rs $(objtree)/rust/bindings/pid_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/pid.ll: $(src)/pid.rs $(objtree)/rust/bindings/pid_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
quiet_cmd_pid_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_pid_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/pid.rsi: $(src)/pid.rs $(objtree)/rust/bindings/pid_generated.rs FORCE
	+$(call if_changed_dep,pid_rust_rsi)

# Dedicated unlinked object for source-interleaved listing, including LTO.
$(obj)/.rust-listing/pid.o: private target-stem := pid
$(obj)/.rust-listing/pid.o: private part-of-builtin = $(if $(filter $(obj)/pid.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/pid.o: private part-of-module = $(if $(filter $(obj)/pid.o,$(real-obj-m)),y)
$(obj)/.rust-listing/pid.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/pid.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/pid.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/pid.o: $(src)/pid.rs $(objtree)/rust/bindings/pid_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_pid_rust_lst_elf = LD      $@
      cmd_pid_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/pid.o: $(obj)/.rust-listing/pid.o FORCE
	$(call if_changed,pid_rust_lst_elf)
quiet_cmd_pid_rust_lst = MKLST   $@
      cmd_pid_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/pid.lst: $(obj)/.rust-listing-elf/pid.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,pid_rust_lst)
targets += pid.s pid.ll pid.rsi pid.lst \
	.rust-listing/pid.o .rust-listing-elf/pid.o
endif
