# SPDX-License-Identifier: GPL-2.0
# Source proposal only. The original fair.c bytes are not edited or excluded.
# An explicit Rust rule supersedes implicit C only for the admitted config.
ifeq ($(CONFIG_RUST_SCHED_FAIR),y)
ifneq ($(CONFIG_RUST_SCHED_FAIR_NATIVE_POLICY),y)
$(error Rust fair scheduler requires the integrated configured native-policy admission)
endif
include $(srctree)/scripts/Makefile.rust-native-policy
obj-y += fair_native.o
CONTEXT_ANALYSIS_fair_native.o := y
CFLAGS_fair_native.o += $(CFLAGS_fair.o)
RUSTFLAGS_fair.o += -Zfunction-sections=n
RUSTC_OUT_DIR_fair.o = $(dir $@).$(notdir $@).rustc/
fair-rust-inputs := $(src)/fair_foundation.rs $(src)/fair_tail.rs $(src)/fair_once.rs \
    $(objtree)/rust/bindings/sched_fair_generated.rs $(rust-native-policy-inputs)
$(obj)/fair.o $(obj)/fair.s $(obj)/fair.ll: private target-stem := fair
$(obj)/fair.o $(obj)/fair.s $(obj)/fair.ll: private part-of-builtin := y
$(obj)/fair.o $(obj)/fair.s $(obj)/fair.ll: private part-of-module :=
$(obj)/fair.o: $(src)/fair.rs $(fair-rust-inputs) FORCE
	+$(call if_changed_rule,$(if $(rust-native-policy),rust_native_o,rustc_o_rs))
$(obj)/fair.s: $(src)/fair.rs $(fair-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_s,rustc_s_rs))
$(obj)/fair.ll: $(src)/fair.rs $(fair-rust-inputs) FORCE
	+$(call if_changed_dep,$(if $(rust-native-policy),rust_native_ll,rustc_ll_rs))
# Directory KCOV/KCSAN/barrier/context and all security/compiler flags are
# inherited unchanged. This proposal never disables them to admit an owner.
endif
