# SPDX-License-Identifier: GPL-2.0
# Retain owner admission and native header prerequisites. Only the canonical
# sched_ext_bindgen.mk owns a generated target or parameter input.
ifeq ($(CONFIG_RUST_SCHED_EXT_CORE),y)
$(error SOURCE ONLY HOLD: Rust sched_ext core bindings and BTF are unqualified)
endif

sched-ext-native-headers += \
    $(srctree)/kernel/sched/ext/sched_ext_core_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_shared_access.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_slice_cursor_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_task_lifetime_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_cgroup_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_object_lifetime_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_exit_dump_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_enqueue_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_consume_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_bypass_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_root_transition_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_placement_tick_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_structops_btf_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_query_events_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_pick_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_deferred_kick_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_kfunc_dispatch_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_kfunc_dsq_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_registration_init_bindings.h \
    $(srctree)/kernel/sched/ext/internal.h $(srctree)/kernel/sched/ext/cid.h \
    $(srctree)/kernel/sched/ext/arena.h $(srctree)/kernel/sched/ext/idle.h \
    $(srctree)/kernel/sched/ext/sub.h $(srctree)/kernel/sched/ext/inlines.h
