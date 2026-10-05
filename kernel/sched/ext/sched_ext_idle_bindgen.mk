# SPDX-License-Identifier: GPL-2.0
# Retain owner admission and native header prerequisites. Only the canonical
# sched_ext_bindgen.mk owns a generated target or parameter input.
ifeq ($(CONFIG_RUST_SCHED_EXT_IDLE),y)
$(error SOURCE ONLY HOLD: Rust sched_ext idle bindings and BTF are unqualified)
endif

sched-ext-native-headers += \
    $(srctree)/kernel/sched/ext/sched_ext_idle_bindings.h \
    $(srctree)/kernel/sched/ext/internal.h $(srctree)/kernel/sched/ext/idle.h \
    $(srctree)/kernel/sched/ext/cid.h $(srctree)/kernel/sched/ext/sub.h
