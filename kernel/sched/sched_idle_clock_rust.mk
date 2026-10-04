# SPDX-License-Identifier: GPL-2.0-only
# Included at the END of kernel/sched/Makefile. Native C inclusions are gated
# separately by integration.patch. No original scheduler object is removed.
ifneq ($(filter y,$(CONFIG_RUST_SCHED_IDLE) $(CONFIG_RUST_SCHED_CLOCK)),)
# Preserve original per-function notrace/no-profile/init exemptions. These
# configurations need emitted native policy work; never silently instrument
# RCU-off paths or turn off the user's instrumentation to admit an owner.
ifeq ($(CONFIG_FUNCTION_TRACER),y)
$(error Rust scheduler idle/clock requires native notrace attribute mapping before FUNCTION_TRACER admission)
endif
ifneq ($(filter y,$(CONFIG_GCOV_KERNEL) $(CONFIG_AUTOFDO_CLANG) $(CONFIG_PROPELLER_CLANG)),)
$(error Rust scheduler idle/clock requires native no-profile attribute mapping before profiling admission)
endif
ifneq ($(filter y,$(CONFIG_GCC_PLUGIN_LATENT_ENTROPY) $(CONFIG_KSTACK_ERASE)),)
$(error Rust scheduler idle/clock requires native per-init instrumentation mapping before latent-entropy or stack-erase admission)
endif
include $(srctree)/scripts/Makefile.rust-native-policy
endif
# The original includes were first among these aggregate source modules.
# Keep their setup/initcall metadata ahead of the rest of each aggregate.
ifeq ($(CONFIG_RUST_SCHED_IDLE),y)
obj-y := $(patsubst build_policy.o,idle.o sched_idle_primitives.o build_policy.o,$(obj-y))
endif
ifeq ($(CONFIG_RUST_SCHED_CLOCK),y)
obj-y := $(patsubst build_utility.o,clock.o sched_clock_primitives.o build_utility.o,$(obj-y))
endif
# Retain the flags of the original aggregate translation-unit owners.
CFLAGS_idle.o += $(CFLAGS_build_policy.o)
CFLAGS_sched_idle_primitives.o += $(CFLAGS_build_policy.o)
CFLAGS_clock.o += $(CFLAGS_build_utility.o)
CFLAGS_sched_clock_primitives.o += $(CFLAGS_build_utility.o)
RUST_ALLOWED_FEATURES_idle.o += cfi_encoding,no_sanitize
RUST_ALLOWED_FEATURES_clock.o += cfi_encoding,no_sanitize

RUSTC_OUT_DIR_idle.o = $(dir $@).$(notdir $@).rustc/
RUSTC_OUT_DIR_clock.o = $(dir $@).$(notdir $@).rustc/
RUSTFLAGS_idle.o += -Zfunction-sections=n
RUSTFLAGS_clock.o += -Zfunction-sections=n

sched-idle-rust-inputs := $(src)/sched_idle_layout.rs \
    $(objtree)/rust/bindings/sched_idle_generated.rs $(rust-native-policy-inputs)
sched-clock-rust-inputs := \
    $(objtree)/rust/bindings/sched_clock_generated.rs $(rust-native-policy-inputs)

ifeq ($(CONFIG_RUST_SCHED_IDLE),y)
sched-idle-rust-targets := $(addprefix $(obj)/,idle.o idle.s idle.ll idle.rsi \
    .sched-idle-clock-listing/idle.o)
$(sched-idle-rust-targets): private target-stem := idle
$(sched-idle-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(sched-idle-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/idle.o,$(real-obj-y) $(lib-y)),y)
$(sched-idle-rust-targets): private part-of-module :=
$(obj)/idle.o: $(src)/idle.rs $(sched-idle-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/idle.s: $(src)/idle.rs $(sched-idle-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/idle.ll: $(src)/idle.rs $(sched-idle-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
$(obj)/idle.rsi: $(src)/idle.rs $(sched-idle-rust-inputs) FORCE
	+$(call if_changed_dep,sched_idle_clock_rsi)
$(obj)/.sched-idle-clock-listing/idle.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.sched-idle-clock-listing/idle.o: private override KBUILD_CFLAGS += -g
$(obj)/.sched-idle-clock-listing/idle.o: $(src)/idle.rs $(sched-idle-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/.sched-idle-clock-listing-elf/idle.o: $(obj)/.sched-idle-clock-listing/idle.o FORCE
	$(call if_changed,sched_idle_clock_lst_elf)
$(obj)/idle.lst: $(obj)/.sched-idle-clock-listing-elf/idle.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,sched_idle_clock_lst)
targets += idle.s idle.ll idle.rsi idle.lst \
    .sched-idle-clock-listing/idle.o .sched-idle-clock-listing-elf/idle.o
endif

ifeq ($(CONFIG_RUST_SCHED_CLOCK),y)
sched-clock-rust-targets := $(addprefix $(obj)/,clock.o clock.s clock.ll clock.rsi \
    .sched-idle-clock-listing/clock.o)
$(sched-clock-rust-targets): private target-stem := clock
$(sched-clock-rust-targets): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(sched-clock-rust-targets): private part-of-builtin = \
    $(if $(filter $(obj)/clock.o,$(real-obj-y) $(lib-y)),y)
$(sched-clock-rust-targets): private part-of-module :=
$(obj)/clock.o: $(src)/clock.rs $(sched-clock-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/clock.s: $(src)/clock.rs $(sched-clock-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/clock.ll: $(src)/clock.rs $(sched-clock-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
$(obj)/clock.rsi: $(src)/clock.rs $(sched-clock-rust-inputs) FORCE
	+$(call if_changed_dep,sched_idle_clock_rsi)
$(obj)/.sched-idle-clock-listing/clock.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.sched-idle-clock-listing/clock.o: private override KBUILD_CFLAGS += -g
$(obj)/.sched-idle-clock-listing/clock.o: $(src)/clock.rs $(sched-clock-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/.sched-idle-clock-listing-elf/clock.o: $(obj)/.sched-idle-clock-listing/clock.o FORCE
	$(call if_changed,sched_idle_clock_lst_elf)
$(obj)/clock.lst: $(obj)/.sched-idle-clock-listing-elf/clock.o \
    $(srctree)/scripts/makelst $(wildcard System.map) FORCE
	$(call if_changed,sched_idle_clock_lst)
targets += clock.s clock.ll clock.rsi clock.lst \
    .sched-idle-clock-listing/clock.o .sched-idle-clock-listing-elf/clock.o
endif

quiet_cmd_sched_idle_clock_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_sched_idle_clock_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
quiet_cmd_sched_idle_clock_lst_elf = LD      $@
      cmd_sched_idle_clock_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
quiet_cmd_sched_idle_clock_lst = MKLST   $@
      cmd_sched_idle_clock_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
clean-files += .sched-idle-clock-listing/ .sched-idle-clock-listing-elf/
clean-files += $(foreach ext,o s ll rsi,.idle.$(ext).rustc/ .clock.$(ext).rustc/)
