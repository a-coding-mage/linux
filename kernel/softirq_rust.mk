# SPDX-License-Identifier: GPL-2.0-only
# Keep softirq.o at its original archive position and its early initcall
# metadata immediately adjacent. Never add the original C provider as well.
ifeq ($(CONFIG_RUST_SOFTIRQ),y)
softirq_rust_sources := $(addprefix $(src)/,softirq_bh.rs softirq_tasklet.rs softirq_storage.rs softirq_header.rs softirq_layout.rs)
KCOV_INSTRUMENT_softirq_helpers.o := n
KCSAN_SANITIZE_softirq_helpers.o := n
obj-y := $(patsubst softirq.o,softirq.o softirq_helpers.o,$(obj-y))
RUSTFLAGS_softirq.o += -Zfunction-sections=n
# Native caller-IP primitives in this object walk its current x86_64 frame.
# Match the top-level frame-pointer workaround on older rustc versions.
RUSTFLAGS_softirq.o += -Cforce-frame-pointers=y
ifndef CONFIG_FRAME_POINTER
RUSTFLAGS_softirq.o += $(if $(call rustc-min-version,109800),,-Zllvm_module_flag=frame-pointer:u32:2:max)
endif
RUST_ALLOWED_FEATURES_softirq.o += linkage
$(obj)/softirq.o: $(src)/softirq.rs $(softirq_rust_sources) $(objtree)/rust/bindings/softirq_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Explicit targets need the same stem, module ownership and per-object policy.
$(addprefix $(obj)/,softirq.o softirq.s softirq.ll softirq.rsi .rust-listing/softirq.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,softirq.s softirq.ll softirq.rsi): private target-stem := softirq
$(addprefix $(obj)/,softirq.s softirq.ll softirq.rsi): private part-of-builtin = $(if $(filter $(obj)/softirq.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,softirq.s softirq.ll softirq.rsi): private part-of-module = $(if $(filter $(obj)/softirq.o,$(real-obj-m)),y)
$(obj)/softirq.s: $(src)/softirq.rs $(softirq_rust_sources) $(objtree)/rust/bindings/softirq_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/softirq.ll: $(src)/softirq.rs $(softirq_rust_sources) $(objtree)/rust/bindings/softirq_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
quiet_cmd_softirq_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_softirq_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/softirq.rsi: $(src)/softirq.rs $(softirq_rust_sources) $(objtree)/rust/bindings/softirq_generated.rs FORCE
	+$(call if_changed_dep,softirq_rust_rsi)

# Dedicated unlinked object for source-interleaved listing, including LTO.
$(obj)/.rust-listing/softirq.o: private target-stem := softirq
$(obj)/.rust-listing/softirq.o: private part-of-builtin = $(if $(filter $(obj)/softirq.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/softirq.o: private part-of-module = $(if $(filter $(obj)/softirq.o,$(real-obj-m)),y)
$(obj)/.rust-listing/softirq.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/softirq.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/softirq.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/softirq.o: $(src)/softirq.rs $(softirq_rust_sources) $(objtree)/rust/bindings/softirq_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_softirq_rust_lst_elf = LD      $@
      cmd_softirq_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/softirq.o: $(obj)/.rust-listing/softirq.o FORCE
	$(call if_changed,softirq_rust_lst_elf)
quiet_cmd_softirq_rust_lst = MKLST   $@
      cmd_softirq_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/softirq.lst: $(obj)/.rust-listing-elf/softirq.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,softirq_rust_lst)
targets += softirq.s softirq.ll softirq.rsi softirq.lst \
	.rust-listing/softirq.o .rust-listing-elf/softirq.o
endif
