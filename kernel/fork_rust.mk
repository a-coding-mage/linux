ifeq ($(CONFIG_RUST_FORK),y)
include $(srctree)/scripts/Makefile.rust-native-policy
# Keep the original fork subsys_initcall before later kernel owners.
obj-y := $(patsubst fork.o,fork.o fork_helpers.o,$(obj-y))
RUST_ALLOWED_FEATURES_fork.o := linkage
RUSTFLAGS_fork.o += -Zfunction-sections=n
$(obj)/fork.o: $(src)/fork.rs $(objtree)/rust/bindings/fork_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))

# Explicit targets have no implicit stem; keep the canonical owner identity.
$(addprefix $(obj)/,fork.o fork.s fork.ll fork.rsi .rust-listing/fork.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))

# Inspection uses the selected source and its canonical per-object policy.
$(addprefix $(obj)/,fork.s fork.ll fork.rsi): private target-stem := fork
$(addprefix $(obj)/,fork.s fork.ll fork.rsi): private part-of-builtin = $(if $(filter $(obj)/fork.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,fork.s fork.ll fork.rsi): private part-of-module = $(if $(filter $(obj)/fork.o,$(real-obj-m)),y)
$(obj)/fork.s: $(src)/fork.rs $(objtree)/rust/bindings/fork_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/fork.ll: $(src)/fork.rs $(objtree)/rust/bindings/fork_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
$(obj)/fork.rsi: $(src)/fork.rs $(objtree)/rust/bindings/fork_generated.rs FORCE
	+$(call if_changed_dep,fork_rust_rsi)

quiet_cmd_fork_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_fork_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	$(RUSTFMT) $@

# Debug information belongs only to this unlinked listing object.
$(obj)/.rust-listing/fork.o: private target-stem := fork
$(obj)/.rust-listing/fork.o: private part-of-builtin = $(if $(filter $(obj)/fork.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/fork.o: private part-of-module = $(if $(filter $(obj)/fork.o,$(real-obj-m)),y)
$(obj)/.rust-listing/fork.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/fork.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/fork.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/fork.o: $(src)/fork.rs $(objtree)/rust/bindings/fork_generated.rs $(rust-native-policy-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
quiet_cmd_fork_rust_lst_elf = LD      $@
      cmd_fork_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/fork.o: $(obj)/.rust-listing/fork.o FORCE
	$(call if_changed,fork_rust_lst_elf)
# Keep makelst's real System.map behavior, including creation and deletion.
quiet_cmd_fork_rust_lst = MKLST   $@
      cmd_fork_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/fork.lst: $(obj)/.rust-listing-elf/fork.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,fork_rust_lst)
targets += fork.s fork.ll fork.rsi fork.lst \
	.rust-listing/fork.o .rust-listing-elf/fork.o
endif
