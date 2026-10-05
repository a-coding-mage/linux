/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_SUB_METADATA_H
#define LUPOS_SCHED_EXT_SUB_METADATA_H
#ifdef CONFIG_EXT_SUB_SCHED
bool lupos_scx_sub_dispatch(u64 cgroup_id, const struct bpf_prog_aux *aux);
#endif
s32 lupos_scx_sub_grant(u64 cgroup_id, u64 caps, const struct scx_cmask *cmask, struct scx_cmask *denied, const struct bpf_prog_aux *aux);
void lupos_scx_sub_revoke(u64 cgroup_id, u64 caps, const struct scx_cmask *cmask, const struct bpf_prog_aux *aux);
s32 lupos_scx_sub_caps(u64 cgroup_id, u64 caps, struct scx_cmask *out, const struct bpf_prog_aux *aux);
s32 lupos_scx_sub_kill_bstr(u64 cgroup_id, char *fmt, unsigned long long *data, u32 data__sz, const struct bpf_prog_aux *aux);
#endif
