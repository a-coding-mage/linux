# SPDX-License-Identifier: GPL-2.0
# Retain owner admission and native header prerequisites. Only the canonical
# sched_ext_bindgen.mk owns a generated target or parameter input.
ifeq ($(CONFIG_RUST_SCHED_EXT_SUB),y)
$(error SOURCE ONLY HOLD: Rust sched_ext sub bindings and BTF are unqualified)
endif

sched-ext-native-headers += \
    $(srctree)/kernel/sched/ext/sched_ext_sub_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_shared_access.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_rescue_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_ecaps_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_caps_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_branch_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_sub_lifecycle_bindings.h \
    $(srctree)/kernel/sched/ext/internal.h $(srctree)/kernel/sched/ext/sub.h \
    $(srctree)/kernel/sched/ext/cid.h $(srctree)/kernel/sched/ext/arena.h \
    $(srctree)/kernel/sched/ext/inlines.h
