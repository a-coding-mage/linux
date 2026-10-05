/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_SUB_BRANCH_BINDINGS_H
#define LUPOS_SCHED_EXT_SUB_BRANCH_BINDINGS_H
/* Keep separate native profiling sites for each original sub.c branch. Actual
 * original source attribution remains an unqualified admission requirement. */
bool lupos_scx_sub_likely_resolve_caps(bool value);
bool lupos_scx_sub_unlikely_forced_admit(bool value);
bool lupos_scx_sub_likely_rescue_enabled(bool value);
bool lupos_scx_sub_likely_reenq_caps(bool value);
bool lupos_scx_sub_likely_caps_deliver(bool value);
bool lupos_scx_sub_likely_sync_empty(bool value);
bool lupos_scx_sub_unlikely_sync_inactive(bool value);
bool lupos_scx_sub_unlikely_recursion(bool value);
bool lupos_scx_sub_unlikely_dispatch_parent(bool value);
bool lupos_scx_sub_unlikely_dispatch_child(bool value);
bool lupos_scx_sub_unlikely_dispatch_distant(bool value);
bool lupos_scx_sub_unlikely_preamble_parent(bool value);
bool lupos_scx_sub_unlikely_preamble_child(bool value);
bool lupos_scx_sub_unlikely_preamble_distant(bool value);
bool lupos_scx_sub_unlikely_preamble_caps(bool value);
bool lupos_scx_sub_unlikely_read_sched(bool value);
bool lupos_scx_sub_unlikely_read_caps(bool value);
bool lupos_scx_sub_unlikely_read_child(bool value);
bool lupos_scx_sub_unlikely_read_distant(bool value);
bool lupos_scx_sub_unlikely_read_uninitialized(bool value);
bool lupos_scx_sub_unlikely_kill_parent(bool value);
bool lupos_scx_sub_unlikely_kill_child(bool value);
bool lupos_scx_sub_unlikely_kill_distant(bool value);
#endif
