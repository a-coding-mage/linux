// SPDX-License-Identifier: GPL-2.0
/* Original annotation sites remain distinct; explicit unqualified runtime C. */
#error "SOURCE ONLY HOLD: sched_ext sub branch profiling admission is pending"
#include "sched_ext_sub_bindings.h"
#ifdef CONFIG_EXT_SUB_SCHED
bool lupos_scx_sub_likely_resolve_caps(bool value) { return likely(value); }
bool lupos_scx_sub_unlikely_forced_admit(bool value) { return unlikely(value); }
bool lupos_scx_sub_likely_rescue_enabled(bool value) { return likely(value); }
bool lupos_scx_sub_likely_reenq_caps(bool value) { return likely(value); }
bool lupos_scx_sub_likely_caps_deliver(bool value) { return likely(value); }
bool lupos_scx_sub_likely_sync_empty(bool value) { return likely(value); }
bool lupos_scx_sub_unlikely_sync_inactive(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_recursion(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_dispatch_parent(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_dispatch_child(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_dispatch_distant(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_preamble_parent(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_preamble_child(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_preamble_distant(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_preamble_caps(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_read_sched(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_read_caps(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_read_child(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_read_distant(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_read_uninitialized(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_kill_parent(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_kill_child(bool value) { return unlikely(value); }
bool lupos_scx_sub_unlikely_kill_distant(bool value) { return unlikely(value); }
#endif
