// SPDX-License-Identifier: GPL-2.0
/* Native BPF names and annotations stay in the same ext policy translation
 * unit. ext.c owns registration/context-filter ID sets. This file owns no
 * registration list and contains no old sub.c algorithm or disabled fallback. */
#error "SOURCE ONLY HOLD: sched_ext sub BTF/CFI admission is pending"
#include "sched_ext_sub_bindings.h"
#include "sched_ext_sub_metadata.h"

__bpf_kfunc_start_defs();
#ifdef CONFIG_EXT_SUB_SCHED
__bpf_kfunc bool scx_bpf_sub_dispatch(u64 cgroup_id, const struct bpf_prog_aux *aux)
{
	return lupos_scx_sub_dispatch(cgroup_id, aux);
}
#endif
__bpf_kfunc s32 scx_bpf_sub_grant(u64 cgroup_id, u64 caps,
				  const struct scx_cmask *cmask__arena,
				  struct scx_cmask *denied_out__arena__nullable,
				  const struct bpf_prog_aux *aux)
{
	return lupos_scx_sub_grant(cgroup_id, caps, cmask__arena, denied_out__arena__nullable, aux);
}
__bpf_kfunc void scx_bpf_sub_revoke(u64 cgroup_id, u64 caps,
				    const struct scx_cmask *cmask__arena,
				    const struct bpf_prog_aux *aux)
{
	lupos_scx_sub_revoke(cgroup_id, caps, cmask__arena, aux);
}
__bpf_kfunc s32 scx_bpf_sub_caps(u64 cgroup_id, u64 caps, struct scx_cmask *out__arena,
				 const struct bpf_prog_aux *aux)
{
	return lupos_scx_sub_caps(cgroup_id, caps, out__arena, aux);
}
#ifdef CONFIG_EXT_SUB_SCHED
__printf(2, 0)
#endif
__bpf_kfunc s32 scx_bpf_sub_kill_bstr(u64 cgroup_id, char *fmt,
				      unsigned long long *data, u32 data__sz,
				      const struct bpf_prog_aux *aux)
{
	return lupos_scx_sub_kill_bstr(cgroup_id, fmt, data, data__sz, aux);
}
__bpf_kfunc_end_defs();
