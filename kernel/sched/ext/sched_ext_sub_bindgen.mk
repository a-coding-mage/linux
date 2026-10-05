# SPDX-License-Identifier: GPL-2.0
# Source registration only. Shared canonical types must precede admission;
# separate configured projections do not establish cross-owner type identity.
ifeq ($(CONFIG_RUST_SCHED_EXT_SUB),y)
$(error SOURCE ONLY HOLD: Rust sched_ext sub bindings and BTF are unqualified)
endif
targets += bindings/sched_ext_sub_generated.rs
always-$(CONFIG_RUST_SCHED_EXT_SUB) += bindings/sched_ext_sub_generated.rs
$(obj)/bindings/sched_ext_sub_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/ext/sched_ext_sub_bindgen_parameters)
$(obj)/bindings/sched_ext_sub_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_ext_sub_generated.rs: $(srctree)/kernel/sched/ext/sched_ext_sub_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_shared_access.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_rescue_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_ecaps_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_caps_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_branch_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_lifecycle_bindings.h \
    $(srctree)/kernel/sched/ext/internal.h $(srctree)/kernel/sched/ext/sub.h \
    $(srctree)/kernel/sched/ext/cid.h $(srctree)/kernel/sched/ext/arena.h \
    $(srctree)/kernel/sched/ext/inlines.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_EXT_SUB),y)
$(obj)/bindings.o: $(obj)/bindings/sched_ext_sub_generated.rs
endif
