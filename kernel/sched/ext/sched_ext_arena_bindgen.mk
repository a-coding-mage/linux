# SPDX-License-Identifier: GPL-2.0-only
# Retain owner admission and native header prerequisites. Only the canonical
# sched_ext_bindgen.mk owns a generated target or parameter input.
ifeq ($(CONFIG_RUST_SCHED_EXT_ARENA),y)
$(error SOURCE ONLY HOLD: Rust sched_ext arena bindings are unqualified)
endif

sched-ext-native-headers += \
    $(srctree)/kernel/sched/ext/sched_ext_arena_bindings.h \
    $(srctree)/kernel/sched/ext/internal.h $(srctree)/kernel/sched/ext/arena.h
