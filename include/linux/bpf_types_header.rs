/* SPDX-License-Identifier: GPL-2.0 */
/* Internal file - do not include directly. */

// These macro invocations are supplied by the surrounding BPF implementation.
// Configuration conditions mirror the original C preprocessor conditions.

#[cfg(feature = "CONFIG_NET")]
BPF_PROG_TYPE!(BPF_PROG_TYPE_SOCKET_FILTER, sk_filter, __sk_buff, sk_buff);
#[cfg(feature = "CONFIG_NET")]
BPF_PROG_TYPE!(BPF_PROG_TYPE_SCHED_CLS, tc_cls_act, __sk_buff, sk_buff);
#[cfg(feature = "CONFIG_NET")]
BPF_PROG_TYPE!(BPF_PROG_TYPE_SCHED_ACT, tc_cls_act, __sk_buff, sk_buff);
#[cfg(feature = "CONFIG_NET")]
BPF_PROG_TYPE!(BPF_PROG_TYPE_XDP, xdp, xdp_md, xdp_buff);

#[cfg(all(feature = "CONFIG_NET", feature = "CONFIG_CGROUP_BPF"))]
#[cfg(feature = "CONFIG_NET")]
#[cfg(feature = "CONFIG_CGROUP_BPF")]
BPF_PROG_TYPE!(BPF_PROG_TYPE_CGROUP_SKB, cg_skb, __sk_buff, sk_buff);
#[cfg(feature = "CONFIG_NET")]
#[cfg(feature = "CONFIG_CGROUP_BPF")]
BPF_PROG_TYPE!(BPF_PROG_TYPE_CGROUP_SOCK, cg_sock, bpf_sock, sock);
#[cfg(feature = "CONFIG_NET")]
#[cfg(feature = "CONFIG_CGROUP_BPF")]
BPF_PROG_TYPE!(BPF_PROG_TYPE_CGROUP_SOCK_ADDR, cg_sock_addr, bpf_sock_addr, bpf_sock_addr_kern);

#[cfg(feature = "CONFIG_NET")]
    BPF_PROG_TYPE!(BPF_PROG_TYPE_LWT_IN, lwt_in, __sk_buff, sk_buff);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_LWT_OUT, lwt_out, __sk_buff, sk_buff);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_LWT_XMIT, lwt_xmit, __sk_buff, sk_buff);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_LWT_SEG6LOCAL, lwt_seg6local, __sk_buff, sk_buff);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_SOCK_OPS, sock_ops, bpf_sock_ops, bpf_sock_ops_kern);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_SK_SKB, sk_skb, __sk_buff, sk_buff);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_SK_MSG, sk_msg, sk_msg_md, sk_msg);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_FLOW_DISSECTOR, flow_dissector, __sk_buff, bpf_flow_dissector);

#[cfg(feature = "CONFIG_BPF_EVENTS")]
    BPF_PROG_TYPE!(BPF_PROG_TYPE_KPROBE, kprobe, bpf_user_pt_regs_t, pt_regs);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_TRACEPOINT, tracepoint, __u64, u64);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_PERF_EVENT, perf_event, bpf_perf_event_data, bpf_perf_event_data_kern);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_RAW_TRACEPOINT, raw_tracepoint, bpf_raw_tracepoint_args, u64);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE, raw_tracepoint_writable, bpf_raw_tracepoint_args, u64);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_TRACING, tracing, *mut core::ffi::c_void, *mut core::ffi::c_void);

#[cfg(feature = "CONFIG_CGROUP_BPF")]
    BPF_PROG_TYPE!(BPF_PROG_TYPE_CGROUP_DEVICE, cg_dev, bpf_cgroup_dev_ctx, bpf_cgroup_dev_ctx);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_CGROUP_SYSCTL, cg_sysctl, bpf_sysctl, bpf_sysctl_kern);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_CGROUP_SOCKOPT, cg_sockopt, bpf_sockopt, bpf_sockopt_kern);

#[cfg(feature = "CONFIG_BPF_LIRC_MODE2")]
BPF_PROG_TYPE!(BPF_PROG_TYPE_LIRC_MODE2, lirc_mode2, __u32, u32);

#[cfg(feature = "CONFIG_INET")]
    BPF_PROG_TYPE!(BPF_PROG_TYPE_SK_REUSEPORT, sk_reuseport, sk_reuseport_md, sk_reuseport_kern);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_SK_LOOKUP, sk_lookup, bpf_sk_lookup, bpf_sk_lookup_kern);

#[cfg(feature = "CONFIG_BPF_JIT")]
    BPF_PROG_TYPE!(BPF_PROG_TYPE_STRUCT_OPS, bpf_struct_ops, *mut core::ffi::c_void, *mut core::ffi::c_void);
    BPF_PROG_TYPE!(BPF_PROG_TYPE_EXT, bpf_extension, *mut core::ffi::c_void, *mut core::ffi::c_void);
    #[cfg(feature = "CONFIG_BPF_LSM")]
    BPF_PROG_TYPE!(BPF_PROG_TYPE_LSM, lsm, *mut core::ffi::c_void, *mut core::ffi::c_void);

BPF_PROG_TYPE!(BPF_PROG_TYPE_SYSCALL, bpf_syscall, *mut core::ffi::c_void, *mut core::ffi::c_void);

#[cfg(feature = "CONFIG_NETFILTER_BPF_LINK")]
BPF_PROG_TYPE!(BPF_PROG_TYPE_NETFILTER, netfilter, bpf_nf_ctx, bpf_nf_ctx);

BPF_MAP_TYPE!(BPF_MAP_TYPE_ARRAY, array_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_PERCPU_ARRAY, percpu_array_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_PROG_ARRAY, prog_array_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_PERF_EVENT_ARRAY, perf_event_array_map_ops);
#[cfg(feature = "CONFIG_CGROUPS")]
    BPF_MAP_TYPE!(BPF_MAP_TYPE_CGROUP_ARRAY, cgroup_array_map_ops);
    BPF_MAP_TYPE!(BPF_MAP_TYPE_CGRP_STORAGE, cgrp_storage_map_ops);
#[cfg(feature = "CONFIG_CGROUP_BPF")]
    BPF_MAP_TYPE!(BPF_MAP_TYPE_CGROUP_STORAGE, cgroup_storage_map_ops);
    BPF_MAP_TYPE!(BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE, cgroup_storage_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_HASH, htab_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_PERCPU_HASH, htab_percpu_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_LRU_HASH, htab_lru_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_LRU_PERCPU_HASH, htab_lru_percpu_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_LPM_TRIE, trie_map_ops);
#[cfg(feature = "CONFIG_PERF_EVENTS")]
BPF_MAP_TYPE!(BPF_MAP_TYPE_STACK_TRACE, stack_trace_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_ARRAY_OF_MAPS, array_of_maps_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_HASH_OF_MAPS, htab_of_maps_map_ops);
#[cfg(feature = "CONFIG_BPF_LSM")]
BPF_MAP_TYPE!(BPF_MAP_TYPE_INODE_STORAGE, inode_storage_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_TASK_STORAGE, task_storage_map_ops);
#[cfg(feature = "CONFIG_NET")]
    BPF_MAP_TYPE!(BPF_MAP_TYPE_DEVMAP, dev_map_ops);
    BPF_MAP_TYPE!(BPF_MAP_TYPE_DEVMAP_HASH, dev_map_hash_ops);
    BPF_MAP_TYPE!(BPF_MAP_TYPE_SK_STORAGE, sk_storage_map_ops);
    BPF_MAP_TYPE!(BPF_MAP_TYPE_CPUMAP, cpu_map_ops);
    #[cfg(feature = "CONFIG_XDP_SOCKETS")]
    BPF_MAP_TYPE!(BPF_MAP_TYPE_XSKMAP, xsk_map_ops);
    #[cfg(feature = "CONFIG_INET")]
    {
        BPF_MAP_TYPE!(BPF_MAP_TYPE_SOCKMAP, sock_map_ops);
        BPF_MAP_TYPE!(BPF_MAP_TYPE_SOCKHASH, sock_hash_ops);
        BPF_MAP_TYPE!(BPF_MAP_TYPE_REUSEPORT_SOCKARRAY, reuseport_array_ops);
    }
BPF_MAP_TYPE!(BPF_MAP_TYPE_QUEUE, queue_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_STACK, stack_map_ops);
#[cfg(feature = "CONFIG_BPF_JIT")]
BPF_MAP_TYPE!(BPF_MAP_TYPE_STRUCT_OPS, bpf_struct_ops_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_RINGBUF, ringbuf_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_BLOOM_FILTER, bloom_filter_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_USER_RINGBUF, user_ringbuf_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_ARENA, arena_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_INSN_ARRAY, insn_array_map_ops);
BPF_MAP_TYPE!(BPF_MAP_TYPE_RHASH, rhtab_map_ops);

BPF_LINK_TYPE!(BPF_LINK_TYPE_RAW_TRACEPOINT, raw_tracepoint);
BPF_LINK_TYPE!(BPF_LINK_TYPE_TRACING, tracing);
#[cfg(feature = "CONFIG_CGROUP_BPF")]
BPF_LINK_TYPE!(BPF_LINK_TYPE_CGROUP, cgroup);
BPF_LINK_TYPE!(BPF_LINK_TYPE_ITER, iter);
#[cfg(feature = "CONFIG_NET")]
{
    BPF_LINK_TYPE!(BPF_LINK_TYPE_NETNS, netns);
    BPF_LINK_TYPE!(BPF_LINK_TYPE_XDP, xdp);
    BPF_LINK_TYPE!(BPF_LINK_TYPE_NETFILTER, netfilter);
    BPF_LINK_TYPE!(BPF_LINK_TYPE_TCX, tcx);
    BPF_LINK_TYPE!(BPF_LINK_TYPE_NETKIT, netkit);
    BPF_LINK_TYPE!(BPF_LINK_TYPE_SOCKMAP, sockmap);
}
#[cfg(feature = "CONFIG_PERF_EVENTS")]
BPF_LINK_TYPE!(BPF_LINK_TYPE_PERF_EVENT, perf);
BPF_LINK_TYPE!(BPF_LINK_TYPE_KPROBE_MULTI, kprobe_multi);
BPF_LINK_TYPE!(BPF_LINK_TYPE_STRUCT_OPS, struct_ops);
BPF_LINK_TYPE!(BPF_LINK_TYPE_UPROBE_MULTI, uprobe_multi);
BPF_LINK_TYPE!(BPF_LINK_TYPE_TRACING_MULTI, tracing_multi);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
