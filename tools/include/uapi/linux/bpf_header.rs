/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of the isolated C UAPI header ./bpf.h. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

pub type __u8 = u8;
pub type __s16 = i16;
pub type __s32 = i32;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __aligned_u64 = u64;
pub type c_char = i8;

/* Extended instruction set based on top of classic BPF */

pub const BPF_JMP32: u32 = 0x06; /* jmp mode in word width */
pub const BPF_ALU64: u32 = 0x07; /* alu mode in double word width */
pub const BPF_DW: u32 = 0x18; /* double word (64-bit) */
pub const BPF_MEMSX: u32 = 0x80; /* load with sign extension */
pub const BPF_ATOMIC: u32 = 0xc0; /* atomic memory ops - op type in immediate */
pub const BPF_XADD: u32 = 0xc0; /* exclusive add - legacy name */
pub const BPF_MOV: u32 = 0xb0; /* mov reg to reg */
pub const BPF_ARSH: u32 = 0xc0; /* sign extending arithmetic shift right */
pub const BPF_END: u32 = 0xd0; /* flags for endianness conversion */
pub const BPF_TO_LE: u32 = 0x00;
pub const BPF_TO_BE: u32 = 0x08;
pub const BPF_FROM_LE: u32 = BPF_TO_LE;
pub const BPF_FROM_BE: u32 = BPF_TO_BE;
pub const BPF_JNE: u32 = 0x50;
pub const BPF_JLT: u32 = 0xa0;
pub const BPF_JLE: u32 = 0xb0;
pub const BPF_JSGT: u32 = 0x60;
pub const BPF_JSGE: u32 = 0x70;
pub const BPF_JSLT: u32 = 0xc0;
pub const BPF_JSLE: u32 = 0xd0;
pub const BPF_JCOND: u32 = 0xe0;
pub const BPF_CALL: u32 = 0x80;
pub const BPF_EXIT: u32 = 0x90;
pub const BPF_FETCH: u32 = 0x01;
pub const BPF_XCHG: u32 = 0xe0 | BPF_FETCH;
pub const BPF_CMPXCHG: u32 = 0xf0 | BPF_FETCH;
pub const BPF_LOAD_ACQ: u32 = 0x100;
pub const BPF_STORE_REL: u32 = 0x110;

/* enum bpf_cond_pseudo_jmp */
pub const BPF_MAY_GOTO: u32 = (0) as u32;

/* enum anonymous */
pub const BPF_REG_0: u32 = (0) as u32;
pub const BPF_REG_1: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_REG_2: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_REG_3: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_REG_4: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_REG_5: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_REG_6: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_REG_7: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_REG_8: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_REG_9: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_REG_10: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const __MAX_BPF_REG: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */

/* enum bpf_cgroup_iter_order */
pub const BPF_CGROUP_ITER_ORDER_UNSPEC: u32 = (0) as u32;
pub const BPF_CGROUP_ITER_SELF_ONLY: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_CGROUP_ITER_DESCENDANTS_PRE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_CGROUP_ITER_DESCENDANTS_POST: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_CGROUP_ITER_ANCESTORS_UP: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */

/* enum bpf_cmd */
pub const BPF_MAP_CREATE: u32 = 0;
pub const BPF_MAP_LOOKUP_ELEM: u32 = 1;
pub const BPF_MAP_UPDATE_ELEM: u32 = 2;
pub const BPF_MAP_DELETE_ELEM: u32 = 3;
pub const BPF_MAP_GET_NEXT_KEY: u32 = 4;
pub const BPF_PROG_LOAD: u32 = 5;
pub const BPF_OBJ_PIN: u32 = 6;
pub const BPF_OBJ_GET: u32 = 7;
pub const BPF_PROG_ATTACH: u32 = 8;
pub const BPF_PROG_DETACH: u32 = 9;
pub const BPF_PROG_TEST_RUN: u32 = 10;
pub const BPF_PROG_RUN: u32 = (BPF_PROG_TEST_RUN) as u32;
pub const BPF_PROG_GET_NEXT_ID: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_GET_NEXT_ID: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_PROG_GET_FD_BY_ID: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_GET_FD_BY_ID: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_OBJ_GET_INFO_BY_FD: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_PROG_QUERY: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_RAW_TRACEPOINT_OPEN: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_BTF_LOAD: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_BTF_GET_FD_BY_ID: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TASK_FD_QUERY: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_LOOKUP_AND_DELETE_ELEM: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_FREEZE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_BTF_GET_NEXT_ID: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_LOOKUP_BATCH: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_LOOKUP_AND_DELETE_BATCH: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_UPDATE_BATCH: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_DELETE_BATCH: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_LINK_CREATE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_LINK_UPDATE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_LINK_GET_FD_BY_ID: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_LINK_GET_NEXT_ID: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_ENABLE_STATS: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_ITER_CREATE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_LINK_DETACH: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_PROG_BIND_MAP: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TOKEN_CREATE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_PROG_STREAM_READ_BY_FD: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_PROG_ASSOC_STRUCT_OPS: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const __MAX_BPF_CMD: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_COMMON_ATTRS: u32 = (1 << 16) as u32;

/* enum bpf_map_type */
pub const BPF_MAP_TYPE_UNSPEC: u32 = 0;
pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;
pub const BPF_MAP_TYPE_PERF_EVENT_ARRAY: u32 = 4;
pub const BPF_MAP_TYPE_PERCPU_HASH: u32 = 5;
pub const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 6;
pub const BPF_MAP_TYPE_STACK_TRACE: u32 = 7;
pub const BPF_MAP_TYPE_CGROUP_ARRAY: u32 = 8;
pub const BPF_MAP_TYPE_LRU_HASH: u32 = 9;
pub const BPF_MAP_TYPE_LRU_PERCPU_HASH: u32 = 10;
pub const BPF_MAP_TYPE_LPM_TRIE: u32 = 11;
pub const BPF_MAP_TYPE_ARRAY_OF_MAPS: u32 = 12;
pub const BPF_MAP_TYPE_HASH_OF_MAPS: u32 = 13;
pub const BPF_MAP_TYPE_DEVMAP: u32 = 14;
pub const BPF_MAP_TYPE_SOCKMAP: u32 = 15;
pub const BPF_MAP_TYPE_CPUMAP: u32 = 16;
pub const BPF_MAP_TYPE_XSKMAP: u32 = 17;
pub const BPF_MAP_TYPE_SOCKHASH: u32 = 18;
pub const BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED: u32 = 19;
pub const BPF_MAP_TYPE_CGROUP_STORAGE: u32 = (BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED) as u32;
pub const BPF_MAP_TYPE_REUSEPORT_SOCKARRAY: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE_DEPRECATED: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE: u32 = (BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE_DEPRECATED) as u32;
pub const BPF_MAP_TYPE_QUEUE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_STACK: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_SK_STORAGE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_DEVMAP_HASH: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_STRUCT_OPS: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_RINGBUF: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_INODE_STORAGE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_TASK_STORAGE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_BLOOM_FILTER: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_USER_RINGBUF: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_CGRP_STORAGE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_ARENA: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_INSN_ARRAY: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_MAP_TYPE_RHASH: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const __MAX_BPF_MAP_TYPE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */

/* enum bpf_prog_type */
pub const BPF_PROG_TYPE_UNSPEC: u32 = 0;
pub const BPF_PROG_TYPE_SOCKET_FILTER: u32 = 1;
pub const BPF_PROG_TYPE_KPROBE: u32 = 2;
pub const BPF_PROG_TYPE_SCHED_CLS: u32 = 3;
pub const BPF_PROG_TYPE_SCHED_ACT: u32 = 4;
pub const BPF_PROG_TYPE_TRACEPOINT: u32 = 5;
pub const BPF_PROG_TYPE_XDP: u32 = 6;
pub const BPF_PROG_TYPE_PERF_EVENT: u32 = 7;
pub const BPF_PROG_TYPE_CGROUP_SKB: u32 = 8;
pub const BPF_PROG_TYPE_CGROUP_SOCK: u32 = 9;
pub const BPF_PROG_TYPE_LWT_IN: u32 = 10;
pub const BPF_PROG_TYPE_LWT_OUT: u32 = 11;
pub const BPF_PROG_TYPE_LWT_XMIT: u32 = 12;
pub const BPF_PROG_TYPE_SOCK_OPS: u32 = 13;
pub const BPF_PROG_TYPE_SK_SKB: u32 = 14;
pub const BPF_PROG_TYPE_CGROUP_DEVICE: u32 = 15;
pub const BPF_PROG_TYPE_SK_MSG: u32 = 16;
pub const BPF_PROG_TYPE_RAW_TRACEPOINT: u32 = 17;
pub const BPF_PROG_TYPE_CGROUP_SOCK_ADDR: u32 = 18;
pub const BPF_PROG_TYPE_LWT_SEG6LOCAL: u32 = 19;
pub const BPF_PROG_TYPE_LIRC_MODE2: u32 = 20;
pub const BPF_PROG_TYPE_SK_REUSEPORT: u32 = 21;
pub const BPF_PROG_TYPE_FLOW_DISSECTOR: u32 = 22;
pub const BPF_PROG_TYPE_CGROUP_SYSCTL: u32 = 23;
pub const BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE: u32 = 24;
pub const BPF_PROG_TYPE_CGROUP_SOCKOPT: u32 = 25;
pub const BPF_PROG_TYPE_TRACING: u32 = 26;
pub const BPF_PROG_TYPE_STRUCT_OPS: u32 = 27;
pub const BPF_PROG_TYPE_EXT: u32 = 28;
pub const BPF_PROG_TYPE_LSM: u32 = 29;
pub const BPF_PROG_TYPE_SK_LOOKUP: u32 = 30;
pub const BPF_PROG_TYPE_SYSCALL: u32 = 31;
pub const BPF_PROG_TYPE_NETFILTER: u32 = 32;
pub const __MAX_BPF_PROG_TYPE: u32 = 33;

/* enum bpf_attach_type */
pub const BPF_CGROUP_INET_INGRESS: u32 = 0;
pub const BPF_CGROUP_INET_EGRESS: u32 = 1;
pub const BPF_CGROUP_INET_SOCK_CREATE: u32 = 2;
pub const BPF_CGROUP_SOCK_OPS: u32 = 3;
pub const BPF_SK_SKB_STREAM_PARSER: u32 = 4;
pub const BPF_SK_SKB_STREAM_VERDICT: u32 = 5;
pub const BPF_CGROUP_DEVICE: u32 = 6;
pub const BPF_SK_MSG_VERDICT: u32 = 7;
pub const BPF_CGROUP_INET4_BIND: u32 = 8;
pub const BPF_CGROUP_INET6_BIND: u32 = 9;
pub const BPF_CGROUP_INET4_CONNECT: u32 = 10;
pub const BPF_CGROUP_INET6_CONNECT: u32 = 11;
pub const BPF_CGROUP_INET4_POST_BIND: u32 = 12;
pub const BPF_CGROUP_INET6_POST_BIND: u32 = 13;
pub const BPF_CGROUP_UDP4_SENDMSG: u32 = 14;
pub const BPF_CGROUP_UDP6_SENDMSG: u32 = 15;
pub const BPF_LIRC_MODE2: u32 = 16;
pub const BPF_FLOW_DISSECTOR: u32 = 17;
pub const BPF_CGROUP_SYSCTL: u32 = 18;
pub const BPF_CGROUP_UDP4_RECVMSG: u32 = 19;
pub const BPF_CGROUP_UDP6_RECVMSG: u32 = 20;
pub const BPF_CGROUP_GETSOCKOPT: u32 = 21;
pub const BPF_CGROUP_SETSOCKOPT: u32 = 22;
pub const BPF_TRACE_RAW_TP: u32 = 23;
pub const BPF_TRACE_FENTRY: u32 = 24;
pub const BPF_TRACE_FEXIT: u32 = 25;
pub const BPF_MODIFY_RETURN: u32 = 26;
pub const BPF_LSM_MAC: u32 = 27;
pub const BPF_TRACE_ITER: u32 = 28;
pub const BPF_CGROUP_INET4_GETPEERNAME: u32 = 29;
pub const BPF_CGROUP_INET6_GETPEERNAME: u32 = 30;
pub const BPF_CGROUP_INET4_GETSOCKNAME: u32 = 31;
pub const BPF_CGROUP_INET6_GETSOCKNAME: u32 = 32;
pub const BPF_XDP_DEVMAP: u32 = 33;
pub const BPF_CGROUP_INET_SOCK_RELEASE: u32 = 34;
pub const BPF_XDP_CPUMAP: u32 = 35;
pub const BPF_SK_LOOKUP: u32 = 36;
pub const BPF_XDP: u32 = 37;
pub const BPF_SK_SKB_VERDICT: u32 = 38;
pub const BPF_SK_REUSEPORT_SELECT: u32 = 39;
pub const BPF_SK_REUSEPORT_SELECT_OR_MIGRATE: u32 = 40;
pub const BPF_PERF_EVENT: u32 = 41;
pub const BPF_TRACE_KPROBE_MULTI: u32 = 42;
pub const BPF_LSM_CGROUP: u32 = 43;
pub const BPF_STRUCT_OPS: u32 = 44;
pub const BPF_NETFILTER: u32 = 45;
pub const BPF_TCX_INGRESS: u32 = 46;
pub const BPF_TCX_EGRESS: u32 = 47;
pub const BPF_TRACE_UPROBE_MULTI: u32 = 48;
pub const BPF_CGROUP_UNIX_CONNECT: u32 = 49;
pub const BPF_CGROUP_UNIX_SENDMSG: u32 = 50;
pub const BPF_CGROUP_UNIX_RECVMSG: u32 = 51;
pub const BPF_CGROUP_UNIX_GETPEERNAME: u32 = 52;
pub const BPF_CGROUP_UNIX_GETSOCKNAME: u32 = 53;
pub const BPF_NETKIT_PRIMARY: u32 = 54;
pub const BPF_NETKIT_PEER: u32 = 55;
pub const BPF_TRACE_KPROBE_SESSION: u32 = 56;
pub const BPF_TRACE_UPROBE_SESSION: u32 = 57;
pub const BPF_TRACE_FSESSION: u32 = 58;
pub const BPF_TRACE_FENTRY_MULTI: u32 = 59;
pub const BPF_TRACE_FEXIT_MULTI: u32 = 60;
pub const BPF_TRACE_FSESSION_MULTI: u32 = 61;
pub const __MAX_BPF_ATTACH_TYPE: u32 = 62;

/* enum bpf_link_type */
pub const BPF_LINK_TYPE_UNSPEC: u32 = (0) as u32;
pub const BPF_LINK_TYPE_RAW_TRACEPOINT: u32 = (1) as u32;
pub const BPF_LINK_TYPE_TRACING: u32 = (2) as u32;
pub const BPF_LINK_TYPE_CGROUP: u32 = (3) as u32;
pub const BPF_LINK_TYPE_ITER: u32 = (4) as u32;
pub const BPF_LINK_TYPE_NETNS: u32 = (5) as u32;
pub const BPF_LINK_TYPE_XDP: u32 = (6) as u32;
pub const BPF_LINK_TYPE_PERF_EVENT: u32 = (7) as u32;
pub const BPF_LINK_TYPE_KPROBE_MULTI: u32 = (8) as u32;
pub const BPF_LINK_TYPE_STRUCT_OPS: u32 = (9) as u32;
pub const BPF_LINK_TYPE_NETFILTER: u32 = (10) as u32;
pub const BPF_LINK_TYPE_TCX: u32 = (11) as u32;
pub const BPF_LINK_TYPE_UPROBE_MULTI: u32 = (12) as u32;
pub const BPF_LINK_TYPE_NETKIT: u32 = (13) as u32;
pub const BPF_LINK_TYPE_SOCKMAP: u32 = (14) as u32;
pub const BPF_LINK_TYPE_TRACING_MULTI: u32 = (15) as u32;
pub const __MAX_BPF_LINK_TYPE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */

/* enum bpf_perf_event_type */
pub const BPF_PERF_EVENT_UNSPEC: u32 = (0) as u32;
pub const BPF_PERF_EVENT_UPROBE: u32 = (1) as u32;
pub const BPF_PERF_EVENT_URETPROBE: u32 = (2) as u32;
pub const BPF_PERF_EVENT_KPROBE: u32 = (3) as u32;
pub const BPF_PERF_EVENT_KRETPROBE: u32 = (4) as u32;
pub const BPF_PERF_EVENT_TRACEPOINT: u32 = (5) as u32;
pub const BPF_PERF_EVENT_EVENT: u32 = (6) as u32;

/* enum anonymous */
pub const BPF_F_KPROBE_MULTI_RETURN: u32 = ((1u32 << 0)) as u32;

/* enum anonymous */
pub const BPF_F_UPROBE_MULTI_RETURN: u32 = ((1u32 << 0)) as u32;
pub const BPF_F_UPROBE_MULTI_PATH_FD: u32 = ((1u32 << 1)) as u32;

/* enum bpf_addr_space_cast */
pub const BPF_ADDR_SPACE_CAST: u32 = (1) as u32;

/* enum anonymous */
pub const BPF_ANY: u32 = (0) as u32;
pub const BPF_NOEXIST: u32 = (1) as u32;
pub const BPF_EXIST: u32 = (2) as u32;
pub const BPF_F_LOCK: u32 = (4) as u32;
pub const BPF_F_CPU: u32 = (8) as u32;

/* enum anonymous */
pub const BPF_F_NO_PREALLOC: u32 = ((1u32 << 0)) as u32;
pub const BPF_F_NUMA_NODE: u32 = ((1u32 << 2)) as u32;
pub const BPF_F_RDONLY: u32 = ((1u32 << 3)) as u32;
pub const BPF_F_WRONLY: u32 = ((1u32 << 4)) as u32;
pub const BPF_F_ZERO_SEED: u32 = ((1u32 << 6)) as u32;
pub const BPF_F_RDONLY_PROG: u32 = ((1u32 << 7)) as u32;
pub const BPF_F_WRONLY_PROG: u32 = ((1u32 << 8)) as u32;
pub const BPF_F_CLONE: u32 = ((1u32 << 9)) as u32;
pub const BPF_F_MMAPABLE: u32 = ((1u32 << 10)) as u32;
pub const BPF_F_PRESERVE_ELEMS: u32 = ((1u32 << 11)) as u32;
pub const BPF_F_INNER_MAP: u32 = ((1u32 << 12)) as u32;
pub const BPF_F_LINK: u32 = ((1u32 << 13)) as u32;
pub const BPF_F_PATH_FD: u32 = ((1u32 << 14)) as u32;
pub const BPF_F_TOKEN_FD: u32 = ((1u32 << 16)) as u32;
pub const BPF_F_SEGV_ON_FAULT: u32 = ((1u32 << 17)) as u32;
pub const BPF_F_NO_USER_CONV: u32 = ((1u32 << 18)) as u32;
pub const BPF_F_RB_OVERWRITE: u32 = ((1u32 << 19)) as u32;

/* enum bpf_stats_type */
pub const BPF_STATS_RUN_TIME: u32 = (0) as u32;

/* enum bpf_stack_build_id_status */
pub const BPF_STACK_BUILD_ID_EMPTY: u32 = (0) as u32;
pub const BPF_STACK_BUILD_ID_VALID: u32 = (1) as u32;

/* enum anonymous */
pub const BPF_STREAM_STDOUT: u32 = (1) as u32;
pub const BPF_STREAM_STDERR: u32 = (2) as u32;

/* enum bpf_func_id */

/* enum anonymous */
pub const BPF_F_RECOMPUTE_CSUM: u32 = ((1ULL << 0)) as u32;
pub const BPF_F_INVALIDATE_HASH: u32 = ((1ULL << 1)) as u32;

/* enum anonymous */
pub const BPF_F_HDR_FIELD_MASK: u32 = (0xfULL) as u32;

/* enum anonymous */
pub const BPF_F_PSEUDO_HDR: u32 = ((1ULL << 4)) as u32;
pub const BPF_F_MARK_MANGLED_0: u32 = ((1ULL << 5)) as u32;
pub const BPF_F_MARK_ENFORCE: u32 = ((1ULL << 6)) as u32;
pub const BPF_F_IPV6: u32 = ((1ULL << 7)) as u32;

/* enum anonymous */
pub const BPF_F_TUNINFO_IPV6: u32 = ((1ULL << 0)) as u32;

/* enum anonymous */
pub const BPF_F_SKIP_FIELD_MASK: u32 = (0xffULL) as u32;
pub const BPF_F_USER_STACK: u32 = ((1ULL << 8)) as u32;
pub const BPF_F_FAST_STACK_CMP: u32 = ((1ULL << 9)) as u32;
pub const BPF_F_REUSE_STACKID: u32 = ((1ULL << 10)) as u32;
pub const BPF_F_USER_BUILD_ID: u32 = ((1ULL << 11)) as u32;

/* enum anonymous */
pub const BPF_F_ZERO_CSUM_TX: u32 = ((1ULL << 1)) as u32;
pub const BPF_F_DONT_FRAGMENT: u32 = ((1ULL << 2)) as u32;
pub const BPF_F_SEQ_NUMBER: u32 = ((1ULL << 3)) as u32;
pub const BPF_F_NO_TUNNEL_KEY: u32 = ((1ULL << 4)) as u32;

/* enum anonymous */
pub const BPF_F_TUNINFO_FLAGS: u32 = ((1ULL << 4)) as u32;

/* enum anonymous */
pub const BPF_F_INDEX_MASK: u32 = (0xffffffffULL) as u32;
pub const BPF_F_CURRENT_CPU: u32 = (BPF_F_INDEX_MASK) as u32;
pub const BPF_F_CTXLEN_MASK: u32 = ((0xfffffULL << 32)) as u32;

/* enum anonymous */
pub const BPF_F_CURRENT_NETNS: u32 = ((-1L)) as u32;

/* enum anonymous */
pub const BPF_CSUM_LEVEL_QUERY: u32 = 0;
pub const BPF_CSUM_LEVEL_INC: u32 = 1;
pub const BPF_CSUM_LEVEL_DEC: u32 = 2;
pub const BPF_CSUM_LEVEL_RESET: u32 = 3;

/* enum bpf_adj_room_flags */
pub const BPF_F_ADJ_ROOM_FIXED_GSO: u32 = ((1ULL << 0)) as u32;
pub const BPF_F_ADJ_ROOM_ENCAP_L3_IPV4: u32 = ((1ULL << 1)) as u32;
pub const BPF_F_ADJ_ROOM_ENCAP_L3_IPV6: u32 = ((1ULL << 2)) as u32;
pub const BPF_F_ADJ_ROOM_ENCAP_L4_GRE: u32 = ((1ULL << 3)) as u32;
pub const BPF_F_ADJ_ROOM_ENCAP_L4_UDP: u32 = ((1ULL << 4)) as u32;
pub const BPF_F_ADJ_ROOM_NO_CSUM_RESET: u32 = ((1ULL << 5)) as u32;
pub const BPF_F_ADJ_ROOM_ENCAP_L2_ETH: u32 = ((1ULL << 6)) as u32;
pub const BPF_F_ADJ_ROOM_DECAP_L3_IPV4: u32 = ((1ULL << 7)) as u32;
pub const BPF_F_ADJ_ROOM_DECAP_L3_IPV6: u32 = ((1ULL << 8)) as u32;
pub const BPF_F_ADJ_ROOM_DECAP_L4_GRE: u32 = ((1ULL << 9)) as u32;
pub const BPF_F_ADJ_ROOM_DECAP_L4_UDP: u32 = ((1ULL << 10)) as u32;
pub const BPF_F_ADJ_ROOM_DECAP_IPXIP4: u32 = ((1ULL << 11)) as u32;
pub const BPF_F_ADJ_ROOM_DECAP_IPXIP6: u32 = ((1ULL << 12)) as u32;

/* enum anonymous */
pub const BPF_ADJ_ROOM_ENCAP_L2_MASK: u32 = (0xff) as u32;
pub const BPF_ADJ_ROOM_ENCAP_L2_SHIFT: u32 = (56) as u32;

/* enum anonymous */
pub const BPF_F_SYSCTL_BASE_NAME: u32 = ((1ULL << 0)) as u32;

/* enum anonymous */
pub const BPF_LOCAL_STORAGE_GET_F_CREATE: u32 = ((1ULL << 0)) as u32;
pub const BPF_SK_STORAGE_GET_F_CREATE: u32 = (BPF_LOCAL_STORAGE_GET_F_CREATE) as u32;

/* enum anonymous */
pub const BPF_F_GET_BRANCH_RECORDS_SIZE: u32 = ((1ULL << 0)) as u32;

/* enum anonymous */
pub const BPF_RB_NO_WAKEUP: u32 = ((1ULL << 0)) as u32;
pub const BPF_RB_FORCE_WAKEUP: u32 = ((1ULL << 1)) as u32;

/* enum anonymous */
pub const BPF_RB_AVAIL_DATA: u32 = (0) as u32;
pub const BPF_RB_RING_SIZE: u32 = (1) as u32;
pub const BPF_RB_CONS_POS: u32 = (2) as u32;
pub const BPF_RB_PROD_POS: u32 = (3) as u32;
pub const BPF_RB_OVERWRITE_POS: u32 = (4) as u32;

/* enum anonymous */
pub const BPF_RINGBUF_BUSY_BIT: u32 = ((1u32 << 31)) as u32;
pub const BPF_RINGBUF_DISCARD_BIT: u32 = ((1u32 << 30)) as u32;
pub const BPF_RINGBUF_HDR_SZ: u32 = (8) as u32;

/* enum anonymous */
pub const BPF_SK_LOOKUP_F_REPLACE: u32 = ((1ULL << 0)) as u32;
pub const BPF_SK_LOOKUP_F_NO_REUSEPORT: u32 = ((1ULL << 1)) as u32;

/* enum bpf_adj_room_mode */
pub const BPF_ADJ_ROOM_NET: u32 = 0;
pub const BPF_ADJ_ROOM_MAC: u32 = 1;

/* enum bpf_hdr_start_off */
pub const BPF_HDR_START_MAC: u32 = 0;
pub const BPF_HDR_START_NET: u32 = 1;

/* enum bpf_lwt_encap_mode */
pub const BPF_LWT_ENCAP_SEG6: u32 = 0;
pub const BPF_LWT_ENCAP_SEG6_INLINE: u32 = 1;
pub const BPF_LWT_ENCAP_IP: u32 = 2;

/* enum anonymous */
pub const BPF_F_BPRM_SECUREEXEC: u32 = ((1ULL << 0)) as u32;

/* enum anonymous */
pub const BPF_F_INGRESS: u32 = ((1ULL << 0)) as u32;
pub const BPF_F_EGRESS: u32 = ((1ULL << 1)) as u32;
pub const BPF_F_BROADCAST: u32 = ((1ULL << 3)) as u32;
pub const BPF_F_EXCLUDE_INGRESS: u32 = ((1ULL << 4)) as u32;

/* enum anonymous */
pub const BPF_SKB_TSTAMP_UNSPEC: u32 = (0) as u32;
pub const BPF_SKB_TSTAMP_DELIVERY_MONO: u32 = (1) as u32;
pub const BPF_SKB_CLOCK_REALTIME: u32 = (0) as u32;
pub const BPF_SKB_CLOCK_MONOTONIC: u32 = (1) as u32;
pub const BPF_SKB_CLOCK_TAI: u32 = (2) as u32;

/* enum bpf_ret_code */
pub const BPF_OK: u32 = (0) as u32;
pub const BPF_DROP: u32 = (2) as u32;
pub const BPF_REDIRECT: u32 = (7) as u32;

/* enum tcx_action_base */
pub const TCX_NEXT: u32 = (-1) as u32;
pub const TCX_PASS: u32 = (0) as u32;
pub const TCX_DROP: u32 = (2) as u32;
pub const TCX_REDIRECT: u32 = (7) as u32;

/* enum xdp_action */
pub const XDP_ABORTED: u32 = (0) as u32;
pub const XDP_DROP: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const XDP_PASS: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const XDP_TX: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const XDP_REDIRECT: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */

/* enum sk_action */
pub const SK_DROP: u32 = (0) as u32;
pub const SK_PASS: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */

/* enum anonymous */
pub const BPF_SOCK_OPS_RTO_CB_FLAG: u32 = ((1<<0)) as u32;
pub const BPF_SOCK_OPS_RETRANS_CB_FLAG: u32 = ((1<<1)) as u32;
pub const BPF_SOCK_OPS_STATE_CB_FLAG: u32 = ((1<<2)) as u32;
pub const BPF_SOCK_OPS_RTT_CB_FLAG: u32 = ((1<<3)) as u32;
pub const BPF_SOCK_OPS_PARSE_ALL_HDR_OPT_CB_FLAG: u32 = ((1<<4)) as u32;
pub const BPF_SOCK_OPS_PARSE_UNKNOWN_HDR_OPT_CB_FLAG: u32 = ((1<<5)) as u32;
pub const BPF_SOCK_OPS_WRITE_HDR_OPT_CB_FLAG: u32 = ((1<<6)) as u32;
pub const BPF_SOCK_OPS_ALL_CB_FLAGS: u32 = (0x7F) as u32;

/* enum anonymous */
pub const SK_BPF_CB_TX_TIMESTAMPING: u32 = (1<<0) as u32;
pub const SK_BPF_CB_MASK: u32 = ((SK_BPF_CB_TX_TIMESTAMPING - 1) | SK_BPF_CB_TX_TIMESTAMPING) as u32;

/* enum anonymous */
pub const BPF_SOCK_OPS_VOID: u32 = 0;
pub const BPF_SOCK_OPS_TIMEOUT_INIT: u32 = 1;
pub const BPF_SOCK_OPS_RWND_INIT: u32 = 2;
pub const BPF_SOCK_OPS_TCP_CONNECT_CB: u32 = 3;
pub const BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: u32 = 4;
pub const BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: u32 = 5;
pub const BPF_SOCK_OPS_NEEDS_ECN: u32 = 6;
pub const BPF_SOCK_OPS_BASE_RTT: u32 = 7;
pub const BPF_SOCK_OPS_RTO_CB: u32 = 8;
pub const BPF_SOCK_OPS_RETRANS_CB: u32 = 9;
pub const BPF_SOCK_OPS_STATE_CB: u32 = 10;
pub const BPF_SOCK_OPS_TCP_LISTEN_CB: u32 = 11;
pub const BPF_SOCK_OPS_PARSE_HDR_OPT_CB: u32 = 12;
pub const BPF_SOCK_OPS_HDR_OPT_LEN_CB: u32 = 13;
pub const ACK: u32 = 14;
pub const ACK: u32 = 15;
pub const BPF_SOCK_OPS_TSTAMP_SND_SW_CB: u32 = 16;
pub const BPF_SOCK_OPS_TSTAMP_SND_HW_CB: u32 = 17;
pub const BPF_SOCK_OPS_TSTAMP_ACK_CB: u32 = 18;
pub const BPF_SOCK_OPS_TSTAMP_SENDMSG_CB: u32 = 19;

/* enum anonymous */
pub const BPF_TCP_ESTABLISHED: u32 = (1) as u32;
pub const BPF_TCP_SYN_SENT: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_SYN_RECV: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_FIN_WAIT1: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_FIN_WAIT2: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_TIME_WAIT: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_CLOSE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_CLOSE_WAIT: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_LAST_ACK: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_LISTEN: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_CLOSING: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_NEW_SYN_RECV: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_BOUND_INACTIVE: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */
pub const BPF_TCP_MAX_STATES: u32 = 0; /* C enum auto value follows previous explicit expression; see preserved source below. */

/* enum anonymous */
pub const TCP_BPF_IW: u32 = (1001) as u32;
pub const TCP_BPF_SNDCWND_CLAMP: u32 = (1002) as u32;
pub const TCP_BPF_DELACK_MAX: u32 = (1003) as u32;
pub const TCP_BPF_RTO_MIN: u32 = (1004) as u32;
pub const TCP_BPF_SYN_IP: u32 = (1006) as u32;
pub const TCP_BPF_SYN_MAC: u32 = (1007) as u32;
pub const SK_BPF_CB_FLAGS: u32 = (1009) as u32;
pub const SK_BPF_BYPASS_PROT_MEM: u32 = (1010) as u32;

/* enum anonymous */
pub const BPF_LOAD_HDR_OPT_TCP_SYN: u32 = ((1ULL << 0)) as u32;

/* enum anonymous */
pub const BPF_WRITE_HDR_TCP_CURRENT_MSS: u32 = (1) as u32;
pub const BPF_WRITE_HDR_TCP_SYNACK_COOKIE: u32 = (2) as u32;

/* enum anonymous */
pub const BPF_DEVCG_ACC_MKNOD: u32 = ((1ULL << 0)) as u32;
pub const BPF_DEVCG_ACC_READ: u32 = ((1ULL << 1)) as u32;
pub const BPF_DEVCG_ACC_WRITE: u32 = ((1ULL << 2)) as u32;

/* enum anonymous */
pub const BPF_DEVCG_DEV_BLOCK: u32 = ((1ULL << 0)) as u32;
pub const BPF_DEVCG_DEV_CHAR: u32 = ((1ULL << 1)) as u32;

/* enum anonymous */
pub const BPF_FIB_LOOKUP_DIRECT: u32 = ((1u32 << 0)) as u32;
pub const BPF_FIB_LOOKUP_OUTPUT: u32 = ((1u32 << 1)) as u32;
pub const BPF_FIB_LOOKUP_SKIP_NEIGH: u32 = ((1u32 << 2)) as u32;
pub const BPF_FIB_LOOKUP_TBID: u32 = ((1u32 << 3)) as u32;
pub const BPF_FIB_LOOKUP_SRC: u32 = ((1u32 << 4)) as u32;
pub const BPF_FIB_LOOKUP_MARK: u32 = ((1u32 << 5)) as u32;
pub const BPF_FIB_LOOKUP_VLAN: u32 = ((1u32 << 6)) as u32;
pub const BPF_FIB_LOOKUP_VLAN_INPUT: u32 = ((1u32 << 7)) as u32;

/* enum anonymous */
pub const BPF_FIB_LKUP_RET_SUCCESS: u32 = 0;
pub const BPF_FIB_LKUP_RET_BLACKHOLE: u32 = 1;
pub const BPF_FIB_LKUP_RET_UNREACHABLE: u32 = 2;
pub const BPF_FIB_LKUP_RET_PROHIBIT: u32 = 3;
pub const BPF_FIB_LKUP_RET_NOT_FWDED: u32 = 4;
pub const BPF_FIB_LKUP_RET_FWD_DISABLED: u32 = 5;
pub const BPF_FIB_LKUP_RET_UNSUPP_LWT: u32 = 6;
pub const BPF_FIB_LKUP_RET_NO_NEIGH: u32 = 7;
pub const BPF_FIB_LKUP_RET_FRAG_NEEDED: u32 = 8;
pub const BPF_FIB_LKUP_RET_NO_SRC_ADDR: u32 = 9;
pub const BPF_FIB_LKUP_RET_VLAN_FAILURE: u32 = 10;

/* enum bpf_check_mtu_flags */
pub const BPF_MTU_CHK_SEGS: u32 = ((1u32 << 0)) as u32;

/* enum bpf_check_mtu_ret */
pub const BPF_MTU_CHK_RET_SUCCESS: u32 = 0;
pub const BPF_MTU_CHK_RET_FRAG_NEEDED: u32 = 1;
pub const BPF_MTU_CHK_RET_SEGS_TOOBIG: u32 = 2;

/* enum bpf_task_fd_type */
pub const BPF_FD_TYPE_RAW_TRACEPOINT: u32 = 0;
pub const BPF_FD_TYPE_TRACEPOINT: u32 = 1;
pub const BPF_FD_TYPE_KPROBE: u32 = 2;
pub const BPF_FD_TYPE_KRETPROBE: u32 = 3;
pub const BPF_FD_TYPE_UPROBE: u32 = 4;
pub const BPF_FD_TYPE_URETPROBE: u32 = 5;

/* enum anonymous */
pub const BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG: u32 = ((1u32 << 0)) as u32;
pub const BPF_FLOW_DISSECTOR_F_STOP_AT_FLOW_LABEL: u32 = ((1u32 << 1)) as u32;
pub const BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP: u32 = ((1u32 << 2)) as u32;

/* enum anonymous */
pub const BTF_F_COMPACT: u32 = ((1ULL << 0)) as u32;
pub const BTF_F_NONAME: u32 = ((1ULL << 1)) as u32;
pub const BTF_F_PTR_RAW: u32 = ((1ULL << 2)) as u32;
pub const BTF_F_ZERO: u32 = ((1ULL << 3)) as u32;

/* enum bpf_core_relo_kind */
pub const BPF_CORE_FIELD_BYTE_OFFSET: u32 = (0) as u32;
pub const BPF_CORE_FIELD_BYTE_SIZE: u32 = (1) as u32;
pub const BPF_CORE_FIELD_EXISTS: u32 = (2) as u32;
pub const BPF_CORE_FIELD_SIGNED: u32 = (3) as u32;
pub const BPF_CORE_FIELD_RSHIFT_U64: u32 = (5) as u32;
pub const BPF_CORE_TYPE_ID_LOCAL: u32 = (6) as u32;
pub const BPF_CORE_TYPE_ID_TARGET: u32 = (7) as u32;
pub const BPF_CORE_TYPE_EXISTS: u32 = (8) as u32;
pub const BPF_CORE_TYPE_SIZE: u32 = (9) as u32;
pub const BPF_CORE_ENUMVAL_EXISTS: u32 = (10) as u32;
pub const BPF_CORE_ENUMVAL_VALUE: u32 = (11) as u32;
pub const BPF_CORE_TYPE_MATCHES: u32 = (12) as u32;

/* enum anonymous */
pub const BPF_F_TIMER_ABS: u32 = ((1ULL << 0)) as u32;
pub const BPF_F_TIMER_CPU_PIN: u32 = ((1ULL << 1)) as u32;

/* enum bpf_kfunc_flags */
pub const BPF_F_PAD_ZEROS: u32 = ((1ULL << 0)) as u32;

pub const MAX_BPF_REG: u32 = __MAX_BPF_REG;
pub const MAX_BPF_ATTACH_TYPE: u32 = __MAX_BPF_ATTACH_TYPE;
pub const MAX_BPF_LINK_TYPE: u32 = __MAX_BPF_LINK_TYPE;
pub const BPF_BUILD_ID_SIZE: usize = 20;
pub const BPF_OBJ_NAME_LEN: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn {
    pub code: __u8,
    /* C bitfields: dst_reg:4 and src_reg:4 share this byte. */
    pub _bitfield_1: __u8,
    pub off: __s16,
    pub imm: __s32,
}

#[repr(C)]
pub struct bpf_lpm_trie_key { pub prefixlen: __u32, pub data: [__u8; 0] }
#[repr(C)]
pub struct bpf_lpm_trie_key_hdr { pub prefixlen: __u32 }
#[repr(C)]
pub union bpf_lpm_trie_key_u8_prefix { pub hdr: bpf_lpm_trie_key_hdr, pub prefixlen: __u32 }
#[repr(C)]
pub struct bpf_lpm_trie_key_u8 { pub u: bpf_lpm_trie_key_u8_prefix, pub data: [__u8; 0] }
#[repr(C)]
pub struct bpf_cgroup_storage_key { pub cgroup_inode_id: __u64, pub attach_type: __u32 }
#[repr(C)]
pub struct bpf_common_attr { pub log_buf: __aligned_u64, pub log_size: __u32, pub log_level: __u32, pub log_true_size: __u32 }
#[repr(C)]
pub union bpf_stack_build_id_addr { pub offset: __u64, pub ip: __u64 }
#[repr(C)]
pub struct bpf_stack_build_id { pub status: __s32, pub build_id: [u8; BPF_BUILD_ID_SIZE], pub u: bpf_stack_build_id_addr }

// The rest of the C header is preserved verbatim as line comments.
// It includes nested anonymous unions/structs, helper documentation, macros,
// and ABI records whose direct Rust layout depends on broader kernel bindings.
//C /* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
//C /* Copyright (c) 2011-2014 PLUMgrid, http://plumgrid.com
//C  *
//C  * This program is free software; you can redistribute it and/or
//C  * modify it under the terms of version 2 of the GNU General Public
//C  * License as published by the Free Software Foundation.
//C  */
//C #ifndef _UAPI__LINUX_BPF_H__
//C #define _UAPI__LINUX_BPF_H__
//C 
//C #include <linux/types.h>
//C #include <linux/bpf_common.h>
//C 
//C /* Extended instruction set based on top of classic BPF */
//C 
//C /* instruction classes */
//C #define BPF_JMP32	0x06	/* jmp mode in word width */
//C #define BPF_ALU64	0x07	/* alu mode in double word width */
//C 
//C /* ld/ldx fields */
//C #define BPF_DW		0x18	/* double word (64-bit) */
//C #define BPF_MEMSX	0x80	/* load with sign extension */
//C #define BPF_ATOMIC	0xc0	/* atomic memory ops - op type in immediate */
//C #define BPF_XADD	0xc0	/* exclusive add - legacy name */
//C 
//C /* alu/jmp fields */
//C #define BPF_MOV		0xb0	/* mov reg to reg */
//C #define BPF_ARSH	0xc0	/* sign extending arithmetic shift right */
//C 
//C /* change endianness of a register */
//C #define BPF_END		0xd0	/* flags for endianness conversion: */
//C #define BPF_TO_LE	0x00	/* convert to little-endian */
//C #define BPF_TO_BE	0x08	/* convert to big-endian */
//C #define BPF_FROM_LE	BPF_TO_LE
//C #define BPF_FROM_BE	BPF_TO_BE
//C 
//C /* jmp encodings */
//C #define BPF_JNE		0x50	/* jump != */
//C #define BPF_JLT		0xa0	/* LT is unsigned, '<' */
//C #define BPF_JLE		0xb0	/* LE is unsigned, '<=' */
//C #define BPF_JSGT	0x60	/* SGT is signed '>', GT in x86 */
//C #define BPF_JSGE	0x70	/* SGE is signed '>=', GE in x86 */
//C #define BPF_JSLT	0xc0	/* SLT is signed, '<' */
//C #define BPF_JSLE	0xd0	/* SLE is signed, '<=' */
//C #define BPF_JCOND	0xe0	/* conditional pseudo jumps: may_goto, goto_or_nop */
//C #define BPF_CALL	0x80	/* function call */
//C #define BPF_EXIT	0x90	/* function return */
//C 
//C /* atomic op type fields (stored in immediate) */
//C #define BPF_FETCH	0x01	/* not an opcode on its own, used to build others */
//C #define BPF_XCHG	(0xe0 | BPF_FETCH)	/* atomic exchange */
//C #define BPF_CMPXCHG	(0xf0 | BPF_FETCH)	/* atomic compare-and-write */
//C 
//C #define BPF_LOAD_ACQ	0x100	/* load-acquire */
//C #define BPF_STORE_REL	0x110	/* store-release */
//C 
//C enum bpf_cond_pseudo_jmp {
//C 	BPF_MAY_GOTO = 0,
//C };
//C 
//C /* Register numbers */
//C enum {
//C 	BPF_REG_0 = 0,
//C 	BPF_REG_1,
//C 	BPF_REG_2,
//C 	BPF_REG_3,
//C 	BPF_REG_4,
//C 	BPF_REG_5,
//C 	BPF_REG_6,
//C 	BPF_REG_7,
//C 	BPF_REG_8,
//C 	BPF_REG_9,
//C 	BPF_REG_10,
//C 	__MAX_BPF_REG,
//C };
//C 
//C /* BPF has 10 general purpose 64-bit registers and stack frame. */
//C #define MAX_BPF_REG	__MAX_BPF_REG
//C 
//C struct bpf_insn {
//C 	__u8	code;		/* opcode */
//C 	__u8	dst_reg:4;	/* dest register */
//C 	__u8	src_reg:4;	/* source register */
//C 	__s16	off;		/* signed offset */
//C 	__s32	imm;		/* signed immediate constant */
//C };
//C 
//C /* Deprecated: use struct bpf_lpm_trie_key_u8 (when the "data" member is needed for
//C  * byte access) or struct bpf_lpm_trie_key_hdr (when using an alternative type for
//C  * the trailing flexible array member) instead.
//C  */
//C struct bpf_lpm_trie_key {
//C 	__u32	prefixlen;	/* up to 32 for AF_INET, 128 for AF_INET6 */
//C 	__u8	data[0];	/* Arbitrary size */
//C };
//C 
//C /* Header for bpf_lpm_trie_key structs */
//C struct bpf_lpm_trie_key_hdr {
//C 	__u32	prefixlen;
//C };
//C 
//C /* Key of an a BPF_MAP_TYPE_LPM_TRIE entry, with trailing byte array. */
//C struct bpf_lpm_trie_key_u8 {
//C 	union {
//C 		struct bpf_lpm_trie_key_hdr	hdr;
//C 		__u32				prefixlen;
//C 	};
//C 	__u8	data[];		/* Arbitrary size */
//C };
//C 
//C struct bpf_cgroup_storage_key {
//C 	__u64	cgroup_inode_id;	/* cgroup inode id */
//C 	__u32	attach_type;		/* program attach type (enum bpf_attach_type) */
//C };
//C 
//C enum bpf_cgroup_iter_order {
//C 	BPF_CGROUP_ITER_ORDER_UNSPEC = 0,
//C 	BPF_CGROUP_ITER_SELF_ONLY,		/* process only a single object. */
//C 	BPF_CGROUP_ITER_DESCENDANTS_PRE,	/* walk descendants in pre-order. */
//C 	BPF_CGROUP_ITER_DESCENDANTS_POST,	/* walk descendants in post-order. */
//C 	BPF_CGROUP_ITER_ANCESTORS_UP,		/* walk ancestors upward. */
//C 	/*
//C 	 * Walks the immediate children of the specified parent
//C 	 * cgroup_subsys_state. Unlike BPF_CGROUP_ITER_DESCENDANTS_PRE,
//C 	 * BPF_CGROUP_ITER_DESCENDANTS_POST, and BPF_CGROUP_ITER_ANCESTORS_UP
//C 	 * the iterator does not include the specified parent as one of the
//C 	 * returned iterator elements.
//C 	 */
//C 	BPF_CGROUP_ITER_CHILDREN,
//C };
//C 
//C union bpf_iter_link_info {
//C 	struct {
//C 		__u32	map_fd;
//C 	} map;
//C 	struct {
//C 		enum bpf_cgroup_iter_order order;
//C 
//C 		/* At most one of cgroup_fd and cgroup_id can be non-zero. If
//C 		 * both are zero, the walk starts from the default cgroup v2
//C 		 * root. For walking v1 hierarchy, one should always explicitly
//C 		 * specify cgroup_fd.
//C 		 */
//C 		__u32	cgroup_fd;
//C 		__u64	cgroup_id;
//C 	} cgroup;
//C 	/* Parameters of task iterators. */
//C 	struct {
//C 		__u32	tid;
//C 		__u32	pid;
//C 		__u32	pid_fd;
//C 	} task;
//C };
//C 
//C /* BPF syscall commands, see bpf(2) man-page for more details. */
//C /**
//C  * DOC: eBPF Syscall Preamble
//C  *
//C  * The operation to be performed by the **bpf**\ () system call is determined
//C  * by the *cmd* argument. Each operation takes an accompanying argument,
//C  * provided via *attr*, which is a pointer to a union of type *bpf_attr* (see
//C  * below). The size argument is the size of the union pointed to by *attr*.
//C  */
//C /**
//C  * DOC: eBPF Syscall Commands
//C  *
//C  * BPF_MAP_CREATE
//C  *	Description
//C  *		Create a map and return a file descriptor that refers to the
//C  *		map. The close-on-exec file descriptor flag (see **fcntl**\ (2))
//C  *		is automatically enabled for the new file descriptor.
//C  *
//C  *		Applying **close**\ (2) to the file descriptor returned by
//C  *		**BPF_MAP_CREATE** will delete the map (but see NOTES).
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_MAP_LOOKUP_ELEM
//C  *	Description
//C  *		Look up an element with a given *key* in the map referred to
//C  *		by the file descriptor *map_fd*.
//C  *
//C  *		The *flags* argument may be specified as one of the
//C  *		following:
//C  *
//C  *		**BPF_F_LOCK**
//C  *			Look up the value of a spin-locked map without
//C  *			returning the lock. This must be specified if the
//C  *			elements contain a spinlock.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_MAP_UPDATE_ELEM
//C  *	Description
//C  *		Create or update an element (key/value pair) in a specified map.
//C  *
//C  *		The *flags* argument should be specified as one of the
//C  *		following:
//C  *
//C  *		**BPF_ANY**
//C  *			Create a new element or update an existing element.
//C  *		**BPF_NOEXIST**
//C  *			Create a new element only if it did not exist.
//C  *		**BPF_EXIST**
//C  *			Update an existing element.
//C  *		**BPF_F_LOCK**
//C  *			Update a spin_lock-ed map element.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  *		May set *errno* to **EINVAL**, **EPERM**, **ENOMEM**,
//C  *		**E2BIG**, **EEXIST**, or **ENOENT**.
//C  *
//C  *		**E2BIG**
//C  *			The number of elements in the map reached the
//C  *			*max_entries* limit specified at map creation time.
//C  *		**EEXIST**
//C  *			If *flags* specifies **BPF_NOEXIST** and the element
//C  *			with *key* already exists in the map.
//C  *		**ENOENT**
//C  *			If *flags* specifies **BPF_EXIST** and the element with
//C  *			*key* does not exist in the map.
//C  *
//C  * BPF_MAP_DELETE_ELEM
//C  *	Description
//C  *		Look up and delete an element by key in a specified map.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_MAP_GET_NEXT_KEY
//C  *	Description
//C  *		Look up an element by key in a specified map and return the key
//C  *		of the next element. Can be used to iterate over all elements
//C  *		in the map.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  *		The following cases can be used to iterate over all elements of
//C  *		the map:
//C  *
//C  *		* If *key* is not found, the operation returns zero and sets
//C  *		  the *next_key* pointer to the key of the first element.
//C  *		* If *key* is found, the operation returns zero and sets the
//C  *		  *next_key* pointer to the key of the next element.
//C  *		* If *key* is the last element, returns -1 and *errno* is set
//C  *		  to **ENOENT**.
//C  *
//C  *		May set *errno* to **ENOMEM**, **EFAULT**, **EPERM**, or
//C  *		**EINVAL** on error.
//C  *
//C  * BPF_PROG_LOAD
//C  *	Description
//C  *		Verify and load an eBPF program, returning a new file
//C  *		descriptor associated with the program.
//C  *
//C  *		Applying **close**\ (2) to the file descriptor returned by
//C  *		**BPF_PROG_LOAD** will unload the eBPF program (but see NOTES).
//C  *
//C  *		The close-on-exec file descriptor flag (see **fcntl**\ (2)) is
//C  *		automatically enabled for the new file descriptor.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_OBJ_PIN
//C  *	Description
//C  *		Pin an eBPF program or map referred by the specified *bpf_fd*
//C  *		to the provided *pathname* on the filesystem.
//C  *
//C  *		The *pathname* argument must not contain a dot (".").
//C  *
//C  *		On success, *pathname* retains a reference to the eBPF object,
//C  *		preventing deallocation of the object when the original
//C  *		*bpf_fd* is closed. This allow the eBPF object to live beyond
//C  *		**close**\ (\ *bpf_fd*\ ), and hence the lifetime of the parent
//C  *		process.
//C  *
//C  *		Applying **unlink**\ (2) or similar calls to the *pathname*
//C  *		unpins the object from the filesystem, removing the reference.
//C  *		If no other file descriptors or filesystem nodes refer to the
//C  *		same object, it will be deallocated (see NOTES).
//C  *
//C  *		The filesystem type for the parent directory of *pathname* must
//C  *		be **BPF_FS_MAGIC**.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_OBJ_GET
//C  *	Description
//C  *		Open a file descriptor for the eBPF object pinned to the
//C  *		specified *pathname*.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_PROG_ATTACH
//C  *	Description
//C  *		Attach an eBPF program to a *target_fd* at the specified
//C  *		*attach_type* hook.
//C  *
//C  *		The *attach_type* specifies the eBPF attachment point to
//C  *		attach the program to, and must be one of *bpf_attach_type*
//C  *		(see below).
//C  *
//C  *		The *attach_bpf_fd* must be a valid file descriptor for a
//C  *		loaded eBPF program of a cgroup, flow dissector, LIRC, sockmap
//C  *		or sock_ops type corresponding to the specified *attach_type*.
//C  *
//C  *		The *target_fd* must be a valid file descriptor for a kernel
//C  *		object which depends on the attach type of *attach_bpf_fd*:
//C  *
//C  *		**BPF_PROG_TYPE_CGROUP_DEVICE**,
//C  *		**BPF_PROG_TYPE_CGROUP_SKB**,
//C  *		**BPF_PROG_TYPE_CGROUP_SOCK**,
//C  *		**BPF_PROG_TYPE_CGROUP_SOCK_ADDR**,
//C  *		**BPF_PROG_TYPE_CGROUP_SOCKOPT**,
//C  *		**BPF_PROG_TYPE_CGROUP_SYSCTL**,
//C  *		**BPF_PROG_TYPE_SOCK_OPS**
//C  *
//C  *			Control Group v2 hierarchy with the eBPF controller
//C  *			enabled. Requires the kernel to be compiled with
//C  *			**CONFIG_CGROUP_BPF**.
//C  *
//C  *		**BPF_PROG_TYPE_FLOW_DISSECTOR**
//C  *
//C  *			Network namespace (eg /proc/self/ns/net).
//C  *
//C  *		**BPF_PROG_TYPE_LIRC_MODE2**
//C  *
//C  *			LIRC device path (eg /dev/lircN). Requires the kernel
//C  *			to be compiled with **CONFIG_BPF_LIRC_MODE2**.
//C  *
//C  *		**BPF_PROG_TYPE_SK_SKB**,
//C  *		**BPF_PROG_TYPE_SK_MSG**
//C  *
//C  *			eBPF map of socket type (eg **BPF_MAP_TYPE_SOCKHASH**).
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_PROG_DETACH
//C  *	Description
//C  *		Detach the eBPF program associated with the *target_fd* at the
//C  *		hook specified by *attach_type*. The program must have been
//C  *		previously attached using **BPF_PROG_ATTACH**.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_PROG_TEST_RUN
//C  *	Description
//C  *		Run the eBPF program associated with the *prog_fd* a *repeat*
//C  *		number of times against a provided program context *ctx_in* and
//C  *		data *data_in*, and return the modified program context
//C  *		*ctx_out*, *data_out* (for example, packet data), result of the
//C  *		execution *retval*, and *duration* of the test run.
//C  *
//C  *		The sizes of the buffers provided as input and output
//C  *		parameters *ctx_in*, *ctx_out*, *data_in*, and *data_out* must
//C  *		be provided in the corresponding variables *ctx_size_in*,
//C  *		*ctx_size_out*, *data_size_in*, and/or *data_size_out*. If any
//C  *		of these parameters are not provided (ie set to NULL), the
//C  *		corresponding size field must be zero.
//C  *
//C  *		Some program types have particular requirements:
//C  *
//C  *		**BPF_PROG_TYPE_SK_LOOKUP**
//C  *			*data_in* and *data_out* must be NULL.
//C  *
//C  *		**BPF_PROG_TYPE_RAW_TRACEPOINT**,
//C  *		**BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE**
//C  *
//C  *			*ctx_out*, *data_in* and *data_out* must be NULL.
//C  *			*repeat* must be zero.
//C  *
//C  *		BPF_PROG_RUN is an alias for BPF_PROG_TEST_RUN.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  *		**ENOSPC**
//C  *			Either *data_size_out* or *ctx_size_out* is too small.
//C  *		**ENOTSUPP**
//C  *			This command is not supported by the program type of
//C  *			the program referred to by *prog_fd*.
//C  *
//C  * BPF_PROG_GET_NEXT_ID
//C  *	Description
//C  *		Fetch the next eBPF program currently loaded into the kernel.
//C  *
//C  *		Looks for the eBPF program with an id greater than *start_id*
//C  *		and updates *next_id* on success. If no other eBPF programs
//C  *		remain with ids higher than *start_id*, returns -1 and sets
//C  *		*errno* to **ENOENT**.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, or when no id remains, -1
//C  *		is returned and *errno* is set appropriately.
//C  *
//C  * BPF_MAP_GET_NEXT_ID
//C  *	Description
//C  *		Fetch the next eBPF map currently loaded into the kernel.
//C  *
//C  *		Looks for the eBPF map with an id greater than *start_id*
//C  *		and updates *next_id* on success. If no other eBPF maps
//C  *		remain with ids higher than *start_id*, returns -1 and sets
//C  *		*errno* to **ENOENT**.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, or when no id remains, -1
//C  *		is returned and *errno* is set appropriately.
//C  *
//C  * BPF_PROG_GET_FD_BY_ID
//C  *	Description
//C  *		Open a file descriptor for the eBPF program corresponding to
//C  *		*prog_id*.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_MAP_GET_FD_BY_ID
//C  *	Description
//C  *		Open a file descriptor for the eBPF map corresponding to
//C  *		*map_id*.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_OBJ_GET_INFO_BY_FD
//C  *	Description
//C  *		Obtain information about the eBPF object corresponding to
//C  *		*bpf_fd*.
//C  *
//C  *		Populates up to *info_len* bytes of *info*, which will be in
//C  *		one of the following formats depending on the eBPF object type
//C  *		of *bpf_fd*:
//C  *
//C  *		* **struct bpf_prog_info**
//C  *		* **struct bpf_map_info**
//C  *		* **struct bpf_btf_info**
//C  *		* **struct bpf_link_info**
//C  *		* **struct bpf_token_info**
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_PROG_QUERY
//C  *	Description
//C  *		Obtain information about eBPF programs associated with the
//C  *		specified *attach_type* hook.
//C  *
//C  *		The *target_fd* must be a valid file descriptor for a kernel
//C  *		object which depends on the attach type of *attach_bpf_fd*:
//C  *
//C  *		**BPF_PROG_TYPE_CGROUP_DEVICE**,
//C  *		**BPF_PROG_TYPE_CGROUP_SKB**,
//C  *		**BPF_PROG_TYPE_CGROUP_SOCK**,
//C  *		**BPF_PROG_TYPE_CGROUP_SOCK_ADDR**,
//C  *		**BPF_PROG_TYPE_CGROUP_SOCKOPT**,
//C  *		**BPF_PROG_TYPE_CGROUP_SYSCTL**,
//C  *		**BPF_PROG_TYPE_SOCK_OPS**
//C  *
//C  *			Control Group v2 hierarchy with the eBPF controller
//C  *			enabled. Requires the kernel to be compiled with
//C  *			**CONFIG_CGROUP_BPF**.
//C  *
//C  *		**BPF_PROG_TYPE_FLOW_DISSECTOR**
//C  *
//C  *			Network namespace (eg /proc/self/ns/net).
//C  *
//C  *		**BPF_PROG_TYPE_LIRC_MODE2**
//C  *
//C  *			LIRC device path (eg /dev/lircN). Requires the kernel
//C  *			to be compiled with **CONFIG_BPF_LIRC_MODE2**.
//C  *
//C  *		**BPF_PROG_QUERY** always fetches the number of programs
//C  *		attached and the *attach_flags* which were used to attach those
//C  *		programs. Additionally, if *prog_ids* is nonzero and the number
//C  *		of attached programs is less than *prog_cnt*, populates
//C  *		*prog_ids* with the eBPF program ids of the programs attached
//C  *		at *target_fd*.
//C  *
//C  *		The following flags may alter the result:
//C  *
//C  *		**BPF_F_QUERY_EFFECTIVE**
//C  *			Only return information regarding programs which are
//C  *			currently effective at the specified *target_fd*.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_RAW_TRACEPOINT_OPEN
//C  *	Description
//C  *		Attach an eBPF program to a tracepoint *name* to access kernel
//C  *		internal arguments of the tracepoint in their raw form.
//C  *
//C  *		The *prog_fd* must be a valid file descriptor associated with
//C  *		a loaded eBPF program of type **BPF_PROG_TYPE_RAW_TRACEPOINT**.
//C  *
//C  *		No ABI guarantees are made about the content of tracepoint
//C  *		arguments exposed to the corresponding eBPF program.
//C  *
//C  *		Applying **close**\ (2) to the file descriptor returned by
//C  *		**BPF_RAW_TRACEPOINT_OPEN** will delete the map (but see NOTES).
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_BTF_LOAD
//C  *	Description
//C  *		Verify and load BPF Type Format (BTF) metadata into the kernel,
//C  *		returning a new file descriptor associated with the metadata.
//C  *		BTF is described in more detail at
//C  *		https://www.kernel.org/doc/html/latest/bpf/btf.html.
//C  *
//C  *		The *btf* parameter must point to valid memory providing
//C  *		*btf_size* bytes of BTF binary metadata.
//C  *
//C  *		The returned file descriptor can be passed to other **bpf**\ ()
//C  *		subcommands such as **BPF_PROG_LOAD** or **BPF_MAP_CREATE** to
//C  *		associate the BTF with those objects.
//C  *
//C  *		Similar to **BPF_PROG_LOAD**, **BPF_BTF_LOAD** has optional
//C  *		parameters to specify a *btf_log_buf*, *btf_log_size* and
//C  *		*btf_log_level* which allow the kernel to return freeform log
//C  *		output regarding the BTF verification process.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_BTF_GET_FD_BY_ID
//C  *	Description
//C  *		Open a file descriptor for the BPF Type Format (BTF)
//C  *		corresponding to *btf_id*.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_TASK_FD_QUERY
//C  *	Description
//C  *		Obtain information about eBPF programs associated with the
//C  *		target process identified by *pid* and *fd*.
//C  *
//C  *		If the *pid* and *fd* are associated with a tracepoint, kprobe
//C  *		or uprobe perf event, then the *prog_id* and *fd_type* will
//C  *		be populated with the eBPF program id and file descriptor type
//C  *		of type **bpf_task_fd_type**. If associated with a kprobe or
//C  *		uprobe, the  *probe_offset* and *probe_addr* will also be
//C  *		populated. Optionally, if *buf* is provided, then up to
//C  *		*buf_len* bytes of *buf* will be populated with the name of
//C  *		the tracepoint, kprobe or uprobe.
//C  *
//C  *		The resulting *prog_id* may be introspected in deeper detail
//C  *		using **BPF_PROG_GET_FD_BY_ID** and **BPF_OBJ_GET_INFO_BY_FD**.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_MAP_LOOKUP_AND_DELETE_ELEM
//C  *	Description
//C  *		Look up an element with the given *key* in the map referred to
//C  *		by the file descriptor *fd*, and if found, delete the element.
//C  *
//C  *		For **BPF_MAP_TYPE_QUEUE** and **BPF_MAP_TYPE_STACK** map
//C  *		types, the *flags* argument needs to be set to 0, but for other
//C  *		map types, it may be specified as:
//C  *
//C  *		**BPF_F_LOCK**
//C  *			Look up and delete the value of a spin-locked map
//C  *			without returning the lock. This must be specified if
//C  *			the elements contain a spinlock.
//C  *
//C  *		The **BPF_MAP_TYPE_QUEUE** and **BPF_MAP_TYPE_STACK** map types
//C  *		implement this command as a "pop" operation, deleting the top
//C  *		element rather than one corresponding to *key*.
//C  *		The *key* and *key_len* parameters should be zeroed when
//C  *		issuing this operation for these map types.
//C  *
//C  *		This command is only valid for the following map types:
//C  *		* **BPF_MAP_TYPE_QUEUE**
//C  *		* **BPF_MAP_TYPE_STACK**
//C  *		* **BPF_MAP_TYPE_HASH**
//C  *		* **BPF_MAP_TYPE_PERCPU_HASH**
//C  *		* **BPF_MAP_TYPE_LRU_HASH**
//C  *		* **BPF_MAP_TYPE_LRU_PERCPU_HASH**
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_MAP_FREEZE
//C  *	Description
//C  *		Freeze the permissions of the specified map.
//C  *
//C  *		Write permissions may be frozen by passing zero *flags*.
//C  *		Upon success, no future syscall invocations may alter the
//C  *		map state of *map_fd*. Write operations from eBPF programs
//C  *		are still possible for a frozen map.
//C  *
//C  *		Not supported for maps of type **BPF_MAP_TYPE_STRUCT_OPS**.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_BTF_GET_NEXT_ID
//C  *	Description
//C  *		Fetch the next BPF Type Format (BTF) object currently loaded
//C  *		into the kernel.
//C  *
//C  *		Looks for the BTF object with an id greater than *start_id*
//C  *		and updates *next_id* on success. If no other BTF objects
//C  *		remain with ids higher than *start_id*, returns -1 and sets
//C  *		*errno* to **ENOENT**.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, or when no id remains, -1
//C  *		is returned and *errno* is set appropriately.
//C  *
//C  * BPF_MAP_LOOKUP_BATCH
//C  *	Description
//C  *		Iterate and fetch multiple elements in a map.
//C  *
//C  *		Two opaque values are used to manage batch operations,
//C  *		*in_batch* and *out_batch*. Initially, *in_batch* must be set
//C  *		to NULL to begin the batched operation. After each subsequent
//C  *		**BPF_MAP_LOOKUP_BATCH**, the caller should pass the resultant
//C  *		*out_batch* as the *in_batch* for the next operation to
//C  *		continue iteration from the current point. Both *in_batch* and
//C  *		*out_batch* must point to memory large enough to hold a key,
//C  *		except for maps of type **BPF_MAP_TYPE_{HASH, PERCPU_HASH,
//C  *		LRU_HASH, LRU_PERCPU_HASH}**, for which batch parameters
//C  *		must be at least 4 bytes wide regardless of key size.
//C  *
//C  *		The *keys* and *values* are output parameters which must point
//C  *		to memory large enough to hold *count* items based on the key
//C  *		and value size of the map *map_fd*. The *keys* buffer must be
//C  *		of *key_size* * *count*. The *values* buffer must be of
//C  *		*value_size* * *count*.
//C  *
//C  *		The *elem_flags* argument may be specified as one of the
//C  *		following:
//C  *
//C  *		**BPF_F_LOCK**
//C  *			Look up the value of a spin-locked map without
//C  *			returning the lock. This must be specified if the
//C  *			elements contain a spinlock.
//C  *
//C  *		On success, *count* elements from the map are copied into the
//C  *		user buffer, with the keys copied into *keys* and the values
//C  *		copied into the corresponding indices in *values*.
//C  *
//C  *		If an error is returned and *errno* is not **EFAULT**, *count*
//C  *		is set to the number of successfully processed elements.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  *		May set *errno* to **ENOSPC** to indicate that *keys* or
//C  *		*values* is too small to dump an entire bucket during
//C  *		iteration of a hash-based map type.
//C  *
//C  * BPF_MAP_LOOKUP_AND_DELETE_BATCH
//C  *	Description
//C  *		Iterate and delete all elements in a map.
//C  *
//C  *		This operation has the same behavior as
//C  *		**BPF_MAP_LOOKUP_BATCH** with two exceptions:
//C  *
//C  *		* Every element that is successfully returned is also deleted
//C  *		  from the map. This is at least *count* elements. Note that
//C  *		  *count* is both an input and an output parameter.
//C  *		* Upon returning with *errno* set to **EFAULT**, up to
//C  *		  *count* elements may be deleted without returning the keys
//C  *		  and values of the deleted elements.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_MAP_UPDATE_BATCH
//C  *	Description
//C  *		Update multiple elements in a map by *key*.
//C  *
//C  *		The *keys* and *values* are input parameters which must point
//C  *		to memory large enough to hold *count* items based on the key
//C  *		and value size of the map *map_fd*. The *keys* buffer must be
//C  *		of *key_size* * *count*. The *values* buffer must be of
//C  *		*value_size* * *count*.
//C  *
//C  *		Each element specified in *keys* is sequentially updated to the
//C  *		value in the corresponding index in *values*. The *in_batch*
//C  *		and *out_batch* parameters are ignored and should be zeroed.
//C  *
//C  *		The *elem_flags* argument should be specified as one of the
//C  *		following:
//C  *
//C  *		**BPF_ANY**
//C  *			Create new elements or update a existing elements.
//C  *		**BPF_NOEXIST**
//C  *			Create new elements only if they do not exist.
//C  *		**BPF_EXIST**
//C  *			Update existing elements.
//C  *		**BPF_F_LOCK**
//C  *			Update spin_lock-ed map elements. This must be
//C  *			specified if the map value contains a spinlock.
//C  *
//C  *		On success, *count* elements from the map are updated.
//C  *
//C  *		If an error is returned and *errno* is not **EFAULT**, *count*
//C  *		is set to the number of successfully processed elements.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  *		May set *errno* to **EINVAL**, **EPERM**, **ENOMEM**, or
//C  *		**E2BIG**. **E2BIG** indicates that the number of elements in
//C  *		the map reached the *max_entries* limit specified at map
//C  *		creation time.
//C  *
//C  *		May set *errno* to one of the following error codes under
//C  *		specific circumstances:
//C  *
//C  *		**EEXIST**
//C  *			If *flags* specifies **BPF_NOEXIST** and the element
//C  *			with *key* already exists in the map.
//C  *		**ENOENT**
//C  *			If *flags* specifies **BPF_EXIST** and the element with
//C  *			*key* does not exist in the map.
//C  *
//C  * BPF_MAP_DELETE_BATCH
//C  *	Description
//C  *		Delete multiple elements in a map by *key*.
//C  *
//C  *		The *keys* parameter is an input parameter which must point
//C  *		to memory large enough to hold *count* items based on the key
//C  *		size of the map *map_fd*, that is, *key_size* * *count*.
//C  *
//C  *		Each element specified in *keys* is sequentially deleted. The
//C  *		*in_batch*, *out_batch*, and *values* parameters are ignored
//C  *		and should be zeroed.
//C  *
//C  *		The *elem_flags* argument may be specified as one of the
//C  *		following:
//C  *
//C  *		**BPF_F_LOCK**
//C  *			Look up the value of a spin-locked map without
//C  *			returning the lock. This must be specified if the
//C  *			elements contain a spinlock.
//C  *
//C  *		On success, *count* elements from the map are updated.
//C  *
//C  *		If an error is returned and *errno* is not **EFAULT**, *count*
//C  *		is set to the number of successfully processed elements. If
//C  *		*errno* is **EFAULT**, up to *count* elements may be been
//C  *		deleted.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_LINK_CREATE
//C  *	Description
//C  *		Attach an eBPF program to a *target_fd* at the specified
//C  *		*attach_type* hook and return a file descriptor handle for
//C  *		managing the link.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_LINK_UPDATE
//C  *	Description
//C  *		Update the eBPF program in the specified *link_fd* to
//C  *		*new_prog_fd*.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_LINK_GET_FD_BY_ID
//C  *	Description
//C  *		Open a file descriptor for the eBPF Link corresponding to
//C  *		*link_id*.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_LINK_GET_NEXT_ID
//C  *	Description
//C  *		Fetch the next eBPF link currently loaded into the kernel.
//C  *
//C  *		Looks for the eBPF link with an id greater than *start_id*
//C  *		and updates *next_id* on success. If no other eBPF links
//C  *		remain with ids higher than *start_id*, returns -1 and sets
//C  *		*errno* to **ENOENT**.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, or when no id remains, -1
//C  *		is returned and *errno* is set appropriately.
//C  *
//C  * BPF_ENABLE_STATS
//C  *	Description
//C  *		Enable eBPF runtime statistics gathering.
//C  *
//C  *		Runtime statistics gathering for the eBPF runtime is disabled
//C  *		by default to minimize the corresponding performance overhead.
//C  *		This command enables statistics globally.
//C  *
//C  *		Multiple programs may independently enable statistics.
//C  *		After gathering the desired statistics, eBPF runtime statistics
//C  *		may be disabled again by calling **close**\ (2) for the file
//C  *		descriptor returned by this function. Statistics will only be
//C  *		disabled system-wide when all outstanding file descriptors
//C  *		returned by prior calls for this subcommand are closed.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_ITER_CREATE
//C  *	Description
//C  *		Create an iterator on top of the specified *link_fd* (as
//C  *		previously created using **BPF_LINK_CREATE**) and return a
//C  *		file descriptor that can be used to trigger the iteration.
//C  *
//C  *		If the resulting file descriptor is pinned to the filesystem
//C  *		using  **BPF_OBJ_PIN**, then subsequent **read**\ (2) syscalls
//C  *		for that path will trigger the iterator to read kernel state
//C  *		using the eBPF program attached to *link_fd*.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_LINK_DETACH
//C  *	Description
//C  *		Forcefully detach the specified *link_fd* from its
//C  *		corresponding attachment point.
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_PROG_BIND_MAP
//C  *	Description
//C  *		Bind a map to the lifetime of an eBPF program.
//C  *
//C  *		The map identified by *map_fd* is bound to the program
//C  *		identified by *prog_fd* and only released when *prog_fd* is
//C  *		released. This may be used in cases where metadata should be
//C  *		associated with a program which otherwise does not contain any
//C  *		references to the map (for example, embedded in the eBPF
//C  *		program instructions).
//C  *
//C  *	Return
//C  *		Returns zero on success. On error, -1 is returned and *errno*
//C  *		is set appropriately.
//C  *
//C  * BPF_TOKEN_CREATE
//C  *	Description
//C  *		Create BPF token with embedded information about what
//C  *		BPF-related functionality it allows:
//C  *		- a set of allowed bpf() syscall commands;
//C  *		- a set of allowed BPF map types to be created with
//C  *		BPF_MAP_CREATE command, if BPF_MAP_CREATE itself is allowed;
//C  *		- a set of allowed BPF program types and BPF program attach
//C  *		types to be loaded with BPF_PROG_LOAD command, if
//C  *		BPF_PROG_LOAD itself is allowed.
//C  *
//C  *		BPF token is created (derived) from an instance of BPF FS,
//C  *		assuming it has necessary delegation mount options specified.
//C  *		This BPF token can be passed as an extra parameter to various
//C  *		bpf() syscall commands to grant BPF subsystem functionality to
//C  *		unprivileged processes.
//C  *
//C  *		When created, BPF token is "associated" with the owning
//C  *		user namespace of BPF FS instance (super block) that it was
//C  *		derived from, and subsequent BPF operations performed with
//C  *		BPF token would be performing capabilities checks (i.e.,
//C  *		CAP_BPF, CAP_PERFMON, CAP_NET_ADMIN, CAP_SYS_ADMIN) within
//C  *		that user namespace. Without BPF token, such capabilities
//C  *		have to be granted in init user namespace, making bpf()
//C  *		syscall incompatible with user namespace, for the most part.
//C  *
//C  *	Return
//C  *		A new file descriptor (a nonnegative integer), or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_PROG_STREAM_READ_BY_FD
//C  *	Description
//C  *		Read data of a program's BPF stream. The program is identified
//C  *		by *prog_fd*, and the stream is identified by the *stream_id*.
//C  *		The data is copied to a buffer pointed to by *stream_buf*, and
//C  *		filled less than or equal to *stream_buf_len* bytes.
//C  *
//C  *	Return
//C  *		Number of bytes read from the stream on success, or -1 if an
//C  *		error occurred (in which case, *errno* is set appropriately).
//C  *
//C  * BPF_PROG_ASSOC_STRUCT_OPS
//C  * 	Description
//C  * 		Associate a BPF program with a struct_ops map. The struct_ops
//C  * 		map is identified by *map_fd* and the BPF program is
//C  * 		identified by *prog_fd*.
//C  *
//C  * 	Return
//C  * 		0 on success or -1 if an error occurred (in which case,
//C  * 		*errno* is set appropriately).
//C  *
//C  * NOTES
//C  *	eBPF objects (maps and programs) can be shared between processes.
//C  *
//C  *	* After **fork**\ (2), the child inherits file descriptors
//C  *	  referring to the same eBPF objects.
//C  *	* File descriptors referring to eBPF objects can be transferred over
//C  *	  **unix**\ (7) domain sockets.
//C  *	* File descriptors referring to eBPF objects can be duplicated in the
//C  *	  usual way, using **dup**\ (2) and similar calls.
//C  *	* File descriptors referring to eBPF objects can be pinned to the
//C  *	  filesystem using the **BPF_OBJ_PIN** command of **bpf**\ (2).
//C  *
//C  *	An eBPF object is deallocated only after all file descriptors referring
//C  *	to the object have been closed and no references remain pinned to the
//C  *	filesystem or attached (for example, bound to a program or device).
//C  */
//C enum bpf_cmd {
//C 	BPF_MAP_CREATE,
//C 	BPF_MAP_LOOKUP_ELEM,
//C 	BPF_MAP_UPDATE_ELEM,
//C 	BPF_MAP_DELETE_ELEM,
//C 	BPF_MAP_GET_NEXT_KEY,
//C 	BPF_PROG_LOAD,
//C 	BPF_OBJ_PIN,
//C 	BPF_OBJ_GET,
//C 	BPF_PROG_ATTACH,
//C 	BPF_PROG_DETACH,
//C 	BPF_PROG_TEST_RUN,
//C 	BPF_PROG_RUN = BPF_PROG_TEST_RUN,
//C 	BPF_PROG_GET_NEXT_ID,
//C 	BPF_MAP_GET_NEXT_ID,
//C 	BPF_PROG_GET_FD_BY_ID,
//C 	BPF_MAP_GET_FD_BY_ID,
//C 	BPF_OBJ_GET_INFO_BY_FD,
//C 	BPF_PROG_QUERY,
//C 	BPF_RAW_TRACEPOINT_OPEN,
//C 	BPF_BTF_LOAD,
//C 	BPF_BTF_GET_FD_BY_ID,
//C 	BPF_TASK_FD_QUERY,
//C 	BPF_MAP_LOOKUP_AND_DELETE_ELEM,
//C 	BPF_MAP_FREEZE,
//C 	BPF_BTF_GET_NEXT_ID,
//C 	BPF_MAP_LOOKUP_BATCH,
//C 	BPF_MAP_LOOKUP_AND_DELETE_BATCH,
//C 	BPF_MAP_UPDATE_BATCH,
//C 	BPF_MAP_DELETE_BATCH,
//C 	BPF_LINK_CREATE,
//C 	BPF_LINK_UPDATE,
//C 	BPF_LINK_GET_FD_BY_ID,
//C 	BPF_LINK_GET_NEXT_ID,
//C 	BPF_ENABLE_STATS,
//C 	BPF_ITER_CREATE,
//C 	BPF_LINK_DETACH,
//C 	BPF_PROG_BIND_MAP,
//C 	BPF_TOKEN_CREATE,
//C 	BPF_PROG_STREAM_READ_BY_FD,
//C 	BPF_PROG_ASSOC_STRUCT_OPS,
//C 	__MAX_BPF_CMD,
//C 	BPF_COMMON_ATTRS = 1 << 16, /* Indicate carrying syscall common attrs. */
//C };
//C 
//C enum bpf_map_type {
//C 	BPF_MAP_TYPE_UNSPEC,
//C 	BPF_MAP_TYPE_HASH,
//C 	BPF_MAP_TYPE_ARRAY,
//C 	BPF_MAP_TYPE_PROG_ARRAY,
//C 	BPF_MAP_TYPE_PERF_EVENT_ARRAY,
//C 	BPF_MAP_TYPE_PERCPU_HASH,
//C 	BPF_MAP_TYPE_PERCPU_ARRAY,
//C 	BPF_MAP_TYPE_STACK_TRACE,
//C 	BPF_MAP_TYPE_CGROUP_ARRAY,
//C 	BPF_MAP_TYPE_LRU_HASH,
//C 	BPF_MAP_TYPE_LRU_PERCPU_HASH,
//C 	BPF_MAP_TYPE_LPM_TRIE,
//C 	BPF_MAP_TYPE_ARRAY_OF_MAPS,
//C 	BPF_MAP_TYPE_HASH_OF_MAPS,
//C 	BPF_MAP_TYPE_DEVMAP,
//C 	BPF_MAP_TYPE_SOCKMAP,
//C 	BPF_MAP_TYPE_CPUMAP,
//C 	BPF_MAP_TYPE_XSKMAP,
//C 	BPF_MAP_TYPE_SOCKHASH,
//C 	BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED,
//C 	/* BPF_MAP_TYPE_CGROUP_STORAGE is available to bpf programs attaching
//C 	 * to a cgroup. The newer BPF_MAP_TYPE_CGRP_STORAGE is available to
//C 	 * both cgroup-attached and other progs and supports all functionality
//C 	 * provided by BPF_MAP_TYPE_CGROUP_STORAGE. So mark
//C 	 * BPF_MAP_TYPE_CGROUP_STORAGE deprecated.
//C 	 */
//C 	BPF_MAP_TYPE_CGROUP_STORAGE = BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED,
//C 	BPF_MAP_TYPE_REUSEPORT_SOCKARRAY,
//C 	BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE_DEPRECATED,
//C 	/* BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE is available to bpf programs
//C 	 * attaching to a cgroup. The new mechanism (BPF_MAP_TYPE_CGRP_STORAGE +
//C 	 * local percpu kptr) supports all BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE
//C 	 * functionality and more. So mark * BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE
//C 	 * deprecated.
//C 	 */
//C 	BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE = BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE_DEPRECATED,
//C 	BPF_MAP_TYPE_QUEUE,
//C 	BPF_MAP_TYPE_STACK,
//C 	BPF_MAP_TYPE_SK_STORAGE,
//C 	BPF_MAP_TYPE_DEVMAP_HASH,
//C 	BPF_MAP_TYPE_STRUCT_OPS,
//C 	BPF_MAP_TYPE_RINGBUF,
//C 	BPF_MAP_TYPE_INODE_STORAGE,
//C 	BPF_MAP_TYPE_TASK_STORAGE,
//C 	BPF_MAP_TYPE_BLOOM_FILTER,
//C 	BPF_MAP_TYPE_USER_RINGBUF,
//C 	BPF_MAP_TYPE_CGRP_STORAGE,
//C 	BPF_MAP_TYPE_ARENA,
//C 	BPF_MAP_TYPE_INSN_ARRAY,
//C 	BPF_MAP_TYPE_RHASH,
//C 	__MAX_BPF_MAP_TYPE
//C };
//C 
//C /* Note that tracing related programs such as
//C  * BPF_PROG_TYPE_{KPROBE,TRACEPOINT,PERF_EVENT,RAW_TRACEPOINT}
//C  * are not subject to a stable API since kernel internal data
//C  * structures can change from release to release and may
//C  * therefore break existing tracing BPF programs. Tracing BPF
//C  * programs correspond to /a/ specific kernel which is to be
//C  * analyzed, and not /a/ specific kernel /and/ all future ones.
//C  */
//C enum bpf_prog_type {
//C 	BPF_PROG_TYPE_UNSPEC,
//C 	BPF_PROG_TYPE_SOCKET_FILTER,
//C 	BPF_PROG_TYPE_KPROBE,
//C 	BPF_PROG_TYPE_SCHED_CLS,
//C 	BPF_PROG_TYPE_SCHED_ACT,
//C 	BPF_PROG_TYPE_TRACEPOINT,
//C 	BPF_PROG_TYPE_XDP,
//C 	BPF_PROG_TYPE_PERF_EVENT,
//C 	BPF_PROG_TYPE_CGROUP_SKB,
//C 	BPF_PROG_TYPE_CGROUP_SOCK,
//C 	BPF_PROG_TYPE_LWT_IN,
//C 	BPF_PROG_TYPE_LWT_OUT,
//C 	BPF_PROG_TYPE_LWT_XMIT,
//C 	BPF_PROG_TYPE_SOCK_OPS,
//C 	BPF_PROG_TYPE_SK_SKB,
//C 	BPF_PROG_TYPE_CGROUP_DEVICE,
//C 	BPF_PROG_TYPE_SK_MSG,
//C 	BPF_PROG_TYPE_RAW_TRACEPOINT,
//C 	BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
//C 	BPF_PROG_TYPE_LWT_SEG6LOCAL,
//C 	BPF_PROG_TYPE_LIRC_MODE2,
//C 	BPF_PROG_TYPE_SK_REUSEPORT,
//C 	BPF_PROG_TYPE_FLOW_DISSECTOR,
//C 	BPF_PROG_TYPE_CGROUP_SYSCTL,
//C 	BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE,
//C 	BPF_PROG_TYPE_CGROUP_SOCKOPT,
//C 	BPF_PROG_TYPE_TRACING,
//C 	BPF_PROG_TYPE_STRUCT_OPS,
//C 	BPF_PROG_TYPE_EXT,
//C 	BPF_PROG_TYPE_LSM,
//C 	BPF_PROG_TYPE_SK_LOOKUP,
//C 	BPF_PROG_TYPE_SYSCALL, /* a program that can execute syscalls */
//C 	BPF_PROG_TYPE_NETFILTER,
//C 	__MAX_BPF_PROG_TYPE
//C };
//C 
//C enum bpf_attach_type {
//C 	BPF_CGROUP_INET_INGRESS,
//C 	BPF_CGROUP_INET_EGRESS,
//C 	BPF_CGROUP_INET_SOCK_CREATE,
//C 	BPF_CGROUP_SOCK_OPS,
//C 	BPF_SK_SKB_STREAM_PARSER,
//C 	BPF_SK_SKB_STREAM_VERDICT,
//C 	BPF_CGROUP_DEVICE,
//C 	BPF_SK_MSG_VERDICT,
//C 	BPF_CGROUP_INET4_BIND,
//C 	BPF_CGROUP_INET6_BIND,
//C 	BPF_CGROUP_INET4_CONNECT,
//C 	BPF_CGROUP_INET6_CONNECT,
//C 	BPF_CGROUP_INET4_POST_BIND,
//C 	BPF_CGROUP_INET6_POST_BIND,
//C 	BPF_CGROUP_UDP4_SENDMSG,
//C 	BPF_CGROUP_UDP6_SENDMSG,
//C 	BPF_LIRC_MODE2,
//C 	BPF_FLOW_DISSECTOR,
//C 	BPF_CGROUP_SYSCTL,
//C 	BPF_CGROUP_UDP4_RECVMSG,
//C 	BPF_CGROUP_UDP6_RECVMSG,
//C 	BPF_CGROUP_GETSOCKOPT,
//C 	BPF_CGROUP_SETSOCKOPT,
//C 	BPF_TRACE_RAW_TP,
//C 	BPF_TRACE_FENTRY,
//C 	BPF_TRACE_FEXIT,
//C 	BPF_MODIFY_RETURN,
//C 	BPF_LSM_MAC,
//C 	BPF_TRACE_ITER,
//C 	BPF_CGROUP_INET4_GETPEERNAME,
//C 	BPF_CGROUP_INET6_GETPEERNAME,
//C 	BPF_CGROUP_INET4_GETSOCKNAME,
//C 	BPF_CGROUP_INET6_GETSOCKNAME,
//C 	BPF_XDP_DEVMAP,
//C 	BPF_CGROUP_INET_SOCK_RELEASE,
//C 	BPF_XDP_CPUMAP,
//C 	BPF_SK_LOOKUP,
//C 	BPF_XDP,
//C 	BPF_SK_SKB_VERDICT,
//C 	BPF_SK_REUSEPORT_SELECT,
//C 	BPF_SK_REUSEPORT_SELECT_OR_MIGRATE,
//C 	BPF_PERF_EVENT,
//C 	BPF_TRACE_KPROBE_MULTI,
//C 	BPF_LSM_CGROUP,
//C 	BPF_STRUCT_OPS,
//C 	BPF_NETFILTER,
//C 	BPF_TCX_INGRESS,
//C 	BPF_TCX_EGRESS,
//C 	BPF_TRACE_UPROBE_MULTI,
//C 	BPF_CGROUP_UNIX_CONNECT,
//C 	BPF_CGROUP_UNIX_SENDMSG,
//C 	BPF_CGROUP_UNIX_RECVMSG,
//C 	BPF_CGROUP_UNIX_GETPEERNAME,
//C 	BPF_CGROUP_UNIX_GETSOCKNAME,
//C 	BPF_NETKIT_PRIMARY,
//C 	BPF_NETKIT_PEER,
//C 	BPF_TRACE_KPROBE_SESSION,
//C 	BPF_TRACE_UPROBE_SESSION,
//C 	BPF_TRACE_FSESSION,
//C 	BPF_TRACE_FENTRY_MULTI,
//C 	BPF_TRACE_FEXIT_MULTI,
//C 	BPF_TRACE_FSESSION_MULTI,
//C 	__MAX_BPF_ATTACH_TYPE
//C };
//C 
//C #define MAX_BPF_ATTACH_TYPE __MAX_BPF_ATTACH_TYPE
//C 
//C /* Add BPF_LINK_TYPE(type, name) in bpf_types.h to keep bpf_link_type_strs[]
//C  * in sync with the definitions below.
//C  */
//C enum bpf_link_type {
//C 	BPF_LINK_TYPE_UNSPEC = 0,
//C 	BPF_LINK_TYPE_RAW_TRACEPOINT = 1,
//C 	BPF_LINK_TYPE_TRACING = 2,
//C 	BPF_LINK_TYPE_CGROUP = 3,
//C 	BPF_LINK_TYPE_ITER = 4,
//C 	BPF_LINK_TYPE_NETNS = 5,
//C 	BPF_LINK_TYPE_XDP = 6,
//C 	BPF_LINK_TYPE_PERF_EVENT = 7,
//C 	BPF_LINK_TYPE_KPROBE_MULTI = 8,
//C 	BPF_LINK_TYPE_STRUCT_OPS = 9,
//C 	BPF_LINK_TYPE_NETFILTER = 10,
//C 	BPF_LINK_TYPE_TCX = 11,
//C 	BPF_LINK_TYPE_UPROBE_MULTI = 12,
//C 	BPF_LINK_TYPE_NETKIT = 13,
//C 	BPF_LINK_TYPE_SOCKMAP = 14,
//C 	BPF_LINK_TYPE_TRACING_MULTI = 15,
//C 	__MAX_BPF_LINK_TYPE,
//C };
//C 
//C #define MAX_BPF_LINK_TYPE __MAX_BPF_LINK_TYPE
//C 
//C enum bpf_perf_event_type {
//C 	BPF_PERF_EVENT_UNSPEC = 0,
//C 	BPF_PERF_EVENT_UPROBE = 1,
//C 	BPF_PERF_EVENT_URETPROBE = 2,
//C 	BPF_PERF_EVENT_KPROBE = 3,
//C 	BPF_PERF_EVENT_KRETPROBE = 4,
//C 	BPF_PERF_EVENT_TRACEPOINT = 5,
//C 	BPF_PERF_EVENT_EVENT = 6,
//C };
//C 
//C /* cgroup-bpf attach flags used in BPF_PROG_ATTACH command
//C  *
//C  * NONE(default): No further bpf programs allowed in the subtree.
//C  *
//C  * BPF_F_ALLOW_OVERRIDE: If a sub-cgroup installs some bpf program,
//C  * the program in this cgroup yields to sub-cgroup program.
//C  *
//C  * BPF_F_ALLOW_MULTI: If a sub-cgroup installs some bpf program,
//C  * that cgroup program gets run in addition to the program in this cgroup.
//C  *
//C  * Only one program is allowed to be attached to a cgroup with
//C  * NONE or BPF_F_ALLOW_OVERRIDE flag.
//C  * Attaching another program on top of NONE or BPF_F_ALLOW_OVERRIDE will
//C  * release old program and attach the new one. Attach flags has to match.
//C  *
//C  * Multiple programs are allowed to be attached to a cgroup with
//C  * BPF_F_ALLOW_MULTI flag. They are executed in FIFO order
//C  * (those that were attached first, run first)
//C  * The programs of sub-cgroup are executed first, then programs of
//C  * this cgroup and then programs of parent cgroup.
//C  * When children program makes decision (like picking TCP CA or sock bind)
//C  * parent program has a chance to override it.
//C  *
//C  * With BPF_F_ALLOW_MULTI a new program is added to the end of the list of
//C  * programs for a cgroup. Though it's possible to replace an old program at
//C  * any position by also specifying BPF_F_REPLACE flag and position itself in
//C  * replace_bpf_fd attribute. Old program at this position will be released.
//C  *
//C  * A cgroup with MULTI or OVERRIDE flag allows any attach flags in sub-cgroups.
//C  * A cgroup with NONE doesn't allow any programs in sub-cgroups.
//C  * Ex1:
//C  * cgrp1 (MULTI progs A, B) ->
//C  *    cgrp2 (OVERRIDE prog C) ->
//C  *      cgrp3 (MULTI prog D) ->
//C  *        cgrp4 (OVERRIDE prog E) ->
//C  *          cgrp5 (NONE prog F)
//C  * the event in cgrp5 triggers execution of F,D,A,B in that order.
//C  * if prog F is detached, the execution is E,D,A,B
//C  * if prog F and D are detached, the execution is E,A,B
//C  * if prog F, E and D are detached, the execution is C,A,B
//C  *
//C  * All eligible programs are executed regardless of return code from
//C  * earlier programs.
//C  */
//C #define BPF_F_ALLOW_OVERRIDE	(1U << 0)
//C #define BPF_F_ALLOW_MULTI	(1U << 1)
//C /* Generic attachment flags. */
//C #define BPF_F_REPLACE		(1U << 2)
//C #define BPF_F_BEFORE		(1U << 3)
//C #define BPF_F_AFTER		(1U << 4)
//C #define BPF_F_ID		(1U << 5)
//C #define BPF_F_PREORDER		(1U << 6)
//C #define BPF_F_LINK		BPF_F_LINK /* 1 << 13 */
//C 
//C /* If BPF_F_STRICT_ALIGNMENT is used in BPF_PROG_LOAD command, the
//C  * verifier will perform strict alignment checking as if the kernel
//C  * has been built with CONFIG_EFFICIENT_UNALIGNED_ACCESS not set,
//C  * and NET_IP_ALIGN defined to 2.
//C  */
//C #define BPF_F_STRICT_ALIGNMENT	(1U << 0)
//C 
//C /* If BPF_F_ANY_ALIGNMENT is used in BPF_PROG_LOAD command, the
//C  * verifier will allow any alignment whatsoever.  On platforms
//C  * with strict alignment requirements for loads ands stores (such
//C  * as sparc and mips) the verifier validates that all loads and
//C  * stores provably follow this requirement.  This flag turns that
//C  * checking and enforcement off.
//C  *
//C  * It is mostly used for testing when we want to validate the
//C  * context and memory access aspects of the verifier, but because
//C  * of an unaligned access the alignment check would trigger before
//C  * the one we are interested in.
//C  */
//C #define BPF_F_ANY_ALIGNMENT	(1U << 1)
//C 
//C /* BPF_F_TEST_RND_HI32 is used in BPF_PROG_LOAD command for testing purpose.
//C  * Verifier does sub-register def/use analysis and identifies instructions whose
//C  * def only matters for low 32-bit, high 32-bit is never referenced later
//C  * through implicit zero extension. Therefore verifier notifies JIT back-ends
//C  * that it is safe to ignore clearing high 32-bit for these instructions. This
//C  * saves some back-ends a lot of code-gen. However such optimization is not
//C  * necessary on some arches, for example x86_64, arm64 etc, whose JIT back-ends
//C  * hence hasn't used verifier's analysis result. But, we really want to have a
//C  * way to be able to verify the correctness of the described optimization on
//C  * x86_64 on which testsuites are frequently exercised.
//C  *
//C  * So, this flag is introduced. Once it is set, verifier will randomize high
//C  * 32-bit for those instructions who has been identified as safe to ignore them.
//C  * Then, if verifier is not doing correct analysis, such randomization will
//C  * regress tests to expose bugs.
//C  */
//C #define BPF_F_TEST_RND_HI32	(1U << 2)
//C 
//C /* The verifier internal test flag. Behavior is undefined */
//C #define BPF_F_TEST_STATE_FREQ	(1U << 3)
//C 
//C /* If BPF_F_SLEEPABLE is used in BPF_PROG_LOAD command, the verifier will
//C  * restrict map and helper usage for such programs. Sleepable BPF programs can
//C  * only be attached to hooks where kernel execution context allows sleeping.
//C  * Such programs are allowed to use helpers that may sleep like
//C  * bpf_copy_from_user().
//C  */
//C #define BPF_F_SLEEPABLE		(1U << 4)
//C 
//C /* If BPF_F_XDP_HAS_FRAGS is used in BPF_PROG_LOAD command, the loaded program
//C  * fully support xdp frags.
//C  */
//C #define BPF_F_XDP_HAS_FRAGS	(1U << 5)
//C 
//C /* If BPF_F_XDP_DEV_BOUND_ONLY is used in BPF_PROG_LOAD command, the loaded
//C  * program becomes device-bound but can access XDP metadata.
//C  */
//C #define BPF_F_XDP_DEV_BOUND_ONLY	(1U << 6)
//C 
//C /* The verifier internal test flag. Behavior is undefined */
//C #define BPF_F_TEST_REG_INVARIANTS	(1U << 7)
//C 
//C /* link_create.kprobe_multi.flags used in LINK_CREATE command for
//C  * BPF_TRACE_KPROBE_MULTI attach type to create return probe.
//C  */
//C enum {
//C 	BPF_F_KPROBE_MULTI_RETURN = (1U << 0)
//C };
//C 
//C /* link_create.uprobe_multi.flags used in LINK_CREATE command for
//C  * BPF_TRACE_UPROBE_MULTI attach type to create return probe.
//C  */
//C enum {
//C 	/* Get return uprobe. */
//C 	BPF_F_UPROBE_MULTI_RETURN     = (1U << 0),
//C 
//C 	/* Get path from provided path_fd. */
//C 	BPF_F_UPROBE_MULTI_PATH_FD    = (1U << 1),
//C };
//C 
//C /* link_create.netfilter.flags used in LINK_CREATE command for
//C  * BPF_PROG_TYPE_NETFILTER to enable IP packet defragmentation.
//C  */
//C #define BPF_F_NETFILTER_IP_DEFRAG (1U << 0)
//C 
//C /* When BPF ldimm64's insn[0].src_reg != 0 then this can have
//C  * the following extensions:
//C  *
//C  * insn[0].src_reg:  BPF_PSEUDO_MAP_[FD|IDX]
//C  * insn[0].imm:      map fd or fd_idx
//C  * insn[1].imm:      0
//C  * insn[0].off:      0
//C  * insn[1].off:      0
//C  * ldimm64 rewrite:  address of map
//C  * verifier type:    CONST_PTR_TO_MAP
//C  */
//C #define BPF_PSEUDO_MAP_FD	1
//C #define BPF_PSEUDO_MAP_IDX	5
//C 
//C /* insn[0].src_reg:  BPF_PSEUDO_MAP_[IDX_]VALUE
//C  * insn[0].imm:      map fd or fd_idx
//C  * insn[1].imm:      offset into value
//C  * insn[0].off:      0
//C  * insn[1].off:      0
//C  * ldimm64 rewrite:  address of map[0]+offset
//C  * verifier type:    PTR_TO_MAP_VALUE
//C  */
//C #define BPF_PSEUDO_MAP_VALUE		2
//C #define BPF_PSEUDO_MAP_IDX_VALUE	6
//C 
//C /* insn[0].src_reg:  BPF_PSEUDO_BTF_ID
//C  * insn[0].imm:      kernel btd id of VAR
//C  * insn[1].imm:      0
//C  * insn[0].off:      0
//C  * insn[1].off:      0
//C  * ldimm64 rewrite:  address of the kernel variable
//C  * verifier type:    PTR_TO_BTF_ID or PTR_TO_MEM, depending on whether the var
//C  *                   is struct/union.
//C  */
//C #define BPF_PSEUDO_BTF_ID	3
//C /* insn[0].src_reg:  BPF_PSEUDO_FUNC
//C  * insn[0].imm:      insn offset to the func
//C  * insn[1].imm:      0
//C  * insn[0].off:      0
//C  * insn[1].off:      0
//C  * ldimm64 rewrite:  address of the function
//C  * verifier type:    PTR_TO_FUNC.
//C  */
//C #define BPF_PSEUDO_FUNC		4
//C 
//C /* when bpf_call->src_reg == BPF_PSEUDO_CALL, bpf_call->imm == pc-relative
//C  * offset to another bpf function
//C  */
//C #define BPF_PSEUDO_CALL		1
//C /* when bpf_call->src_reg == BPF_PSEUDO_KFUNC_CALL,
//C  * bpf_call->imm == btf_id of a BTF_KIND_FUNC in the running kernel
//C  */
//C #define BPF_PSEUDO_KFUNC_CALL	2
//C 
//C enum bpf_addr_space_cast {
//C 	BPF_ADDR_SPACE_CAST = 1,
//C };
//C 
//C /* flags for BPF_MAP_UPDATE_ELEM command */
//C enum {
//C 	BPF_ANY		= 0, /* create new element or update existing */
//C 	BPF_NOEXIST	= 1, /* create new element if it didn't exist */
//C 	BPF_EXIST	= 2, /* update existing element */
//C 	BPF_F_LOCK	= 4, /* spin_lock-ed map_lookup/map_update */
//C 	BPF_F_CPU	= 8, /* cpu flag for percpu maps, upper 32-bit of flags is a cpu number */
//C 	BPF_F_ALL_CPUS	= 16, /* update value across all CPUs for percpu maps */
//C };
//C 
//C /* flags for BPF_MAP_CREATE command */
//C enum {
//C 	BPF_F_NO_PREALLOC	= (1U << 0),
//C /* Instead of having one common LRU list in the
//C  * BPF_MAP_TYPE_LRU_[PERCPU_]HASH map, use a percpu LRU list
//C  * which can scale and perform better.
//C  * Note, the LRU nodes (including free nodes) cannot be moved
//C  * across different LRU lists.
//C  */
//C 	BPF_F_NO_COMMON_LRU	= (1U << 1),
//C /* Specify numa node during map creation */
//C 	BPF_F_NUMA_NODE		= (1U << 2),
//C 
//C /* Flags for accessing BPF object from syscall side. */
//C 	BPF_F_RDONLY		= (1U << 3),
//C 	BPF_F_WRONLY		= (1U << 4),
//C 
//C /* Flag for stack_map, store build_id+offset instead of pointer */
//C 	BPF_F_STACK_BUILD_ID	= (1U << 5),
//C 
//C /* Zero-initialize hash function seed. This should only be used for testing. */
//C 	BPF_F_ZERO_SEED		= (1U << 6),
//C 
//C /* Flags for accessing BPF object from program side. */
//C 	BPF_F_RDONLY_PROG	= (1U << 7),
//C 	BPF_F_WRONLY_PROG	= (1U << 8),
//C 
//C /* Clone map from listener for newly accepted socket */
//C 	BPF_F_CLONE		= (1U << 9),
//C 
//C /* Enable memory-mapping BPF map */
//C 	BPF_F_MMAPABLE		= (1U << 10),
//C 
//C /* Share perf_event among processes */
//C 	BPF_F_PRESERVE_ELEMS	= (1U << 11),
//C 
//C /* Create a map that is suitable to be an inner map with dynamic max entries */
//C 	BPF_F_INNER_MAP		= (1U << 12),
//C 
//C /* Create a map that will be registered/unregesitered by the backed bpf_link */
//C 	BPF_F_LINK		= (1U << 13),
//C 
//C /* Get path from provided FD in BPF_OBJ_PIN/BPF_OBJ_GET commands */
//C 	BPF_F_PATH_FD		= (1U << 14),
//C 
//C /* Flag for value_type_btf_obj_fd, the fd is available */
//C 	BPF_F_VTYPE_BTF_OBJ_FD	= (1U << 15),
//C 
//C /* BPF token FD is passed in a corresponding command's token_fd field */
//C 	BPF_F_TOKEN_FD          = (1U << 16),
//C 
//C /* When user space page faults in bpf_arena send SIGSEGV instead of inserting new page */
//C 	BPF_F_SEGV_ON_FAULT	= (1U << 17),
//C 
//C /* Do not translate kernel bpf_arena pointers to user pointers */
//C 	BPF_F_NO_USER_CONV	= (1U << 18),
//C 
//C /* Enable BPF ringbuf overwrite mode */
//C 	BPF_F_RB_OVERWRITE	= (1U << 19),
//C };
//C 
//C /* Flags for BPF_PROG_QUERY. */
//C 
//C /* Query effective (directly attached + inherited from ancestor cgroups)
//C  * programs that will be executed for events within a cgroup.
//C  * attach_flags with this flag are always returned 0.
//C  */
//C #define BPF_F_QUERY_EFFECTIVE	(1U << 0)
//C 
//C /* Flags for BPF_PROG_TEST_RUN */
//C 
//C /* If set, run the test on the cpu specified by bpf_attr.test.cpu */
//C #define BPF_F_TEST_RUN_ON_CPU	(1U << 0)
//C /* If set, XDP frames will be transmitted after processing */
//C #define BPF_F_TEST_XDP_LIVE_FRAMES	(1U << 1)
//C /* If set, apply CHECKSUM_COMPLETE to skb and validate the checksum */
//C #define BPF_F_TEST_SKB_CHECKSUM_COMPLETE	(1U << 2)
//C 
//C /* type for BPF_ENABLE_STATS */
//C enum bpf_stats_type {
//C 	/* enabled run_time_ns and run_cnt */
//C 	BPF_STATS_RUN_TIME = 0,
//C };
//C 
//C enum bpf_stack_build_id_status {
//C 	/* user space need an empty entry to identify end of a trace */
//C 	BPF_STACK_BUILD_ID_EMPTY = 0,
//C 	/* with valid build_id and offset */
//C 	BPF_STACK_BUILD_ID_VALID = 1,
//C 	/* couldn't get build_id, fallback to ip */
//C 	BPF_STACK_BUILD_ID_IP = 2,
//C };
//C 
//C #define BPF_BUILD_ID_SIZE 20
//C struct bpf_stack_build_id {
//C 	__s32		status;
//C 	unsigned char	build_id[BPF_BUILD_ID_SIZE];
//C 	union {
//C 		__u64	offset;
//C 		__u64	ip;
//C 	};
//C };
//C 
//C struct bpf_common_attr {
//C 	__aligned_u64 log_buf;
//C 	__u32 log_size;
//C 	__u32 log_level;
//C 	__u32 log_true_size;
//C };
//C 
//C #define BPF_OBJ_NAME_LEN 16U
//C 
//C enum {
//C 	BPF_STREAM_STDOUT = 1,
//C 	BPF_STREAM_STDERR = 2,
//C };
//C 
//C union bpf_attr {
//C 	struct { /* anonymous struct used by BPF_MAP_CREATE command */
//C 		__u32	map_type;	/* one of enum bpf_map_type */
//C 		__u32	key_size;	/* size of key in bytes */
//C 		__u32	value_size;	/* size of value in bytes */
//C 		__u32	max_entries;	/* max number of entries in a map */
//C 		__u32	map_flags;	/* BPF_MAP_CREATE related
//C 					 * flags defined above.
//C 					 */
//C 		__u32	inner_map_fd;	/* fd pointing to the inner map */
//C 		__u32	numa_node;	/* numa node (effective only if
//C 					 * BPF_F_NUMA_NODE is set).
//C 					 */
//C 		char	map_name[BPF_OBJ_NAME_LEN];
//C 		__u32	map_ifindex;	/* ifindex of netdev to create on */
//C 		__u32	btf_fd;		/* fd pointing to a BTF type data */
//C 		__u32	btf_key_type_id;	/* BTF type_id of the key */
//C 		__u32	btf_value_type_id;	/* BTF type_id of the value */
//C 		__u32	btf_vmlinux_value_type_id;/* BTF type_id of a kernel-
//C 						   * struct stored as the
//C 						   * map value
//C 						   */
//C 		/* Any per-map-type extra fields
//C 		 *
//C 		 * BPF_MAP_TYPE_BLOOM_FILTER - the lowest 4 bits indicate the
//C 		 * number of hash functions (if 0, the bloom filter will default
//C 		 * to using 5 hash functions).
//C 		 *
//C 		 * BPF_MAP_TYPE_ARENA - contains the address where user space
//C 		 * is going to mmap() the arena. It has to be page aligned.
//C 		 *
//C 		 * BPF_MAP_TYPE_RHASH - initial table size hint
//C 		 * (nelem_hint). 0 = use rhashtable default. Must be
//C 		 * <= min(max_entries, U16_MAX). Upper 32 bits reserved,
//C 		 * must be zero.
//C 		 */
//C 		__u64	map_extra;
//C 
//C 		__s32   value_type_btf_obj_fd;	/* fd pointing to a BTF
//C 						 * type data for
//C 						 * btf_vmlinux_value_type_id.
//C 						 */
//C 		/* BPF token FD to use with BPF_MAP_CREATE operation.
//C 		 * If provided, map_flags should have BPF_F_TOKEN_FD flag set.
//C 		 */
//C 		__s32	map_token_fd;
//C 
//C 		/* Hash of the program that has exclusive access to the map.
//C 		 */
//C 		__aligned_u64 excl_prog_hash;
//C 		/* Size of the passed excl_prog_hash. */
//C 		__u32 excl_prog_hash_size;
//C 	};
//C 
//C 	struct { /* anonymous struct used by BPF_MAP_*_ELEM and BPF_MAP_FREEZE commands */
//C 		__u32		map_fd;
//C 		__aligned_u64	key;
//C 		union {
//C 			__aligned_u64 value;
//C 			__aligned_u64 next_key;
//C 		};
//C 		__u64		flags;
//C 	};
//C 
//C 	struct { /* struct used by BPF_MAP_*_BATCH commands */
//C 		__aligned_u64	in_batch;	/* start batch,
//C 						 * NULL to start from beginning
//C 						 */
//C 		__aligned_u64	out_batch;	/* output: next start batch */
//C 		__aligned_u64	keys;
//C 		__aligned_u64	values;
//C 		__u32		count;		/* input/output:
//C 						 * input: # of key/value
//C 						 * elements
//C 						 * output: # of filled elements
//C 						 */
//C 		__u32		map_fd;
//C 		__u64		elem_flags;
//C 		__u64		flags;
//C 	} batch;
//C 
//C 	struct { /* anonymous struct used by BPF_PROG_LOAD command */
//C 		__u32		prog_type;	/* one of enum bpf_prog_type */
//C 		__u32		insn_cnt;
//C 		__aligned_u64	insns;
//C 		__aligned_u64	license;
//C 		__u32		log_level;	/* verbosity level of verifier */
//C 		__u32		log_size;	/* size of user buffer */
//C 		__aligned_u64	log_buf;	/* user supplied buffer */
//C 		__u32		kern_version;	/* not used */
//C 		__u32		prog_flags;
//C 		char		prog_name[BPF_OBJ_NAME_LEN];
//C 		__u32		prog_ifindex;	/* ifindex of netdev to prep for */
//C 		/* For some prog types expected attach type must be known at
//C 		 * load time to verify attach type specific parts of prog
//C 		 * (context accesses, allowed helpers, etc).
//C 		 */
//C 		__u32		expected_attach_type;
//C 		__u32		prog_btf_fd;	/* fd pointing to BTF type data */
//C 		__u32		func_info_rec_size;	/* userspace bpf_func_info size */
//C 		__aligned_u64	func_info;	/* func info */
//C 		__u32		func_info_cnt;	/* number of bpf_func_info records */
//C 		__u32		line_info_rec_size;	/* userspace bpf_line_info size */
//C 		__aligned_u64	line_info;	/* line info */
//C 		__u32		line_info_cnt;	/* number of bpf_line_info records */
//C 		__u32		attach_btf_id;	/* in-kernel BTF type id to attach to */
//C 		union {
//C 			/* valid prog_fd to attach to bpf prog */
//C 			__u32		attach_prog_fd;
//C 			/* or valid module BTF object fd or 0 to attach to vmlinux */
//C 			__u32		attach_btf_obj_fd;
//C 		};
//C 		__u32		core_relo_cnt;	/* number of bpf_core_relo */
//C 		__aligned_u64	fd_array;	/* array of FDs */
//C 		__aligned_u64	core_relos;
//C 		__u32		core_relo_rec_size; /* sizeof(struct bpf_core_relo) */
//C 		/* output: actual total log contents size (including termintaing zero).
//C 		 * It could be both larger than original log_size (if log was
//C 		 * truncated), or smaller (if log buffer wasn't filled completely).
//C 		 */
//C 		__u32		log_true_size;
//C 		/* BPF token FD to use with BPF_PROG_LOAD operation.
//C 		 * If provided, prog_flags should have BPF_F_TOKEN_FD flag set.
//C 		 */
//C 		__s32		prog_token_fd;
//C 		/* The fd_array_cnt can be used to pass the length of the
//C 		 * fd_array array. In this case all the [map] file descriptors
//C 		 * passed in this array will be bound to the program, even if
//C 		 * the maps are not referenced directly. The functionality is
//C 		 * similar to the BPF_PROG_BIND_MAP syscall, but maps can be
//C 		 * used by the verifier during the program load. If provided,
//C 		 * then the fd_array[0,...,fd_array_cnt-1] is expected to be
//C 		 * continuous.
//C 		 */
//C 		__u32		fd_array_cnt;
//C 		/* Pointer to a buffer containing the signature of the BPF
//C 		 * program.
//C 		 */
//C 		__aligned_u64   signature;
//C 		/* Size of the signature buffer in bytes. */
//C 		__u32 		signature_size;
//C 		/* ID of the kernel keyring to be used for signature
//C 		 * verification.
//C 		 */
//C 		__s32		keyring_id;
//C 	};
//C 
//C 	struct { /* anonymous struct used by BPF_OBJ_* commands */
//C 		__aligned_u64	pathname;
//C 		__u32		bpf_fd;
//C 		__u32		file_flags;
//C 		/* Same as dirfd in openat() syscall; see openat(2)
//C 		 * manpage for details of path FD and pathname semantics;
//C 		 * path_fd should accompanied by BPF_F_PATH_FD flag set in
//C 		 * file_flags field, otherwise it should be set to zero;
//C 		 * if BPF_F_PATH_FD flag is not set, AT_FDCWD is assumed.
//C 		 */
//C 		__s32		path_fd;
//C 	};
//C 
//C 	struct { /* anonymous struct used by BPF_PROG_ATTACH/DETACH commands */
//C 		union {
//C 			__u32	target_fd;	/* target object to attach to or ... */
//C 			__u32	target_ifindex;	/* target ifindex */
//C 		};
//C 		__u32		attach_bpf_fd;
//C 		__u32		attach_type;
//C 		__u32		attach_flags;
//C 		__u32		replace_bpf_fd;
//C 		union {
//C 			__u32	relative_fd;
//C 			__u32	relative_id;
//C 		};
//C 		__u64		expected_revision;
//C 	};
//C 
//C 	struct { /* anonymous struct used by BPF_PROG_TEST_RUN command */
//C 		__u32		prog_fd;
//C 		__u32		retval;
//C 		__u32		data_size_in;	/* input: len of data_in */
//C 		__u32		data_size_out;	/* input/output: len of data_out
//C 						 *   returns ENOSPC if data_out
//C 						 *   is too small.
//C 						 */
//C 		__aligned_u64	data_in;
//C 		__aligned_u64	data_out;
//C 		__u32		repeat;
//C 		__u32		duration;
//C 		__u32		ctx_size_in;	/* input: len of ctx_in */
//C 		__u32		ctx_size_out;	/* input/output: len of ctx_out
//C 						 *   returns ENOSPC if ctx_out
//C 						 *   is too small.
//C 						 */
//C 		__aligned_u64	ctx_in;
//C 		__aligned_u64	ctx_out;
//C 		__u32		flags;
//C 		__u32		cpu;
//C 		__u32		batch_size;
//C 	} test;
//C 
//C 	struct { /* anonymous struct used by BPF_*_GET_*_ID */
//C 		union {
//C 			__u32		start_id;
//C 			__u32		prog_id;
//C 			__u32		map_id;
//C 			__u32		btf_id;
//C 			__u32		link_id;
//C 		};
//C 		__u32		next_id;
//C 		__u32		open_flags;
//C 		__s32		fd_by_id_token_fd;
//C 	};
//C 
//C 	struct { /* anonymous struct used by BPF_OBJ_GET_INFO_BY_FD */
//C 		__u32		bpf_fd;
//C 		__u32		info_len;
//C 		__aligned_u64	info;
//C 	} info;
//C 
//C 	struct { /* anonymous struct used by BPF_PROG_QUERY command */
//C 		union {
//C 			__u32	target_fd;	/* target object to query or ... */
//C 			__u32	target_ifindex;	/* target ifindex */
//C 		};
//C 		__u32		attach_type;
//C 		__u32		query_flags;
//C 		__u32		attach_flags;
//C 		__aligned_u64	prog_ids;
//C 		union {
//C 			__u32	prog_cnt;
//C 			__u32	count;
//C 		};
//C 		__u32		:32;
//C 		/* output: per-program attach_flags.
//C 		 * not allowed to be set during effective query.
//C 		 */
//C 		__aligned_u64	prog_attach_flags;
//C 		__aligned_u64	link_ids;
//C 		__aligned_u64	link_attach_flags;
//C 		__u64		revision;
//C 	} query;
//C 
//C 	struct { /* anonymous struct used by BPF_RAW_TRACEPOINT_OPEN command */
//C 		__u64		name;
//C 		__u32		prog_fd;
//C 		__u32		:32;
//C 		__aligned_u64	cookie;
//C 	} raw_tracepoint;
//C 
//C 	struct { /* anonymous struct for BPF_BTF_LOAD */
//C 		__aligned_u64	btf;
//C 		__aligned_u64	btf_log_buf;
//C 		__u32		btf_size;
//C 		__u32		btf_log_size;
//C 		__u32		btf_log_level;
//C 		/* output: actual total log contents size (including termintaing zero).
//C 		 * It could be both larger than original log_size (if log was
//C 		 * truncated), or smaller (if log buffer wasn't filled completely).
//C 		 */
//C 		__u32		btf_log_true_size;
//C 		__u32		btf_flags;
//C 		/* BPF token FD to use with BPF_BTF_LOAD operation.
//C 		 * If provided, btf_flags should have BPF_F_TOKEN_FD flag set.
//C 		 */
//C 		__s32		btf_token_fd;
//C 	};
//C 
//C 	struct {
//C 		__u32		pid;		/* input: pid */
//C 		__u32		fd;		/* input: fd */
//C 		__u32		flags;		/* input: flags */
//C 		__u32		buf_len;	/* input/output: buf len */
//C 		__aligned_u64	buf;		/* input/output:
//C 						 *   tp_name for tracepoint
//C 						 *   symbol for kprobe
//C 						 *   filename for uprobe
//C 						 */
//C 		__u32		prog_id;	/* output: prod_id */
//C 		__u32		fd_type;	/* output: BPF_FD_TYPE_* */
//C 		__u64		probe_offset;	/* output: probe_offset */
//C 		__u64		probe_addr;	/* output: probe_addr */
//C 	} task_fd_query;
//C 
//C 	struct { /* struct used by BPF_LINK_CREATE command */
//C 		union {
//C 			__u32		prog_fd;	/* eBPF program to attach */
//C 			__u32		map_fd;		/* struct_ops to attach */
//C 		};
//C 		union {
//C 			__u32	target_fd;	/* target object to attach to or ... */
//C 			__u32	target_ifindex; /* target ifindex */
//C 		};
//C 		__u32		attach_type;	/* attach type */
//C 		__u32		flags;		/* extra flags */
//C 		union {
//C 			__u32	target_btf_id;	/* btf_id of target to attach to */
//C 			struct {
//C 				__aligned_u64	iter_info;	/* extra bpf_iter_link_info */
//C 				__u32		iter_info_len;	/* iter_info length */
//C 			};
//C 			struct {
//C 				/* black box user-provided value passed through
//C 				 * to BPF program at the execution time and
//C 				 * accessible through bpf_get_attach_cookie() BPF helper
//C 				 */
//C 				__u64		bpf_cookie;
//C 			} perf_event;
//C 			struct {
//C 				__u32		flags;
//C 				__u32		cnt;
//C 				__aligned_u64	syms;
//C 				__aligned_u64	addrs;
//C 				__aligned_u64	cookies;
//C 			} kprobe_multi;
//C 			struct {
//C 				/* this is overlaid with the target_btf_id above. */
//C 				__u32		target_btf_id;
//C 				/* black box user-provided value passed through
//C 				 * to BPF program at the execution time and
//C 				 * accessible through bpf_get_attach_cookie() BPF helper
//C 				 */
//C 				__u64		cookie;
//C 			} tracing;
//C 			struct {
//C 				__u32		pf;
//C 				__u32		hooknum;
//C 				__s32		priority;
//C 				__u32		flags;
//C 			} netfilter;
//C 			struct {
//C 				union {
//C 					__u32	relative_fd;
//C 					__u32	relative_id;
//C 				};
//C 				__u64		expected_revision;
//C 			} tcx;
//C 			struct {
//C 				__aligned_u64	path;
//C 				__aligned_u64	offsets;
//C 				__aligned_u64	ref_ctr_offsets;
//C 				__aligned_u64	cookies;
//C 				__u32		cnt;
//C 				__u32		flags;
//C 				__u32		pid;
//C 				__u32		path_fd;
//C 			} uprobe_multi;
//C 			struct {
//C 				union {
//C 					__u32	relative_fd;
//C 					__u32	relative_id;
//C 				};
//C 				__u64		expected_revision;
//C 			} netkit;
//C 			struct {
//C 				union {
//C 					__u32	relative_fd;
//C 					__u32	relative_id;
//C 				};
//C 				__u64		expected_revision;
//C 			} cgroup;
//C 			struct {
//C 				__aligned_u64	ids;
//C 				__aligned_u64	cookies;
//C 				__u32		cnt;
//C 			} tracing_multi;
//C 		};
//C 	} link_create;
//C 
//C 	struct { /* struct used by BPF_LINK_UPDATE command */
//C 		__u32		link_fd;	/* link fd */
//C 		union {
//C 			/* new program fd to update link with */
//C 			__u32		new_prog_fd;
//C 			/* new struct_ops map fd to update link with */
//C 			__u32           new_map_fd;
//C 		};
//C 		__u32		flags;		/* extra flags */
//C 		union {
//C 			/* expected link's program fd; is specified only if
//C 			 * BPF_F_REPLACE flag is set in flags.
//C 			 */
//C 			__u32		old_prog_fd;
//C 			/* expected link's map fd; is specified only
//C 			 * if BPF_F_REPLACE flag is set.
//C 			 */
//C 			__u32           old_map_fd;
//C 		};
//C 	} link_update;
//C 
//C 	struct {
//C 		__u32		link_fd;
//C 	} link_detach;
//C 
//C 	struct { /* struct used by BPF_ENABLE_STATS command */
//C 		__u32		type;
//C 	} enable_stats;
//C 
//C 	struct { /* struct used by BPF_ITER_CREATE command */
//C 		__u32		link_fd;
//C 		__u32		flags;
//C 	} iter_create;
//C 
//C 	struct { /* struct used by BPF_PROG_BIND_MAP command */
//C 		__u32		prog_fd;
//C 		__u32		map_fd;
//C 		__u32		flags;		/* extra flags */
//C 	} prog_bind_map;
//C 
//C 	struct { /* struct used by BPF_TOKEN_CREATE command */
//C 		__u32		flags;
//C 		__u32		bpffs_fd;
//C 	} token_create;
//C 
//C 	struct {
//C 		__aligned_u64	stream_buf;
//C 		__u32		stream_buf_len;
//C 		__u32		stream_id;
//C 		__u32		prog_fd;
//C 	} prog_stream_read;
//C 
//C 	struct {
//C 		__u32		map_fd;
//C 		__u32		prog_fd;
//C 		__u32		flags;
//C 	} prog_assoc_struct_ops;
//C 
//C } __attribute__((aligned(8)));
//C 
//C /* The description below is an attempt at providing documentation to eBPF
//C  * developers about the multiple available eBPF helper functions. It can be
//C  * parsed and used to produce a manual page. The workflow is the following,
//C  * and requires the rst2man utility:
//C  *
//C  *     $ ./scripts/bpf_doc.py \
//C  *             --filename include/uapi/linux/bpf.h > /tmp/bpf-helpers.rst
//C  *     $ rst2man /tmp/bpf-helpers.rst > /tmp/bpf-helpers.7
//C  *     $ man /tmp/bpf-helpers.7
//C  *
//C  * Note that in order to produce this external documentation, some RST
//C  * formatting is used in the descriptions to get "bold" and "italics" in
//C  * manual pages. Also note that the few trailing white spaces are
//C  * intentional, removing them would break paragraphs for rst2man.
//C  *
//C  * Start of BPF helper function descriptions:
//C  *
//C  * void *bpf_map_lookup_elem(struct bpf_map *map, const void *key)
//C  * 	Description
//C  * 		Perform a lookup in *map* for an entry associated to *key*.
//C  * 	Return
//C  * 		Map value associated to *key*, or **NULL** if no entry was
//C  * 		found.
//C  *
//C  * long bpf_map_update_elem(struct bpf_map *map, const void *key, const void *value, u64 flags)
//C  * 	Description
//C  * 		Add or update the value of the entry associated to *key* in
//C  * 		*map* with *value*. *flags* is one of:
//C  *
//C  * 		**BPF_NOEXIST**
//C  * 			The entry for *key* must not exist in the map.
//C  * 		**BPF_EXIST**
//C  * 			The entry for *key* must already exist in the map.
//C  * 		**BPF_ANY**
//C  * 			No condition on the existence of the entry for *key*.
//C  *
//C  * 		Flag value **BPF_NOEXIST** cannot be used for maps of types
//C  * 		**BPF_MAP_TYPE_ARRAY** or **BPF_MAP_TYPE_PERCPU_ARRAY**  (all
//C  * 		elements always exist), the helper would return an error.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_map_delete_elem(struct bpf_map *map, const void *key)
//C  * 	Description
//C  * 		Delete entry with *key* from *map*.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_probe_read(void *dst, u32 size, const void *unsafe_ptr)
//C  * 	Description
//C  * 		For tracing programs, safely attempt to read *size* bytes from
//C  * 		kernel space address *unsafe_ptr* and store the data in *dst*.
//C  *
//C  * 		Generally, use **bpf_probe_read_user**\ () or
//C  * 		**bpf_probe_read_kernel**\ () instead.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * u64 bpf_ktime_get_ns(void)
//C  * 	Description
//C  * 		Return the time elapsed since system boot, in nanoseconds.
//C  * 		Does not include time the system was suspended.
//C  * 		See: **clock_gettime**\ (**CLOCK_MONOTONIC**)
//C  * 	Return
//C  * 		Current *ktime*.
//C  *
//C  * long bpf_trace_printk(const char *fmt, u32 fmt_size, ...)
//C  * 	Description
//C  * 		This helper is a "printk()-like" facility for debugging. It
//C  * 		prints a message defined by format *fmt* (of size *fmt_size*)
//C  * 		to file *\/sys/kernel/tracing/trace* from TraceFS, if
//C  * 		available. It can take up to three additional **u64**
//C  * 		arguments (as an eBPF helpers, the total number of arguments is
//C  * 		limited to five).
//C  *
//C  * 		Each time the helper is called, it appends a line to the trace.
//C  * 		Lines are discarded while *\/sys/kernel/tracing/trace* is
//C  * 		open, use *\/sys/kernel/tracing/trace_pipe* to avoid this.
//C  * 		The format of the trace is customizable, and the exact output
//C  * 		one will get depends on the options set in
//C  * 		*\/sys/kernel/tracing/trace_options* (see also the
//C  * 		*README* file under the same directory). However, it usually
//C  * 		defaults to something like:
//C  *
//C  * 		::
//C  *
//C  * 			telnet-470   [001] .N.. 419421.045894: 0x00000001: <formatted msg>
//C  *
//C  * 		In the above:
//C  *
//C  * 			* ``telnet`` is the name of the current task.
//C  * 			* ``470`` is the PID of the current task.
//C  * 			* ``001`` is the CPU number on which the task is
//C  * 			  running.
//C  * 			* In ``.N..``, each character refers to a set of
//C  * 			  options (whether irqs are enabled, scheduling
//C  * 			  options, whether hard/softirqs are running, level of
//C  * 			  preempt_disabled respectively). **N** means that
//C  * 			  **TIF_NEED_RESCHED** and **PREEMPT_NEED_RESCHED**
//C  * 			  are set.
//C  * 			* ``419421.045894`` is a timestamp.
//C  * 			* ``0x00000001`` is a fake value used by BPF for the
//C  * 			  instruction pointer register.
//C  * 			* ``<formatted msg>`` is the message formatted with
//C  * 			  *fmt*.
//C  *
//C  * 		The conversion specifiers supported by *fmt* are similar, but
//C  * 		more limited than for printk(). They are **%d**, **%i**,
//C  * 		**%u**, **%x**, **%ld**, **%li**, **%lu**, **%lx**, **%lld**,
//C  * 		**%lli**, **%llu**, **%llx**, **%p**, **%s**. No modifier (size
//C  * 		of field, padding with zeroes, etc.) is available, and the
//C  * 		helper will return **-EINVAL** (but print nothing) if it
//C  * 		encounters an unknown specifier.
//C  *
//C  * 		Also, note that **bpf_trace_printk**\ () is slow, and should
//C  * 		only be used for debugging purposes. For this reason, a notice
//C  * 		block (spanning several lines) is printed to kernel logs and
//C  * 		states that the helper should not be used "for production use"
//C  * 		the first time this helper is used (or more precisely, when
//C  * 		**trace_printk**\ () buffers are allocated). For passing values
//C  * 		to user space, perf events should be preferred.
//C  * 	Return
//C  * 		The number of bytes written to the buffer, or a negative error
//C  * 		in case of failure.
//C  *
//C  * u32 bpf_get_prandom_u32(void)
//C  * 	Description
//C  * 		Get a pseudo-random number.
//C  *
//C  * 		From a security point of view, this helper uses its own
//C  * 		pseudo-random internal state, and cannot be used to infer the
//C  * 		seed of other random functions in the kernel. However, it is
//C  * 		essential to note that the generator used by the helper is not
//C  * 		cryptographically secure.
//C  * 	Return
//C  * 		A random 32-bit unsigned value.
//C  *
//C  * u32 bpf_get_smp_processor_id(void)
//C  * 	Description
//C  * 		Get the SMP (symmetric multiprocessing) processor id. Note that
//C  * 		all programs run with migration disabled, which means that the
//C  * 		SMP processor id is stable during all the execution of the
//C  * 		program.
//C  * 	Return
//C  * 		The SMP id of the processor running the program.
//C  * 	Attributes
//C  * 		__bpf_fastcall
//C  *
//C  * long bpf_skb_store_bytes(struct sk_buff *skb, u32 offset, const void *from, u32 len, u64 flags)
//C  * 	Description
//C  * 		Store *len* bytes from address *from* into the packet
//C  * 		associated to *skb*, at *offset*. The *flags* are a combination
//C  * 		of the following values:
//C  *
//C  * 		**BPF_F_RECOMPUTE_CSUM**
//C  * 			Automatically update *skb*\ **->csum** after storing the
//C  * 			bytes.
//C  * 		**BPF_F_INVALIDATE_HASH**
//C  * 			Set *skb*\ **->hash**, *skb*\ **->swhash** and *skb*\
//C  * 			**->l4hash** to 0.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_l3_csum_replace(struct sk_buff *skb, u32 offset, u64 from, u64 to, u64 size)
//C  * 	Description
//C  * 		Recompute the layer 3 (e.g. IP) checksum for the packet
//C  * 		associated to *skb*. Computation is incremental, so the helper
//C  * 		must know the former value of the header field that was
//C  * 		modified (*from*), the new value of this field (*to*), and the
//C  * 		number of bytes (2 or 4) for this field, stored in *size*.
//C  * 		Alternatively, it is possible to store the difference between
//C  * 		the previous and the new values of the header field in *to*, by
//C  * 		setting *from* and *size* to 0. For both methods, *offset*
//C  * 		indicates the location of the IP checksum within the packet.
//C  *
//C  * 		This helper works in combination with **bpf_csum_diff**\ (),
//C  * 		which does not update the checksum in-place, but offers more
//C  * 		flexibility and can handle sizes larger than 2 or 4 for the
//C  * 		checksum to update.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_l4_csum_replace(struct sk_buff *skb, u32 offset, u64 from, u64 to, u64 flags)
//C  * 	Description
//C  * 		Recompute the layer 4 (e.g. TCP, UDP or ICMP) checksum for the
//C  * 		packet associated to *skb*. Computation is incremental, so the
//C  * 		helper must know the former value of the header field that was
//C  * 		modified (*from*), the new value of this field (*to*), and the
//C  * 		number of bytes (2 or 4) for this field, stored on the lowest
//C  * 		four bits of *flags*. Alternatively, it is possible to store
//C  * 		the difference between the previous and the new values of the
//C  * 		header field in *to*, by setting *from* and the four lowest
//C  * 		bits of *flags* to 0. For both methods, *offset* indicates the
//C  * 		location of the IP checksum within the packet. In addition to
//C  * 		the size of the field, *flags* can be added (bitwise OR) actual
//C  * 		flags. With **BPF_F_MARK_MANGLED_0**, a null checksum is left
//C  * 		untouched (unless **BPF_F_MARK_ENFORCE** is added as well), and
//C  * 		for updates resulting in a null checksum the value is set to
//C  * 		**CSUM_MANGLED_0** instead. Flag **BPF_F_PSEUDO_HDR** indicates
//C  * 		that the modified header field is part of the pseudo-header.
//C  * 		Flag **BPF_F_IPV6** should be set for IPv6 packets.
//C  *
//C  * 		This helper works in combination with **bpf_csum_diff**\ (),
//C  * 		which does not update the checksum in-place, but offers more
//C  * 		flexibility and can handle sizes larger than 2 or 4 for the
//C  * 		checksum to update.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_tail_call(void *ctx, struct bpf_map *prog_array_map, u32 index)
//C  * 	Description
//C  * 		This special helper is used to trigger a "tail call", or in
//C  * 		other words, to jump into another eBPF program. The same stack
//C  * 		frame is used (but values on stack and in registers for the
//C  * 		caller are not accessible to the callee). This mechanism allows
//C  * 		for program chaining, either for raising the maximum number of
//C  * 		available eBPF instructions, or to execute given programs in
//C  * 		conditional blocks. For security reasons, there is an upper
//C  * 		limit to the number of successive tail calls that can be
//C  * 		performed.
//C  *
//C  * 		Upon call of this helper, the program attempts to jump into a
//C  * 		program referenced at index *index* in *prog_array_map*, a
//C  * 		special map of type **BPF_MAP_TYPE_PROG_ARRAY**, and passes
//C  * 		*ctx*, a pointer to the context.
//C  *
//C  * 		If the call succeeds, the kernel immediately runs the first
//C  * 		instruction of the new program. This is not a function call,
//C  * 		and it never returns to the previous program. If the call
//C  * 		fails, then the helper has no effect, and the caller continues
//C  * 		to run its subsequent instructions. A call can fail if the
//C  * 		destination program for the jump does not exist (i.e. *index*
//C  * 		is superior to the number of entries in *prog_array_map*), or
//C  * 		if the maximum number of tail calls has been reached for this
//C  * 		chain of programs. This limit is defined in the kernel by the
//C  * 		macro **MAX_TAIL_CALL_CNT** (not accessible to user space),
//C  *		which is currently set to 33.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_clone_redirect(struct sk_buff *skb, u32 ifindex, u64 flags)
//C  * 	Description
//C  * 		Clone and redirect the packet associated to *skb* to another
//C  * 		net device of index *ifindex*. Both ingress and egress
//C  * 		interfaces can be used for redirection. The **BPF_F_INGRESS**
//C  * 		value in *flags* is used to make the distinction (ingress path
//C  * 		is selected if the flag is present, egress path otherwise).
//C  * 		This is the only flag supported for now.
//C  *
//C  * 		In comparison with **bpf_redirect**\ () helper,
//C  * 		**bpf_clone_redirect**\ () has the associated cost of
//C  * 		duplicating the packet buffer, but this can be executed out of
//C  * 		the eBPF program. Conversely, **bpf_redirect**\ () is more
//C  * 		efficient, but it is handled through an action code where the
//C  * 		redirection happens only after the eBPF program has returned.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure. Positive
//C  * 		error indicates a potential drop or congestion in the target
//C  * 		device. The particular positive error codes are not defined.
//C  *
//C  * u64 bpf_get_current_pid_tgid(void)
//C  * 	Description
//C  * 		Get the current pid and tgid.
//C  * 	Return
//C  * 		A 64-bit integer containing the current tgid and pid, and
//C  * 		created as such:
//C  * 		*current_task*\ **->tgid << 32 \|**
//C  * 		*current_task*\ **->pid**.
//C  *
//C  * u64 bpf_get_current_uid_gid(void)
//C  * 	Description
//C  * 		Get the current uid and gid.
//C  * 	Return
//C  * 		A 64-bit integer containing the current GID and UID, and
//C  * 		created as such: *current_gid* **<< 32 \|** *current_uid*.
//C  *
//C  * long bpf_get_current_comm(void *buf, u32 size_of_buf)
//C  * 	Description
//C  * 		Copy the **comm** attribute of the current task into *buf* of
//C  * 		*size_of_buf*. The **comm** attribute contains the name of
//C  * 		the executable (excluding the path) for the current task. The
//C  * 		*size_of_buf* must be strictly positive. On success, the
//C  * 		helper makes sure that the *buf* is NUL-terminated. On failure,
//C  * 		it is filled with zeroes.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * u32 bpf_get_cgroup_classid(struct sk_buff *skb)
//C  * 	Description
//C  * 		Retrieve the classid for the current task, i.e. for the net_cls
//C  * 		cgroup to which *skb* belongs.
//C  *
//C  * 		This helper can be used on TC egress path, but not on ingress.
//C  *
//C  * 		The net_cls cgroup provides an interface to tag network packets
//C  * 		based on a user-provided identifier for all traffic coming from
//C  * 		the tasks belonging to the related cgroup. See also the related
//C  * 		kernel documentation, available from the Linux sources in file
//C  * 		*Documentation/admin-guide/cgroup-v1/net_cls.rst*.
//C  *
//C  * 		The Linux kernel has two versions for cgroups: there are
//C  * 		cgroups v1 and cgroups v2. Both are available to users, who can
//C  * 		use a mixture of them, but note that the net_cls cgroup is for
//C  * 		cgroup v1 only. This makes it incompatible with BPF programs
//C  * 		run on cgroups, which is a cgroup-v2-only feature (a socket can
//C  * 		only hold data for one version of cgroups at a time).
//C  *
//C  * 		This helper is only available is the kernel was compiled with
//C  * 		the **CONFIG_CGROUP_NET_CLASSID** configuration option set to
//C  * 		"**y**" or to "**m**".
//C  * 	Return
//C  * 		The classid, or 0 for the default unconfigured classid.
//C  *
//C  * long bpf_skb_vlan_push(struct sk_buff *skb, __be16 vlan_proto, u16 vlan_tci)
//C  * 	Description
//C  * 		Push a *vlan_tci* (VLAN tag control information) of protocol
//C  * 		*vlan_proto* to the packet associated to *skb*, then update
//C  * 		the checksum. Note that if *vlan_proto* is different from
//C  * 		**ETH_P_8021Q** and **ETH_P_8021AD**, it is considered to
//C  * 		be **ETH_P_8021Q**.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_vlan_pop(struct sk_buff *skb)
//C  * 	Description
//C  * 		Pop a VLAN header from the packet associated to *skb*.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_get_tunnel_key(struct sk_buff *skb, struct bpf_tunnel_key *key, u32 size, u64 flags)
//C  * 	Description
//C  * 		Get tunnel metadata. This helper takes a pointer *key* to an
//C  * 		empty **struct bpf_tunnel_key** of **size**, that will be
//C  * 		filled with tunnel metadata for the packet associated to *skb*.
//C  * 		The *flags* can be set to **BPF_F_TUNINFO_IPV6**, which
//C  * 		indicates that the tunnel is based on IPv6 protocol instead of
//C  * 		IPv4.
//C  *
//C  * 		The **struct bpf_tunnel_key** is an object that generalizes the
//C  * 		principal parameters used by various tunneling protocols into a
//C  * 		single struct. This way, it can be used to easily make a
//C  * 		decision based on the contents of the encapsulation header,
//C  * 		"summarized" in this struct. In particular, it holds the IP
//C  * 		address of the remote end (IPv4 or IPv6, depending on the case)
//C  * 		in *key*\ **->remote_ipv4** or *key*\ **->remote_ipv6**. Also,
//C  * 		this struct exposes the *key*\ **->tunnel_id**, which is
//C  * 		generally mapped to a VNI (Virtual Network Identifier), making
//C  * 		it programmable together with the **bpf_skb_set_tunnel_key**\
//C  * 		() helper.
//C  *
//C  * 		Let's imagine that the following code is part of a program
//C  * 		attached to the TC ingress interface, on one end of a GRE
//C  * 		tunnel, and is supposed to filter out all messages coming from
//C  * 		remote ends with IPv4 address other than 10.0.0.1:
//C  *
//C  * 		::
//C  *
//C  * 			int ret;
//C  * 			struct bpf_tunnel_key key = {};
//C  *
//C  * 			ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key), 0);
//C  * 			if (ret < 0)
//C  * 				return TC_ACT_SHOT;	// drop packet
//C  *
//C  * 			if (key.remote_ipv4 != 0x0a000001)
//C  * 				return TC_ACT_SHOT;	// drop packet
//C  *
//C  * 			return TC_ACT_OK;		// accept packet
//C  *
//C  * 		This interface can also be used with all encapsulation devices
//C  * 		that can operate in "collect metadata" mode: instead of having
//C  * 		one network device per specific configuration, the "collect
//C  * 		metadata" mode only requires a single device where the
//C  * 		configuration can be extracted from this helper.
//C  *
//C  * 		This can be used together with various tunnels such as VXLan,
//C  * 		Geneve, GRE or IP in IP (IPIP).
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_set_tunnel_key(struct sk_buff *skb, struct bpf_tunnel_key *key, u32 size, u64 flags)
//C  * 	Description
//C  * 		Populate tunnel metadata for packet associated to *skb.* The
//C  * 		tunnel metadata is set to the contents of *key*, of *size*. The
//C  * 		*flags* can be set to a combination of the following values:
//C  *
//C  * 		**BPF_F_TUNINFO_IPV6**
//C  * 			Indicate that the tunnel is based on IPv6 protocol
//C  * 			instead of IPv4.
//C  * 		**BPF_F_ZERO_CSUM_TX**
//C  * 			For IPv4 packets, add a flag to tunnel metadata
//C  * 			indicating that checksum computation should be skipped
//C  * 			and checksum set to zeroes.
//C  * 		**BPF_F_DONT_FRAGMENT**
//C  * 			Add a flag to tunnel metadata indicating that the
//C  * 			packet should not be fragmented.
//C  * 		**BPF_F_SEQ_NUMBER**
//C  * 			Add a flag to tunnel metadata indicating that a
//C  * 			sequence number should be added to tunnel header before
//C  * 			sending the packet. This flag was added for GRE
//C  * 			encapsulation, but might be used with other protocols
//C  * 			as well in the future.
//C  * 		**BPF_F_NO_TUNNEL_KEY**
//C  * 			Add a flag to tunnel metadata indicating that no tunnel
//C  * 			key should be set in the resulting tunnel header.
//C  *
//C  * 		Here is a typical usage on the transmit path:
//C  *
//C  * 		::
//C  *
//C  * 			struct bpf_tunnel_key key;
//C  * 			     populate key ...
//C  * 			bpf_skb_set_tunnel_key(skb, &key, sizeof(key), 0);
//C  * 			bpf_clone_redirect(skb, vxlan_dev_ifindex, 0);
//C  *
//C  * 		See also the description of the **bpf_skb_get_tunnel_key**\ ()
//C  * 		helper for additional information.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * u64 bpf_perf_event_read(struct bpf_map *map, u64 flags)
//C  * 	Description
//C  * 		Read the value of a perf event counter. This helper relies on a
//C  * 		*map* of type **BPF_MAP_TYPE_PERF_EVENT_ARRAY**. The nature of
//C  * 		the perf event counter is selected when *map* is updated with
//C  * 		perf event file descriptors. The *map* is an array whose size
//C  * 		is the number of available CPUs, and each cell contains a value
//C  * 		relative to one CPU. The value to retrieve is indicated by
//C  * 		*flags*, that contains the index of the CPU to look up, masked
//C  * 		with **BPF_F_INDEX_MASK**. Alternatively, *flags* can be set to
//C  * 		**BPF_F_CURRENT_CPU** to indicate that the value for the
//C  * 		current CPU should be retrieved.
//C  *
//C  * 		Note that before Linux 4.13, only hardware perf event can be
//C  * 		retrieved.
//C  *
//C  * 		Also, be aware that the newer helper
//C  * 		**bpf_perf_event_read_value**\ () is recommended over
//C  * 		**bpf_perf_event_read**\ () in general. The latter has some ABI
//C  * 		quirks where error and counter value are used as a return code
//C  * 		(which is wrong to do since ranges may overlap). This issue is
//C  * 		fixed with **bpf_perf_event_read_value**\ (), which at the same
//C  * 		time provides more features over the **bpf_perf_event_read**\
//C  * 		() interface. Please refer to the description of
//C  * 		**bpf_perf_event_read_value**\ () for details.
//C  * 	Return
//C  * 		The value of the perf event counter read from the map, or a
//C  * 		negative error code in case of failure.
//C  *
//C  * long bpf_redirect(u32 ifindex, u64 flags)
//C  * 	Description
//C  * 		Redirect the packet to another net device of index *ifindex*.
//C  * 		This helper is somewhat similar to **bpf_clone_redirect**\
//C  * 		(), except that the packet is not cloned, which provides
//C  * 		increased performance.
//C  *
//C  * 		Except for XDP, both ingress and egress interfaces can be used
//C  * 		for redirection. The **BPF_F_INGRESS** value in *flags* is used
//C  * 		to make the distinction (ingress path is selected if the flag
//C  * 		is present, egress path otherwise). Currently, XDP only
//C  * 		supports redirection to the egress interface, and accepts no
//C  * 		flag at all.
//C  *
//C  * 		The same effect can also be attained with the more generic
//C  * 		**bpf_redirect_map**\ (), which uses a BPF map to store the
//C  * 		redirect target instead of providing it directly to the helper.
//C  * 	Return
//C  * 		For XDP, the helper returns **XDP_REDIRECT** on success or
//C  * 		**XDP_ABORTED** on error. For other program types, the values
//C  * 		are **TC_ACT_REDIRECT** on success or **TC_ACT_SHOT** on
//C  * 		error.
//C  *
//C  * u32 bpf_get_route_realm(struct sk_buff *skb)
//C  * 	Description
//C  * 		Retrieve the realm or the route, that is to say the
//C  * 		**tclassid** field of the destination for the *skb*. The
//C  * 		identifier retrieved is a user-provided tag, similar to the
//C  * 		one used with the net_cls cgroup (see description for
//C  * 		**bpf_get_cgroup_classid**\ () helper), but here this tag is
//C  * 		held by a route (a destination entry), not by a task.
//C  *
//C  * 		Retrieving this identifier works with the clsact TC egress hook
//C  * 		(see also **tc-bpf(8)**), or alternatively on conventional
//C  * 		classful egress qdiscs, but not on TC ingress path. In case of
//C  * 		clsact TC egress hook, this has the advantage that, internally,
//C  * 		the destination entry has not been dropped yet in the transmit
//C  * 		path. Therefore, the destination entry does not need to be
//C  * 		artificially held via **netif_keep_dst**\ () for a classful
//C  * 		qdisc until the *skb* is freed.
//C  *
//C  * 		This helper is available only if the kernel was compiled with
//C  * 		**CONFIG_IP_ROUTE_CLASSID** configuration option.
//C  * 	Return
//C  * 		The realm of the route for the packet associated to *skb*, or 0
//C  * 		if none was found.
//C  *
//C  * long bpf_perf_event_output(void *ctx, struct bpf_map *map, u64 flags, void *data, u64 size)
//C  * 	Description
//C  * 		Write raw *data* blob into a special BPF perf event held by
//C  * 		*map* of type **BPF_MAP_TYPE_PERF_EVENT_ARRAY**. This perf
//C  * 		event must have the following attributes: **PERF_SAMPLE_RAW**
//C  * 		as **sample_type**, **PERF_TYPE_SOFTWARE** as **type**, and
//C  * 		**PERF_COUNT_SW_BPF_OUTPUT** as **config**.
//C  *
//C  * 		The *flags* are used to indicate the index in *map* for which
//C  * 		the value must be put, masked with **BPF_F_INDEX_MASK**.
//C  * 		Alternatively, *flags* can be set to **BPF_F_CURRENT_CPU**
//C  * 		to indicate that the index of the current CPU core should be
//C  * 		used.
//C  *
//C  * 		The value to write, of *size*, is passed through eBPF stack and
//C  * 		pointed by *data*.
//C  *
//C  * 		The context of the program *ctx* needs also be passed to the
//C  * 		helper.
//C  *
//C  * 		On user space, a program willing to read the values needs to
//C  * 		call **perf_event_open**\ () on the perf event (either for
//C  * 		one or for all CPUs) and to store the file descriptor into the
//C  * 		*map*. This must be done before the eBPF program can send data
//C  * 		into it. An example is available in file
//C  * 		*samples/bpf/trace_output_user.c* in the Linux kernel source
//C  * 		tree (the eBPF program counterpart is in
//C  *		*samples/bpf/trace_output.bpf.c*).
//C  *
//C  * 		**bpf_perf_event_output**\ () achieves better performance
//C  * 		than **bpf_trace_printk**\ () for sharing data with user
//C  * 		space, and is much better suitable for streaming data from eBPF
//C  * 		programs.
//C  *
//C  * 		Note that this helper is not restricted to tracing use cases
//C  * 		and can be used with programs attached to TC or XDP as well,
//C  * 		where it allows for passing data to user space listeners. Data
//C  * 		can be:
//C  *
//C  * 		* Only custom structs,
//C  * 		* Only the packet payload, or
//C  * 		* A combination of both.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_load_bytes(const void *skb, u32 offset, void *to, u32 len)
//C  * 	Description
//C  * 		This helper was provided as an easy way to load data from a
//C  * 		packet. It can be used to load *len* bytes from *offset* from
//C  * 		the packet associated to *skb*, into the buffer pointed by
//C  * 		*to*.
//C  *
//C  * 		Since Linux 4.7, usage of this helper has mostly been replaced
//C  * 		by "direct packet access", enabling packet data to be
//C  * 		manipulated with *skb*\ **->data** and *skb*\ **->data_end**
//C  * 		pointing respectively to the first byte of packet data and to
//C  * 		the byte after the last byte of packet data. However, it
//C  * 		remains useful if one wishes to read large quantities of data
//C  * 		at once from a packet into the eBPF stack.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_get_stackid(void *ctx, struct bpf_map *map, u64 flags)
//C  * 	Description
//C  * 		Walk a user or a kernel stack and return its id. To achieve
//C  * 		this, the helper needs *ctx*, which is a pointer to the context
//C  * 		on which the tracing program is executed, and a pointer to a
//C  * 		*map* of type **BPF_MAP_TYPE_STACK_TRACE**.
//C  *
//C  * 		The last argument, *flags*, holds the number of stack frames to
//C  * 		skip (from 0 to 255), masked with
//C  * 		**BPF_F_SKIP_FIELD_MASK**. The next bits can be used to set
//C  * 		a combination of the following flags:
//C  *
//C  * 		**BPF_F_USER_STACK**
//C  * 			Collect a user space stack instead of a kernel stack.
//C  * 		**BPF_F_FAST_STACK_CMP**
//C  * 			Compare stacks by hash only.
//C  * 		**BPF_F_REUSE_STACKID**
//C  * 			If two different stacks hash into the same *stackid*,
//C  * 			discard the old one.
//C  *
//C  * 		The stack id retrieved is a 32 bit long integer handle which
//C  * 		can be further combined with other data (including other stack
//C  * 		ids) and used as a key into maps. This can be useful for
//C  * 		generating a variety of graphs (such as flame graphs or off-cpu
//C  * 		graphs).
//C  *
//C  * 		For walking a stack, this helper is an improvement over
//C  * 		**bpf_probe_read**\ (), which can be used with unrolled loops
//C  * 		but is not efficient and consumes a lot of eBPF instructions.
//C  * 		Instead, **bpf_get_stackid**\ () can collect up to
//C  * 		**PERF_MAX_STACK_DEPTH** both kernel and user frames. Note that
//C  * 		this limit can be controlled with the **sysctl** program, and
//C  * 		that it should be manually increased in order to profile long
//C  * 		user stacks (such as stacks for Java programs). To do so, use:
//C  *
//C  * 		::
//C  *
//C  * 			# sysctl kernel.perf_event_max_stack=<new value>
//C  * 	Return
//C  * 		The positive or null stack id on success, or a negative error
//C  * 		in case of failure.
//C  *
//C  * s64 bpf_csum_diff(__be32 *from, u32 from_size, __be32 *to, u32 to_size, __wsum seed)
//C  * 	Description
//C  * 		Compute a checksum difference, from the raw buffer pointed by
//C  * 		*from*, of length *from_size* (that must be a multiple of 4),
//C  * 		towards the raw buffer pointed by *to*, of size *to_size*
//C  * 		(same remark). An optional *seed* can be added to the value
//C  * 		(this can be cascaded, the seed may come from a previous call
//C  * 		to the helper).
//C  *
//C  * 		This is flexible enough to be used in several ways:
//C  *
//C  * 		* With *from_size* == 0, *to_size* > 0 and *seed* set to
//C  * 		  checksum, it can be used when pushing new data.
//C  * 		* With *from_size* > 0, *to_size* == 0 and *seed* set to
//C  * 		  checksum, it can be used when removing data from a packet.
//C  * 		* With *from_size* > 0, *to_size* > 0 and *seed* set to 0, it
//C  * 		  can be used to compute a diff. Note that *from_size* and
//C  * 		  *to_size* do not need to be equal.
//C  *
//C  * 		This helper can be used in combination with
//C  * 		**bpf_l3_csum_replace**\ () and **bpf_l4_csum_replace**\ (), to
//C  * 		which one can feed in the difference computed with
//C  * 		**bpf_csum_diff**\ ().
//C  * 	Return
//C  * 		The checksum result, or a negative error code in case of
//C  * 		failure.
//C  *
//C  * long bpf_skb_get_tunnel_opt(struct sk_buff *skb, void *opt, u32 size)
//C  * 	Description
//C  * 		Retrieve tunnel options metadata for the packet associated to
//C  * 		*skb*, and store the raw tunnel option data to the buffer *opt*
//C  * 		of *size*.
//C  *
//C  * 		This helper can be used with encapsulation devices that can
//C  * 		operate in "collect metadata" mode (please refer to the related
//C  * 		note in the description of **bpf_skb_get_tunnel_key**\ () for
//C  * 		more details). A particular example where this can be used is
//C  * 		in combination with the Geneve encapsulation protocol, where it
//C  * 		allows for pushing (with **bpf_skb_get_tunnel_opt**\ () helper)
//C  * 		and retrieving arbitrary TLVs (Type-Length-Value headers) from
//C  * 		the eBPF program. This allows for full customization of these
//C  * 		headers.
//C  * 	Return
//C  * 		The size of the option data retrieved.
//C  *
//C  * long bpf_skb_set_tunnel_opt(struct sk_buff *skb, void *opt, u32 size)
//C  * 	Description
//C  * 		Set tunnel options metadata for the packet associated to *skb*
//C  * 		to the option data contained in the raw buffer *opt* of *size*.
//C  *
//C  * 		See also the description of the **bpf_skb_get_tunnel_opt**\ ()
//C  * 		helper for additional information.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_change_proto(struct sk_buff *skb, __be16 proto, u64 flags)
//C  * 	Description
//C  * 		Change the protocol of the *skb* to *proto*. Currently
//C  * 		supported are transition from IPv4 to IPv6, and from IPv6 to
//C  * 		IPv4. The helper takes care of the groundwork for the
//C  * 		transition, including resizing the socket buffer. The eBPF
//C  * 		program is expected to fill the new headers, if any, via
//C  * 		**skb_store_bytes**\ () and to recompute the checksums with
//C  * 		**bpf_l3_csum_replace**\ () and **bpf_l4_csum_replace**\
//C  * 		(). The main case for this helper is to perform NAT64
//C  * 		operations out of an eBPF program.
//C  *
//C  * 		Internally, the GSO type is marked as dodgy so that headers are
//C  * 		checked and segments are recalculated by the GSO/GRO engine.
//C  * 		The size for GSO target is adapted as well.
//C  *
//C  * 		All values for *flags* are reserved for future usage, and must
//C  * 		be left at zero.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_change_type(struct sk_buff *skb, u32 type)
//C  * 	Description
//C  * 		Change the packet type for the packet associated to *skb*. This
//C  * 		comes down to setting *skb*\ **->pkt_type** to *type*, except
//C  * 		the eBPF program does not have a write access to *skb*\
//C  * 		**->pkt_type** beside this helper. Using a helper here allows
//C  * 		for graceful handling of errors.
//C  *
//C  * 		The major use case is to change incoming *skb*s to
//C  * 		**PACKET_HOST** in a programmatic way instead of having to
//C  * 		recirculate via **redirect**\ (..., **BPF_F_INGRESS**), for
//C  * 		example.
//C  *
//C  * 		Note that *type* only allows certain values. At this time, they
//C  * 		are:
//C  *
//C  * 		**PACKET_HOST**
//C  * 			Packet is for us.
//C  * 		**PACKET_BROADCAST**
//C  * 			Send packet to all.
//C  * 		**PACKET_MULTICAST**
//C  * 			Send packet to group.
//C  * 		**PACKET_OTHERHOST**
//C  * 			Send packet to someone else.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_under_cgroup(struct sk_buff *skb, struct bpf_map *map, u32 index)
//C  * 	Description
//C  * 		Check whether *skb* is a descendant of the cgroup2 held by
//C  * 		*map* of type **BPF_MAP_TYPE_CGROUP_ARRAY**, at *index*.
//C  * 	Return
//C  * 		The return value depends on the result of the test, and can be:
//C  *
//C  * 		* 0, if the *skb* failed the cgroup2 descendant test.
//C  * 		* 1, if the *skb* succeeded the cgroup2 descendant test.
//C  * 		* A negative error code, if an error occurred.
//C  *
//C  * u32 bpf_get_hash_recalc(struct sk_buff *skb)
//C  * 	Description
//C  * 		Retrieve the hash of the packet, *skb*\ **->hash**. If it is
//C  * 		not set, in particular if the hash was cleared due to mangling,
//C  * 		recompute this hash. Later accesses to the hash can be done
//C  * 		directly with *skb*\ **->hash**.
//C  *
//C  * 		Calling **bpf_set_hash_invalid**\ (), changing a packet
//C  * 		prototype with **bpf_skb_change_proto**\ (), or calling
//C  * 		**bpf_skb_store_bytes**\ () with the
//C  * 		**BPF_F_INVALIDATE_HASH** are actions susceptible to clear
//C  * 		the hash and to trigger a new computation for the next call to
//C  * 		**bpf_get_hash_recalc**\ ().
//C  * 	Return
//C  * 		The 32-bit hash.
//C  *
//C  * u64 bpf_get_current_task(void)
//C  * 	Description
//C  * 		Get the current task.
//C  * 	Return
//C  * 		A pointer to the current task struct.
//C  *
//C  * long bpf_probe_write_user(void *dst, const void *src, u32 len)
//C  * 	Description
//C  * 		Attempt in a safe way to write *len* bytes from the buffer
//C  * 		*src* to *dst* in memory. It only works for threads that are in
//C  * 		user context, and *dst* must be a valid user space address.
//C  *
//C  * 		This helper should not be used to implement any kind of
//C  * 		security mechanism because of TOC-TOU attacks, but rather to
//C  * 		debug, divert, and manipulate execution of semi-cooperative
//C  * 		processes.
//C  *
//C  * 		Keep in mind that this feature is meant for experiments, and it
//C  * 		has a risk of crashing the system and running programs.
//C  * 		Therefore, when an eBPF program using this helper is attached,
//C  * 		a warning including PID and process name is printed to kernel
//C  * 		logs.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_current_task_under_cgroup(struct bpf_map *map, u32 index)
//C  * 	Description
//C  * 		Check whether the probe is being run is the context of a given
//C  * 		subset of the cgroup2 hierarchy. The cgroup2 to test is held by
//C  * 		*map* of type **BPF_MAP_TYPE_CGROUP_ARRAY**, at *index*.
//C  * 	Return
//C  * 		The return value depends on the result of the test, and can be:
//C  *
//C  *		* 1, if current task belongs to the cgroup2.
//C  *		* 0, if current task does not belong to the cgroup2.
//C  * 		* A negative error code, if an error occurred.
//C  *
//C  * long bpf_skb_change_tail(struct sk_buff *skb, u32 len, u64 flags)
//C  * 	Description
//C  * 		Resize (trim or grow) the packet associated to *skb* to the
//C  * 		new *len*. The *flags* are reserved for future usage, and must
//C  * 		be left at zero.
//C  *
//C  * 		The basic idea is that the helper performs the needed work to
//C  * 		change the size of the packet, then the eBPF program rewrites
//C  * 		the rest via helpers like **bpf_skb_store_bytes**\ (),
//C  * 		**bpf_l3_csum_replace**\ (), **bpf_l3_csum_replace**\ ()
//C  * 		and others. This helper is a slow path utility intended for
//C  * 		replies with control messages. And because it is targeted for
//C  * 		slow path, the helper itself can afford to be slow: it
//C  * 		implicitly linearizes, unclones and drops offloads from the
//C  * 		*skb*.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_pull_data(struct sk_buff *skb, u32 len)
//C  * 	Description
//C  * 		Pull in non-linear data in case the *skb* is non-linear and not
//C  * 		all of *len* are part of the linear section. Make *len* bytes
//C  * 		from *skb* readable and writable. If a zero value is passed for
//C  *		*len*, then all bytes in the linear part of *skb* will be made
//C  *		readable and writable.
//C  *
//C  * 		This helper is only needed for reading and writing with direct
//C  * 		packet access.
//C  *
//C  * 		For direct packet access, testing that offsets to access
//C  * 		are within packet boundaries (test on *skb*\ **->data_end**) is
//C  * 		susceptible to fail if offsets are invalid, or if the requested
//C  * 		data is in non-linear parts of the *skb*. On failure the
//C  * 		program can just bail out, or in the case of a non-linear
//C  * 		buffer, use a helper to make the data available. The
//C  * 		**bpf_skb_load_bytes**\ () helper is a first solution to access
//C  * 		the data. Another one consists in using **bpf_skb_pull_data**
//C  * 		to pull in once the non-linear parts, then retesting and
//C  * 		eventually access the data.
//C  *
//C  * 		At the same time, this also makes sure the *skb* is uncloned,
//C  * 		which is a necessary condition for direct write. As this needs
//C  * 		to be an invariant for the write part only, the verifier
//C  * 		detects writes and adds a prologue that is calling
//C  * 		**bpf_skb_pull_data()** to effectively unclone the *skb* from
//C  * 		the very beginning in case it is indeed cloned.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * s64 bpf_csum_update(struct sk_buff *skb, __wsum csum)
//C  * 	Description
//C  * 		Add the checksum *csum* into *skb*\ **->csum** in case the
//C  * 		driver has supplied a checksum for the entire packet into that
//C  * 		field. Return an error otherwise. This helper is intended to be
//C  * 		used in combination with **bpf_csum_diff**\ (), in particular
//C  * 		when the checksum needs to be updated after data has been
//C  * 		written into the packet through direct packet access.
//C  * 	Return
//C  * 		The checksum on success, or a negative error code in case of
//C  * 		failure.
//C  *
//C  * void bpf_set_hash_invalid(struct sk_buff *skb)
//C  * 	Description
//C  * 		Invalidate the current *skb*\ **->hash**. It can be used after
//C  * 		mangling on headers through direct packet access, in order to
//C  * 		indicate that the hash is outdated and to trigger a
//C  * 		recalculation the next time the kernel tries to access this
//C  * 		hash or when the **bpf_get_hash_recalc**\ () helper is called.
//C  * 	Return
//C  * 		void.
//C  *
//C  * long bpf_get_numa_node_id(void)
//C  * 	Description
//C  * 		Return the id of the current NUMA node. The primary use case
//C  * 		for this helper is the selection of sockets for the local NUMA
//C  * 		node, when the program is attached to sockets using the
//C  * 		**SO_ATTACH_REUSEPORT_EBPF** option (see also **socket(7)**),
//C  * 		but the helper is also available to other eBPF program types,
//C  * 		similarly to **bpf_get_smp_processor_id**\ ().
//C  * 	Return
//C  * 		The id of current NUMA node.
//C  *
//C  * long bpf_skb_change_head(struct sk_buff *skb, u32 len, u64 flags)
//C  * 	Description
//C  * 		Grows headroom of packet associated to *skb* and adjusts the
//C  * 		offset of the MAC header accordingly, adding *len* bytes of
//C  * 		space. It automatically extends and reallocates memory as
//C  * 		required.
//C  *
//C  * 		This helper can be used on a layer 3 *skb* to push a MAC header
//C  * 		for redirection into a layer 2 device.
//C  *
//C  * 		All values for *flags* are reserved for future usage, and must
//C  * 		be left at zero.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_xdp_adjust_head(struct xdp_buff *xdp_md, int delta)
//C  * 	Description
//C  * 		Adjust (move) *xdp_md*\ **->data** by *delta* bytes. Note that
//C  * 		it is possible to use a negative value for *delta*. This helper
//C  * 		can be used to prepare the packet for pushing or popping
//C  * 		headers.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_probe_read_str(void *dst, u32 size, const void *unsafe_ptr)
//C  * 	Description
//C  * 		Copy a NUL terminated string from an unsafe kernel address
//C  * 		*unsafe_ptr* to *dst*. See **bpf_probe_read_kernel_str**\ () for
//C  * 		more details.
//C  *
//C  * 		Generally, use **bpf_probe_read_user_str**\ () or
//C  * 		**bpf_probe_read_kernel_str**\ () instead.
//C  * 	Return
//C  * 		On success, the strictly positive length of the string,
//C  * 		including the trailing NUL character. On error, a negative
//C  * 		value.
//C  *
//C  * u64 bpf_get_socket_cookie(struct sk_buff *skb)
//C  * 	Description
//C  * 		If the **struct sk_buff** pointed by *skb* has a known socket,
//C  * 		retrieve the cookie (generated by the kernel) of this socket.
//C  * 		If no cookie has been set yet, generate a new cookie. Once
//C  * 		generated, the socket cookie remains stable for the life of the
//C  * 		socket. This helper can be useful for monitoring per socket
//C  * 		networking traffic statistics as it provides a global socket
//C  * 		identifier that can be assumed unique.
//C  * 	Return
//C  * 		A 8-byte long unique number on success, or 0 if the socket
//C  * 		field is missing inside *skb*.
//C  *
//C  * u64 bpf_get_socket_cookie(struct bpf_sock_addr *ctx)
//C  * 	Description
//C  * 		Equivalent to bpf_get_socket_cookie() helper that accepts
//C  * 		*skb*, but gets socket from **struct bpf_sock_addr** context.
//C  * 	Return
//C  * 		A 8-byte long unique number.
//C  *
//C  * u64 bpf_get_socket_cookie(struct bpf_sock_ops *ctx)
//C  * 	Description
//C  * 		Equivalent to **bpf_get_socket_cookie**\ () helper that accepts
//C  * 		*skb*, but gets socket from **struct bpf_sock_ops** context.
//C  * 	Return
//C  * 		A 8-byte long unique number.
//C  *
//C  * u64 bpf_get_socket_cookie(struct sock *sk)
//C  * 	Description
//C  * 		Equivalent to **bpf_get_socket_cookie**\ () helper that accepts
//C  * 		*sk*, but gets socket from a BTF **struct sock**. This helper
//C  * 		also works for sleepable programs.
//C  * 	Return
//C  * 		A 8-byte long unique number or 0 if *sk* is NULL.
//C  *
//C  * u32 bpf_get_socket_uid(struct sk_buff *skb)
//C  * 	Description
//C  * 		Get the owner UID of the socked associated to *skb*.
//C  * 	Return
//C  * 		The owner UID of the socket associated to *skb*. If the socket
//C  * 		is **NULL**, or if it is not a full socket (i.e. if it is a
//C  * 		time-wait or a request socket instead), **overflowuid** value
//C  * 		is returned (note that **overflowuid** might also be the actual
//C  * 		UID value for the socket).
//C  *
//C  * long bpf_set_hash(struct sk_buff *skb, u32 hash)
//C  * 	Description
//C  * 		Set the full hash for *skb* (set the field *skb*\ **->hash**)
//C  * 		to value *hash*.
//C  * 	Return
//C  * 		0
//C  *
//C  * long bpf_setsockopt(void *bpf_socket, int level, int optname, void *optval, int optlen)
//C  * 	Description
//C  * 		Emulate a call to **setsockopt()** on the socket associated to
//C  * 		*bpf_socket*, which must be a full socket. The *level* at
//C  * 		which the option resides and the name *optname* of the option
//C  * 		must be specified, see **setsockopt(2)** for more information.
//C  * 		The option value of length *optlen* is pointed by *optval*.
//C  *
//C  * 		*bpf_socket* should be one of the following:
//C  *
//C  * 		* **struct bpf_sock_ops** for **BPF_PROG_TYPE_SOCK_OPS**.
//C  *		* **struct bpf_sock_addr** for **BPF_CGROUP_INET4_CONNECT**,
//C  *		  **BPF_CGROUP_INET6_CONNECT** and **BPF_CGROUP_UNIX_CONNECT**.
//C  *
//C  * 		This helper actually implements a subset of **setsockopt()**.
//C  * 		It supports the following *level*\ s:
//C  *
//C  * 		* **SOL_SOCKET**, which supports the following *optname*\ s:
//C  * 		  **SO_RCVBUF**, **SO_SNDBUF**, **SO_MAX_PACING_RATE**,
//C  * 		  **SO_PRIORITY**, **SO_RCVLOWAT**, **SO_MARK**,
//C  * 		  **SO_BINDTODEVICE**, **SO_KEEPALIVE**, **SO_REUSEADDR**,
//C  * 		  **SO_REUSEPORT**, **SO_BINDTOIFINDEX**, **SO_TXREHASH**.
//C  * 		* **IPPROTO_TCP**, which supports the following *optname*\ s:
//C  * 		  **TCP_CONGESTION**, **TCP_BPF_IW**,
//C  * 		  **TCP_BPF_SNDCWND_CLAMP**, **TCP_SAVE_SYN**,
//C  * 		  **TCP_KEEPIDLE**, **TCP_KEEPINTVL**, **TCP_KEEPCNT**,
//C  * 		  **TCP_SYNCNT**, **TCP_USER_TIMEOUT**, **TCP_NOTSENT_LOWAT**,
//C  * 		  **TCP_NODELAY**, **TCP_MAXSEG**, **TCP_WINDOW_CLAMP**,
//C  * 		  **TCP_THIN_LINEAR_TIMEOUTS**, **TCP_BPF_DELACK_MAX**,
//C  *		  **TCP_BPF_RTO_MIN**, **TCP_BPF_SOCK_OPS_CB_FLAGS**.
//C  * 		* **IPPROTO_IP**, which supports *optname* **IP_TOS**.
//C  * 		* **IPPROTO_IPV6**, which supports the following *optname*\ s:
//C  * 		  **IPV6_TCLASS**, **IPV6_AUTOFLOWLABEL**.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_adjust_room(struct sk_buff *skb, s32 len_diff, u32 mode, u64 flags)
//C  * 	Description
//C  * 		Grow or shrink the room for data in the packet associated to
//C  * 		*skb* by *len_diff*, and according to the selected *mode*.
//C  *
//C  * 		By default, the helper will reset any offloaded checksum
//C  * 		indicator of the skb to CHECKSUM_NONE. This can be avoided
//C  * 		by the following flag:
//C  *
//C  * 		* **BPF_F_ADJ_ROOM_NO_CSUM_RESET**: Do not reset offloaded
//C  * 		  checksum data of the skb to CHECKSUM_NONE.
//C  *
//C  *		There are two supported modes at this time:
//C  *
//C  *		* **BPF_ADJ_ROOM_MAC**: Adjust room at the mac layer
//C  * 		  (room space is added or removed between the layer 2 and
//C  * 		  layer 3 headers).
//C  *
//C  * 		* **BPF_ADJ_ROOM_NET**: Adjust room at the network layer
//C  * 		  (room space is added or removed between the layer 3 and
//C  * 		  layer 4 headers).
//C  *
//C  *		The following flags are supported at this time:
//C  *
//C  *		* **BPF_F_ADJ_ROOM_FIXED_GSO**: Do not adjust gso_size.
//C  *		  Adjusting mss in this way is not allowed for datagrams.
//C  *
//C  *		* **BPF_F_ADJ_ROOM_ENCAP_L3_IPV4**,
//C  *		  **BPF_F_ADJ_ROOM_ENCAP_L3_IPV6**:
//C  *		  Any new space is reserved to hold a tunnel header.
//C  *		  Configure skb offsets and other fields accordingly.
//C  *
//C  *		* **BPF_F_ADJ_ROOM_ENCAP_L4_GRE**,
//C  *		  **BPF_F_ADJ_ROOM_ENCAP_L4_UDP**:
//C  *		  Use with ENCAP_L3 flags to further specify the tunnel type.
//C  *
//C  *		* **BPF_F_ADJ_ROOM_ENCAP_L2**\ (*len*):
//C  *		  Use with ENCAP_L3/L4 flags to further specify the tunnel
//C  *		  type; *len* is the length of the inner MAC header.
//C  *
//C  *		* **BPF_F_ADJ_ROOM_ENCAP_L2_ETH**:
//C  *		  Use with BPF_F_ADJ_ROOM_ENCAP_L2 flag to further specify the
//C  *		  L2 type as Ethernet.
//C  *
//C  *		* **BPF_F_ADJ_ROOM_DECAP_L3_IPV4**,
//C  *		  **BPF_F_ADJ_ROOM_DECAP_L3_IPV6**:
//C  *		  Indicate the new IP header version after decapsulating the
//C  *		  outer IP header. Used when the inner and outer IP versions
//C  *		  are different. These flags only trigger a protocol change
//C  *		  without clearing any tunnel-specific GSO flags.
//C  *
//C  *		* **BPF_F_ADJ_ROOM_DECAP_L4_GRE**:
//C  *		  Clear GRE tunnel GSO flags (SKB_GSO_GRE and SKB_GSO_GRE_CSUM)
//C  *		  when decapsulating a GRE tunnel.
//C  *
//C  *		* **BPF_F_ADJ_ROOM_DECAP_L4_UDP**:
//C  *		  Clear UDP tunnel GSO flags (SKB_GSO_UDP_TUNNEL and
//C  *		  SKB_GSO_UDP_TUNNEL_CSUM) when decapsulating a UDP tunnel.
//C  *
//C  *		* **BPF_F_ADJ_ROOM_DECAP_IPXIP4**:
//C  *		  Clear IPIP/SIT tunnel GSO flag (SKB_GSO_IPXIP4) when decapsulating
//C  *		  a tunnel with an outer IPv4 header (IPv4-in-IPv4 or IPv6-in-IPv4).
//C  *
//C  *		* **BPF_F_ADJ_ROOM_DECAP_IPXIP6**:
//C  *		  Clear IPv6 encapsulation tunnel GSO flag (SKB_GSO_IPXIP6) when
//C  *		  decapsulating a tunnel with an outer IPv6 header (IPv6-in-IPv6
//C  *		  or IPv4-in-IPv6).
//C  *
//C  *		When using the decapsulation flags above, the skb->encapsulation
//C  *		flag is automatically cleared if all tunnel-specific GSO flags
//C  *		(SKB_GSO_UDP_TUNNEL, SKB_GSO_UDP_TUNNEL_CSUM, SKB_GSO_GRE,
//C  *		SKB_GSO_GRE_CSUM, SKB_GSO_IPXIP4, SKB_GSO_IPXIP6) have been
//C  *		removed from the packet. This handles cases where all tunnel
//C  *		layers have been decapsulated.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_redirect_map(struct bpf_map *map, u64 key, u64 flags)
//C  * 	Description
//C  * 		Redirect the packet to the endpoint referenced by *map* at
//C  * 		index *key*. Depending on its type, this *map* can contain
//C  * 		references to net devices (for forwarding packets through other
//C  * 		ports), or to CPUs (for redirecting XDP frames to another CPU;
//C  * 		but this is only implemented for native XDP (with driver
//C  * 		support) as of this writing).
//C  *
//C  * 		The lower two bits of *flags* are used as the return code if
//C  * 		the map lookup fails. This is so that the return value can be
//C  * 		one of the XDP program return codes up to **XDP_TX**, as chosen
//C  * 		by the caller. The higher bits of *flags* can be set to
//C  * 		BPF_F_BROADCAST or BPF_F_EXCLUDE_INGRESS as defined below.
//C  *
//C  * 		With BPF_F_BROADCAST the packet will be broadcasted to all the
//C  * 		interfaces in the map, with BPF_F_EXCLUDE_INGRESS the ingress
//C  * 		interface will be excluded when do broadcasting.
//C  *
//C  * 		See also **bpf_redirect**\ (), which only supports redirecting
//C  * 		to an ifindex, but doesn't require a map to do so.
//C  * 	Return
//C  * 		**XDP_REDIRECT** on success, or the value of the two lower bits
//C  * 		of the *flags* argument on error.
//C  *
//C  * long bpf_sk_redirect_map(struct sk_buff *skb, struct bpf_map *map, u32 key, u64 flags)
//C  * 	Description
//C  * 		Redirect the packet to the socket referenced by *map* (of type
//C  * 		**BPF_MAP_TYPE_SOCKMAP**) at index *key*. Both ingress and
//C  * 		egress interfaces can be used for redirection. The
//C  * 		**BPF_F_INGRESS** value in *flags* is used to make the
//C  * 		distinction (ingress path is selected if the flag is present,
//C  * 		egress path otherwise). This is the only flag supported for now.
//C  * 	Return
//C  * 		**SK_PASS** on success, or **SK_DROP** on error.
//C  *
//C  * long bpf_sock_map_update(struct bpf_sock_ops *skops, struct bpf_map *map, void *key, u64 flags)
//C  * 	Description
//C  * 		Add an entry to, or update a *map* referencing sockets. The
//C  * 		*skops* is used as a new value for the entry associated to
//C  * 		*key*. *flags* is one of:
//C  *
//C  * 		**BPF_NOEXIST**
//C  * 			The entry for *key* must not exist in the map.
//C  * 		**BPF_EXIST**
//C  * 			The entry for *key* must already exist in the map.
//C  * 		**BPF_ANY**
//C  * 			No condition on the existence of the entry for *key*.
//C  *
//C  * 		If the *map* has eBPF programs (parser and verdict), those will
//C  * 		be inherited by the socket being added. If the socket is
//C  * 		already attached to eBPF programs, this results in an error.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_xdp_adjust_meta(struct xdp_buff *xdp_md, int delta)
//C  * 	Description
//C  * 		Adjust the address pointed by *xdp_md*\ **->data_meta** by
//C  * 		*delta* (which can be positive or negative). Note that this
//C  * 		operation modifies the address stored in *xdp_md*\ **->data**,
//C  * 		so the latter must be loaded only after the helper has been
//C  * 		called.
//C  *
//C  * 		The use of *xdp_md*\ **->data_meta** is optional and programs
//C  * 		are not required to use it. The rationale is that when the
//C  * 		packet is processed with XDP (e.g. as DoS filter), it is
//C  * 		possible to push further meta data along with it before passing
//C  * 		to the stack, and to give the guarantee that an ingress eBPF
//C  * 		program attached as a TC classifier on the same device can pick
//C  * 		this up for further post-processing. Since TC works with socket
//C  * 		buffers, it remains possible to set from XDP the **mark** or
//C  * 		**priority** pointers, or other pointers for the socket buffer.
//C  * 		Having this scratch space generic and programmable allows for
//C  * 		more flexibility as the user is free to store whatever meta
//C  * 		data they need.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_perf_event_read_value(struct bpf_map *map, u64 flags, struct bpf_perf_event_value *buf, u32 buf_size)
//C  * 	Description
//C  * 		Read the value of a perf event counter, and store it into *buf*
//C  * 		of size *buf_size*. This helper relies on a *map* of type
//C  * 		**BPF_MAP_TYPE_PERF_EVENT_ARRAY**. The nature of the perf event
//C  * 		counter is selected when *map* is updated with perf event file
//C  * 		descriptors. The *map* is an array whose size is the number of
//C  * 		available CPUs, and each cell contains a value relative to one
//C  * 		CPU. The value to retrieve is indicated by *flags*, that
//C  * 		contains the index of the CPU to look up, masked with
//C  * 		**BPF_F_INDEX_MASK**. Alternatively, *flags* can be set to
//C  * 		**BPF_F_CURRENT_CPU** to indicate that the value for the
//C  * 		current CPU should be retrieved.
//C  *
//C  * 		This helper behaves in a way close to
//C  * 		**bpf_perf_event_read**\ () helper, save that instead of
//C  * 		just returning the value observed, it fills the *buf*
//C  * 		structure. This allows for additional data to be retrieved: in
//C  * 		particular, the enabled and running times (in *buf*\
//C  * 		**->enabled** and *buf*\ **->running**, respectively) are
//C  * 		copied. In general, **bpf_perf_event_read_value**\ () is
//C  * 		recommended over **bpf_perf_event_read**\ (), which has some
//C  * 		ABI issues and provides fewer functionalities.
//C  *
//C  * 		These values are interesting, because hardware PMU (Performance
//C  * 		Monitoring Unit) counters are limited resources. When there are
//C  * 		more PMU based perf events opened than available counters,
//C  * 		kernel will multiplex these events so each event gets certain
//C  * 		percentage (but not all) of the PMU time. In case that
//C  * 		multiplexing happens, the number of samples or counter value
//C  * 		will not reflect the case compared to when no multiplexing
//C  * 		occurs. This makes comparison between different runs difficult.
//C  * 		Typically, the counter value should be normalized before
//C  * 		comparing to other experiments. The usual normalization is done
//C  * 		as follows.
//C  *
//C  * 		::
//C  *
//C  * 			normalized_counter = counter * t_enabled / t_running
//C  *
//C  * 		Where t_enabled is the time enabled for event and t_running is
//C  * 		the time running for event since last normalization. The
//C  * 		enabled and running times are accumulated since the perf event
//C  * 		open. To achieve scaling factor between two invocations of an
//C  * 		eBPF program, users can use CPU id as the key (which is
//C  * 		typical for perf array usage model) to remember the previous
//C  * 		value and do the calculation inside the eBPF program.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_perf_prog_read_value(struct bpf_perf_event_data *ctx, struct bpf_perf_event_value *buf, u32 buf_size)
//C  * 	Description
//C  * 		For an eBPF program attached to a perf event, retrieve the
//C  * 		value of the event counter associated to *ctx* and store it in
//C  * 		the structure pointed by *buf* and of size *buf_size*. Enabled
//C  * 		and running times are also stored in the structure (see
//C  * 		description of helper **bpf_perf_event_read_value**\ () for
//C  * 		more details).
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_getsockopt(void *bpf_socket, int level, int optname, void *optval, int optlen)
//C  * 	Description
//C  * 		Emulate a call to **getsockopt()** on the socket associated to
//C  * 		*bpf_socket*, which must be a full socket. The *level* at
//C  * 		which the option resides and the name *optname* of the option
//C  * 		must be specified, see **getsockopt(2)** for more information.
//C  * 		The retrieved value is stored in the structure pointed by
//C  * 		*opval* and of length *optlen*.
//C  *
//C  * 		*bpf_socket* should be one of the following:
//C  *
//C  * 		* **struct bpf_sock_ops** for **BPF_PROG_TYPE_SOCK_OPS**.
//C  *		* **struct bpf_sock_addr** for **BPF_CGROUP_INET4_CONNECT**,
//C  *		  **BPF_CGROUP_INET6_CONNECT** and **BPF_CGROUP_UNIX_CONNECT**.
//C  *
//C  * 		This helper actually implements a subset of **getsockopt()**.
//C  * 		It supports the same set of *optname*\ s that is supported by
//C  * 		the **bpf_setsockopt**\ () helper.  The exceptions are
//C  * 		**TCP_BPF_*** is **bpf_setsockopt**\ () only and
//C  * 		**TCP_SAVED_SYN** is **bpf_getsockopt**\ () only.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_override_return(struct pt_regs *regs, u64 rc)
//C  * 	Description
//C  * 		Used for error injection, this helper uses kprobes to override
//C  * 		the return value of the probed function, and to set it to *rc*.
//C  * 		The first argument is the context *regs* on which the kprobe
//C  * 		works.
//C  *
//C  * 		This helper works by setting the PC (program counter)
//C  * 		to an override function which is run in place of the original
//C  * 		probed function. This means the probed function is not run at
//C  * 		all. The replacement function just returns with the required
//C  * 		value.
//C  *
//C  * 		This helper has security implications, and thus is subject to
//C  * 		restrictions. It is only available if the kernel was compiled
//C  * 		with the **CONFIG_BPF_KPROBE_OVERRIDE** configuration
//C  * 		option, and in this case it only works on functions tagged with
//C  * 		**ALLOW_ERROR_INJECTION** in the kernel code.
//C  * 	Return
//C  * 		0
//C  *
//C  * long bpf_sock_ops_cb_flags_set(struct bpf_sock_ops *bpf_sock, int argval)
//C  * 	Description
//C  * 		Attempt to set the value of the **bpf_sock_ops_cb_flags** field
//C  * 		for the full TCP socket associated to *bpf_sock_ops* to
//C  * 		*argval*.
//C  *
//C  * 		The primary use of this field is to determine if there should
//C  * 		be calls to eBPF programs of type
//C  * 		**BPF_PROG_TYPE_SOCK_OPS** at various points in the TCP
//C  * 		code. A program of the same type can change its value, per
//C  * 		connection and as necessary, when the connection is
//C  * 		established. This field is directly accessible for reading, but
//C  * 		this helper must be used for updates in order to return an
//C  * 		error if an eBPF program tries to set a callback that is not
//C  * 		supported in the current kernel.
//C  *
//C  * 		*argval* is a flag array which can combine these flags:
//C  *
//C  * 		* **BPF_SOCK_OPS_RTO_CB_FLAG** (retransmission time out)
//C  * 		* **BPF_SOCK_OPS_RETRANS_CB_FLAG** (retransmission)
//C  * 		* **BPF_SOCK_OPS_STATE_CB_FLAG** (TCP state change)
//C  * 		* **BPF_SOCK_OPS_RTT_CB_FLAG** (every RTT)
//C  *
//C  * 		Therefore, this function can be used to clear a callback flag by
//C  * 		setting the appropriate bit to zero. e.g. to disable the RTO
//C  * 		callback:
//C  *
//C  * 		**bpf_sock_ops_cb_flags_set(bpf_sock,**
//C  * 			**bpf_sock->bpf_sock_ops_cb_flags & ~BPF_SOCK_OPS_RTO_CB_FLAG)**
//C  *
//C  * 		Here are some examples of where one could call such eBPF
//C  * 		program:
//C  *
//C  * 		* When RTO fires.
//C  * 		* When a packet is retransmitted.
//C  * 		* When the connection terminates.
//C  * 		* When a packet is sent.
//C  * 		* When a packet is received.
//C  * 	Return
//C  * 		Code **-EINVAL** if the socket is not a full TCP socket;
//C  * 		otherwise, a positive number containing the bits that could not
//C  * 		be set is returned (which comes down to 0 if all bits were set
//C  * 		as required).
//C  *
//C  * long bpf_msg_redirect_map(struct sk_msg_buff *msg, struct bpf_map *map, u32 key, u64 flags)
//C  * 	Description
//C  * 		This helper is used in programs implementing policies at the
//C  * 		socket level. If the message *msg* is allowed to pass (i.e. if
//C  * 		the verdict eBPF program returns **SK_PASS**), redirect it to
//C  * 		the socket referenced by *map* (of type
//C  * 		**BPF_MAP_TYPE_SOCKMAP**) at index *key*. Both ingress and
//C  * 		egress interfaces can be used for redirection. The
//C  * 		**BPF_F_INGRESS** value in *flags* is used to make the
//C  * 		distinction (ingress path is selected if the flag is present,
//C  * 		egress path otherwise). This is the only flag supported for now.
//C  * 	Return
//C  * 		**SK_PASS** on success, or **SK_DROP** on error.
//C  *
//C  * long bpf_msg_apply_bytes(struct sk_msg_buff *msg, u32 bytes)
//C  * 	Description
//C  * 		For socket policies, apply the verdict of the eBPF program to
//C  * 		the next *bytes* (number of bytes) of message *msg*.
//C  *
//C  * 		For example, this helper can be used in the following cases:
//C  *
//C  * 		* A single **sendmsg**\ () or **sendfile**\ () system call
//C  * 		  contains multiple logical messages that the eBPF program is
//C  * 		  supposed to read and for which it should apply a verdict.
//C  * 		* An eBPF program only cares to read the first *bytes* of a
//C  * 		  *msg*. If the message has a large payload, then setting up
//C  * 		  and calling the eBPF program repeatedly for all bytes, even
//C  * 		  though the verdict is already known, would create unnecessary
//C  * 		  overhead.
//C  *
//C  * 		When called from within an eBPF program, the helper sets a
//C  * 		counter internal to the BPF infrastructure, that is used to
//C  * 		apply the last verdict to the next *bytes*. If *bytes* is
//C  * 		smaller than the current data being processed from a
//C  * 		**sendmsg**\ () or **sendfile**\ () system call, the first
//C  * 		*bytes* will be sent and the eBPF program will be re-run with
//C  * 		the pointer for start of data pointing to byte number *bytes*
//C  * 		**+ 1**. If *bytes* is larger than the current data being
//C  * 		processed, then the eBPF verdict will be applied to multiple
//C  * 		**sendmsg**\ () or **sendfile**\ () calls until *bytes* are
//C  * 		consumed.
//C  *
//C  * 		Note that if a socket closes with the internal counter holding
//C  * 		a non-zero value, this is not a problem because data is not
//C  * 		being buffered for *bytes* and is sent as it is received.
//C  * 	Return
//C  * 		0
//C  *
//C  * long bpf_msg_cork_bytes(struct sk_msg_buff *msg, u32 bytes)
//C  * 	Description
//C  * 		For socket policies, prevent the execution of the verdict eBPF
//C  * 		program for message *msg* until *bytes* (byte number) have been
//C  * 		accumulated.
//C  *
//C  * 		This can be used when one needs a specific number of bytes
//C  * 		before a verdict can be assigned, even if the data spans
//C  * 		multiple **sendmsg**\ () or **sendfile**\ () calls. The extreme
//C  * 		case would be a user calling **sendmsg**\ () repeatedly with
//C  * 		1-byte long message segments. Obviously, this is bad for
//C  * 		performance, but it is still valid. If the eBPF program needs
//C  * 		*bytes* bytes to validate a header, this helper can be used to
//C  * 		prevent the eBPF program to be called again until *bytes* have
//C  * 		been accumulated.
//C  * 	Return
//C  * 		0
//C  *
//C  * long bpf_msg_pull_data(struct sk_msg_buff *msg, u32 start, u32 end, u64 flags)
//C  * 	Description
//C  * 		For socket policies, pull in non-linear data from user space
//C  * 		for *msg* and set pointers *msg*\ **->data** and *msg*\
//C  * 		**->data_end** to *start* and *end* bytes offsets into *msg*,
//C  * 		respectively.
//C  *
//C  * 		If a program of type **BPF_PROG_TYPE_SK_MSG** is run on a
//C  * 		*msg* it can only parse data that the (**data**, **data_end**)
//C  * 		pointers have already consumed. For **sendmsg**\ () hooks this
//C  * 		is likely the first scatterlist element. But for calls relying
//C  * 		on the **sendpage** handler (e.g. **sendfile**\ ()) this will
//C  * 		be the range (**0**, **0**) because the data is shared with
//C  * 		user space and by default the objective is to avoid allowing
//C  * 		user space to modify data while (or after) eBPF verdict is
//C  * 		being decided. This helper can be used to pull in data and to
//C  * 		set the start and end pointer to given values. Data will be
//C  * 		copied if necessary (i.e. if data was not linear and if start
//C  * 		and end pointers do not point to the same chunk).
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  *
//C  * 		All values for *flags* are reserved for future usage, and must
//C  * 		be left at zero.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_bind(struct bpf_sock_addr *ctx, struct sockaddr *addr, int addr_len)
//C  * 	Description
//C  * 		Bind the socket associated to *ctx* to the address pointed by
//C  * 		*addr*, of length *addr_len*. This allows for making outgoing
//C  * 		connection from the desired IP address, which can be useful for
//C  * 		example when all processes inside a cgroup should use one
//C  * 		single IP address on a host that has multiple IP configured.
//C  *
//C  * 		This helper works for IPv4 and IPv6, TCP and UDP sockets. The
//C  * 		domain (*addr*\ **->sa_family**) must be **AF_INET** (or
//C  * 		**AF_INET6**). It's advised to pass zero port (**sin_port**
//C  * 		or **sin6_port**) which triggers IP_BIND_ADDRESS_NO_PORT-like
//C  * 		behavior and lets the kernel efficiently pick up an unused
//C  * 		port as long as 4-tuple is unique. Passing non-zero port might
//C  * 		lead to degraded performance.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_xdp_adjust_tail(struct xdp_buff *xdp_md, int delta)
//C  * 	Description
//C  * 		Adjust (move) *xdp_md*\ **->data_end** by *delta* bytes. It is
//C  * 		possible to both shrink and grow the packet tail.
//C  * 		Shrink done via *delta* being a negative integer.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_get_xfrm_state(struct sk_buff *skb, u32 index, struct bpf_xfrm_state *xfrm_state, u32 size, u64 flags)
//C  * 	Description
//C  * 		Retrieve the XFRM state (IP transform framework, see also
//C  * 		**ip-xfrm(8)**) at *index* in XFRM "security path" for *skb*.
//C  *
//C  * 		The retrieved value is stored in the **struct bpf_xfrm_state**
//C  * 		pointed by *xfrm_state* and of length *size*.
//C  *
//C  * 		All values for *flags* are reserved for future usage, and must
//C  * 		be left at zero.
//C  *
//C  * 		This helper is available only if the kernel was compiled with
//C  * 		**CONFIG_XFRM** configuration option.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_get_stack(void *ctx, void *buf, u32 size, u64 flags)
//C  * 	Description
//C  * 		Return a user or a kernel stack in bpf program provided buffer.
//C  * 		To achieve this, the helper needs *ctx*, which is a pointer
//C  * 		to the context on which the tracing program is executed.
//C  * 		To store the stacktrace, the bpf program provides *buf* with
//C  * 		a nonnegative *size*.
//C  *
//C  * 		The last argument, *flags*, holds the number of stack frames to
//C  * 		skip (from 0 to 255), masked with
//C  * 		**BPF_F_SKIP_FIELD_MASK**. The next bits can be used to set
//C  * 		the following flags:
//C  *
//C  * 		**BPF_F_USER_STACK**
//C  * 			Collect a user space stack instead of a kernel stack.
//C  * 		**BPF_F_USER_BUILD_ID**
//C  * 			Collect (build_id, file_offset) instead of ips for user
//C  * 			stack, only valid if **BPF_F_USER_STACK** is also
//C  * 			specified.
//C  *
//C  * 			*file_offset* is an offset relative to the beginning
//C  * 			of the executable or shared object file backing the vma
//C  * 			which the *ip* falls in. It is *not* an offset relative
//C  * 			to that object's base address. Accordingly, it must be
//C  * 			adjusted by adding (sh_addr - sh_offset), where
//C  * 			sh_{addr,offset} correspond to the executable section
//C  * 			containing *file_offset* in the object, for comparisons
//C  * 			to symbols' st_value to be valid.
//C  *
//C  * 		**bpf_get_stack**\ () can collect up to
//C  * 		**PERF_MAX_STACK_DEPTH** both kernel and user frames, subject
//C  * 		to sufficient large buffer size. Note that
//C  * 		this limit can be controlled with the **sysctl** program, and
//C  * 		that it should be manually increased in order to profile long
//C  * 		user stacks (such as stacks for Java programs). To do so, use:
//C  *
//C  * 		::
//C  *
//C  * 			# sysctl kernel.perf_event_max_stack=<new value>
//C  * 	Return
//C  * 		The non-negative copied *buf* length equal to or less than
//C  * 		*size* on success, or a negative error in case of failure.
//C  *
//C  * long bpf_skb_load_bytes_relative(const void *skb, u32 offset, void *to, u32 len, u32 start_header)
//C  * 	Description
//C  * 		This helper is similar to **bpf_skb_load_bytes**\ () in that
//C  * 		it provides an easy way to load *len* bytes from *offset*
//C  * 		from the packet associated to *skb*, into the buffer pointed
//C  * 		by *to*. The difference to **bpf_skb_load_bytes**\ () is that
//C  * 		a fifth argument *start_header* exists in order to select a
//C  * 		base offset to start from. *start_header* can be one of:
//C  *
//C  * 		**BPF_HDR_START_MAC**
//C  * 			Base offset to load data from is *skb*'s mac header.
//C  * 		**BPF_HDR_START_NET**
//C  * 			Base offset to load data from is *skb*'s network header.
//C  *
//C  * 		In general, "direct packet access" is the preferred method to
//C  * 		access packet data, however, this helper is in particular useful
//C  * 		in socket filters where *skb*\ **->data** does not always point
//C  * 		to the start of the mac header and where "direct packet access"
//C  * 		is not available.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_fib_lookup(void *ctx, struct bpf_fib_lookup *params, int plen, u32 flags)
//C  *	Description
//C  *		Do FIB lookup in kernel tables using parameters in *params*.
//C  *		If lookup is successful and result shows packet is to be
//C  *		forwarded, the neighbor tables are searched for the nexthop.
//C  *		If successful (ie., FIB lookup shows forwarding and nexthop
//C  *		is resolved), the nexthop address is returned in ipv4_dst
//C  *		or ipv6_dst based on family, smac is set to mac address of
//C  *		egress device, dmac is set to nexthop mac address, rt_metric
//C  *		is set to metric from route (IPv4/IPv6 only), and ifindex
//C  *		is set to the device index of the nexthop from the FIB lookup.
//C  *
//C  *		*plen* argument is the size of the passed in struct.
//C  *		*flags* argument can be a combination of one or more of the
//C  *		following values:
//C  *
//C  *		**BPF_FIB_LOOKUP_DIRECT**
//C  *			Do a direct table lookup vs full lookup using FIB
//C  *			rules.
//C  *		**BPF_FIB_LOOKUP_TBID**
//C  *			Used with BPF_FIB_LOOKUP_DIRECT.
//C  *			Use the routing table ID present in *params*->tbid
//C  *			for the fib lookup.
//C  *		**BPF_FIB_LOOKUP_OUTPUT**
//C  *			Perform lookup from an egress perspective (default is
//C  *			ingress).
//C  *		**BPF_FIB_LOOKUP_SKIP_NEIGH**
//C  *			Skip the neighbour table lookup. *params*->dmac
//C  *			and *params*->smac will not be set as output. A common
//C  *			use case is to call **bpf_redirect_neigh**\ () after
//C  *			doing **bpf_fib_lookup**\ ().
//C  *		**BPF_FIB_LOOKUP_SRC**
//C  *			Derive and set source IP addr in *params*->ipv{4,6}_src
//C  *			for the nexthop. If the src addr cannot be derived,
//C  *			**BPF_FIB_LKUP_RET_NO_SRC_ADDR** is returned. In this
//C  *			case, *params*->dmac and *params*->smac are not set either.
//C  *		**BPF_FIB_LOOKUP_MARK**
//C  *			Use the mark present in *params*->mark for the fib lookup.
//C  *			This option should not be used with BPF_FIB_LOOKUP_DIRECT,
//C  *			as it only has meaning for full lookups.
//C  *		**BPF_FIB_LOOKUP_VLAN**
//C  *			If the fib lookup resolves to a VLAN device whose
//C  *			parent is a real (non-VLAN) device, set
//C  *			*params*->h_vlan_proto and *params*->h_vlan_TCI from
//C  *			the VLAN device and replace *params*->ifindex with the
//C  *			parent's ifindex. *params*->h_vlan_TCI carries the VID
//C  *			only, with PCP and DEI bits zero; a consumer wanting to
//C  *			set egress priority writes PCP itself. *params*->smac is
//C  *			the VLAN device's own address, which can differ from the
//C  *			parent's. Only the immediate parent is resolved; if it
//C  *			is itself a VLAN device (QinQ) or in another namespace,
//C  *			the egress cannot be reduced to a physical device plus
//C  *			one tag and the lookup returns
//C  *			**BPF_FIB_LKUP_RET_VLAN_FAILURE** with *params*->ifindex
//C  *			left at the input. To obtain the VLAN device's own
//C  *			ifindex, repeat the lookup without
//C  *			**BPF_FIB_LOOKUP_VLAN**, re-initializing *params*
//C  *			first: output fields overwrite the inputs they share
//C  *			storage with. The swap and the vlan fields
//C  *			are written only on success; other output fields keep
//C  *			the helper's existing behaviour, so a frag-needed result
//C  *			still reports the route mtu in *params*->mtu_result.
//C  *			This flag is only valid for XDP programs; tc programs
//C  *			receive -EINVAL since they can redirect to the VLAN
//C  *			device directly.
//C  *		**BPF_FIB_LOOKUP_VLAN_INPUT**
//C  *			Treat *params*->h_vlan_proto and *params*->h_vlan_TCI
//C  *			as an input VLAN tag and run the lookup as if ingress
//C  *			had happened on the VLAN subinterface carrying that tag
//C  *			on *params*->ifindex. The VID is the low 12 bits of
//C  *			*params*->h_vlan_TCI; *params*->h_vlan_proto must be
//C  *			ETH_P_8021Q or ETH_P_8021AD in network byte order, else
//C  *			**-EINVAL**. If *params*->ifindex is itself a VLAN
//C  *			device, its inner (QinQ) subinterface is matched; for a
//C  *			bond or team, pass the master's ifindex. An unmatched
//C  *			tag, a down device, or one in another namespace returns
//C  *			**BPF_FIB_LKUP_RET_NOT_FWDED**, mirroring real ingress.
//C  *			A VID of 0 is looked up literally, so do not set this
//C  *			flag for priority-tagged frames. Cannot be combined with
//C  *			**BPF_FIB_LOOKUP_TBID** or **BPF_FIB_LOOKUP_OUTPUT**
//C  *			(returns **-EINVAL**).
//C  *
//C  *		*ctx* is either **struct xdp_md** for XDP programs or
//C  *		**struct sk_buff** tc cls_act programs.
//C  *	Return
//C  *		* < 0 if any input argument is invalid
//C  *		*   0 on success (packet is forwarded, nexthop neighbor exists)
//C  *		* > 0 one of **BPF_FIB_LKUP_RET_** codes explaining why the
//C  *		  packet is not forwarded or needs assist from full stack
//C  *
//C  *		If lookup fails with BPF_FIB_LKUP_RET_FRAG_NEEDED, then the MTU
//C  *		was exceeded and output params->mtu_result contains the MTU.
//C  *
//C  * long bpf_sock_hash_update(struct bpf_sock_ops *skops, struct bpf_map *map, void *key, u64 flags)
//C  *	Description
//C  *		Add an entry to, or update a sockhash *map* referencing sockets.
//C  *		The *skops* is used as a new value for the entry associated to
//C  *		*key*. *flags* is one of:
//C  *
//C  *		**BPF_NOEXIST**
//C  *			The entry for *key* must not exist in the map.
//C  *		**BPF_EXIST**
//C  *			The entry for *key* must already exist in the map.
//C  *		**BPF_ANY**
//C  *			No condition on the existence of the entry for *key*.
//C  *
//C  *		If the *map* has eBPF programs (parser and verdict), those will
//C  *		be inherited by the socket being added. If the socket is
//C  *		already attached to eBPF programs, this results in an error.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_msg_redirect_hash(struct sk_msg_buff *msg, struct bpf_map *map, void *key, u64 flags)
//C  *	Description
//C  *		This helper is used in programs implementing policies at the
//C  *		socket level. If the message *msg* is allowed to pass (i.e. if
//C  *		the verdict eBPF program returns **SK_PASS**), redirect it to
//C  *		the socket referenced by *map* (of type
//C  *		**BPF_MAP_TYPE_SOCKHASH**) using hash *key*. Both ingress and
//C  *		egress interfaces can be used for redirection. The
//C  *		**BPF_F_INGRESS** value in *flags* is used to make the
//C  *		distinction (ingress path is selected if the flag is present,
//C  *		egress path otherwise). This is the only flag supported for now.
//C  *	Return
//C  *		**SK_PASS** on success, or **SK_DROP** on error.
//C  *
//C  * long bpf_sk_redirect_hash(struct sk_buff *skb, struct bpf_map *map, void *key, u64 flags)
//C  *	Description
//C  *		This helper is used in programs implementing policies at the
//C  *		skb socket level. If the sk_buff *skb* is allowed to pass (i.e.
//C  *		if the verdict eBPF program returns **SK_PASS**), redirect it
//C  *		to the socket referenced by *map* (of type
//C  *		**BPF_MAP_TYPE_SOCKHASH**) using hash *key*. Both ingress and
//C  *		egress interfaces can be used for redirection. The
//C  *		**BPF_F_INGRESS** value in *flags* is used to make the
//C  *		distinction (ingress path is selected if the flag is present,
//C  *		egress otherwise). This is the only flag supported for now.
//C  *	Return
//C  *		**SK_PASS** on success, or **SK_DROP** on error.
//C  *
//C  * long bpf_lwt_push_encap(struct sk_buff *skb, u32 type, void *hdr, u32 len)
//C  *	Description
//C  *		Encapsulate the packet associated to *skb* within a Layer 3
//C  *		protocol header. This header is provided in the buffer at
//C  *		address *hdr*, with *len* its size in bytes. *type* indicates
//C  *		the protocol of the header and can be one of:
//C  *
//C  *		**BPF_LWT_ENCAP_SEG6**
//C  *			IPv6 encapsulation with Segment Routing Header
//C  *			(**struct ipv6_sr_hdr**). *hdr* only contains the SRH,
//C  *			the IPv6 header is computed by the kernel.
//C  *		**BPF_LWT_ENCAP_SEG6_INLINE**
//C  *			Only works if *skb* contains an IPv6 packet. Insert a
//C  *			Segment Routing Header (**struct ipv6_sr_hdr**) inside
//C  *			the IPv6 header.
//C  *		**BPF_LWT_ENCAP_IP**
//C  *			IP encapsulation (GRE/GUE/IPIP/etc). The outer header
//C  *			must be IPv4 or IPv6, followed by zero or more
//C  *			additional headers, up to **LWT_BPF_MAX_HEADROOM**
//C  *			total bytes in all prepended headers. Please note that
//C  *			if **skb_is_gso**\ (*skb*) is true, no more than two
//C  *			headers can be prepended, and the inner header, if
//C  *			present, should be either GRE or UDP/GUE.
//C  *
//C  *		**BPF_LWT_ENCAP_SEG6**\ \* types can be called by BPF programs
//C  *		of type **BPF_PROG_TYPE_LWT_IN**; **BPF_LWT_ENCAP_IP** type can
//C  *		be called by bpf programs of types **BPF_PROG_TYPE_LWT_IN** and
//C  *		**BPF_PROG_TYPE_LWT_XMIT**.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  *	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_lwt_seg6_store_bytes(struct sk_buff *skb, u32 offset, const void *from, u32 len)
//C  *	Description
//C  *		Store *len* bytes from address *from* into the packet
//C  *		associated to *skb*, at *offset*. Only the flags, tag and TLVs
//C  *		inside the outermost IPv6 Segment Routing Header can be
//C  *		modified through this helper.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  *	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_lwt_seg6_adjust_srh(struct sk_buff *skb, u32 offset, s32 delta)
//C  *	Description
//C  *		Adjust the size allocated to TLVs in the outermost IPv6
//C  *		Segment Routing Header contained in the packet associated to
//C  *		*skb*, at position *offset* by *delta* bytes. Only offsets
//C  *		after the segments are accepted. *delta* can be as well
//C  *		positive (growing) as negative (shrinking).
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  *	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_lwt_seg6_action(struct sk_buff *skb, u32 action, void *param, u32 param_len)
//C  *	Description
//C  *		Apply an IPv6 Segment Routing action of type *action* to the
//C  *		packet associated to *skb*. Each action takes a parameter
//C  *		contained at address *param*, and of length *param_len* bytes.
//C  *		*action* can be one of:
//C  *
//C  *		**SEG6_LOCAL_ACTION_END_X**
//C  *			End.X action: Endpoint with Layer-3 cross-connect.
//C  *			Type of *param*: **struct in6_addr**.
//C  *		**SEG6_LOCAL_ACTION_END_T**
//C  *			End.T action: Endpoint with specific IPv6 table lookup.
//C  *			Type of *param*: **int**.
//C  *		**SEG6_LOCAL_ACTION_END_B6**
//C  *			End.B6 action: Endpoint bound to an SRv6 policy.
//C  *			Type of *param*: **struct ipv6_sr_hdr**.
//C  *		**SEG6_LOCAL_ACTION_END_B6_ENCAP**
//C  *			End.B6.Encap action: Endpoint bound to an SRv6
//C  *			encapsulation policy.
//C  *			Type of *param*: **struct ipv6_sr_hdr**.
//C  *
//C  * 		A call to this helper is susceptible to change the underlying
//C  * 		packet buffer. Therefore, at load time, all checks on pointers
//C  * 		previously done by the verifier are invalidated and must be
//C  * 		performed again, if the helper is used in combination with
//C  * 		direct packet access.
//C  *	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_rc_repeat(void *ctx)
//C  *	Description
//C  *		This helper is used in programs implementing IR decoding, to
//C  *		report a successfully decoded repeat key message. This delays
//C  *		the generation of a key up event for previously generated
//C  *		key down event.
//C  *
//C  *		Some IR protocols like NEC have a special IR message for
//C  *		repeating last button, for when a button is held down.
//C  *
//C  *		The *ctx* should point to the lirc sample as passed into
//C  *		the program.
//C  *
//C  *		This helper is only available is the kernel was compiled with
//C  *		the **CONFIG_BPF_LIRC_MODE2** configuration option set to
//C  *		"**y**".
//C  *	Return
//C  *		0
//C  *
//C  * long bpf_rc_keydown(void *ctx, u32 protocol, u64 scancode, u32 toggle)
//C  *	Description
//C  *		This helper is used in programs implementing IR decoding, to
//C  *		report a successfully decoded key press with *scancode*,
//C  *		*toggle* value in the given *protocol*. The scancode will be
//C  *		translated to a keycode using the rc keymap, and reported as
//C  *		an input key down event. After a period a key up event is
//C  *		generated. This period can be extended by calling either
//C  *		**bpf_rc_keydown**\ () again with the same values, or calling
//C  *		**bpf_rc_repeat**\ ().
//C  *
//C  *		Some protocols include a toggle bit, in case the button was
//C  *		released and pressed again between consecutive scancodes.
//C  *
//C  *		The *ctx* should point to the lirc sample as passed into
//C  *		the program.
//C  *
//C  *		The *protocol* is the decoded protocol number (see
//C  *		**enum rc_proto** for some predefined values).
//C  *
//C  *		This helper is only available is the kernel was compiled with
//C  *		the **CONFIG_BPF_LIRC_MODE2** configuration option set to
//C  *		"**y**".
//C  *	Return
//C  *		0
//C  *
//C  * u64 bpf_skb_cgroup_id(struct sk_buff *skb)
//C  * 	Description
//C  * 		Return the cgroup v2 id of the socket associated with the *skb*.
//C  * 		This is roughly similar to the **bpf_get_cgroup_classid**\ ()
//C  * 		helper for cgroup v1 by providing a tag resp. identifier that
//C  * 		can be matched on or used for map lookups e.g. to implement
//C  * 		policy. The cgroup v2 id of a given path in the hierarchy is
//C  * 		exposed in user space through the f_handle API in order to get
//C  * 		to the same 64-bit id.
//C  *
//C  * 		This helper can be used on TC egress path, but not on ingress,
//C  * 		and is available only if the kernel was compiled with the
//C  * 		**CONFIG_SOCK_CGROUP_DATA** configuration option.
//C  * 	Return
//C  * 		The id is returned or 0 in case the id could not be retrieved.
//C  *
//C  * u64 bpf_get_current_cgroup_id(void)
//C  * 	Description
//C  * 		Get the current cgroup id based on the cgroup within which
//C  * 		the current task is running.
//C  * 	Return
//C  * 		A 64-bit integer containing the current cgroup id based
//C  * 		on the cgroup within which the current task is running.
//C  *
//C  * void *bpf_get_local_storage(void *map, u64 flags)
//C  *	Description
//C  *		Get the pointer to the local storage area.
//C  *		The type and the size of the local storage is defined
//C  *		by the *map* argument.
//C  *		The *flags* meaning is specific for each map type,
//C  *		and has to be 0 for cgroup local storage.
//C  *
//C  *		Depending on the BPF program type, a local storage area
//C  *		can be shared between multiple instances of the BPF program,
//C  *		running simultaneously.
//C  *
//C  *		A user should care about the synchronization by himself.
//C  *		For example, by using the **BPF_ATOMIC** instructions to alter
//C  *		the shared data.
//C  *	Return
//C  *		A pointer to the local storage area.
//C  *
//C  * long bpf_sk_select_reuseport(struct sk_reuseport_md *reuse, struct bpf_map *map, void *key, u64 flags)
//C  *	Description
//C  *		Select a **SO_REUSEPORT** socket from a
//C  *		**BPF_MAP_TYPE_REUSEPORT_SOCKARRAY** *map*.
//C  *		It checks the selected socket is matching the incoming
//C  *		request in the socket buffer.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * u64 bpf_skb_ancestor_cgroup_id(struct sk_buff *skb, int ancestor_level)
//C  *	Description
//C  *		Return id of cgroup v2 that is ancestor of cgroup associated
//C  *		with the *skb* at the *ancestor_level*.  The root cgroup is at
//C  *		*ancestor_level* zero and each step down the hierarchy
//C  *		increments the level. If *ancestor_level* == level of cgroup
//C  *		associated with *skb*, then return value will be same as that
//C  *		of **bpf_skb_cgroup_id**\ ().
//C  *
//C  *		The helper is useful to implement policies based on cgroups
//C  *		that are upper in hierarchy than immediate cgroup associated
//C  *		with *skb*.
//C  *
//C  *		The format of returned id and helper limitations are same as in
//C  *		**bpf_skb_cgroup_id**\ ().
//C  *	Return
//C  *		The id is returned or 0 in case the id could not be retrieved.
//C  *
//C  * struct bpf_sock *bpf_sk_lookup_tcp(void *ctx, struct bpf_sock_tuple *tuple, u32 tuple_size, u64 netns, u64 flags)
//C  *	Description
//C  *		Look for TCP socket matching *tuple*, optionally in a child
//C  *		network namespace *netns*. The return value must be checked,
//C  *		and if non-**NULL**, released via **bpf_sk_release**\ ().
//C  *
//C  *		The *ctx* should point to the context of the program, such as
//C  *		the skb or socket (depending on the hook in use). This is used
//C  *		to determine the base network namespace for the lookup.
//C  *
//C  *		*tuple_size* must be one of:
//C  *
//C  *		**sizeof**\ (*tuple*\ **->ipv4**)
//C  *			Look for an IPv4 socket.
//C  *		**sizeof**\ (*tuple*\ **->ipv6**)
//C  *			Look for an IPv6 socket.
//C  *
//C  *		If the *netns* is a negative signed 32-bit integer, then the
//C  *		socket lookup table in the netns associated with the *ctx*
//C  *		will be used. For the TC hooks, this is the netns of the device
//C  *		in the skb. For socket hooks, this is the netns of the socket.
//C  *		If *netns* is any other signed 32-bit value greater than or
//C  *		equal to zero then it specifies the ID of the netns relative to
//C  *		the netns associated with the *ctx*. *netns* values beyond the
//C  *		range of 32-bit integers are reserved for future use.
//C  *
//C  *		All values for *flags* are reserved for future usage, and must
//C  *		be left at zero.
//C  *
//C  *		This helper is available only if the kernel was compiled with
//C  *		**CONFIG_NET** configuration option.
//C  *	Return
//C  *		Pointer to **struct bpf_sock**, or **NULL** in case of failure.
//C  *		For sockets with reuseport option, the **struct bpf_sock**
//C  *		result is from *reuse*\ **->socks**\ [] using the hash of the
//C  *		tuple.
//C  *
//C  * struct bpf_sock *bpf_sk_lookup_udp(void *ctx, struct bpf_sock_tuple *tuple, u32 tuple_size, u64 netns, u64 flags)
//C  *	Description
//C  *		Look for UDP socket matching *tuple*, optionally in a child
//C  *		network namespace *netns*. The return value must be checked,
//C  *		and if non-**NULL**, released via **bpf_sk_release**\ ().
//C  *
//C  *		The *ctx* should point to the context of the program, such as
//C  *		the skb or socket (depending on the hook in use). This is used
//C  *		to determine the base network namespace for the lookup.
//C  *
//C  *		*tuple_size* must be one of:
//C  *
//C  *		**sizeof**\ (*tuple*\ **->ipv4**)
//C  *			Look for an IPv4 socket.
//C  *		**sizeof**\ (*tuple*\ **->ipv6**)
//C  *			Look for an IPv6 socket.
//C  *
//C  *		If the *netns* is a negative signed 32-bit integer, then the
//C  *		socket lookup table in the netns associated with the *ctx*
//C  *		will be used. For the TC hooks, this is the netns of the device
//C  *		in the skb. For socket hooks, this is the netns of the socket.
//C  *		If *netns* is any other signed 32-bit value greater than or
//C  *		equal to zero then it specifies the ID of the netns relative to
//C  *		the netns associated with the *ctx*. *netns* values beyond the
//C  *		range of 32-bit integers are reserved for future use.
//C  *
//C  *		All values for *flags* are reserved for future usage, and must
//C  *		be left at zero.
//C  *
//C  *		This helper is available only if the kernel was compiled with
//C  *		**CONFIG_NET** configuration option.
//C  *	Return
//C  *		Pointer to **struct bpf_sock**, or **NULL** in case of failure.
//C  *		For sockets with reuseport option, the **struct bpf_sock**
//C  *		result is from *reuse*\ **->socks**\ [] using the hash of the
//C  *		tuple.
//C  *
//C  * long bpf_sk_release(void *sock)
//C  *	Description
//C  *		Release the reference held by *sock*. *sock* must be a
//C  *		non-**NULL** pointer that was returned from
//C  *		**bpf_sk_lookup_xxx**\ ().
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_map_push_elem(struct bpf_map *map, const void *value, u64 flags)
//C  * 	Description
//C  * 		Push an element *value* in *map*. *flags* is one of:
//C  *
//C  * 		**BPF_EXIST**
//C  * 			If the queue/stack is full, the oldest element is
//C  * 			removed to make room for this.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_map_pop_elem(struct bpf_map *map, void *value)
//C  * 	Description
//C  * 		Pop an element from *map*.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_map_peek_elem(struct bpf_map *map, void *value)
//C  * 	Description
//C  * 		Get an element from *map* without removing it.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_msg_push_data(struct sk_msg_buff *msg, u32 start, u32 len, u64 flags)
//C  *	Description
//C  *		For socket policies, insert *len* bytes into *msg* at offset
//C  *		*start*.
//C  *
//C  *		If a program of type **BPF_PROG_TYPE_SK_MSG** is run on a
//C  *		*msg* it may want to insert metadata or options into the *msg*.
//C  *		This can later be read and used by any of the lower layer BPF
//C  *		hooks.
//C  *
//C  *		This helper may fail if under memory pressure (a malloc
//C  *		fails) in these cases BPF programs will get an appropriate
//C  *		error and BPF programs will need to handle them.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_msg_pop_data(struct sk_msg_buff *msg, u32 start, u32 len, u64 flags)
//C  *	Description
//C  *		Will remove *len* bytes from a *msg* starting at byte *start*.
//C  *		This may result in **ENOMEM** errors under certain situations if
//C  *		an allocation and copy are required due to a full ring buffer.
//C  *		However, the helper will try to avoid doing the allocation
//C  *		if possible. Other errors can occur if input parameters are
//C  *		invalid either due to *start* byte not being valid part of *msg*
//C  *		payload and/or *pop* value being to large.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_rc_pointer_rel(void *ctx, s32 rel_x, s32 rel_y)
//C  *	Description
//C  *		This helper is used in programs implementing IR decoding, to
//C  *		report a successfully decoded pointer movement.
//C  *
//C  *		The *ctx* should point to the lirc sample as passed into
//C  *		the program.
//C  *
//C  *		This helper is only available is the kernel was compiled with
//C  *		the **CONFIG_BPF_LIRC_MODE2** configuration option set to
//C  *		"**y**".
//C  *	Return
//C  *		0
//C  *
//C  * long bpf_spin_lock(struct bpf_spin_lock *lock)
//C  *	Description
//C  *		Acquire a spinlock represented by the pointer *lock*, which is
//C  *		stored as part of a value of a map. Taking the lock allows to
//C  *		safely update the rest of the fields in that value. The
//C  *		spinlock can (and must) later be released with a call to
//C  *		**bpf_spin_unlock**\ (\ *lock*\ ).
//C  *
//C  *		Spinlocks in BPF programs come with a number of restrictions
//C  *		and constraints:
//C  *
//C  *		* **bpf_spin_lock** objects are only allowed inside maps of
//C  *		  types **BPF_MAP_TYPE_HASH** and **BPF_MAP_TYPE_ARRAY** (this
//C  *		  list could be extended in the future).
//C  *		* BTF description of the map is mandatory.
//C  *		* The BPF program can take ONE lock at a time, since taking two
//C  *		  or more could cause dead locks.
//C  *		* Only one **struct bpf_spin_lock** is allowed per map element.
//C  *		* When the lock is taken, calls (either BPF to BPF or helpers)
//C  *		  are not allowed.
//C  *		* The **BPF_LD_ABS** and **BPF_LD_IND** instructions are not
//C  *		  allowed inside a spinlock-ed region.
//C  *		* The BPF program MUST call **bpf_spin_unlock**\ () to release
//C  *		  the lock, on all execution paths, before it returns.
//C  *		* The BPF program can access **struct bpf_spin_lock** only via
//C  *		  the **bpf_spin_lock**\ () and **bpf_spin_unlock**\ ()
//C  *		  helpers. Loading or storing data into the **struct
//C  *		  bpf_spin_lock** *lock*\ **;** field of a map is not allowed.
//C  *		* To use the **bpf_spin_lock**\ () helper, the BTF description
//C  *		  of the map value must be a struct and have **struct
//C  *		  bpf_spin_lock** *anyname*\ **;** field at the top level.
//C  *		  Nested lock inside another struct is not allowed.
//C  *		* The **struct bpf_spin_lock** *lock* field in a map value must
//C  *		  be aligned on a multiple of 4 bytes in that value.
//C  *		* Syscall with command **BPF_MAP_LOOKUP_ELEM** does not copy
//C  *		  the **bpf_spin_lock** field to user space.
//C  *		* Syscall with command **BPF_MAP_UPDATE_ELEM**, or update from
//C  *		  a BPF program, do not update the **bpf_spin_lock** field.
//C  *		* **bpf_spin_lock** cannot be on the stack or inside a
//C  *		  networking packet (it can only be inside of a map values).
//C  *		* **bpf_spin_lock** is available to root only.
//C  *		* Tracing programs and socket filter programs cannot use
//C  *		  **bpf_spin_lock**\ () due to insufficient preemption checks
//C  *		  (but this may change in the future).
//C  *		* **bpf_spin_lock** is not allowed in inner maps of map-in-map.
//C  *	Return
//C  *		0
//C  *
//C  * long bpf_spin_unlock(struct bpf_spin_lock *lock)
//C  *	Description
//C  *		Release the *lock* previously locked by a call to
//C  *		**bpf_spin_lock**\ (\ *lock*\ ).
//C  *	Return
//C  *		0
//C  *
//C  * struct bpf_sock *bpf_sk_fullsock(struct bpf_sock *sk)
//C  *	Description
//C  *		This helper gets a **struct bpf_sock** pointer such
//C  *		that all the fields in this **bpf_sock** can be accessed.
//C  *	Return
//C  *		A **struct bpf_sock** pointer on success, or **NULL** in
//C  *		case of failure.
//C  *
//C  * struct bpf_tcp_sock *bpf_tcp_sock(struct bpf_sock *sk)
//C  *	Description
//C  *		This helper gets a **struct bpf_tcp_sock** pointer from a
//C  *		**struct bpf_sock** pointer.
//C  *	Return
//C  *		A **struct bpf_tcp_sock** pointer on success, or **NULL** in
//C  *		case of failure.
//C  *
//C  * long bpf_skb_ecn_set_ce(struct sk_buff *skb)
//C  *	Description
//C  *		Set ECN (Explicit Congestion Notification) field of IP header
//C  *		to **CE** (Congestion Encountered) if current value is **ECT**
//C  *		(ECN Capable Transport). Otherwise, do nothing. Works with IPv6
//C  *		and IPv4.
//C  *	Return
//C  *		1 if the **CE** flag is set (either by the current helper call
//C  *		or because it was already present), 0 if it is not set.
//C  *
//C  * struct bpf_sock *bpf_get_listener_sock(struct bpf_sock *sk)
//C  *	Description
//C  *		Return a **struct bpf_sock** pointer in **TCP_LISTEN** state.
//C  *		**bpf_sk_release**\ () is unnecessary and not allowed.
//C  *	Return
//C  *		A **struct bpf_sock** pointer on success, or **NULL** in
//C  *		case of failure.
//C  *
//C  * struct bpf_sock *bpf_skc_lookup_tcp(void *ctx, struct bpf_sock_tuple *tuple, u32 tuple_size, u64 netns, u64 flags)
//C  *	Description
//C  *		Look for TCP socket matching *tuple*, optionally in a child
//C  *		network namespace *netns*. The return value must be checked,
//C  *		and if non-**NULL**, released via **bpf_sk_release**\ ().
//C  *
//C  *		This function is identical to **bpf_sk_lookup_tcp**\ (), except
//C  *		that it also returns timewait or request sockets. Use
//C  *		**bpf_sk_fullsock**\ () or **bpf_tcp_sock**\ () to access the
//C  *		full structure.
//C  *
//C  *		This helper is available only if the kernel was compiled with
//C  *		**CONFIG_NET** configuration option.
//C  *	Return
//C  *		Pointer to **struct bpf_sock**, or **NULL** in case of failure.
//C  *		For sockets with reuseport option, the **struct bpf_sock**
//C  *		result is from *reuse*\ **->socks**\ [] using the hash of the
//C  *		tuple.
//C  *
//C  * long bpf_tcp_check_syncookie(void *sk, void *iph, u32 iph_len, struct tcphdr *th, u32 th_len)
//C  * 	Description
//C  * 		Check whether *iph* and *th* contain a valid SYN cookie ACK for
//C  * 		the listening socket in *sk*.
//C  *
//C  * 		*iph* points to the start of the IPv4 or IPv6 header, while
//C  * 		*iph_len* contains **sizeof**\ (**struct iphdr**) or
//C  * 		**sizeof**\ (**struct ipv6hdr**).
//C  *
//C  * 		*th* points to the start of the TCP header, while *th_len*
//C  *		contains the length of the TCP header (at least
//C  *		**sizeof**\ (**struct tcphdr**)).
//C  * 	Return
//C  * 		0 if *iph* and *th* are a valid SYN cookie ACK, or a negative
//C  * 		error otherwise.
//C  *
//C  * long bpf_sysctl_get_name(struct bpf_sysctl *ctx, char *buf, size_t buf_len, u64 flags)
//C  *	Description
//C  *		Get name of sysctl in /proc/sys/ and copy it into provided by
//C  *		program buffer *buf* of size *buf_len*.
//C  *
//C  *		The buffer is always NUL terminated, unless it's zero-sized.
//C  *
//C  *		If *flags* is zero, full name (e.g. "net/ipv4/tcp_mem") is
//C  *		copied. Use **BPF_F_SYSCTL_BASE_NAME** flag to copy base name
//C  *		only (e.g. "tcp_mem").
//C  *	Return
//C  *		Number of character copied (not including the trailing NUL).
//C  *
//C  *		**-E2BIG** if the buffer wasn't big enough (*buf* will contain
//C  *		truncated name in this case).
//C  *
//C  * long bpf_sysctl_get_current_value(struct bpf_sysctl *ctx, char *buf, size_t buf_len)
//C  *	Description
//C  *		Get current value of sysctl as it is presented in /proc/sys
//C  *		(incl. newline, etc), and copy it as a string into provided
//C  *		by program buffer *buf* of size *buf_len*.
//C  *
//C  *		The whole value is copied, no matter what file position user
//C  *		space issued e.g. sys_read at.
//C  *
//C  *		The buffer is always NUL terminated, unless it's zero-sized.
//C  *	Return
//C  *		Number of character copied (not including the trailing NUL).
//C  *
//C  *		**-E2BIG** if the buffer wasn't big enough (*buf* will contain
//C  *		truncated name in this case).
//C  *
//C  *		**-EINVAL** if current value was unavailable, e.g. because
//C  *		sysctl is uninitialized and read returns -EIO for it.
//C  *
//C  * long bpf_sysctl_get_new_value(struct bpf_sysctl *ctx, char *buf, size_t buf_len)
//C  *	Description
//C  *		Get new value being written by user space to sysctl (before
//C  *		the actual write happens) and copy it as a string into
//C  *		provided by program buffer *buf* of size *buf_len*.
//C  *
//C  *		User space may write new value at file position > 0.
//C  *
//C  *		The buffer is always NUL terminated, unless it's zero-sized.
//C  *	Return
//C  *		Number of character copied (not including the trailing NUL).
//C  *
//C  *		**-E2BIG** if the buffer wasn't big enough (*buf* will contain
//C  *		truncated name in this case).
//C  *
//C  *		**-EINVAL** if sysctl is being read.
//C  *
//C  * long bpf_sysctl_set_new_value(struct bpf_sysctl *ctx, const char *buf, size_t buf_len)
//C  *	Description
//C  *		Override new value being written by user space to sysctl with
//C  *		value provided by program in buffer *buf* of size *buf_len*.
//C  *
//C  *		*buf* should contain a string in same form as provided by user
//C  *		space on sysctl write.
//C  *
//C  *		User space may write new value at file position > 0. To override
//C  *		the whole sysctl value file position should be set to zero.
//C  *	Return
//C  *		0 on success.
//C  *
//C  *		**-E2BIG** if the *buf_len* is too big.
//C  *
//C  *		**-EINVAL** if sysctl is being read.
//C  *
//C  * long bpf_strtol(const char *buf, size_t buf_len, u64 flags, long *res)
//C  *	Description
//C  *		Convert the initial part of the string from buffer *buf* of
//C  *		size *buf_len* to a long integer according to the given base
//C  *		and save the result in *res*.
//C  *
//C  *		The string may begin with an arbitrary amount of white space
//C  *		(as determined by **isspace**\ (3)) followed by a single
//C  *		optional '**-**' sign.
//C  *
//C  *		Five least significant bits of *flags* encode base, other bits
//C  *		are currently unused.
//C  *
//C  *		Base must be either 8, 10, 16 or 0 to detect it automatically
//C  *		similar to user space **strtol**\ (3).
//C  *	Return
//C  *		Number of characters consumed on success. Must be positive but
//C  *		no more than *buf_len*.
//C  *
//C  *		**-EINVAL** if no valid digits were found or unsupported base
//C  *		was provided.
//C  *
//C  *		**-ERANGE** if resulting value was out of range.
//C  *
//C  * long bpf_strtoul(const char *buf, size_t buf_len, u64 flags, unsigned long *res)
//C  *	Description
//C  *		Convert the initial part of the string from buffer *buf* of
//C  *		size *buf_len* to an unsigned long integer according to the
//C  *		given base and save the result in *res*.
//C  *
//C  *		The string may begin with an arbitrary amount of white space
//C  *		(as determined by **isspace**\ (3)).
//C  *
//C  *		Five least significant bits of *flags* encode base, other bits
//C  *		are currently unused.
//C  *
//C  *		Base must be either 8, 10, 16 or 0 to detect it automatically
//C  *		similar to user space **strtoul**\ (3).
//C  *	Return
//C  *		Number of characters consumed on success. Must be positive but
//C  *		no more than *buf_len*.
//C  *
//C  *		**-EINVAL** if no valid digits were found or unsupported base
//C  *		was provided.
//C  *
//C  *		**-ERANGE** if resulting value was out of range.
//C  *
//C  * void *bpf_sk_storage_get(struct bpf_map *map, void *sk, void *value, u64 flags)
//C  *	Description
//C  *		Get a bpf-local-storage from a *sk*.
//C  *
//C  *		Logically, it could be thought of getting the value from
//C  *		a *map* with *sk* as the **key**.  From this
//C  *		perspective,  the usage is not much different from
//C  *		**bpf_map_lookup_elem**\ (*map*, **&**\ *sk*) except this
//C  *		helper enforces the key must be a full socket and the map must
//C  *		be a **BPF_MAP_TYPE_SK_STORAGE** also.
//C  *
//C  *		Underneath, the value is stored locally at *sk* instead of
//C  *		the *map*.  The *map* is used as the bpf-local-storage
//C  *		"type". The bpf-local-storage "type" (i.e. the *map*) is
//C  *		searched against all bpf-local-storages residing at *sk*.
//C  *
//C  *		*sk* is a kernel **struct sock** pointer for LSM program.
//C  *		*sk* is a **struct bpf_sock** pointer for other program types.
//C  *
//C  *		An optional *flags* (**BPF_SK_STORAGE_GET_F_CREATE**) can be
//C  *		used such that a new bpf-local-storage will be
//C  *		created if one does not exist.  *value* can be used
//C  *		together with **BPF_SK_STORAGE_GET_F_CREATE** to specify
//C  *		the initial value of a bpf-local-storage.  If *value* is
//C  *		**NULL**, the new bpf-local-storage will be zero initialized.
//C  *	Return
//C  *		A bpf-local-storage pointer is returned on success.
//C  *
//C  *		**NULL** if not found or there was an error in adding
//C  *		a new bpf-local-storage.
//C  *
//C  * long bpf_sk_storage_delete(struct bpf_map *map, void *sk)
//C  *	Description
//C  *		Delete a bpf-local-storage from a *sk*.
//C  *	Return
//C  *		0 on success.
//C  *
//C  *		**-ENOENT** if the bpf-local-storage cannot be found.
//C  *		**-EINVAL** if sk is not a fullsock (e.g. a request_sock).
//C  *
//C  * long bpf_send_signal(u32 sig)
//C  *	Description
//C  *		Send signal *sig* to the process of the current task.
//C  *		The signal may be delivered to any of this process's threads.
//C  *	Return
//C  *		0 on success or successfully queued.
//C  *
//C  *		**-EBUSY** if work queue under nmi is full.
//C  *
//C  *		**-EINVAL** if *sig* is invalid.
//C  *
//C  *		**-EPERM** if no permission to send the *sig*.
//C  *
//C  *		**-EAGAIN** if bpf program can try again.
//C  *
//C  * s64 bpf_tcp_gen_syncookie(void *sk, void *iph, u32 iph_len, struct tcphdr *th, u32 th_len)
//C  *	Description
//C  *		Try to issue a SYN cookie for the packet with corresponding
//C  *		IP/TCP headers, *iph* and *th*, on the listening socket in *sk*.
//C  *
//C  *		*iph* points to the start of the IPv4 or IPv6 header, while
//C  *		*iph_len* contains **sizeof**\ (**struct iphdr**) or
//C  *		**sizeof**\ (**struct ipv6hdr**).
//C  *
//C  *		*th* points to the start of the TCP header, while *th_len*
//C  *		contains the length of the TCP header with options (at least
//C  *		**sizeof**\ (**struct tcphdr**)).
//C  *	Return
//C  *		On success, lower 32 bits hold the generated SYN cookie in
//C  *		followed by 16 bits which hold the MSS value for that cookie,
//C  *		and the top 16 bits are unused.
//C  *
//C  *		On failure, the returned value is one of the following:
//C  *
//C  *		**-EINVAL** SYN cookie cannot be issued due to error
//C  *
//C  *		**-ENOENT** SYN cookie should not be issued (no SYN flood)
//C  *
//C  *		**-EOPNOTSUPP** kernel configuration does not enable SYN cookies
//C  *
//C  *		**-EPROTONOSUPPORT** IP packet version is not 4 or 6
//C  *
//C  * long bpf_skb_output(void *ctx, struct bpf_map *map, u64 flags, void *data, u64 size)
//C  * 	Description
//C  * 		Write raw *data* blob into a special BPF perf event held by
//C  * 		*map* of type **BPF_MAP_TYPE_PERF_EVENT_ARRAY**. This perf
//C  * 		event must have the following attributes: **PERF_SAMPLE_RAW**
//C  * 		as **sample_type**, **PERF_TYPE_SOFTWARE** as **type**, and
//C  * 		**PERF_COUNT_SW_BPF_OUTPUT** as **config**.
//C  *
//C  * 		The *flags* are used to indicate the index in *map* for which
//C  * 		the value must be put, masked with **BPF_F_INDEX_MASK**.
//C  * 		Alternatively, *flags* can be set to **BPF_F_CURRENT_CPU**
//C  * 		to indicate that the index of the current CPU core should be
//C  * 		used.
//C  *
//C  * 		The value to write, of *size*, is passed through eBPF stack and
//C  * 		pointed by *data*.
//C  *
//C  * 		*ctx* is a pointer to in-kernel struct sk_buff.
//C  *
//C  * 		This helper is similar to **bpf_perf_event_output**\ () but
//C  * 		restricted to raw_tracepoint bpf programs.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_probe_read_user(void *dst, u32 size, const void *unsafe_ptr)
//C  * 	Description
//C  * 		Safely attempt to read *size* bytes from user space address
//C  * 		*unsafe_ptr* and store the data in *dst*.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_probe_read_kernel(void *dst, u32 size, const void *unsafe_ptr)
//C  * 	Description
//C  * 		Safely attempt to read *size* bytes from kernel space address
//C  * 		*unsafe_ptr* and store the data in *dst*.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_probe_read_user_str(void *dst, u32 size, const void *unsafe_ptr)
//C  * 	Description
//C  * 		Copy a NUL terminated string from an unsafe user address
//C  * 		*unsafe_ptr* to *dst*. The *size* should include the
//C  * 		terminating NUL byte. In case the string length is smaller than
//C  * 		*size*, the target is not padded with further NUL bytes. If the
//C  * 		string length is larger than *size*, just *size*-1 bytes are
//C  * 		copied and the last byte is set to NUL.
//C  *
//C  * 		On success, returns the number of bytes that were written,
//C  * 		including the terminal NUL. This makes this helper useful in
//C  * 		tracing programs for reading strings, and more importantly to
//C  * 		get its length at runtime. See the following snippet:
//C  *
//C  * 		::
//C  *
//C  * 			SEC("kprobe/sys_open")
//C  * 			void bpf_sys_open(struct pt_regs *ctx)
//C  * 			{
//C  * 			        char buf[PATHLEN]; // PATHLEN is defined to 256
//C  * 			        int res = bpf_probe_read_user_str(buf, sizeof(buf),
//C  * 				                                  ctx->di);
//C  *
//C  * 				// Consume buf, for example push it to
//C  * 				// userspace via bpf_perf_event_output(); we
//C  * 				// can use res (the string length) as event
//C  * 				// size, after checking its boundaries.
//C  * 			}
//C  *
//C  * 		In comparison, using **bpf_probe_read_user**\ () helper here
//C  * 		instead to read the string would require to estimate the length
//C  * 		at compile time, and would often result in copying more memory
//C  * 		than necessary.
//C  *
//C  * 		Another useful use case is when parsing individual process
//C  * 		arguments or individual environment variables navigating
//C  * 		*current*\ **->mm->arg_start** and *current*\
//C  * 		**->mm->env_start**: using this helper and the return value,
//C  * 		one can quickly iterate at the right offset of the memory area.
//C  * 	Return
//C  * 		On success, the strictly positive length of the output string,
//C  * 		including the trailing NUL character. On error, a negative
//C  * 		value.
//C  *
//C  * long bpf_probe_read_kernel_str(void *dst, u32 size, const void *unsafe_ptr)
//C  * 	Description
//C  * 		Copy a NUL terminated string from an unsafe kernel address *unsafe_ptr*
//C  * 		to *dst*. Same semantics as with **bpf_probe_read_user_str**\ () apply.
//C  * 	Return
//C  * 		On success, the strictly positive length of the string, including
//C  * 		the trailing NUL character. On error, a negative value.
//C  *
//C  * long bpf_tcp_send_ack(void *tp, u32 rcv_nxt)
//C  *	Description
//C  *		Send out a tcp-ack. *tp* is the in-kernel struct **tcp_sock**.
//C  *		*rcv_nxt* is the ack_seq to be sent out.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_send_signal_thread(u32 sig)
//C  *	Description
//C  *		Send signal *sig* to the thread corresponding to the current task.
//C  *	Return
//C  *		0 on success or successfully queued.
//C  *
//C  *		**-EBUSY** if work queue under nmi is full.
//C  *
//C  *		**-EINVAL** if *sig* is invalid.
//C  *
//C  *		**-EPERM** if no permission to send the *sig*.
//C  *
//C  *		**-EAGAIN** if bpf program can try again.
//C  *
//C  * u64 bpf_jiffies64(void)
//C  *	Description
//C  *		Obtain the 64bit jiffies
//C  *	Return
//C  *		The 64 bit jiffies
//C  *
//C  * long bpf_read_branch_records(struct bpf_perf_event_data *ctx, void *buf, u32 size, u64 flags)
//C  *	Description
//C  *		For an eBPF program attached to a perf event, retrieve the
//C  *		branch records (**struct perf_branch_entry**) associated to *ctx*
//C  *		and store it in the buffer pointed by *buf* up to size
//C  *		*size* bytes.
//C  *	Return
//C  *		On success, number of bytes written to *buf*. On error, a
//C  *		negative value.
//C  *
//C  *		The *flags* can be set to **BPF_F_GET_BRANCH_RECORDS_SIZE** to
//C  *		instead return the number of bytes required to store all the
//C  *		branch entries. If this flag is set, *buf* may be NULL.
//C  *
//C  *		**-EINVAL** if arguments invalid or **size** not a multiple
//C  *		of **sizeof**\ (**struct perf_branch_entry**\ ).
//C  *
//C  *		**-ENOENT** if architecture does not support branch records.
//C  *
//C  * long bpf_get_ns_current_pid_tgid(u64 dev, u64 ino, struct bpf_pidns_info *nsdata, u32 size)
//C  *	Description
//C  *		Returns 0 on success, values for *pid* and *tgid* as seen from the current
//C  *		*namespace* will be returned in *nsdata*.
//C  *	Return
//C  *		0 on success, or one of the following in case of failure:
//C  *
//C  *		**-EINVAL** if dev and inum supplied don't match dev_t and inode number
//C  *              with nsfs of current task, or if dev conversion to dev_t lost high bits.
//C  *
//C  *		**-ENOENT** if pidns does not exists for the current task.
//C  *
//C  * long bpf_xdp_output(void *ctx, struct bpf_map *map, u64 flags, void *data, u64 size)
//C  *	Description
//C  *		Write raw *data* blob into a special BPF perf event held by
//C  *		*map* of type **BPF_MAP_TYPE_PERF_EVENT_ARRAY**. This perf
//C  *		event must have the following attributes: **PERF_SAMPLE_RAW**
//C  *		as **sample_type**, **PERF_TYPE_SOFTWARE** as **type**, and
//C  *		**PERF_COUNT_SW_BPF_OUTPUT** as **config**.
//C  *
//C  *		The *flags* are used to indicate the index in *map* for which
//C  *		the value must be put, masked with **BPF_F_INDEX_MASK**.
//C  *		Alternatively, *flags* can be set to **BPF_F_CURRENT_CPU**
//C  *		to indicate that the index of the current CPU core should be
//C  *		used.
//C  *
//C  *		The value to write, of *size*, is passed through eBPF stack and
//C  *		pointed by *data*.
//C  *
//C  *		*ctx* is a pointer to in-kernel struct xdp_buff.
//C  *
//C  *		This helper is similar to **bpf_perf_eventoutput**\ () but
//C  *		restricted to raw_tracepoint bpf programs.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * u64 bpf_get_netns_cookie(void *ctx)
//C  * 	Description
//C  * 		Retrieve the cookie (generated by the kernel) of the network
//C  * 		namespace the input *ctx* is associated with. The network
//C  * 		namespace cookie remains stable for its lifetime and provides
//C  * 		a global identifier that can be assumed unique. If *ctx* is
//C  * 		NULL, then the helper returns the cookie for the initial
//C  * 		network namespace. The cookie itself is very similar to that
//C  * 		of **bpf_get_socket_cookie**\ () helper, but for network
//C  * 		namespaces instead of sockets.
//C  * 	Return
//C  * 		A 8-byte long opaque number.
//C  *
//C  * u64 bpf_get_current_ancestor_cgroup_id(int ancestor_level)
//C  * 	Description
//C  * 		Return id of cgroup v2 that is ancestor of the cgroup associated
//C  * 		with the current task at the *ancestor_level*. The root cgroup
//C  * 		is at *ancestor_level* zero and each step down the hierarchy
//C  * 		increments the level. If *ancestor_level* == level of cgroup
//C  * 		associated with the current task, then return value will be the
//C  * 		same as that of **bpf_get_current_cgroup_id**\ ().
//C  *
//C  * 		The helper is useful to implement policies based on cgroups
//C  * 		that are upper in hierarchy than immediate cgroup associated
//C  * 		with the current task.
//C  *
//C  * 		The format of returned id and helper limitations are same as in
//C  * 		**bpf_get_current_cgroup_id**\ ().
//C  * 	Return
//C  * 		The id is returned or 0 in case the id could not be retrieved.
//C  *
//C  * long bpf_sk_assign(struct sk_buff *skb, void *sk, u64 flags)
//C  *	Description
//C  *		Helper is overloaded depending on BPF program type. This
//C  *		description applies to **BPF_PROG_TYPE_SCHED_CLS** and
//C  *		**BPF_PROG_TYPE_SCHED_ACT** programs.
//C  *
//C  *		Assign the *sk* to the *skb*. When combined with appropriate
//C  *		routing configuration to receive the packet towards the socket,
//C  *		will cause *skb* to be delivered to the specified socket.
//C  *		Subsequent redirection of *skb* via  **bpf_redirect**\ (),
//C  *		**bpf_clone_redirect**\ () or other methods outside of BPF may
//C  *		interfere with successful delivery to the socket.
//C  *
//C  *		This operation is only valid from TC ingress path.
//C  *
//C  *		The *flags* argument must be zero.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure:
//C  *
//C  *		**-EINVAL** if specified *flags* are not supported.
//C  *
//C  *		**-ENOENT** if the socket is unavailable for assignment.
//C  *
//C  *		**-ENETUNREACH** if the socket is unreachable (wrong netns).
//C  *
//C  *		**-EOPNOTSUPP** if the operation is not supported, for example
//C  *		a call from outside of TC ingress.
//C  *
//C  * long bpf_sk_assign(struct bpf_sk_lookup *ctx, struct bpf_sock *sk, u64 flags)
//C  *	Description
//C  *		Helper is overloaded depending on BPF program type. This
//C  *		description applies to **BPF_PROG_TYPE_SK_LOOKUP** programs.
//C  *
//C  *		Select the *sk* as a result of a socket lookup.
//C  *
//C  *		For the operation to succeed passed socket must be compatible
//C  *		with the packet description provided by the *ctx* object.
//C  *
//C  *		L4 protocol (**IPPROTO_TCP** or **IPPROTO_UDP**) must
//C  *		be an exact match. While IP family (**AF_INET** or
//C  *		**AF_INET6**) must be compatible, that is IPv6 sockets
//C  *		that are not v6-only can be selected for IPv4 packets.
//C  *
//C  *		Only TCP listeners and UDP unconnected sockets can be
//C  *		selected. *sk* can also be NULL to reset any previous
//C  *		selection.
//C  *
//C  *		*flags* argument can combination of following values:
//C  *
//C  *		* **BPF_SK_LOOKUP_F_REPLACE** to override the previous
//C  *		  socket selection, potentially done by a BPF program
//C  *		  that ran before us.
//C  *
//C  *		* **BPF_SK_LOOKUP_F_NO_REUSEPORT** to skip
//C  *		  load-balancing within reuseport group for the socket
//C  *		  being selected.
//C  *
//C  *		On success *ctx->sk* will point to the selected socket.
//C  *
//C  *	Return
//C  *		0 on success, or a negative errno in case of failure.
//C  *
//C  *		* **-EAFNOSUPPORT** if socket family (*sk->family*) is
//C  *		  not compatible with packet family (*ctx->family*).
//C  *
//C  *		* **-EEXIST** if socket has been already selected,
//C  *		  potentially by another program, and
//C  *		  **BPF_SK_LOOKUP_F_REPLACE** flag was not specified.
//C  *
//C  *		* **-EINVAL** if unsupported flags were specified.
//C  *
//C  *		* **-EPROTOTYPE** if socket L4 protocol
//C  *		  (*sk->protocol*) doesn't match packet protocol
//C  *		  (*ctx->protocol*).
//C  *
//C  *		* **-ESOCKTNOSUPPORT** if socket is not in allowed
//C  *		  state (TCP listening or UDP unconnected).
//C  *
//C  * u64 bpf_ktime_get_boot_ns(void)
//C  * 	Description
//C  * 		Return the time elapsed since system boot, in nanoseconds.
//C  * 		Does include the time the system was suspended.
//C  * 		See: **clock_gettime**\ (**CLOCK_BOOTTIME**)
//C  * 	Return
//C  * 		Current *ktime*.
//C  *
//C  * long bpf_seq_printf(struct seq_file *m, const char *fmt, u32 fmt_size, const void *data, u32 data_len)
//C  * 	Description
//C  * 		**bpf_seq_printf**\ () uses seq_file **seq_printf**\ () to print
//C  * 		out the format string.
//C  * 		The *m* represents the seq_file. The *fmt* and *fmt_size* are for
//C  * 		the format string itself. The *data* and *data_len* are format string
//C  * 		arguments. The *data* are a **u64** array and corresponding format string
//C  * 		values are stored in the array. For strings and pointers where pointees
//C  * 		are accessed, only the pointer values are stored in the *data* array.
//C  * 		The *data_len* is the size of *data* in bytes - must be a multiple of 8.
//C  *
//C  *		Formats **%s**, **%p{i,I}{4,6}** requires to read kernel memory.
//C  *		Reading kernel memory may fail due to either invalid address or
//C  *		valid address but requiring a major memory fault. If reading kernel memory
//C  *		fails, the string for **%s** will be an empty string, and the ip
//C  *		address for **%p{i,I}{4,6}** will be 0. Not returning error to
//C  *		bpf program is consistent with what **bpf_trace_printk**\ () does for now.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure:
//C  *
//C  *		**-EBUSY** if per-CPU memory copy buffer is busy, can try again
//C  *		by returning 1 from bpf program.
//C  *
//C  *		**-EINVAL** if arguments are invalid, or if *fmt* is invalid/unsupported.
//C  *
//C  *		**-E2BIG** if *fmt* contains too many format specifiers.
//C  *
//C  *		**-EOVERFLOW** if an overflow happened: The same object will be tried again.
//C  *
//C  * long bpf_seq_write(struct seq_file *m, const void *data, u32 len)
//C  * 	Description
//C  * 		**bpf_seq_write**\ () uses seq_file **seq_write**\ () to write the data.
//C  * 		The *m* represents the seq_file. The *data* and *len* represent the
//C  * 		data to write in bytes.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure:
//C  *
//C  *		**-EOVERFLOW** if an overflow happened: The same object will be tried again.
//C  *
//C  * u64 bpf_sk_cgroup_id(void *sk)
//C  *	Description
//C  *		Return the cgroup v2 id of the socket *sk*.
//C  *
//C  *		*sk* must be a non-**NULL** pointer to a socket, e.g. one
//C  *		returned from **bpf_sk_lookup_xxx**\ (),
//C  *		**bpf_sk_fullsock**\ (), etc. The format of returned id is
//C  *		same as in **bpf_skb_cgroup_id**\ ().
//C  *
//C  *		This helper is available only if the kernel was compiled with
//C  *		the **CONFIG_SOCK_CGROUP_DATA** configuration option.
//C  *	Return
//C  *		The id is returned or 0 in case the id could not be retrieved.
//C  *
//C  * u64 bpf_sk_ancestor_cgroup_id(void *sk, int ancestor_level)
//C  *	Description
//C  *		Return id of cgroup v2 that is ancestor of cgroup associated
//C  *		with the *sk* at the *ancestor_level*.  The root cgroup is at
//C  *		*ancestor_level* zero and each step down the hierarchy
//C  *		increments the level. If *ancestor_level* == level of cgroup
//C  *		associated with *sk*, then return value will be same as that
//C  *		of **bpf_sk_cgroup_id**\ ().
//C  *
//C  *		The helper is useful to implement policies based on cgroups
//C  *		that are upper in hierarchy than immediate cgroup associated
//C  *		with *sk*.
//C  *
//C  *		The format of returned id and helper limitations are same as in
//C  *		**bpf_sk_cgroup_id**\ ().
//C  *	Return
//C  *		The id is returned or 0 in case the id could not be retrieved.
//C  *
//C  * long bpf_ringbuf_output(void *ringbuf, void *data, u64 size, u64 flags)
//C  * 	Description
//C  * 		Copy *size* bytes from *data* into a ring buffer *ringbuf*.
//C  * 		If **BPF_RB_NO_WAKEUP** is specified in *flags*, no notification
//C  * 		of new data availability is sent.
//C  * 		If **BPF_RB_FORCE_WAKEUP** is specified in *flags*, notification
//C  * 		of new data availability is sent unconditionally.
//C  * 		If **0** is specified in *flags*, an adaptive notification
//C  * 		of new data availability is sent.
//C  *
//C  * 		An adaptive notification is a notification sent whenever the user-space
//C  * 		process has caught up and consumed all available payloads. In case the user-space
//C  * 		process is still processing a previous payload, then no notification is needed
//C  * 		as it will process the newly added payload automatically.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * void *bpf_ringbuf_reserve(void *ringbuf, u64 size, u64 flags)
//C  * 	Description
//C  * 		Reserve *size* bytes of payload in a ring buffer *ringbuf*.
//C  * 		*flags* must be 0.
//C  * 	Return
//C  * 		Valid pointer with *size* bytes of memory available; NULL,
//C  * 		otherwise.
//C  *
//C  * void bpf_ringbuf_submit(void *data, u64 flags)
//C  * 	Description
//C  * 		Submit reserved ring buffer sample, pointed to by *data*.
//C  * 		If **BPF_RB_NO_WAKEUP** is specified in *flags*, no notification
//C  * 		of new data availability is sent.
//C  * 		If **BPF_RB_FORCE_WAKEUP** is specified in *flags*, notification
//C  * 		of new data availability is sent unconditionally.
//C  * 		If **0** is specified in *flags*, an adaptive notification
//C  * 		of new data availability is sent.
//C  *
//C  * 		See 'bpf_ringbuf_output()' for the definition of adaptive notification.
//C  * 	Return
//C  * 		Nothing. Always succeeds.
//C  *
//C  * void bpf_ringbuf_discard(void *data, u64 flags)
//C  * 	Description
//C  * 		Discard reserved ring buffer sample, pointed to by *data*.
//C  * 		If **BPF_RB_NO_WAKEUP** is specified in *flags*, no notification
//C  * 		of new data availability is sent. Discarded records remain in
//C  * 		the ring buffer until consumed by user space, so a later submit
//C  * 		using adaptive wakeup might not wake up the consumer.
//C  * 		If **BPF_RB_FORCE_WAKEUP** is specified in *flags*, notification
//C  * 		of new data availability is sent unconditionally.
//C  * 		If **0** is specified in *flags*, an adaptive notification
//C  * 		of new data availability is sent.
//C  *
//C  * 		See 'bpf_ringbuf_output()' for the definition of adaptive notification.
//C  * 	Return
//C  * 		Nothing. Always succeeds.
//C  *
//C  * u64 bpf_ringbuf_query(void *ringbuf, u64 flags)
//C  *	Description
//C  *		Query various characteristics of provided ring buffer. What
//C  *		exactly is queries is determined by *flags*:
//C  *
//C  *		* **BPF_RB_AVAIL_DATA**: Amount of data not yet consumed.
//C  *		* **BPF_RB_RING_SIZE**: The size of ring buffer.
//C  *		* **BPF_RB_CONS_POS**: Consumer position (can wrap around).
//C  *		* **BPF_RB_PROD_POS**: Producer(s) position (can wrap around).
//C  *		* **BPF_RB_OVERWRITE_POS**: Overwrite position (can wrap around).
//C  *
//C  *		Data returned is just a momentary snapshot of actual values
//C  *		and could be inaccurate, so this facility should be used to
//C  *		power heuristics and for reporting, not to make 100% correct
//C  *		calculation.
//C  *	Return
//C  *		Requested value, or 0, if *flags* are not recognized.
//C  *
//C  * long bpf_csum_level(struct sk_buff *skb, u64 level)
//C  * 	Description
//C  * 		Change the skbs checksum level by one layer up or down, or
//C  * 		reset it entirely to none in order to have the stack perform
//C  * 		checksum validation. The level is applicable to the following
//C  * 		protocols: TCP, UDP, GRE, SCTP, FCOE. For example, a decap of
//C  * 		| ETH | IP | UDP | GUE | IP | TCP | into | ETH | IP | TCP |
//C  * 		through **bpf_skb_adjust_room**\ () helper with passing in
//C  * 		**BPF_F_ADJ_ROOM_NO_CSUM_RESET** flag would require one	call
//C  * 		to **bpf_csum_level**\ () with **BPF_CSUM_LEVEL_DEC** since
//C  * 		the UDP header is removed. Similarly, an encap of the latter
//C  * 		into the former could be accompanied by a helper call to
//C  * 		**bpf_csum_level**\ () with **BPF_CSUM_LEVEL_INC** if the
//C  * 		skb is still intended to be processed in higher layers of the
//C  * 		stack instead of just egressing at tc.
//C  *
//C  * 		There are three supported level settings at this time:
//C  *
//C  * 		* **BPF_CSUM_LEVEL_INC**: Increases skb->csum_level for skbs
//C  * 		  with CHECKSUM_UNNECESSARY.
//C  * 		* **BPF_CSUM_LEVEL_DEC**: Decreases skb->csum_level for skbs
//C  * 		  with CHECKSUM_UNNECESSARY.
//C  * 		* **BPF_CSUM_LEVEL_RESET**: Resets skb->csum_level to 0 and
//C  * 		  sets CHECKSUM_NONE to force checksum validation by the stack.
//C  * 		* **BPF_CSUM_LEVEL_QUERY**: No-op, returns the current
//C  * 		  skb->csum_level.
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure. In the
//C  * 		case of **BPF_CSUM_LEVEL_QUERY**, the current skb->csum_level
//C  * 		is returned or the error code -EACCES in case the skb is not
//C  * 		subject to CHECKSUM_UNNECESSARY.
//C  *
//C  * struct tcp6_sock *bpf_skc_to_tcp6_sock(void *sk)
//C  *	Description
//C  *		Dynamically cast a *sk* pointer to a *tcp6_sock* pointer.
//C  *	Return
//C  *		*sk* if casting is valid, or **NULL** otherwise.
//C  *
//C  * struct tcp_sock *bpf_skc_to_tcp_sock(void *sk)
//C  *	Description
//C  *		Dynamically cast a *sk* pointer to a *tcp_sock* pointer.
//C  *	Return
//C  *		*sk* if casting is valid, or **NULL** otherwise.
//C  *
//C  * struct tcp_timewait_sock *bpf_skc_to_tcp_timewait_sock(void *sk)
//C  * 	Description
//C  *		Dynamically cast a *sk* pointer to a *tcp_timewait_sock* pointer.
//C  *	Return
//C  *		*sk* if casting is valid, or **NULL** otherwise.
//C  *
//C  * struct tcp_request_sock *bpf_skc_to_tcp_request_sock(void *sk)
//C  * 	Description
//C  *		Dynamically cast a *sk* pointer to a *tcp_request_sock* pointer.
//C  *	Return
//C  *		*sk* if casting is valid, or **NULL** otherwise.
//C  *
//C  * struct udp6_sock *bpf_skc_to_udp6_sock(void *sk)
//C  * 	Description
//C  *		Dynamically cast a *sk* pointer to a *udp6_sock* pointer.
//C  *	Return
//C  *		*sk* if casting is valid, or **NULL** otherwise.
//C  *
//C  * long bpf_get_task_stack(struct task_struct *task, void *buf, u32 size, u64 flags)
//C  *	Description
//C  *		Return a user or a kernel stack in bpf program provided buffer.
//C  *		Note: the user stack will only be populated if the *task* is
//C  *		the current task; all other tasks will return -EOPNOTSUPP.
//C  *		To achieve this, the helper needs *task*, which is a valid
//C  *		pointer to **struct task_struct**. To store the stacktrace, the
//C  *		bpf program provides *buf* with a nonnegative *size*.
//C  *
//C  *		The last argument, *flags*, holds the number of stack frames to
//C  *		skip (from 0 to 255), masked with
//C  *		**BPF_F_SKIP_FIELD_MASK**. The next bits can be used to set
//C  *		the following flags:
//C  *
//C  *		**BPF_F_USER_STACK**
//C  *			Collect a user space stack instead of a kernel stack.
//C  *			The *task* must be the current task.
//C  *		**BPF_F_USER_BUILD_ID**
//C  *			Collect buildid+offset instead of ips for user stack,
//C  *			only valid if **BPF_F_USER_STACK** is also specified.
//C  *
//C  *		**bpf_get_task_stack**\ () can collect up to
//C  *		**PERF_MAX_STACK_DEPTH** both kernel and user frames, subject
//C  *		to sufficient large buffer size. Note that
//C  *		this limit can be controlled with the **sysctl** program, and
//C  *		that it should be manually increased in order to profile long
//C  *		user stacks (such as stacks for Java programs). To do so, use:
//C  *
//C  *		::
//C  *
//C  *			# sysctl kernel.perf_event_max_stack=<new value>
//C  *	Return
//C  * 		The non-negative copied *buf* length equal to or less than
//C  * 		*size* on success, or a negative error in case of failure.
//C  *
//C  * long bpf_load_hdr_opt(struct bpf_sock_ops *skops, void *searchby_res, u32 len, u64 flags)
//C  *	Description
//C  *		Load header option.  Support reading a particular TCP header
//C  *		option for bpf program (**BPF_PROG_TYPE_SOCK_OPS**).
//C  *
//C  *		If *flags* is 0, it will search the option from the
//C  *		*skops*\ **->skb_data**.  The comment in **struct bpf_sock_ops**
//C  *		has details on what skb_data contains under different
//C  *		*skops*\ **->op**.
//C  *
//C  *		The first byte of the *searchby_res* specifies the
//C  *		kind that it wants to search.
//C  *
//C  *		If the searching kind is an experimental kind
//C  *		(i.e. 253 or 254 according to RFC6994).  It also
//C  *		needs to specify the "magic" which is either
//C  *		2 bytes or 4 bytes.  It then also needs to
//C  *		specify the size of the magic by using
//C  *		the 2nd byte which is "kind-length" of a TCP
//C  *		header option and the "kind-length" also
//C  *		includes the first 2 bytes "kind" and "kind-length"
//C  *		itself as a normal TCP header option also does.
//C  *
//C  *		For example, to search experimental kind 254 with
//C  *		2 byte magic 0xeB9F, the searchby_res should be
//C  *		[ 254, 4, 0xeB, 0x9F, 0, 0, .... 0 ].
//C  *
//C  *		To search for the standard window scale option (3),
//C  *		the *searchby_res* should be [ 3, 0, 0, .... 0 ].
//C  *		Note, kind-length must be 0 for regular option.
//C  *
//C  *		Searching for No-Op (0) and End-of-Option-List (1) are
//C  *		not supported.
//C  *
//C  *		*len* must be at least 2 bytes which is the minimal size
//C  *		of a header option.
//C  *
//C  *		Supported flags:
//C  *
//C  *		* **BPF_LOAD_HDR_OPT_TCP_SYN** to search from the
//C  *		  saved_syn packet or the just-received syn packet.
//C  *
//C  *	Return
//C  *		> 0 when found, the header option is copied to *searchby_res*.
//C  *		The return value is the total length copied. On failure, a
//C  *		negative error code is returned:
//C  *
//C  *		**-EINVAL** if a parameter is invalid.
//C  *
//C  *		**-ENOMSG** if the option is not found.
//C  *
//C  *		**-ENOENT** if no syn packet is available when
//C  *		**BPF_LOAD_HDR_OPT_TCP_SYN** is used.
//C  *
//C  *		**-ENOSPC** if there is not enough space.  Only *len* number of
//C  *		bytes are copied.
//C  *
//C  *		**-EFAULT** on failure to parse the header options in the
//C  *		packet.
//C  *
//C  *		**-EPERM** if the helper cannot be used under the current
//C  *		*skops*\ **->op**.
//C  *
//C  * long bpf_store_hdr_opt(struct bpf_sock_ops *skops, const void *from, u32 len, u64 flags)
//C  *	Description
//C  *		Store header option.  The data will be copied
//C  *		from buffer *from* with length *len* to the TCP header.
//C  *
//C  *		The buffer *from* should have the whole option that
//C  *		includes the kind, kind-length, and the actual
//C  *		option data.  The *len* must be at least kind-length
//C  *		long.  The kind-length does not have to be 4 byte
//C  *		aligned.  The kernel will take care of the padding
//C  *		and setting the 4 bytes aligned value to th->doff.
//C  *
//C  *		This helper will check for duplicated option
//C  *		by searching the same option in the outgoing skb.
//C  *
//C  *		This helper can only be called during
//C  *		**BPF_SOCK_OPS_WRITE_HDR_OPT_CB**.
//C  *
//C  *	Return
//C  *		0 on success, or negative error in case of failure:
//C  *
//C  *		**-EINVAL** If param is invalid.
//C  *
//C  *		**-ENOSPC** if there is not enough space in the header.
//C  *		Nothing has been written
//C  *
//C  *		**-EEXIST** if the option already exists.
//C  *
//C  *		**-EFAULT** on failure to parse the existing header options.
//C  *
//C  *		**-EPERM** if the helper cannot be used under the current
//C  *		*skops*\ **->op**.
//C  *
//C  * long bpf_reserve_hdr_opt(struct bpf_sock_ops *skops, u32 len, u64 flags)
//C  *	Description
//C  *		Reserve *len* bytes for the bpf header option.  The
//C  *		space will be used by **bpf_store_hdr_opt**\ () later in
//C  *		**BPF_SOCK_OPS_WRITE_HDR_OPT_CB**.
//C  *
//C  *		If **bpf_reserve_hdr_opt**\ () is called multiple times,
//C  *		the total number of bytes will be reserved.
//C  *
//C  *		This helper can only be called during
//C  *		**BPF_SOCK_OPS_HDR_OPT_LEN_CB**.
//C  *
//C  *	Return
//C  *		0 on success, or negative error in case of failure:
//C  *
//C  *		**-EINVAL** if a parameter is invalid.
//C  *
//C  *		**-ENOSPC** if there is not enough space in the header.
//C  *
//C  *		**-EPERM** if the helper cannot be used under the current
//C  *		*skops*\ **->op**.
//C  *
//C  * void *bpf_inode_storage_get(struct bpf_map *map, void *inode, void *value, u64 flags)
//C  *	Description
//C  *		Get a bpf_local_storage from an *inode*.
//C  *
//C  *		Logically, it could be thought of as getting the value from
//C  *		a *map* with *inode* as the **key**.  From this
//C  *		perspective,  the usage is not much different from
//C  *		**bpf_map_lookup_elem**\ (*map*, **&**\ *inode*) except this
//C  *		helper enforces the key must be an inode and the map must also
//C  *		be a **BPF_MAP_TYPE_INODE_STORAGE**.
//C  *
//C  *		Underneath, the value is stored locally at *inode* instead of
//C  *		the *map*.  The *map* is used as the bpf-local-storage
//C  *		"type". The bpf-local-storage "type" (i.e. the *map*) is
//C  *		searched against all bpf_local_storage residing at *inode*.
//C  *
//C  *		An optional *flags* (**BPF_LOCAL_STORAGE_GET_F_CREATE**) can be
//C  *		used such that a new bpf_local_storage will be
//C  *		created if one does not exist.  *value* can be used
//C  *		together with **BPF_LOCAL_STORAGE_GET_F_CREATE** to specify
//C  *		the initial value of a bpf_local_storage.  If *value* is
//C  *		**NULL**, the new bpf_local_storage will be zero initialized.
//C  *	Return
//C  *		A bpf_local_storage pointer is returned on success.
//C  *
//C  *		**NULL** if not found or there was an error in adding
//C  *		a new bpf_local_storage.
//C  *
//C  * int bpf_inode_storage_delete(struct bpf_map *map, void *inode)
//C  *	Description
//C  *		Delete a bpf_local_storage from an *inode*.
//C  *	Return
//C  *		0 on success.
//C  *
//C  *		**-ENOENT** if the bpf_local_storage cannot be found.
//C  *
//C  * long bpf_d_path(const struct path *path, char *buf, u32 sz)
//C  *	Description
//C  *		Return full path for given **struct path** object, which
//C  *		needs to be the kernel BTF *path* object. The path is
//C  *		returned in the provided buffer *buf* of size *sz* and
//C  *		is zero terminated.
//C  *
//C  *	Return
//C  *		On success, the strictly positive length of the string,
//C  *		including the trailing NUL character. On error, a negative
//C  *		value.
//C  *
//C  * long bpf_copy_from_user(void *dst, u32 size, const void *user_ptr)
//C  * 	Description
//C  * 		Read *size* bytes from user space address *user_ptr* and store
//C  * 		the data in *dst*. This is a wrapper of **copy_from_user**\ ().
//C  * 	Return
//C  * 		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_snprintf_btf(char *str, u32 str_size, struct btf_ptr *ptr, u32 btf_ptr_size, u64 flags)
//C  *	Description
//C  *		Use BTF to store a string representation of *ptr*->ptr in *str*,
//C  *		using *ptr*->type_id.  This value should specify the type
//C  *		that *ptr*->ptr points to. LLVM __builtin_btf_type_id(type, 1)
//C  *		can be used to look up vmlinux BTF type ids. Traversing the
//C  *		data structure using BTF, the type information and values are
//C  *		stored in the first *str_size* - 1 bytes of *str*.  Safe copy of
//C  *		the pointer data is carried out to avoid kernel crashes during
//C  *		operation.  Smaller types can use string space on the stack;
//C  *		larger programs can use map data to store the string
//C  *		representation.
//C  *
//C  *		The string can be subsequently shared with userspace via
//C  *		bpf_perf_event_output() or ring buffer interfaces.
//C  *		bpf_trace_printk() is to be avoided as it places too small
//C  *		a limit on string size to be useful.
//C  *
//C  *		*flags* is a combination of
//C  *
//C  *		**BTF_F_COMPACT**
//C  *			no formatting around type information
//C  *		**BTF_F_NONAME**
//C  *			no struct/union member names/types
//C  *		**BTF_F_PTR_RAW**
//C  *			show raw (unobfuscated) pointer values;
//C  *			equivalent to printk specifier %px.
//C  *		**BTF_F_ZERO**
//C  *			show zero-valued struct/union members; they
//C  *			are not displayed by default
//C  *
//C  *	Return
//C  *		The number of bytes that were written (or would have been
//C  *		written if output had to be truncated due to string size),
//C  *		or a negative error in cases of failure.
//C  *
//C  * long bpf_seq_printf_btf(struct seq_file *m, struct btf_ptr *ptr, u32 ptr_size, u64 flags)
//C  *	Description
//C  *		Use BTF to write to seq_write a string representation of
//C  *		*ptr*->ptr, using *ptr*->type_id as per bpf_snprintf_btf().
//C  *		*flags* are identical to those used for bpf_snprintf_btf.
//C  *	Return
//C  *		0 on success or a negative error in case of failure.
//C  *
//C  * u64 bpf_skb_cgroup_classid(struct sk_buff *skb)
//C  * 	Description
//C  * 		See **bpf_get_cgroup_classid**\ () for the main description.
//C  * 		This helper differs from **bpf_get_cgroup_classid**\ () in that
//C  * 		the cgroup v1 net_cls class is retrieved only from the *skb*'s
//C  * 		associated socket instead of the current process.
//C  * 	Return
//C  * 		The id is returned or 0 in case the id could not be retrieved.
//C  *
//C  * long bpf_redirect_neigh(u32 ifindex, struct bpf_redir_neigh *params, int plen, u64 flags)
//C  * 	Description
//C  * 		Redirect the packet to another net device of index *ifindex*
//C  * 		and fill in L2 addresses from neighboring subsystem. This helper
//C  * 		is somewhat similar to **bpf_redirect**\ (), except that it
//C  * 		populates L2 addresses as well, meaning, internally, the helper
//C  * 		relies on the neighbor lookup for the L2 address of the nexthop.
//C  *
//C  * 		The helper will perform a FIB lookup based on the skb's
//C  * 		networking header to get the address of the next hop, unless
//C  * 		this is supplied by the caller in the *params* argument. The
//C  * 		*plen* argument indicates the len of *params* and should be set
//C  * 		to 0 if *params* is NULL.
//C  *
//C  * 		The *flags* argument is reserved and must be 0. The helper is
//C  * 		currently only supported for tc BPF program types, and enabled
//C  * 		for IPv4 and IPv6 protocols.
//C  * 	Return
//C  * 		The helper returns **TC_ACT_REDIRECT** on success or
//C  * 		**TC_ACT_SHOT** on error.
//C  *
//C  * void *bpf_per_cpu_ptr(const void *percpu_ptr, u32 cpu)
//C  *     Description
//C  *             Take a pointer to a percpu ksym, *percpu_ptr*, and return a
//C  *             pointer to the percpu kernel variable on *cpu*. A ksym is an
//C  *             extern variable decorated with '__ksym'. For ksym, there is a
//C  *             global var (either static or global) defined of the same name
//C  *             in the kernel. The ksym is percpu if the global var is percpu.
//C  *             The returned pointer points to the global percpu var on *cpu*.
//C  *
//C  *             bpf_per_cpu_ptr() has the same semantic as per_cpu_ptr() in the
//C  *             kernel, except that bpf_per_cpu_ptr() may return NULL. This
//C  *             happens if *cpu* is larger than nr_cpu_ids. The caller of
//C  *             bpf_per_cpu_ptr() must check the returned value.
//C  *     Return
//C  *             A pointer pointing to the kernel percpu variable on *cpu*, or
//C  *             NULL, if *cpu* is invalid.
//C  *
//C  * void *bpf_this_cpu_ptr(const void *percpu_ptr)
//C  *	Description
//C  *		Take a pointer to a percpu ksym, *percpu_ptr*, and return a
//C  *		pointer to the percpu kernel variable on this cpu. See the
//C  *		description of 'ksym' in **bpf_per_cpu_ptr**\ ().
//C  *
//C  *		bpf_this_cpu_ptr() has the same semantic as this_cpu_ptr() in
//C  *		the kernel. Different from **bpf_per_cpu_ptr**\ (), it would
//C  *		never return NULL.
//C  *	Return
//C  *		A pointer pointing to the kernel percpu variable on this cpu.
//C  *
//C  * long bpf_redirect_peer(u32 ifindex, u64 flags)
//C  * 	Description
//C  * 		Redirect the packet to another net device of index *ifindex*.
//C  * 		This helper is somewhat similar to **bpf_redirect**\ (), except
//C  * 		that the redirection happens to the *ifindex*' peer device. If
//C  * 		*flags* is 0, the netns switch takes place from ingress to
//C  * 		ingress without going through the CPU's backlog queue. If the
//C  * 		**BPF_F_EGRESS** flag is provided then redirection happens in
//C  * 		the egress direction of the peer device.
//C  *
//C  * 		*skb*\ **->mark** and *skb*\ **->tstamp** are not cleared during
//C  * 		the netns switch.
//C  *
//C  * 		If the *flags* argument is 0, the helper is currently only
//C  * 		supported for tc BPF program types at the ingress hook and for
//C  * 		veth and netkit target device types. The peer device must reside
//C  * 		in a different network namespace.
//C  * 	Return
//C  * 		The helper returns **TC_ACT_REDIRECT** on success or
//C  * 		**TC_ACT_SHOT** on error.
//C  *
//C  * void *bpf_task_storage_get(struct bpf_map *map, struct task_struct *task, void *value, u64 flags)
//C  *	Description
//C  *		Get a bpf_local_storage from the *task*.
//C  *
//C  *		Logically, it could be thought of as getting the value from
//C  *		a *map* with *task* as the **key**.  From this
//C  *		perspective,  the usage is not much different from
//C  *		**bpf_map_lookup_elem**\ (*map*, **&**\ *task*) except this
//C  *		helper enforces the key must be a task_struct and the map must also
//C  *		be a **BPF_MAP_TYPE_TASK_STORAGE**.
//C  *
//C  *		Underneath, the value is stored locally at *task* instead of
//C  *		the *map*.  The *map* is used as the bpf-local-storage
//C  *		"type". The bpf-local-storage "type" (i.e. the *map*) is
//C  *		searched against all bpf_local_storage residing at *task*.
//C  *
//C  *		An optional *flags* (**BPF_LOCAL_STORAGE_GET_F_CREATE**) can be
//C  *		used such that a new bpf_local_storage will be
//C  *		created if one does not exist.  *value* can be used
//C  *		together with **BPF_LOCAL_STORAGE_GET_F_CREATE** to specify
//C  *		the initial value of a bpf_local_storage.  If *value* is
//C  *		**NULL**, the new bpf_local_storage will be zero initialized.
//C  *	Return
//C  *		A bpf_local_storage pointer is returned on success.
//C  *
//C  *		**NULL** if not found or there was an error in adding
//C  *		a new bpf_local_storage.
//C  *
//C  * long bpf_task_storage_delete(struct bpf_map *map, struct task_struct *task)
//C  *	Description
//C  *		Delete a bpf_local_storage from a *task*.
//C  *	Return
//C  *		0 on success.
//C  *
//C  *		**-ENOENT** if the bpf_local_storage cannot be found.
//C  *
//C  * struct task_struct *bpf_get_current_task_btf(void)
//C  *	Description
//C  *		Return a BTF pointer to the "current" task.
//C  *		This pointer can also be used in helpers that accept an
//C  *		*ARG_PTR_TO_BTF_ID* of type *task_struct*.
//C  *	Return
//C  *		Pointer to the current task.
//C  *
//C  * long bpf_bprm_opts_set(struct linux_binprm *bprm, u64 flags)
//C  *	Description
//C  *		Set or clear certain options on *bprm*:
//C  *
//C  *		**BPF_F_BPRM_SECUREEXEC** Set the secureexec bit
//C  *		which sets the **AT_SECURE** auxv for glibc. The bit
//C  *		is cleared if the flag is not specified.
//C  *	Return
//C  *		**-EINVAL** if invalid *flags* are passed, zero otherwise.
//C  *
//C  * u64 bpf_ktime_get_coarse_ns(void)
//C  * 	Description
//C  * 		Return a coarse-grained version of the time elapsed since
//C  * 		system boot, in nanoseconds. Does not include time the system
//C  * 		was suspended.
//C  *
//C  * 		See: **clock_gettime**\ (**CLOCK_MONOTONIC_COARSE**)
//C  * 	Return
//C  * 		Current *ktime*.
//C  *
//C  * long bpf_ima_inode_hash(struct inode *inode, void *dst, u32 size)
//C  *	Description
//C  *		Returns the stored IMA hash of the *inode* (if it's available).
//C  *		If the hash is larger than *size*, then only *size*
//C  *		bytes will be copied to *dst*
//C  *	Return
//C  *		The **hash_algo** is returned on success,
//C  *		**-EOPNOTSUPP** if IMA is disabled or **-EINVAL** if
//C  *		invalid arguments are passed.
//C  *
//C  * struct socket *bpf_sock_from_file(struct file *file)
//C  *	Description
//C  *		If the given file represents a socket, returns the associated
//C  *		socket.
//C  *	Return
//C  *		A pointer to a struct socket on success or NULL if the file is
//C  *		not a socket.
//C  *
//C  * long bpf_check_mtu(void *ctx, u32 ifindex, u32 *mtu_len, s32 len_diff, u64 flags)
//C  *	Description
//C  *		Check packet size against exceeding MTU of net device (based
//C  *		on *ifindex*).  This helper will likely be used in combination
//C  *		with helpers that adjust/change the packet size.
//C  *
//C  *		The argument *len_diff* can be used for querying with a planned
//C  *		size change. This allows to check MTU prior to changing packet
//C  *		ctx. Providing a *len_diff* adjustment that is larger than the
//C  *		actual packet size (resulting in negative packet size) will in
//C  *		principle not exceed the MTU, which is why it is not considered
//C  *		a failure.  Other BPF helpers are needed for performing the
//C  *		planned size change; therefore the responsibility for catching
//C  *		a negative packet size belongs in those helpers.
//C  *
//C  *		Specifying *ifindex* zero means the MTU check is performed
//C  *		against the current net device.  This is practical if this isn't
//C  *		used prior to redirect.
//C  *
//C  *		On input *mtu_len* must be a valid pointer, else verifier will
//C  *		reject BPF program.  If the value *mtu_len* is initialized to
//C  *		zero then the ctx packet size is use.  When value *mtu_len* is
//C  *		provided as input this specify the L3 length that the MTU check
//C  *		is done against. Remember XDP and TC length operate at L2, but
//C  *		this value is L3 as this correlate to MTU and IP-header tot_len
//C  *		values which are L3 (similar behavior as bpf_fib_lookup).
//C  *
//C  *		The Linux kernel route table can configure MTUs on a more
//C  *		specific per route level, which is not provided by this helper.
//C  *		For route level MTU checks use the **bpf_fib_lookup**\ ()
//C  *		helper.
//C  *
//C  *		*ctx* is either **struct xdp_md** for XDP programs or
//C  *		**struct sk_buff** for tc cls_act programs.
//C  *
//C  *		The *flags* argument can be a combination of one or more of the
//C  *		following values:
//C  *
//C  *		**BPF_MTU_CHK_SEGS**
//C  *			This flag will only works for *ctx* **struct sk_buff**.
//C  *			If packet context contains extra packet segment buffers
//C  *			(often knows as GSO skb), then MTU check is harder to
//C  *			check at this point, because in transmit path it is
//C  *			possible for the skb packet to get re-segmented
//C  *			(depending on net device features).  This could still be
//C  *			a MTU violation, so this flag enables performing MTU
//C  *			check against segments, with a different violation
//C  *			return code to tell it apart. Check cannot use len_diff.
//C  *
//C  *		On return *mtu_len* pointer contains the MTU value of the net
//C  *		device.  Remember the net device configured MTU is the L3 size,
//C  *		which is returned here and XDP and TC length operate at L2.
//C  *		Helper take this into account for you, but remember when using
//C  *		MTU value in your BPF-code.
//C  *
//C  *	Return
//C  *		* 0 on success, and populate MTU value in *mtu_len* pointer.
//C  *
//C  *		* < 0 if any input argument is invalid (*mtu_len* not updated)
//C  *
//C  *		MTU violations return positive values, but also populate MTU
//C  *		value in *mtu_len* pointer, as this can be needed for
//C  *		implementing PMTU handing:
//C  *
//C  *		* **BPF_MTU_CHK_RET_FRAG_NEEDED**
//C  *		* **BPF_MTU_CHK_RET_SEGS_TOOBIG**
//C  *
//C  * long bpf_for_each_map_elem(struct bpf_map *map, void *callback_fn, void *callback_ctx, u64 flags)
//C  *	Description
//C  *		For each element in **map**, call **callback_fn** function with
//C  *		**map**, **callback_ctx** and other map-specific parameters.
//C  *		The **callback_fn** should be a static function and
//C  *		the **callback_ctx** should be a pointer to the stack.
//C  *		The **flags** is used to control certain aspects of the helper.
//C  *		Currently, the **flags** must be 0.
//C  *
//C  *		The following are a list of supported map types and their
//C  *		respective expected callback signatures:
//C  *
//C  *		BPF_MAP_TYPE_HASH, BPF_MAP_TYPE_PERCPU_HASH,
//C  *		BPF_MAP_TYPE_LRU_HASH, BPF_MAP_TYPE_LRU_PERCPU_HASH,
//C  *		BPF_MAP_TYPE_ARRAY, BPF_MAP_TYPE_PERCPU_ARRAY
//C  *
//C  *		long (\*callback_fn)(struct bpf_map \*map, const void \*key, void \*value, void \*ctx);
//C  *
//C  *		For per_cpu maps, the map_value is the value on the cpu where the
//C  *		bpf_prog is running.
//C  *
//C  *		If **callback_fn** return 0, the helper will continue to the next
//C  *		element. If return value is 1, the helper will skip the rest of
//C  *		elements and return. Other return values are not used now.
//C  *
//C  *	Return
//C  *		The number of traversed map elements for success, **-EINVAL** for
//C  *		invalid **flags**.
//C  *
//C  * long bpf_snprintf(char *str, u32 str_size, const char *fmt, u64 *data, u32 data_len)
//C  *	Description
//C  *		Outputs a string into the **str** buffer of size **str_size**
//C  *		based on a format string stored in a read-only map pointed by
//C  *		**fmt**.
//C  *
//C  *		Each format specifier in **fmt** corresponds to one u64 element
//C  *		in the **data** array. For strings and pointers where pointees
//C  *		are accessed, only the pointer values are stored in the *data*
//C  *		array. The *data_len* is the size of *data* in bytes - must be
//C  *		a multiple of 8.
//C  *
//C  *		Formats **%s** and **%p{i,I}{4,6}** require to read kernel
//C  *		memory. Reading kernel memory may fail due to either invalid
//C  *		address or valid address but requiring a major memory fault. If
//C  *		reading kernel memory fails, the string for **%s** will be an
//C  *		empty string, and the ip address for **%p{i,I}{4,6}** will be 0.
//C  *		Not returning error to bpf program is consistent with what
//C  *		**bpf_trace_printk**\ () does for now.
//C  *
//C  *	Return
//C  *		The strictly positive length of the formatted string, including
//C  *		the trailing zero character. If the return value is greater than
//C  *		**str_size**, **str** contains a truncated string, guaranteed to
//C  *		be zero-terminated except when **str_size** is 0.
//C  *
//C  *		Or **-EBUSY** if the per-CPU memory copy buffer is busy.
//C  *
//C  * long bpf_sys_bpf(u32 cmd, void *attr, u32 attr_size)
//C  * 	Description
//C  * 		Execute bpf syscall with given arguments.
//C  * 	Return
//C  * 		A syscall result.
//C  *
//C  * long bpf_btf_find_by_name_kind(char *name, int name_sz, u32 kind, int flags)
//C  * 	Description
//C  * 		Find BTF type with given name and kind in vmlinux BTF or in module's BTFs.
//C  * 	Return
//C  * 		Returns btf_id and btf_obj_fd in lower and upper 32 bits.
//C  *
//C  * long bpf_sys_close(u32 fd)
//C  * 	Description
//C  * 		Execute close syscall for given FD.
//C  * 	Return
//C  * 		A syscall result.
//C  *
//C  * long bpf_timer_init(struct bpf_timer *timer, struct bpf_map *map, u64 flags)
//C  *	Description
//C  *		Initialize the timer.
//C  *		First 4 bits of *flags* specify clockid.
//C  *		Only CLOCK_MONOTONIC, CLOCK_REALTIME, CLOCK_BOOTTIME are allowed.
//C  *		All other bits of *flags* are reserved.
//C  *		The verifier will reject the program if *timer* is not from
//C  *		the same *map*.
//C  *	Return
//C  *		0 on success.
//C  *		**-EBUSY** if *timer* is already initialized.
//C  *		**-EINVAL** if invalid *flags* are passed.
//C  *		**-EPERM** if *timer* is in a map that doesn't have any user references.
//C  *		The user space should either hold a file descriptor to a map with timers
//C  *		or pin such map in bpffs. When map is unpinned or file descriptor is
//C  *		closed all timers in the map will be cancelled and freed.
//C  *
//C  * long bpf_timer_set_callback(struct bpf_timer *timer, void *callback_fn)
//C  *	Description
//C  *		Configure the timer to call *callback_fn* static function.
//C  *	Return
//C  *		0 on success.
//C  *		**-EINVAL** if *timer* was not initialized with bpf_timer_init() earlier.
//C  *		**-EPERM** if *timer* is in a map that doesn't have any user references.
//C  *		The user space should either hold a file descriptor to a map with timers
//C  *		or pin such map in bpffs. When map is unpinned or file descriptor is
//C  *		closed all timers in the map will be cancelled and freed.
//C  *
//C  * long bpf_timer_start(struct bpf_timer *timer, u64 nsecs, u64 flags)
//C  *	Description
//C  *		Set timer expiration N nanoseconds from the current time. The
//C  *		configured callback will be invoked in soft irq context on some cpu
//C  *		and will not repeat unless another bpf_timer_start() is made.
//C  *		In such case the next invocation can migrate to a different cpu.
//C  *		Since struct bpf_timer is a field inside map element the map
//C  *		owns the timer. The bpf_timer_set_callback() will increment refcnt
//C  *		of BPF program to make sure that callback_fn code stays valid.
//C  *		When user space reference to a map reaches zero all timers
//C  *		in a map are cancelled and corresponding program's refcnts are
//C  *		decremented. This is done to make sure that Ctrl-C of a user
//C  *		process doesn't leave any timers running. If map is pinned in
//C  *		bpffs the callback_fn can re-arm itself indefinitely.
//C  *		bpf_map_update/delete_elem() helpers and user space sys_bpf commands
//C  *		cancel and free the timer in the given map element.
//C  *		The map can contain timers that invoke callback_fn-s from different
//C  *		programs. The same callback_fn can serve different timers from
//C  *		different maps if key/value layout matches across maps.
//C  *		Every bpf_timer_set_callback() can have different callback_fn.
//C  *
//C  *		*flags* can be one of:
//C  *
//C  *		**BPF_F_TIMER_ABS**
//C  *			Start the timer in absolute expire value instead of the
//C  *			default relative one.
//C  *		**BPF_F_TIMER_CPU_PIN**
//C  *			Timer will be pinned to the CPU of the caller.
//C  *
//C  *	Return
//C  *		0 on success.
//C  *		**-EINVAL** if *timer* was not initialized with bpf_timer_init() earlier
//C  *		or invalid *flags* are passed.
//C  *
//C  * long bpf_timer_cancel(struct bpf_timer *timer)
//C  *	Description
//C  *		Cancel the timer and wait for callback_fn to finish if it was running.
//C  *	Return
//C  *		0 if the timer was not active.
//C  *		1 if the timer was active.
//C  *		**-EINVAL** if *timer* was not initialized with bpf_timer_init() earlier.
//C  *		**-EDEADLK** if callback_fn tried to call bpf_timer_cancel() on its
//C  *		own timer which would have led to a deadlock otherwise.
//C  *
//C  * u64 bpf_get_func_ip(void *ctx)
//C  * 	Description
//C  * 		Get address of the traced function (for tracing and kprobe programs).
//C  *
//C  * 		When called for kprobe program attached as uprobe it returns
//C  * 		probe address for both entry and return uprobe.
//C  *
//C  * 	Return
//C  * 		Address of the traced function for kprobe.
//C  * 		0 for kprobes placed within the function (not at the entry).
//C  * 		Address of the probe for uprobe and return uprobe.
//C  *
//C  * u64 bpf_get_attach_cookie(void *ctx)
//C  * 	Description
//C  * 		Get bpf_cookie value provided (optionally) during the program
//C  * 		attachment. It might be different for each individual
//C  * 		attachment, even if BPF program itself is the same.
//C  * 		Expects BPF program context *ctx* as a first argument.
//C  *
//C  * 		Supported for the following program types:
//C  *			- kprobe/uprobe;
//C  *			- tracepoint;
//C  *			- perf_event.
//C  * 	Return
//C  *		Value specified by user at BPF link creation/attachment time
//C  *		or 0, if it was not specified.
//C  *
//C  * long bpf_task_pt_regs(struct task_struct *task)
//C  *	Description
//C  *		Get the struct pt_regs associated with **task**.
//C  *	Return
//C  *		A pointer to struct pt_regs.
//C  *
//C  * long bpf_get_branch_snapshot(void *entries, u32 size, u64 flags)
//C  *	Description
//C  *		Get branch trace from hardware engines like Intel LBR. The
//C  *		hardware engine is stopped shortly after the helper is
//C  *		called. Therefore, the user need to filter branch entries
//C  *		based on the actual use case. To capture branch trace
//C  *		before the trigger point of the BPF program, the helper
//C  *		should be called at the beginning of the BPF program.
//C  *
//C  *		The data is stored as struct perf_branch_entry into output
//C  *		buffer *entries*. *size* is the size of *entries* in bytes.
//C  *		*flags* is reserved for now and must be zero.
//C  *
//C  *	Return
//C  *		On success, number of bytes written to *buf*. On error, a
//C  *		negative value.
//C  *
//C  *		**-EINVAL** if *flags* is not zero.
//C  *
//C  *		**-ENOENT** if architecture does not support branch records.
//C  *
//C  * long bpf_trace_vprintk(const char *fmt, u32 fmt_size, const void *data, u32 data_len)
//C  *	Description
//C  *		Behaves like **bpf_trace_printk**\ () helper, but takes an array of u64
//C  *		to format and can handle more format args as a result.
//C  *
//C  *		Arguments are to be used as in **bpf_seq_printf**\ () helper.
//C  *	Return
//C  *		The number of bytes written to the buffer, or a negative error
//C  *		in case of failure.
//C  *
//C  * struct unix_sock *bpf_skc_to_unix_sock(void *sk)
//C  * 	Description
//C  *		Dynamically cast a *sk* pointer to a *unix_sock* pointer.
//C  *	Return
//C  *		*sk* if casting is valid, or **NULL** otherwise.
//C  *
//C  * long bpf_kallsyms_lookup_name(const char *name, int name_sz, int flags, u64 *res)
//C  *	Description
//C  *		Get the address of a kernel symbol, returned in *res*. *res* is
//C  *		set to 0 if the symbol is not found.
//C  *	Return
//C  *		On success, zero. On error, a negative value.
//C  *
//C  *		**-EINVAL** if *flags* is not zero.
//C  *
//C  *		**-EINVAL** if string *name* is not the same size as *name_sz*.
//C  *
//C  *		**-ENOENT** if symbol is not found.
//C  *
//C  *		**-EPERM** if caller does not have permission to obtain kernel address.
//C  *
//C  * long bpf_find_vma(struct task_struct *task, u64 addr, void *callback_fn, void *callback_ctx, u64 flags)
//C  *	Description
//C  *		Find vma of *task* that contains *addr*, call *callback_fn*
//C  *		function with *task*, *vma*, and *callback_ctx*.
//C  *		The *callback_fn* should be a static function and
//C  *		the *callback_ctx* should be a pointer to the stack.
//C  *		The *flags* is used to control certain aspects of the helper.
//C  *		Currently, the *flags* must be 0.
//C  *
//C  *		The expected callback signature is
//C  *
//C  *		long (\*callback_fn)(struct task_struct \*task, struct vm_area_struct \*vma, void \*callback_ctx);
//C  *
//C  *	Return
//C  *		0 on success.
//C  *		**-ENOENT** if *task->mm* is NULL, or no vma contains *addr*.
//C  *		**-EBUSY** if failed to try lock mmap_lock.
//C  *		**-EINVAL** for invalid **flags**.
//C  *
//C  * long bpf_loop(u32 nr_loops, void *callback_fn, void *callback_ctx, u64 flags)
//C  *	Description
//C  *		For **nr_loops**, call **callback_fn** function
//C  *		with **callback_ctx** as the context parameter.
//C  *		The **callback_fn** should be a static function and
//C  *		the **callback_ctx** should be a pointer to the stack.
//C  *		The **flags** is used to control certain aspects of the helper.
//C  *		Currently, the **flags** must be 0. Currently, nr_loops is
//C  *		limited to 1 << 23 (~8 million) loops.
//C  *
//C  *		long (\*callback_fn)(u64 index, void \*ctx);
//C  *
//C  *		where **index** is the current index in the loop. The index
//C  *		is zero-indexed.
//C  *
//C  *		If **callback_fn** returns 0, the helper will continue to the next
//C  *		loop. If return value is 1, the helper will skip the rest of
//C  *		the loops and return. Other return values are not used now,
//C  *		and will be rejected by the verifier.
//C  *
//C  *	Return
//C  *		The number of loops performed, **-EINVAL** for invalid **flags**,
//C  *		**-E2BIG** if **nr_loops** exceeds the maximum number of loops.
//C  *
//C  * long bpf_strncmp(const char *s1, u32 s1_sz, const char *s2)
//C  *	Description
//C  *		Do strncmp() between **s1** and **s2**. **s1** doesn't need
//C  *		to be null-terminated and **s1_sz** is the maximum storage
//C  *		size of **s1**. **s2** must be a read-only string.
//C  *	Return
//C  *		An integer less than, equal to, or greater than zero
//C  *		if the first **s1_sz** bytes of **s1** is found to be
//C  *		less than, to match, or be greater than **s2**.
//C  *
//C  * long bpf_get_func_arg(void *ctx, u32 n, u64 *value)
//C  *	Description
//C  *		Get **n**-th argument register (zero based) of the traced function (for tracing programs)
//C  *		returned in **value**.
//C  *
//C  *	Return
//C  *		0 on success.
//C  *		**-EINVAL** if n >= argument register count of traced function.
//C  *
//C  * long bpf_get_func_ret(void *ctx, u64 *value)
//C  *	Description
//C  *		Get return value of the traced function (for tracing programs)
//C  *		in **value**.
//C  *
//C  *	Return
//C  *		0 on success.
//C  *		**-EOPNOTSUPP** for tracing programs other than BPF_TRACE_FEXIT or BPF_MODIFY_RETURN.
//C  *
//C  * long bpf_get_func_arg_cnt(void *ctx)
//C  *	Description
//C  *		Get number of registers of the traced function (for tracing programs) where
//C  *		function arguments are stored in these registers.
//C  *
//C  *	Return
//C  *		The number of argument registers of the traced function.
//C  *
//C  * int bpf_get_retval(void)
//C  *	Description
//C  *		Get the BPF program's return value that will be returned to the upper layers.
//C  *
//C  *		This helper is currently supported by cgroup programs and only by the hooks
//C  *		where BPF program's return value is returned to the userspace via errno.
//C  *	Return
//C  *		The BPF program's return value.
//C  *
//C  * int bpf_set_retval(int retval)
//C  *	Description
//C  *		Set the BPF program's return value that will be returned to the upper layers.
//C  *
//C  *		This helper is currently supported by cgroup programs and only by the hooks
//C  *		where BPF program's return value is returned to the userspace via errno.
//C  *
//C  *		Note that there is the following corner case where the program exports an error
//C  *		via bpf_set_retval but signals success via 'return 1':
//C  *
//C  *			bpf_set_retval(-EPERM);
//C  *			return 1;
//C  *
//C  *		In this case, the BPF program's return value will use helper's -EPERM. This
//C  *		still holds true for cgroup/bind{4,6} which supports extra 'return 3' success case.
//C  *
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * u64 bpf_xdp_get_buff_len(struct xdp_buff *xdp_md)
//C  *	Description
//C  *		Get the total size of a given xdp buff (linear and paged area)
//C  *	Return
//C  *		The total size of a given xdp buffer.
//C  *
//C  * long bpf_xdp_load_bytes(struct xdp_buff *xdp_md, u32 offset, void *buf, u32 len)
//C  *	Description
//C  *		This helper is provided as an easy way to load data from a
//C  *		xdp buffer. It can be used to load *len* bytes from *offset* from
//C  *		the frame associated to *xdp_md*, into the buffer pointed by
//C  *		*buf*.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_xdp_store_bytes(struct xdp_buff *xdp_md, u32 offset, void *buf, u32 len)
//C  *	Description
//C  *		Store *len* bytes from buffer *buf* into the frame
//C  *		associated to *xdp_md*, at *offset*.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * long bpf_copy_from_user_task(void *dst, u32 size, const void *user_ptr, struct task_struct *tsk, u64 flags)
//C  *	Description
//C  *		Read *size* bytes from user space address *user_ptr* in *tsk*'s
//C  *		address space, and stores the data in *dst*. *flags* is not
//C  *		used yet and is provided for future extensibility. This helper
//C  *		can only be used by sleepable programs.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure. On error
//C  *		*dst* buffer is zeroed out.
//C  *
//C  * long bpf_skb_set_tstamp(struct sk_buff *skb, u64 tstamp, u32 tstamp_type)
//C  *	Description
//C  *		Change the __sk_buff->tstamp_type to *tstamp_type*
//C  *		and set *tstamp* to the __sk_buff->tstamp together.
//C  *
//C  *		If there is no need to change the __sk_buff->tstamp_type,
//C  *		the tstamp value can be directly written to __sk_buff->tstamp
//C  *		instead.
//C  *
//C  *		BPF_SKB_TSTAMP_DELIVERY_MONO is the only tstamp that
//C  *		will be kept during bpf_redirect_*().  A non zero
//C  *		*tstamp* must be used with the BPF_SKB_TSTAMP_DELIVERY_MONO
//C  *		*tstamp_type*.
//C  *
//C  *		A BPF_SKB_TSTAMP_UNSPEC *tstamp_type* can only be used
//C  *		with a zero *tstamp*.
//C  *
//C  *		Only IPv4 and IPv6 skb->protocol are supported.
//C  *
//C  *		This function is most useful when it needs to set a
//C  *		mono delivery time to __sk_buff->tstamp and then
//C  *		bpf_redirect_*() to the egress of an iface.  For example,
//C  *		changing the (rcv) timestamp in __sk_buff->tstamp at
//C  *		ingress to a mono delivery time and then bpf_redirect_*()
//C  *		to sch_fq@phy-dev.
//C  *	Return
//C  *		0 on success.
//C  *		**-EINVAL** for invalid input
//C  *		**-EOPNOTSUPP** for unsupported protocol
//C  *
//C  * long bpf_ima_file_hash(struct file *file, void *dst, u32 size)
//C  *	Description
//C  *		Returns a calculated IMA hash of the *file*.
//C  *		If the hash is larger than *size*, then only *size*
//C  *		bytes will be copied to *dst*
//C  *	Return
//C  *		The **hash_algo** is returned on success,
//C  *		**-EOPNOTSUPP** if the hash calculation failed or **-EINVAL** if
//C  *		invalid arguments are passed.
//C  *
//C  * void *bpf_kptr_xchg(void *dst, void *ptr)
//C  *	Description
//C  *		Exchange kptr at pointer *dst* with *ptr*, and return the old value.
//C  *		*dst* can be map value or local kptr. *ptr* can be NULL, otherwise
//C  *		it must be a referenced pointer which will be released when this helper
//C  *		is called.
//C  *	Return
//C  *		The old value of kptr (which can be NULL). The returned pointer
//C  *		if not NULL, is a reference which must be released using its
//C  *		corresponding release function, or moved into a BPF map before
//C  *		program exit.
//C  *
//C  * void *bpf_map_lookup_percpu_elem(struct bpf_map *map, const void *key, u32 cpu)
//C  * 	Description
//C  * 		Perform a lookup in *percpu map* for an entry associated to
//C  * 		*key* on *cpu*.
//C  * 	Return
//C  * 		Map value associated to *key* on *cpu*, or **NULL** if no entry
//C  * 		was found or *cpu* is invalid.
//C  *
//C  * struct mptcp_sock *bpf_skc_to_mptcp_sock(void *sk)
//C  *	Description
//C  *		Dynamically cast a *sk* pointer to a *mptcp_sock* pointer.
//C  *	Return
//C  *		*sk* if casting is valid, or **NULL** otherwise.
//C  *
//C  * long bpf_dynptr_from_mem(void *data, u64 size, u64 flags, struct bpf_dynptr *ptr)
//C  *	Description
//C  *		Get a dynptr to local memory *data*.
//C  *
//C  *		*data* must be a ptr to a map value.
//C  *		The maximum *size* supported is DYNPTR_MAX_SIZE.
//C  *		*flags* is currently unused.
//C  *	Return
//C  *		0 on success, -E2BIG if the size exceeds DYNPTR_MAX_SIZE,
//C  *		-EINVAL if flags is not 0.
//C  *
//C  * long bpf_ringbuf_reserve_dynptr(void *ringbuf, u32 size, u64 flags, struct bpf_dynptr *ptr)
//C  *	Description
//C  *		Reserve *size* bytes of payload in a ring buffer *ringbuf*
//C  *		through the dynptr interface. *flags* must be 0.
//C  *
//C  *		Please note that a corresponding bpf_ringbuf_submit_dynptr or
//C  *		bpf_ringbuf_discard_dynptr must be called on *ptr*, even if the
//C  *		reservation fails. This is enforced by the verifier.
//C  *	Return
//C  *		0 on success, or a negative error in case of failure.
//C  *
//C  * void bpf_ringbuf_submit_dynptr(struct bpf_dynptr *ptr, u64 flags)
//C  *	Description
//C  *		Submit reserved ring buffer sample, pointed to by *data*,
//C  *		through the dynptr interface. This is a no-op if the dynptr is
//C  *		invalid/null.
//C  *
//C  *		For more information on *flags*, please see
//C  *		'bpf_ringbuf_submit'.
//C  *	Return
//C  *		Nothing. Always succeeds.
//C  *
//C  * void bpf_ringbuf_discard_dynptr(struct bpf_dynptr *ptr, u64 flags)
//C  *	Description
//C  *		Discard reserved ring buffer sample through the dynptr
//C  *		interface. This is a no-op if the dynptr is invalid/null.
//C  *
//C  *		For more information on *flags*, please see
//C  *		'bpf_ringbuf_discard'.
//C  *	Return
//C  *		Nothing. Always succeeds.
//C  *
//C  * long bpf_dynptr_read(void *dst, u64 len, const struct bpf_dynptr *src, u64 offset, u64 flags)
//C  *	Description
//C  *		Read *len* bytes from *src* into *dst*, starting from *offset*
//C  *		into *src*.
//C  *		*flags* is currently unused.
//C  *	Return
//C  *		0 on success, -E2BIG if *offset* + *len* exceeds the length
//C  *		of *src*'s data, -EINVAL if *src* is an invalid dynptr or if
//C  *		*flags* is not 0.
//C  *
//C  * long bpf_dynptr_write(const struct bpf_dynptr *dst, u64 offset, void *src, u64 len, u64 flags)
//C  *	Description
//C  *		Write *len* bytes from *src* into *dst*, starting from *offset*
//C  *		into *dst*.
//C  *
//C  *		*flags* must be 0 except for skb-type dynptrs.
//C  *
//C  *		For skb-type dynptrs:
//C  *		    *  All data slices of the dynptr are automatically
//C  *		       invalidated after **bpf_dynptr_write**\ (). This is
//C  *		       because writing may pull the skb and change the
//C  *		       underlying packet buffer.
//C  *
//C  *		    *  For *flags*, please see the flags accepted by
//C  *		       **bpf_skb_store_bytes**\ ().
//C  *	Return
//C  *		0 on success, -E2BIG if *offset* + *len* exceeds the length
//C  *		of *dst*'s data, -EINVAL if *dst* is an invalid dynptr or if *dst*
//C  *		is a read-only dynptr or if *flags* is not correct. For skb-type dynptrs,
//C  *		other errors correspond to errors returned by **bpf_skb_store_bytes**\ ().
//C  *
//C  * void *bpf_dynptr_data(const struct bpf_dynptr *ptr, u64 offset, u64 len)
//C  *	Description
//C  *		Get a pointer to the underlying dynptr data.
//C  *
//C  *		*len* must be a statically known value. The returned data slice
//C  *		is invalidated whenever the dynptr is invalidated.
//C  *
//C  *		skb and xdp type dynptrs may not use bpf_dynptr_data. They should
//C  *		instead use bpf_dynptr_slice and bpf_dynptr_slice_rdwr.
//C  *	Return
//C  *		Pointer to the underlying dynptr data, NULL if the dynptr is
//C  *		read-only, if the dynptr is invalid, or if the offset and length
//C  *		is out of bounds.
//C  *
//C  * s64 bpf_tcp_raw_gen_syncookie_ipv4(struct iphdr *iph, struct tcphdr *th, u32 th_len)
//C  *	Description
//C  *		Try to issue a SYN cookie for the packet with corresponding
//C  *		IPv4/TCP headers, *iph* and *th*, without depending on a
//C  *		listening socket.
//C  *
//C  *		*iph* points to the IPv4 header.
//C  *
//C  *		*th* points to the start of the TCP header, while *th_len*
//C  *		contains the length of the TCP header (at least
//C  *		**sizeof**\ (**struct tcphdr**)).
//C  *	Return
//C  *		On success, lower 32 bits hold the generated SYN cookie in
//C  *		followed by 16 bits which hold the MSS value for that cookie,
//C  *		and the top 16 bits are unused.
//C  *
//C  *		On failure, the returned value is one of the following:
//C  *
//C  *		**-EINVAL** if *th_len* is invalid.
//C  *
//C  * s64 bpf_tcp_raw_gen_syncookie_ipv6(struct ipv6hdr *iph, struct tcphdr *th, u32 th_len)
//C  *	Description
//C  *		Try to issue a SYN cookie for the packet with corresponding
//C  *		IPv6/TCP headers, *iph* and *th*, without depending on a
//C  *		listening socket.
//C  *
//C  *		*iph* points to the IPv6 header.
//C  *
//C  *		*th* points to the start of the TCP header, while *th_len*
//C  *		contains the length of the TCP header (at least
//C  *		**sizeof**\ (**struct tcphdr**)).
//C  *	Return
//C  *		On success, lower 32 bits hold the generated SYN cookie in
//C  *		followed by 16 bits which hold the MSS value for that cookie,
//C  *		and the top 16 bits are unused.
//C  *
//C  *		On failure, the returned value is one of the following:
//C  *
//C  *		**-EINVAL** if *th_len* is invalid.
//C  *
//C  *		**-EPROTONOSUPPORT** if CONFIG_IPV6 is not builtin.
//C  *
//C  * long bpf_tcp_raw_check_syncookie_ipv4(struct iphdr *iph, struct tcphdr *th)
//C  *	Description
//C  *		Check whether *iph* and *th* contain a valid SYN cookie ACK
//C  *		without depending on a listening socket.
//C  *
//C  *		*iph* points to the IPv4 header.
//C  *
//C  *		*th* points to the TCP header.
//C  *	Return
//C  *		0 if *iph* and *th* are a valid SYN cookie ACK.
//C  *
//C  *		On failure, the returned value is one of the following:
//C  *
//C  *		**-EACCES** if the SYN cookie is not valid.
//C  *
//C  * long bpf_tcp_raw_check_syncookie_ipv6(struct ipv6hdr *iph, struct tcphdr *th)
//C  *	Description
//C  *		Check whether *iph* and *th* contain a valid SYN cookie ACK
//C  *		without depending on a listening socket.
//C  *
//C  *		*iph* points to the IPv6 header.
//C  *
//C  *		*th* points to the TCP header.
//C  *	Return
//C  *		0 if *iph* and *th* are a valid SYN cookie ACK.
//C  *
//C  *		On failure, the returned value is one of the following:
//C  *
//C  *		**-EACCES** if the SYN cookie is not valid.
//C  *
//C  *		**-EPROTONOSUPPORT** if CONFIG_IPV6 is not builtin.
//C  *
//C  * u64 bpf_ktime_get_tai_ns(void)
//C  *	Description
//C  *		A nonsettable system-wide clock derived from wall-clock time but
//C  *		ignoring leap seconds.  This clock does not experience
//C  *		discontinuities and backwards jumps caused by NTP inserting leap
//C  *		seconds as CLOCK_REALTIME does.
//C  *
//C  *		See: **clock_gettime**\ (**CLOCK_TAI**)
//C  *	Return
//C  *		Current *ktime*.
//C  *
//C  * long bpf_user_ringbuf_drain(struct bpf_map *map, void *callback_fn, void *ctx, u64 flags)
//C  *	Description
//C  *		Drain samples from the specified user ring buffer, and invoke
//C  *		the provided callback for each such sample:
//C  *
//C  *		long (\*callback_fn)(const struct bpf_dynptr \*dynptr, void \*ctx);
//C  *
//C  *		If **callback_fn** returns 0, the helper will continue to try
//C  *		and drain the next sample, up to a maximum of
//C  *		BPF_MAX_USER_RINGBUF_SAMPLES samples. If the return value is 1,
//C  *		the helper will skip the rest of the samples and return. Other
//C  *		return values are not used now, and will be rejected by the
//C  *		verifier.
//C  *	Return
//C  *		The number of drained samples if no error was encountered while
//C  *		draining samples, or 0 if no samples were present in the ring
//C  *		buffer. If a user-space producer was epoll-waiting on this map,
//C  *		and at least one sample was drained, they will receive an event
//C  *		notification notifying them of available space in the ring
//C  *		buffer. If the BPF_RB_NO_WAKEUP flag is passed to this
//C  *		function, no wakeup notification will be sent. If the
//C  *		BPF_RB_FORCE_WAKEUP flag is passed, a wakeup notification will
//C  *		be sent even if no sample was drained.
//C  *
//C  *		On failure, the returned value is one of the following:
//C  *
//C  *		**-EBUSY** if the ring buffer is contended, and another calling
//C  *		context was concurrently draining the ring buffer.
//C  *
//C  *		**-EINVAL** if user-space is not properly tracking the ring
//C  *		buffer due to the producer position not being aligned to 8
//C  *		bytes, a sample not being aligned to 8 bytes, or the producer
//C  *		position not matching the advertised length of a sample.
//C  *
//C  *		**-E2BIG** if user-space has tried to publish a sample which is
//C  *		larger than the size of the ring buffer, or which cannot fit
//C  *		within a struct bpf_dynptr.
//C  *
//C  * void *bpf_cgrp_storage_get(struct bpf_map *map, struct cgroup *cgroup, void *value, u64 flags)
//C  *	Description
//C  *		Get a bpf_local_storage from the *cgroup*.
//C  *
//C  *		Logically, it could be thought of as getting the value from
//C  *		a *map* with *cgroup* as the **key**.  From this
//C  *		perspective,  the usage is not much different from
//C  *		**bpf_map_lookup_elem**\ (*map*, **&**\ *cgroup*) except this
//C  *		helper enforces the key must be a cgroup struct and the map must also
//C  *		be a **BPF_MAP_TYPE_CGRP_STORAGE**.
//C  *
//C  *		In reality, the local-storage value is embedded directly inside of the
//C  *		*cgroup* object itself, rather than being located in the
//C  *		**BPF_MAP_TYPE_CGRP_STORAGE** map. When the local-storage value is
//C  *		queried for some *map* on a *cgroup* object, the kernel will perform an
//C  *		O(n) iteration over all of the live local-storage values for that
//C  *		*cgroup* object until the local-storage value for the *map* is found.
//C  *
//C  *		An optional *flags* (**BPF_LOCAL_STORAGE_GET_F_CREATE**) can be
//C  *		used such that a new bpf_local_storage will be
//C  *		created if one does not exist.  *value* can be used
//C  *		together with **BPF_LOCAL_STORAGE_GET_F_CREATE** to specify
//C  *		the initial value of a bpf_local_storage.  If *value* is
//C  *		**NULL**, the new bpf_local_storage will be zero initialized.
//C  *	Return
//C  *		A bpf_local_storage pointer is returned on success.
//C  *
//C  *		**NULL** if not found or there was an error in adding
//C  *		a new bpf_local_storage.
//C  *
//C  * long bpf_cgrp_storage_delete(struct bpf_map *map, struct cgroup *cgroup)
//C  *	Description
//C  *		Delete a bpf_local_storage from a *cgroup*.
//C  *	Return
//C  *		0 on success.
//C  *
//C  *		**-ENOENT** if the bpf_local_storage cannot be found.
//C  */
//C #define ___BPF_FUNC_MAPPER(FN, ctx...)			\
//C 	FN(unspec, 0, ##ctx)				\
//C 	FN(map_lookup_elem, 1, ##ctx)			\
//C 	FN(map_update_elem, 2, ##ctx)			\
//C 	FN(map_delete_elem, 3, ##ctx)			\
//C 	FN(probe_read, 4, ##ctx)			\
//C 	FN(ktime_get_ns, 5, ##ctx)			\
//C 	FN(trace_printk, 6, ##ctx)			\
//C 	FN(get_prandom_u32, 7, ##ctx)			\
//C 	FN(get_smp_processor_id, 8, ##ctx)		\
//C 	FN(skb_store_bytes, 9, ##ctx)			\
//C 	FN(l3_csum_replace, 10, ##ctx)			\
//C 	FN(l4_csum_replace, 11, ##ctx)			\
//C 	FN(tail_call, 12, ##ctx)			\
//C 	FN(clone_redirect, 13, ##ctx)			\
//C 	FN(get_current_pid_tgid, 14, ##ctx)		\
//C 	FN(get_current_uid_gid, 15, ##ctx)		\
//C 	FN(get_current_comm, 16, ##ctx)			\
//C 	FN(get_cgroup_classid, 17, ##ctx)		\
//C 	FN(skb_vlan_push, 18, ##ctx)			\
//C 	FN(skb_vlan_pop, 19, ##ctx)			\
//C 	FN(skb_get_tunnel_key, 20, ##ctx)		\
//C 	FN(skb_set_tunnel_key, 21, ##ctx)		\
//C 	FN(perf_event_read, 22, ##ctx)			\
//C 	FN(redirect, 23, ##ctx)				\
//C 	FN(get_route_realm, 24, ##ctx)			\
//C 	FN(perf_event_output, 25, ##ctx)		\
//C 	FN(skb_load_bytes, 26, ##ctx)			\
//C 	FN(get_stackid, 27, ##ctx)			\
//C 	FN(csum_diff, 28, ##ctx)			\
//C 	FN(skb_get_tunnel_opt, 29, ##ctx)		\
//C 	FN(skb_set_tunnel_opt, 30, ##ctx)		\
//C 	FN(skb_change_proto, 31, ##ctx)			\
//C 	FN(skb_change_type, 32, ##ctx)			\
//C 	FN(skb_under_cgroup, 33, ##ctx)			\
//C 	FN(get_hash_recalc, 34, ##ctx)			\
//C 	FN(get_current_task, 35, ##ctx)			\
//C 	FN(probe_write_user, 36, ##ctx)			\
//C 	FN(current_task_under_cgroup, 37, ##ctx)	\
//C 	FN(skb_change_tail, 38, ##ctx)			\
//C 	FN(skb_pull_data, 39, ##ctx)			\
//C 	FN(csum_update, 40, ##ctx)			\
//C 	FN(set_hash_invalid, 41, ##ctx)			\
//C 	FN(get_numa_node_id, 42, ##ctx)			\
//C 	FN(skb_change_head, 43, ##ctx)			\
//C 	FN(xdp_adjust_head, 44, ##ctx)			\
//C 	FN(probe_read_str, 45, ##ctx)			\
//C 	FN(get_socket_cookie, 46, ##ctx)		\
//C 	FN(get_socket_uid, 47, ##ctx)			\
//C 	FN(set_hash, 48, ##ctx)				\
//C 	FN(setsockopt, 49, ##ctx)			\
//C 	FN(skb_adjust_room, 50, ##ctx)			\
//C 	FN(redirect_map, 51, ##ctx)			\
//C 	FN(sk_redirect_map, 52, ##ctx)			\
//C 	FN(sock_map_update, 53, ##ctx)			\
//C 	FN(xdp_adjust_meta, 54, ##ctx)			\
//C 	FN(perf_event_read_value, 55, ##ctx)		\
//C 	FN(perf_prog_read_value, 56, ##ctx)		\
//C 	FN(getsockopt, 57, ##ctx)			\
//C 	FN(override_return, 58, ##ctx)			\
//C 	FN(sock_ops_cb_flags_set, 59, ##ctx)		\
//C 	FN(msg_redirect_map, 60, ##ctx)			\
//C 	FN(msg_apply_bytes, 61, ##ctx)			\
//C 	FN(msg_cork_bytes, 62, ##ctx)			\
//C 	FN(msg_pull_data, 63, ##ctx)			\
//C 	FN(bind, 64, ##ctx)				\
//C 	FN(xdp_adjust_tail, 65, ##ctx)			\
//C 	FN(skb_get_xfrm_state, 66, ##ctx)		\
//C 	FN(get_stack, 67, ##ctx)			\
//C 	FN(skb_load_bytes_relative, 68, ##ctx)		\
//C 	FN(fib_lookup, 69, ##ctx)			\
//C 	FN(sock_hash_update, 70, ##ctx)			\
//C 	FN(msg_redirect_hash, 71, ##ctx)		\
//C 	FN(sk_redirect_hash, 72, ##ctx)			\
//C 	FN(lwt_push_encap, 73, ##ctx)			\
//C 	FN(lwt_seg6_store_bytes, 74, ##ctx)		\
//C 	FN(lwt_seg6_adjust_srh, 75, ##ctx)		\
//C 	FN(lwt_seg6_action, 76, ##ctx)			\
//C 	FN(rc_repeat, 77, ##ctx)			\
//C 	FN(rc_keydown, 78, ##ctx)			\
//C 	FN(skb_cgroup_id, 79, ##ctx)			\
//C 	FN(get_current_cgroup_id, 80, ##ctx)		\
//C 	FN(get_local_storage, 81, ##ctx)		\
//C 	FN(sk_select_reuseport, 82, ##ctx)		\
//C 	FN(skb_ancestor_cgroup_id, 83, ##ctx)		\
//C 	FN(sk_lookup_tcp, 84, ##ctx)			\
//C 	FN(sk_lookup_udp, 85, ##ctx)			\
//C 	FN(sk_release, 86, ##ctx)			\
//C 	FN(map_push_elem, 87, ##ctx)			\
//C 	FN(map_pop_elem, 88, ##ctx)			\
//C 	FN(map_peek_elem, 89, ##ctx)			\
//C 	FN(msg_push_data, 90, ##ctx)			\
//C 	FN(msg_pop_data, 91, ##ctx)			\
//C 	FN(rc_pointer_rel, 92, ##ctx)			\
//C 	FN(spin_lock, 93, ##ctx)			\
//C 	FN(spin_unlock, 94, ##ctx)			\
//C 	FN(sk_fullsock, 95, ##ctx)			\
//C 	FN(tcp_sock, 96, ##ctx)				\
//C 	FN(skb_ecn_set_ce, 97, ##ctx)			\
//C 	FN(get_listener_sock, 98, ##ctx)		\
//C 	FN(skc_lookup_tcp, 99, ##ctx)			\
//C 	FN(tcp_check_syncookie, 100, ##ctx)		\
//C 	FN(sysctl_get_name, 101, ##ctx)			\
//C 	FN(sysctl_get_current_value, 102, ##ctx)	\
//C 	FN(sysctl_get_new_value, 103, ##ctx)		\
//C 	FN(sysctl_set_new_value, 104, ##ctx)		\
//C 	FN(strtol, 105, ##ctx)				\
//C 	FN(strtoul, 106, ##ctx)				\
//C 	FN(sk_storage_get, 107, ##ctx)			\
//C 	FN(sk_storage_delete, 108, ##ctx)		\
//C 	FN(send_signal, 109, ##ctx)			\
//C 	FN(tcp_gen_syncookie, 110, ##ctx)		\
//C 	FN(skb_output, 111, ##ctx)			\
//C 	FN(probe_read_user, 112, ##ctx)			\
//C 	FN(probe_read_kernel, 113, ##ctx)		\
//C 	FN(probe_read_user_str, 114, ##ctx)		\
//C 	FN(probe_read_kernel_str, 115, ##ctx)		\
//C 	FN(tcp_send_ack, 116, ##ctx)			\
//C 	FN(send_signal_thread, 117, ##ctx)		\
//C 	FN(jiffies64, 118, ##ctx)			\
//C 	FN(read_branch_records, 119, ##ctx)		\
//C 	FN(get_ns_current_pid_tgid, 120, ##ctx)		\
//C 	FN(xdp_output, 121, ##ctx)			\
//C 	FN(get_netns_cookie, 122, ##ctx)		\
//C 	FN(get_current_ancestor_cgroup_id, 123, ##ctx)	\
//C 	FN(sk_assign, 124, ##ctx)			\
//C 	FN(ktime_get_boot_ns, 125, ##ctx)		\
//C 	FN(seq_printf, 126, ##ctx)			\
//C 	FN(seq_write, 127, ##ctx)			\
//C 	FN(sk_cgroup_id, 128, ##ctx)			\
//C 	FN(sk_ancestor_cgroup_id, 129, ##ctx)		\
//C 	FN(ringbuf_output, 130, ##ctx)			\
//C 	FN(ringbuf_reserve, 131, ##ctx)			\
//C 	FN(ringbuf_submit, 132, ##ctx)			\
//C 	FN(ringbuf_discard, 133, ##ctx)			\
//C 	FN(ringbuf_query, 134, ##ctx)			\
//C 	FN(csum_level, 135, ##ctx)			\
//C 	FN(skc_to_tcp6_sock, 136, ##ctx)		\
//C 	FN(skc_to_tcp_sock, 137, ##ctx)			\
//C 	FN(skc_to_tcp_timewait_sock, 138, ##ctx)	\
//C 	FN(skc_to_tcp_request_sock, 139, ##ctx)		\
//C 	FN(skc_to_udp6_sock, 140, ##ctx)		\
//C 	FN(get_task_stack, 141, ##ctx)			\
//C 	FN(load_hdr_opt, 142, ##ctx)			\
//C 	FN(store_hdr_opt, 143, ##ctx)			\
//C 	FN(reserve_hdr_opt, 144, ##ctx)			\
//C 	FN(inode_storage_get, 145, ##ctx)		\
//C 	FN(inode_storage_delete, 146, ##ctx)		\
//C 	FN(d_path, 147, ##ctx)				\
//C 	FN(copy_from_user, 148, ##ctx)			\
//C 	FN(snprintf_btf, 149, ##ctx)			\
//C 	FN(seq_printf_btf, 150, ##ctx)			\
//C 	FN(skb_cgroup_classid, 151, ##ctx)		\
//C 	FN(redirect_neigh, 152, ##ctx)			\
//C 	FN(per_cpu_ptr, 153, ##ctx)			\
//C 	FN(this_cpu_ptr, 154, ##ctx)			\
//C 	FN(redirect_peer, 155, ##ctx)			\
//C 	FN(task_storage_get, 156, ##ctx)		\
//C 	FN(task_storage_delete, 157, ##ctx)		\
//C 	FN(get_current_task_btf, 158, ##ctx)		\
//C 	FN(bprm_opts_set, 159, ##ctx)			\
//C 	FN(ktime_get_coarse_ns, 160, ##ctx)		\
//C 	FN(ima_inode_hash, 161, ##ctx)			\
//C 	FN(sock_from_file, 162, ##ctx)			\
//C 	FN(check_mtu, 163, ##ctx)			\
//C 	FN(for_each_map_elem, 164, ##ctx)		\
//C 	FN(snprintf, 165, ##ctx)			\
//C 	FN(sys_bpf, 166, ##ctx)				\
//C 	FN(btf_find_by_name_kind, 167, ##ctx)		\
//C 	FN(sys_close, 168, ##ctx)			\
//C 	FN(timer_init, 169, ##ctx)			\
//C 	FN(timer_set_callback, 170, ##ctx)		\
//C 	FN(timer_start, 171, ##ctx)			\
//C 	FN(timer_cancel, 172, ##ctx)			\
//C 	FN(get_func_ip, 173, ##ctx)			\
//C 	FN(get_attach_cookie, 174, ##ctx)		\
//C 	FN(task_pt_regs, 175, ##ctx)			\
//C 	FN(get_branch_snapshot, 176, ##ctx)		\
//C 	FN(trace_vprintk, 177, ##ctx)			\
//C 	FN(skc_to_unix_sock, 178, ##ctx)		\
//C 	FN(kallsyms_lookup_name, 179, ##ctx)		\
//C 	FN(find_vma, 180, ##ctx)			\
//C 	FN(loop, 181, ##ctx)				\
//C 	FN(strncmp, 182, ##ctx)				\
//C 	FN(get_func_arg, 183, ##ctx)			\
//C 	FN(get_func_ret, 184, ##ctx)			\
//C 	FN(get_func_arg_cnt, 185, ##ctx)		\
//C 	FN(get_retval, 186, ##ctx)			\
//C 	FN(set_retval, 187, ##ctx)			\
//C 	FN(xdp_get_buff_len, 188, ##ctx)		\
//C 	FN(xdp_load_bytes, 189, ##ctx)			\
//C 	FN(xdp_store_bytes, 190, ##ctx)			\
//C 	FN(copy_from_user_task, 191, ##ctx)		\
//C 	FN(skb_set_tstamp, 192, ##ctx)			\
//C 	FN(ima_file_hash, 193, ##ctx)			\
//C 	FN(kptr_xchg, 194, ##ctx)			\
//C 	FN(map_lookup_percpu_elem, 195, ##ctx)		\
//C 	FN(skc_to_mptcp_sock, 196, ##ctx)		\
//C 	FN(dynptr_from_mem, 197, ##ctx)			\
//C 	FN(ringbuf_reserve_dynptr, 198, ##ctx)		\
//C 	FN(ringbuf_submit_dynptr, 199, ##ctx)		\
//C 	FN(ringbuf_discard_dynptr, 200, ##ctx)		\
//C 	FN(dynptr_read, 201, ##ctx)			\
//C 	FN(dynptr_write, 202, ##ctx)			\
//C 	FN(dynptr_data, 203, ##ctx)			\
//C 	FN(tcp_raw_gen_syncookie_ipv4, 204, ##ctx)	\
//C 	FN(tcp_raw_gen_syncookie_ipv6, 205, ##ctx)	\
//C 	FN(tcp_raw_check_syncookie_ipv4, 206, ##ctx)	\
//C 	FN(tcp_raw_check_syncookie_ipv6, 207, ##ctx)	\
//C 	FN(ktime_get_tai_ns, 208, ##ctx)		\
//C 	FN(user_ringbuf_drain, 209, ##ctx)		\
//C 	FN(cgrp_storage_get, 210, ##ctx)		\
//C 	FN(cgrp_storage_delete, 211, ##ctx)		\
//C 	/* This helper list is effectively frozen. If you are trying to	\
//C 	 * add a new helper, you should add a kfunc instead which has	\
//C 	 * less stability guarantees. See Documentation/bpf/kfuncs.rst	\
//C 	 */
//C 
//C /* backwards-compatibility macros for users of __BPF_FUNC_MAPPER that don't
//C  * know or care about integer value that is now passed as second argument
//C  */
//C #define __BPF_FUNC_MAPPER_APPLY(name, value, FN) FN(name),
//C #define __BPF_FUNC_MAPPER(FN) ___BPF_FUNC_MAPPER(__BPF_FUNC_MAPPER_APPLY, FN)
//C 
//C /* integer value in 'imm' field of BPF_CALL instruction selects which helper
//C  * function eBPF program intends to call
//C  */
//C #define __BPF_ENUM_FN(x, y) BPF_FUNC_ ## x = y,
//C enum bpf_func_id {
//C 	___BPF_FUNC_MAPPER(__BPF_ENUM_FN)
//C 	__BPF_FUNC_MAX_ID,
//C };
//C #undef __BPF_ENUM_FN
//C 
//C /* All flags used by eBPF helper functions, placed here. */
//C 
//C /* BPF_FUNC_skb_store_bytes flags. */
//C enum {
//C 	BPF_F_RECOMPUTE_CSUM		= (1ULL << 0),
//C 	BPF_F_INVALIDATE_HASH		= (1ULL << 1),
//C };
//C 
//C /* BPF_FUNC_l3_csum_replace and BPF_FUNC_l4_csum_replace flags.
//C  * First 4 bits are for passing the header field size.
//C  */
//C enum {
//C 	BPF_F_HDR_FIELD_MASK		= 0xfULL,
//C };
//C 
//C /* BPF_FUNC_l4_csum_replace flags. */
//C enum {
//C 	BPF_F_PSEUDO_HDR		= (1ULL << 4),
//C 	BPF_F_MARK_MANGLED_0		= (1ULL << 5),
//C 	BPF_F_MARK_ENFORCE		= (1ULL << 6),
//C 	BPF_F_IPV6			= (1ULL << 7),
//C };
//C 
//C /* BPF_FUNC_skb_set_tunnel_key and BPF_FUNC_skb_get_tunnel_key flags. */
//C enum {
//C 	BPF_F_TUNINFO_IPV6		= (1ULL << 0),
//C };
//C 
//C /* flags for both BPF_FUNC_get_stackid and BPF_FUNC_get_stack. */
//C enum {
//C 	BPF_F_SKIP_FIELD_MASK		= 0xffULL,
//C 	BPF_F_USER_STACK		= (1ULL << 8),
//C /* flags used by BPF_FUNC_get_stackid only. */
//C 	BPF_F_FAST_STACK_CMP		= (1ULL << 9),
//C 	BPF_F_REUSE_STACKID		= (1ULL << 10),
//C /* flags used by BPF_FUNC_get_stack only. */
//C 	BPF_F_USER_BUILD_ID		= (1ULL << 11),
//C };
//C 
//C /* BPF_FUNC_skb_set_tunnel_key flags. */
//C enum {
//C 	BPF_F_ZERO_CSUM_TX		= (1ULL << 1),
//C 	BPF_F_DONT_FRAGMENT		= (1ULL << 2),
//C 	BPF_F_SEQ_NUMBER		= (1ULL << 3),
//C 	BPF_F_NO_TUNNEL_KEY		= (1ULL << 4),
//C };
//C 
//C /* BPF_FUNC_skb_get_tunnel_key flags. */
//C enum {
//C 	BPF_F_TUNINFO_FLAGS		= (1ULL << 4),
//C };
//C 
//C /* BPF_FUNC_perf_event_output, BPF_FUNC_perf_event_read and
//C  * BPF_FUNC_perf_event_read_value flags.
//C  */
//C enum {
//C 	BPF_F_INDEX_MASK		= 0xffffffffULL,
//C 	BPF_F_CURRENT_CPU		= BPF_F_INDEX_MASK,
//C /* BPF_FUNC_perf_event_output for sk_buff input context. */
//C 	BPF_F_CTXLEN_MASK		= (0xfffffULL << 32),
//C };
//C 
//C /* Current network namespace */
//C enum {
//C 	BPF_F_CURRENT_NETNS		= (-1L),
//C };
//C 
//C /* BPF_FUNC_csum_level level values. */
//C enum {
//C 	BPF_CSUM_LEVEL_QUERY,
//C 	BPF_CSUM_LEVEL_INC,
//C 	BPF_CSUM_LEVEL_DEC,
//C 	BPF_CSUM_LEVEL_RESET,
//C };
//C 
//C /* BPF_FUNC_skb_adjust_room flags. */
//C enum bpf_adj_room_flags {
//C 	BPF_F_ADJ_ROOM_FIXED_GSO	= (1ULL << 0),
//C 	BPF_F_ADJ_ROOM_ENCAP_L3_IPV4	= (1ULL << 1),
//C 	BPF_F_ADJ_ROOM_ENCAP_L3_IPV6	= (1ULL << 2),
//C 	BPF_F_ADJ_ROOM_ENCAP_L4_GRE	= (1ULL << 3),
//C 	BPF_F_ADJ_ROOM_ENCAP_L4_UDP	= (1ULL << 4),
//C 	BPF_F_ADJ_ROOM_NO_CSUM_RESET	= (1ULL << 5),
//C 	BPF_F_ADJ_ROOM_ENCAP_L2_ETH	= (1ULL << 6),
//C 	BPF_F_ADJ_ROOM_DECAP_L3_IPV4	= (1ULL << 7),
//C 	BPF_F_ADJ_ROOM_DECAP_L3_IPV6	= (1ULL << 8),
//C 	BPF_F_ADJ_ROOM_DECAP_L4_GRE	= (1ULL << 9),
//C 	BPF_F_ADJ_ROOM_DECAP_L4_UDP	= (1ULL << 10),
//C 	BPF_F_ADJ_ROOM_DECAP_IPXIP4	= (1ULL << 11),
//C 	BPF_F_ADJ_ROOM_DECAP_IPXIP6	= (1ULL << 12),
//C };
//C 
//C enum {
//C 	BPF_ADJ_ROOM_ENCAP_L2_MASK	= 0xff,
//C 	BPF_ADJ_ROOM_ENCAP_L2_SHIFT	= 56,
//C };
//C 
//C #define BPF_F_ADJ_ROOM_ENCAP_L2(len)	(((__u64)len & \
//C 					  BPF_ADJ_ROOM_ENCAP_L2_MASK) \
//C 					 << BPF_ADJ_ROOM_ENCAP_L2_SHIFT)
//C 
//C /* BPF_FUNC_sysctl_get_name flags. */
//C enum {
//C 	BPF_F_SYSCTL_BASE_NAME		= (1ULL << 0),
//C };
//C 
//C /* BPF_FUNC_<kernel_obj>_storage_get flags */
//C enum {
//C 	BPF_LOCAL_STORAGE_GET_F_CREATE	= (1ULL << 0),
//C 	/* BPF_SK_STORAGE_GET_F_CREATE is only kept for backward compatibility
//C 	 * and BPF_LOCAL_STORAGE_GET_F_CREATE must be used instead.
//C 	 */
//C 	BPF_SK_STORAGE_GET_F_CREATE  = BPF_LOCAL_STORAGE_GET_F_CREATE,
//C };
//C 
//C /* BPF_FUNC_read_branch_records flags. */
//C enum {
//C 	BPF_F_GET_BRANCH_RECORDS_SIZE	= (1ULL << 0),
//C };
//C 
//C /* BPF_FUNC_bpf_ringbuf_commit, BPF_FUNC_bpf_ringbuf_discard, and
//C  * BPF_FUNC_bpf_ringbuf_output flags.
//C  */
//C enum {
//C 	BPF_RB_NO_WAKEUP		= (1ULL << 0),
//C 	BPF_RB_FORCE_WAKEUP		= (1ULL << 1),
//C };
//C 
//C /* BPF_FUNC_bpf_ringbuf_query flags */
//C enum {
//C 	BPF_RB_AVAIL_DATA = 0,
//C 	BPF_RB_RING_SIZE = 1,
//C 	BPF_RB_CONS_POS = 2,
//C 	BPF_RB_PROD_POS = 3,
//C 	BPF_RB_OVERWRITE_POS = 4,
//C };
//C 
//C /* BPF ring buffer constants */
//C enum {
//C 	BPF_RINGBUF_BUSY_BIT		= (1U << 31),
//C 	BPF_RINGBUF_DISCARD_BIT		= (1U << 30),
//C 	BPF_RINGBUF_HDR_SZ		= 8,
//C };
//C 
//C /* BPF_FUNC_sk_assign flags in bpf_sk_lookup context. */
//C enum {
//C 	BPF_SK_LOOKUP_F_REPLACE		= (1ULL << 0),
//C 	BPF_SK_LOOKUP_F_NO_REUSEPORT	= (1ULL << 1),
//C };
//C 
//C /* Mode for BPF_FUNC_skb_adjust_room helper. */
//C enum bpf_adj_room_mode {
//C 	BPF_ADJ_ROOM_NET,
//C 	BPF_ADJ_ROOM_MAC,
//C };
//C 
//C /* Mode for BPF_FUNC_skb_load_bytes_relative helper. */
//C enum bpf_hdr_start_off {
//C 	BPF_HDR_START_MAC,
//C 	BPF_HDR_START_NET,
//C };
//C 
//C /* Encapsulation type for BPF_FUNC_lwt_push_encap helper. */
//C enum bpf_lwt_encap_mode {
//C 	BPF_LWT_ENCAP_SEG6,
//C 	BPF_LWT_ENCAP_SEG6_INLINE,
//C 	BPF_LWT_ENCAP_IP,
//C };
//C 
//C /* Flags for bpf_bprm_opts_set helper */
//C enum {
//C 	BPF_F_BPRM_SECUREEXEC	= (1ULL << 0),
//C };
//C 
//C /* Flags for bpf_redirect and bpf_redirect_map helpers */
//C enum {
//C 	BPF_F_INGRESS		= (1ULL << 0), /* used for skb path */
//C 	BPF_F_EGRESS		= (1ULL << 1), /* used for skb path */
//C 	BPF_F_BROADCAST		= (1ULL << 3), /* used for XDP path */
//C 	BPF_F_EXCLUDE_INGRESS	= (1ULL << 4), /* used for XDP path */
//C #define BPF_F_REDIRECT_FLAGS (BPF_F_INGRESS | BPF_F_EGRESS | BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS)
//C };
//C 
//C #define __bpf_md_ptr(type, name)	\
//C union {					\
//C 	type name;			\
//C 	__u64 :64;			\
//C } __attribute__((aligned(8)))
//C 
//C /* The enum used in skb->tstamp_type. It specifies the clock type
//C  * of the time stored in the skb->tstamp.
//C  */
//C enum {
//C 	BPF_SKB_TSTAMP_UNSPEC = 0,		/* DEPRECATED */
//C 	BPF_SKB_TSTAMP_DELIVERY_MONO = 1,	/* DEPRECATED */
//C 	BPF_SKB_CLOCK_REALTIME = 0,
//C 	BPF_SKB_CLOCK_MONOTONIC = 1,
//C 	BPF_SKB_CLOCK_TAI = 2,
//C 	/* For any future BPF_SKB_CLOCK_* that the bpf prog cannot handle,
//C 	 * the bpf prog can try to deduce it by ingress/egress/skb->sk->sk_clockid.
//C 	 */
//C };
//C 
//C /* user accessible mirror of in-kernel sk_buff.
//C  * new fields can only be added to the end of this structure
//C  */
//C struct __sk_buff {
//C 	__u32 len;
//C 	__u32 pkt_type;
//C 	__u32 mark;
//C 	__u32 queue_mapping;
//C 	__u32 protocol;
//C 	__u32 vlan_present;
//C 	__u32 vlan_tci;
//C 	__u32 vlan_proto;
//C 	__u32 priority;
//C 	__u32 ingress_ifindex;
//C 	__u32 ifindex;
//C 	__u32 tc_index;
//C 	__u32 cb[5];
//C 	__u32 hash;
//C 	__u32 tc_classid;
//C 	__u32 data;
//C 	__u32 data_end;
//C 	__u32 napi_id;
//C 
//C 	/* Accessed by BPF_PROG_TYPE_sk_skb types from here to ... */
//C 	__u32 family;
//C 	__u32 remote_ip4;	/* Stored in network byte order */
//C 	__u32 local_ip4;	/* Stored in network byte order */
//C 	__u32 remote_ip6[4];	/* Stored in network byte order */
//C 	__u32 local_ip6[4];	/* Stored in network byte order */
//C 	__u32 remote_port;	/* Stored in network byte order */
//C 	__u32 local_port;	/* stored in host byte order */
//C 	/* ... here. */
//C 
//C 	__u32 data_meta;
//C 	__bpf_md_ptr(struct bpf_flow_keys *, flow_keys);
//C 	__u64 tstamp;
//C 	__u32 wire_len;
//C 	__u32 gso_segs;
//C 	__bpf_md_ptr(struct bpf_sock *, sk);
//C 	__u32 gso_size;
//C 	__u8  tstamp_type;
//C 	__u32 :24;		/* Padding, future use. */
//C 	__u64 hwtstamp;
//C };
//C 
//C struct bpf_tunnel_key {
//C 	__u32 tunnel_id;
//C 	union {
//C 		__u32 remote_ipv4;
//C 		__u32 remote_ipv6[4];
//C 	};
//C 	__u8 tunnel_tos;
//C 	__u8 tunnel_ttl;
//C 	union {
//C 		__u16 tunnel_ext;	/* compat */
//C 		__be16 tunnel_flags;
//C 	};
//C 	__u32 tunnel_label;
//C 	union {
//C 		__u32 local_ipv4;
//C 		__u32 local_ipv6[4];
//C 	};
//C };
//C 
//C /* user accessible mirror of in-kernel xfrm_state.
//C  * new fields can only be added to the end of this structure
//C  */
//C struct bpf_xfrm_state {
//C 	__u32 reqid;
//C 	__u32 spi;	/* Stored in network byte order */
//C 	__u16 family;
//C 	__u16 ext;	/* Padding, future use. */
//C 	union {
//C 		__u32 remote_ipv4;	/* Stored in network byte order */
//C 		__u32 remote_ipv6[4];	/* Stored in network byte order */
//C 	};
//C };
//C 
//C /* Generic BPF return codes which all BPF program types may support.
//C  * The values are binary compatible with their TC_ACT_* counter-part to
//C  * provide backwards compatibility with existing SCHED_CLS and SCHED_ACT
//C  * programs.
//C  *
//C  * XDP is handled seprately, see XDP_*.
//C  */
//C enum bpf_ret_code {
//C 	BPF_OK = 0,
//C 	/* 1 reserved */
//C 	BPF_DROP = 2,
//C 	/* 3-6 reserved */
//C 	BPF_REDIRECT = 7,
//C 	/* >127 are reserved for prog type specific return codes.
//C 	 *
//C 	 * BPF_LWT_REROUTE: used by BPF_PROG_TYPE_LWT_IN and
//C 	 *    BPF_PROG_TYPE_LWT_XMIT to indicate that skb had been
//C 	 *    changed and should be routed based on its new L3 header.
//C 	 *    (This is an L3 redirect, as opposed to L2 redirect
//C 	 *    represented by BPF_REDIRECT above).
//C 	 */
//C 	BPF_LWT_REROUTE = 128,
//C 	/* BPF_FLOW_DISSECTOR_CONTINUE: used by BPF_PROG_TYPE_FLOW_DISSECTOR
//C 	 *   to indicate that no custom dissection was performed, and
//C 	 *   fallback to standard dissector is requested.
//C 	 */
//C 	BPF_FLOW_DISSECTOR_CONTINUE = 129,
//C };
//C 
//C struct bpf_sock {
//C 	__u32 bound_dev_if;
//C 	__u32 family;
//C 	__u32 type;
//C 	__u32 protocol;
//C 	__u32 mark;
//C 	__u32 priority;
//C 	/* IP address also allows 1 and 2 bytes access */
//C 	__u32 src_ip4;
//C 	__u32 src_ip6[4];
//C 	__u32 src_port;		/* host byte order */
//C 	__be16 dst_port;	/* network byte order */
//C 	__u16 :16;		/* zero padding */
//C 	__u32 dst_ip4;
//C 	__u32 dst_ip6[4];
//C 	__u32 state;
//C 	__s32 rx_queue_mapping;
//C };
//C 
//C struct bpf_tcp_sock {
//C 	__u32 snd_cwnd;		/* Sending congestion window		*/
//C 	__u32 srtt_us;		/* smoothed round trip time << 3 in usecs */
//C 	__u32 rtt_min;
//C 	__u32 snd_ssthresh;	/* Slow start size threshold		*/
//C 	__u32 rcv_nxt;		/* What we want to receive next		*/
//C 	__u32 snd_nxt;		/* Next sequence we send		*/
//C 	__u32 snd_una;		/* First byte we want an ack for	*/
//C 	__u32 mss_cache;	/* Cached effective mss, not including SACKS */
//C 	__u32 ecn_flags;	/* ECN status bits.			*/
//C 	__u32 rate_delivered;	/* saved rate sample: packets delivered */
//C 	__u32 rate_interval_us;	/* saved rate sample: time elapsed */
//C 	__u32 packets_out;	/* Packets which are "in flight"	*/
//C 	__u32 retrans_out;	/* Retransmitted packets out		*/
//C 	__u32 total_retrans;	/* Total retransmits for entire connection */
//C 	__u32 segs_in;		/* RFC4898 tcpEStatsPerfSegsIn
//C 				 * total number of segments in.
//C 				 */
//C 	__u32 data_segs_in;	/* RFC4898 tcpEStatsPerfDataSegsIn
//C 				 * total number of data segments in.
//C 				 */
//C 	__u32 segs_out;		/* RFC4898 tcpEStatsPerfSegsOut
//C 				 * The total number of segments sent.
//C 				 */
//C 	__u32 data_segs_out;	/* RFC4898 tcpEStatsPerfDataSegsOut
//C 				 * total number of data segments sent.
//C 				 */
//C 	__u32 lost_out;		/* Lost packets			*/
//C 	__u32 sacked_out;	/* SACK'd packets			*/
//C 	__u64 bytes_received;	/* RFC4898 tcpEStatsAppHCThruOctetsReceived
//C 				 * sum(delta(rcv_nxt)), or how many bytes
//C 				 * were acked.
//C 				 */
//C 	__u64 bytes_acked;	/* RFC4898 tcpEStatsAppHCThruOctetsAcked
//C 				 * sum(delta(snd_una)), or how many bytes
//C 				 * were acked.
//C 				 */
//C 	__u32 dsack_dups;	/* RFC4898 tcpEStatsStackDSACKDups
//C 				 * total number of DSACK blocks received
//C 				 */
//C 	__u32 delivered;	/* Total data packets delivered incl. rexmits */
//C 	__u32 delivered_ce;	/* Like the above but only ECE marked packets */
//C 	__u32 icsk_retransmits;	/* Number of unrecovered [RTO] timeouts */
//C };
//C 
//C struct bpf_sock_tuple {
//C 	union {
//C 		struct {
//C 			__be32 saddr;
//C 			__be32 daddr;
//C 			__be16 sport;
//C 			__be16 dport;
//C 		} ipv4;
//C 		struct {
//C 			__be32 saddr[4];
//C 			__be32 daddr[4];
//C 			__be16 sport;
//C 			__be16 dport;
//C 		} ipv6;
//C 	};
//C };
//C 
//C /* (Simplified) user return codes for tcx prog type.
//C  * A valid tcx program must return one of these defined values. All other
//C  * return codes are reserved for future use. Must remain compatible with
//C  * their TC_ACT_* counter-parts. For compatibility in behavior, unknown
//C  * return codes are mapped to TCX_NEXT.
//C  */
//C enum tcx_action_base {
//C 	TCX_NEXT	= -1,
//C 	TCX_PASS	= 0,
//C 	TCX_DROP	= 2,
//C 	TCX_REDIRECT	= 7,
//C };
//C 
//C struct bpf_xdp_sock {
//C 	__u32 queue_id;
//C };
//C 
//C #define XDP_PACKET_HEADROOM 256
//C 
//C /* User return codes for XDP prog type.
//C  * A valid XDP program must return one of these defined values. All other
//C  * return codes are reserved for future use. Unknown return codes will
//C  * result in packet drops and a warning via bpf_warn_invalid_xdp_action().
//C  */
//C enum xdp_action {
//C 	XDP_ABORTED = 0,
//C 	XDP_DROP,
//C 	XDP_PASS,
//C 	XDP_TX,
//C 	XDP_REDIRECT,
//C };
//C 
//C /* user accessible metadata for XDP packet hook
//C  * new fields must be added to the end of this structure
//C  */
//C struct xdp_md {
//C 	__u32 data;
//C 	__u32 data_end;
//C 	__u32 data_meta;
//C 	/* Below access go through struct xdp_rxq_info */
//C 	__u32 ingress_ifindex; /* rxq->dev->ifindex */
//C 	__u32 rx_queue_index;  /* rxq->queue_index  */
//C 
//C 	__u32 egress_ifindex;  /* txq->dev->ifindex */
//C };
//C 
//C /* DEVMAP map-value layout
//C  *
//C  * The struct data-layout of map-value is a configuration interface.
//C  * New members can only be added to the end of this structure.
//C  */
//C struct bpf_devmap_val {
//C 	__u32 ifindex;   /* device index */
//C 	union {
//C 		int   fd;  /* prog fd on map write */
//C 		__u32 id;  /* prog id on map read */
//C 	} bpf_prog;
//C };
//C 
//C /* CPUMAP map-value layout
//C  *
//C  * The struct data-layout of map-value is a configuration interface.
//C  * New members can only be added to the end of this structure.
//C  */
//C struct bpf_cpumap_val {
//C 	__u32 qsize;	/* queue size to remote target CPU */
//C 	union {
//C 		int   fd;	/* prog fd on map write */
//C 		__u32 id;	/* prog id on map read */
//C 	} bpf_prog;
//C };
//C 
//C enum sk_action {
//C 	SK_DROP = 0,
//C 	SK_PASS,
//C };
//C 
//C /* user accessible metadata for SK_MSG packet hook, new fields must
//C  * be added to the end of this structure
//C  */
//C struct sk_msg_md {
//C 	__bpf_md_ptr(void *, data);
//C 	__bpf_md_ptr(void *, data_end);
//C 
//C 	__u32 family;
//C 	__u32 remote_ip4;	/* Stored in network byte order */
//C 	__u32 local_ip4;	/* Stored in network byte order */
//C 	__u32 remote_ip6[4];	/* Stored in network byte order */
//C 	__u32 local_ip6[4];	/* Stored in network byte order */
//C 	__u32 remote_port;	/* Stored in network byte order */
//C 	__u32 local_port;	/* stored in host byte order */
//C 	__u32 size;		/* Total size of sk_msg */
//C 
//C 	__bpf_md_ptr(struct bpf_sock *, sk); /* current socket */
//C };
//C 
//C struct sk_reuseport_md {
//C 	/*
//C 	 * Start of directly accessible data. It begins from
//C 	 * the tcp/udp header.
//C 	 */
//C 	__bpf_md_ptr(void *, data);
//C 	/* End of directly accessible data */
//C 	__bpf_md_ptr(void *, data_end);
//C 	/*
//C 	 * Total length of packet (starting from the tcp/udp header).
//C 	 * Note that the directly accessible bytes (data_end - data)
//C 	 * could be less than this "len".  Those bytes could be
//C 	 * indirectly read by a helper "bpf_skb_load_bytes()".
//C 	 */
//C 	__u32 len;
//C 	/*
//C 	 * Eth protocol in the mac header (network byte order). e.g.
//C 	 * ETH_P_IP(0x0800) and ETH_P_IPV6(0x86DD)
//C 	 */
//C 	__u32 eth_protocol;
//C 	__u32 ip_protocol;	/* IP protocol. e.g. IPPROTO_TCP, IPPROTO_UDP */
//C 	__u32 bind_inany;	/* Is sock bound to an INANY address? */
//C 	__u32 hash;		/* A hash of the packet 4 tuples */
//C 	/* When reuse->migrating_sk is NULL, it is selecting a sk for the
//C 	 * new incoming connection request (e.g. selecting a listen sk for
//C 	 * the received SYN in the TCP case).  reuse->sk is one of the sk
//C 	 * in the reuseport group. The bpf prog can use reuse->sk to learn
//C 	 * the local listening ip/port without looking into the skb.
//C 	 *
//C 	 * When reuse->migrating_sk is not NULL, reuse->sk is closed and
//C 	 * reuse->migrating_sk is the socket that needs to be migrated
//C 	 * to another listening socket.  migrating_sk could be a fullsock
//C 	 * sk that is fully established or a reqsk that is in-the-middle
//C 	 * of 3-way handshake.
//C 	 */
//C 	__bpf_md_ptr(struct bpf_sock *, sk);
//C 	__bpf_md_ptr(struct bpf_sock *, migrating_sk);
//C };
//C 
//C #define BPF_TAG_SIZE	8
//C 
//C struct bpf_prog_info {
//C 	__u32 type;
//C 	__u32 id;
//C 	__u8  tag[BPF_TAG_SIZE];
//C 	__u32 jited_prog_len;
//C 	__u32 xlated_prog_len;
//C 	__aligned_u64 jited_prog_insns;
//C 	__aligned_u64 xlated_prog_insns;
//C 	__u64 load_time;	/* ns since boottime */
//C 	__u32 created_by_uid;
//C 	__u32 nr_map_ids;
//C 	__aligned_u64 map_ids;
//C 	char name[BPF_OBJ_NAME_LEN];
//C 	__u32 ifindex;
//C 	__u32 gpl_compatible:1;
//C 	__u32 :31; /* alignment pad */
//C 	__u64 netns_dev;
//C 	__u64 netns_ino;
//C 	__u32 nr_jited_ksyms;
//C 	__u32 nr_jited_func_lens;
//C 	__aligned_u64 jited_ksyms;
//C 	__aligned_u64 jited_func_lens;
//C 	__u32 btf_id;
//C 	__u32 func_info_rec_size;
//C 	__aligned_u64 func_info;
//C 	__u32 nr_func_info;
//C 	__u32 nr_line_info;
//C 	__aligned_u64 line_info;
//C 	__aligned_u64 jited_line_info;
//C 	__u32 nr_jited_line_info;
//C 	__u32 line_info_rec_size;
//C 	__u32 jited_line_info_rec_size;
//C 	__u32 nr_prog_tags;
//C 	__aligned_u64 prog_tags;
//C 	__u64 run_time_ns;
//C 	__u64 run_cnt;
//C 	__u64 recursion_misses;
//C 	__u32 verified_insns;
//C 	__u32 attach_btf_obj_id;
//C 	__u32 attach_btf_id;
//C 	__u32 :32;
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_map_info {
//C 	__u32 type;
//C 	__u32 id;
//C 	__u32 key_size;
//C 	__u32 value_size;
//C 	__u32 max_entries;
//C 	__u32 map_flags;
//C 	char  name[BPF_OBJ_NAME_LEN];
//C 	__u32 ifindex;
//C 	__u32 btf_vmlinux_value_type_id;
//C 	__u64 netns_dev;
//C 	__u64 netns_ino;
//C 	__u32 btf_id;
//C 	__u32 btf_key_type_id;
//C 	__u32 btf_value_type_id;
//C 	__u32 btf_vmlinux_id;
//C 	__u64 map_extra;
//C 	__aligned_u64 hash;
//C 	__u32 hash_size;
//C 	__u32 :32;
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_btf_info {
//C 	__aligned_u64 btf;
//C 	__u32 btf_size;
//C 	__u32 id;
//C 	__aligned_u64 name;
//C 	__u32 name_len;
//C 	__u32 kernel_btf;
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_link_info {
//C 	__u32 type;
//C 	__u32 id;
//C 	__u32 prog_id;
//C 	union {
//C 		struct {
//C 			__aligned_u64 tp_name; /* in/out: tp_name buffer ptr */
//C 			__u32 tp_name_len;     /* in/out: tp_name buffer len */
//C 			__u32 :32;
//C 			__u64 cookie;
//C 		} raw_tracepoint;
//C 		struct {
//C 			__u32 attach_type;
//C 			__u32 target_obj_id; /* prog_id for PROG_EXT, otherwise btf object id */
//C 			__u32 target_btf_id; /* BTF type id inside the object */
//C 			__u32 :32;
//C 			__u64 cookie;
//C 		} tracing;
//C 		struct {
//C 			__u64 cgroup_id;
//C 			__u32 attach_type;
//C 		} cgroup;
//C 		struct {
//C 			__aligned_u64 target_name; /* in/out: target_name buffer ptr */
//C 			__u32 target_name_len;	   /* in/out: target_name buffer len */
//C 
//C 			/* If the iter specific field is 32 bits, it can be put
//C 			 * in the first or second union. Otherwise it should be
//C 			 * put in the second union.
//C 			 */
//C 			union {
//C 				struct {
//C 					__u32 map_id;
//C 				} map;
//C 			};
//C 			union {
//C 				struct {
//C 					__u64 cgroup_id;
//C 					__u32 order;
//C 				} cgroup;
//C 				struct {
//C 					__u32 tid;
//C 					__u32 pid;
//C 				} task;
//C 			};
//C 		} iter;
//C 		struct  {
//C 			__u32 netns_ino;
//C 			__u32 attach_type;
//C 		} netns;
//C 		struct {
//C 			__u32 ifindex;
//C 		} xdp;
//C 		struct {
//C 			__u32 map_id;
//C 		} struct_ops;
//C 		struct {
//C 			__u32 pf;
//C 			__u32 hooknum;
//C 			__s32 priority;
//C 			__u32 flags;
//C 		} netfilter;
//C 		struct {
//C 			__aligned_u64 addrs;
//C 			__u32 count; /* in/out: kprobe_multi function count */
//C 			__u32 flags;
//C 			__u64 missed;
//C 			__aligned_u64 cookies;
//C 		} kprobe_multi;
//C 		struct {
//C 			__aligned_u64 path;
//C 			__aligned_u64 offsets;
//C 			__aligned_u64 ref_ctr_offsets;
//C 			__aligned_u64 cookies;
//C 			__u32 path_size; /* in/out: real path size on success, including zero byte */
//C 			__u32 count; /* in/out: uprobe_multi offsets/ref_ctr_offsets/cookies count */
//C 			__u32 flags;
//C 			__u32 pid;
//C 		} uprobe_multi;
//C 		struct {
//C 			__u32 attach_type;
//C 			__u32 count; /* in/out: tracing_multi target count */
//C 			__u32 btf_obj_id;
//C 			__u32 :32;
//C 			__aligned_u64 ids;
//C 			__aligned_u64 addrs;
//C 			__aligned_u64 cookies;
//C 		} tracing_multi;
//C 		struct {
//C 			__u32 type; /* enum bpf_perf_event_type */
//C 			__u32 :32;
//C 			union {
//C 				struct {
//C 					__aligned_u64 file_name; /* in/out */
//C 					__u32 name_len;
//C 					__u32 offset; /* offset from file_name */
//C 					__u64 cookie;
//C 					__u64 ref_ctr_offset;
//C 				} uprobe; /* BPF_PERF_EVENT_UPROBE, BPF_PERF_EVENT_URETPROBE */
//C 				struct {
//C 					__aligned_u64 func_name; /* in/out */
//C 					__u32 name_len;
//C 					__u32 offset; /* offset from func_name */
//C 					__u64 addr;
//C 					__u64 missed;
//C 					__u64 cookie;
//C 				} kprobe; /* BPF_PERF_EVENT_KPROBE, BPF_PERF_EVENT_KRETPROBE */
//C 				struct {
//C 					__aligned_u64 tp_name;   /* in/out */
//C 					__u32 name_len;
//C 					__u32 :32;
//C 					__u64 cookie;
//C 				} tracepoint; /* BPF_PERF_EVENT_TRACEPOINT */
//C 				struct {
//C 					__u64 config;
//C 					__u32 type;
//C 					__u32 :32;
//C 					__u64 cookie;
//C 				} event; /* BPF_PERF_EVENT_EVENT */
//C 			};
//C 		} perf_event;
//C 		struct {
//C 			__u32 ifindex;
//C 			__u32 attach_type;
//C 		} tcx;
//C 		struct {
//C 			__u32 ifindex;
//C 			__u32 attach_type;
//C 		} netkit;
//C 		struct {
//C 			__u32 map_id;
//C 			__u32 attach_type;
//C 		} sockmap;
//C 	};
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_token_info {
//C 	__u64 allowed_cmds;
//C 	__u64 allowed_maps;
//C 	__u64 allowed_progs;
//C 	__u64 allowed_attachs;
//C } __attribute__((aligned(8)));
//C 
//C /* User bpf_sock_addr struct to access socket fields and sockaddr struct passed
//C  * by user and intended to be used by socket (e.g. to bind to, depends on
//C  * attach type).
//C  */
//C struct bpf_sock_addr {
//C 	__u32 user_family;	/* Allows 4-byte read, but no write. */
//C 	__u32 user_ip4;		/* Allows 1,2,4-byte read and 4-byte write.
//C 				 * Stored in network byte order.
//C 				 */
//C 	__u32 user_ip6[4];	/* Allows 1,2,4,8-byte read and 4,8-byte write.
//C 				 * Stored in network byte order.
//C 				 */
//C 	__u32 user_port;	/* Allows 1,2,4-byte read and 4-byte write.
//C 				 * Stored in network byte order
//C 				 */
//C 	__u32 family;		/* Allows 4-byte read, but no write */
//C 	__u32 type;		/* Allows 4-byte read, but no write */
//C 	__u32 protocol;		/* Allows 4-byte read, but no write */
//C 	__u32 msg_src_ip4;	/* Allows 1,2,4-byte read and 4-byte write.
//C 				 * Stored in network byte order.
//C 				 */
//C 	__u32 msg_src_ip6[4];	/* Allows 1,2,4,8-byte read and 4,8-byte write.
//C 				 * Stored in network byte order.
//C 				 */
//C 	__bpf_md_ptr(struct bpf_sock *, sk);
//C };
//C 
//C /* User bpf_sock_ops struct to access socket values and specify request ops
//C  * and their replies.
//C  * Some of this fields are in network (bigendian) byte order and may need
//C  * to be converted before use (bpf_ntohl() defined in samples/bpf/bpf_endian.h).
//C  * New fields can only be added at the end of this structure
//C  */
//C struct bpf_sock_ops {
//C 	__u32 op;
//C 	union {
//C 		__u32 args[4];		/* Optionally passed to bpf program */
//C 		__u32 reply;		/* Returned by bpf program	    */
//C 		__u32 replylong[4];	/* Optionally returned by bpf prog  */
//C 	};
//C 	__u32 family;
//C 	__u32 remote_ip4;	/* Stored in network byte order */
//C 	__u32 local_ip4;	/* Stored in network byte order */
//C 	__u32 remote_ip6[4];	/* Stored in network byte order */
//C 	__u32 local_ip6[4];	/* Stored in network byte order */
//C 	__u32 remote_port;	/* Stored in network byte order */
//C 	__u32 local_port;	/* stored in host byte order */
//C 	__u32 is_fullsock;	/* Some TCP fields are only valid if
//C 				 * there is a full socket. If not, the
//C 				 * fields read as zero.
//C 				 */
//C 	__u32 snd_cwnd;
//C 	__u32 srtt_us;		/* Averaged RTT << 3 in usecs */
//C 	__u32 bpf_sock_ops_cb_flags; /* flags defined in uapi/linux/tcp.h */
//C 	__u32 state;
//C 	__u32 rtt_min;
//C 	__u32 snd_ssthresh;
//C 	__u32 rcv_nxt;
//C 	__u32 snd_nxt;
//C 	__u32 snd_una;
//C 	__u32 mss_cache;
//C 	__u32 ecn_flags;
//C 	__u32 rate_delivered;
//C 	__u32 rate_interval_us;
//C 	__u32 packets_out;
//C 	__u32 retrans_out;
//C 	__u32 total_retrans;
//C 	__u32 segs_in;
//C 	__u32 data_segs_in;
//C 	__u32 segs_out;
//C 	__u32 data_segs_out;
//C 	__u32 lost_out;
//C 	__u32 sacked_out;
//C 	__u32 sk_txhash;
//C 	__u64 bytes_received;
//C 	__u64 bytes_acked;
//C 	__bpf_md_ptr(struct bpf_sock *, sk);
//C 	/* [skb_data, skb_data_end) covers the whole TCP header.
//C 	 *
//C 	 * BPF_SOCK_OPS_PARSE_HDR_OPT_CB: The packet received
//C 	 * BPF_SOCK_OPS_HDR_OPT_LEN_CB:   Not useful because the
//C 	 *                                header has not been written.
//C 	 * BPF_SOCK_OPS_WRITE_HDR_OPT_CB: The header and options have
//C 	 *				  been written so far.
//C 	 * BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB:  The SYNACK that concludes
//C 	 *					the 3WHS.
//C 	 * BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: The ACK that concludes
//C 	 *					the 3WHS.
//C 	 *
//C 	 * bpf_load_hdr_opt() can also be used to read a particular option.
//C 	 */
//C 	__bpf_md_ptr(void *, skb_data);
//C 	__bpf_md_ptr(void *, skb_data_end);
//C 	__u32 skb_len;		/* The total length of a packet.
//C 				 * It includes the header, options,
//C 				 * and payload.
//C 				 */
//C 	__u32 skb_tcp_flags;	/* tcp_flags of the header.  It provides
//C 				 * an easy way to check for tcp_flags
//C 				 * without parsing skb_data.
//C 				 *
//C 				 * In particular, the skb_tcp_flags
//C 				 * will still be available in
//C 				 * BPF_SOCK_OPS_HDR_OPT_LEN even though
//C 				 * the outgoing header has not
//C 				 * been written yet.
//C 				 */
//C 	__u64 skb_hwtstamp;
//C };
//C 
//C /* Definitions for bpf_sock_ops_cb_flags */
//C enum {
//C 	BPF_SOCK_OPS_RTO_CB_FLAG	= (1<<0),
//C 	BPF_SOCK_OPS_RETRANS_CB_FLAG	= (1<<1),
//C 	BPF_SOCK_OPS_STATE_CB_FLAG	= (1<<2),
//C 	BPF_SOCK_OPS_RTT_CB_FLAG	= (1<<3),
//C 	/* Call bpf for all received TCP headers.  The bpf prog will be
//C 	 * called under sock_ops->op == BPF_SOCK_OPS_PARSE_HDR_OPT_CB
//C 	 *
//C 	 * Please refer to the comment in BPF_SOCK_OPS_PARSE_HDR_OPT_CB
//C 	 * for the header option related helpers that will be useful
//C 	 * to the bpf programs.
//C 	 *
//C 	 * It could be used at the client/active side (i.e. connect() side)
//C 	 * when the server told it that the server was in syncookie
//C 	 * mode and required the active side to resend the bpf-written
//C 	 * options.  The active side can keep writing the bpf-options until
//C 	 * it received a valid packet from the server side to confirm
//C 	 * the earlier packet (and options) has been received.  The later
//C 	 * example patch is using it like this at the active side when the
//C 	 * server is in syncookie mode.
//C 	 *
//C 	 * The bpf prog will usually turn this off in the common cases.
//C 	 */
//C 	BPF_SOCK_OPS_PARSE_ALL_HDR_OPT_CB_FLAG	= (1<<4),
//C 	/* Call bpf when kernel has received a header option that
//C 	 * the kernel cannot handle.  The bpf prog will be called under
//C 	 * sock_ops->op == BPF_SOCK_OPS_PARSE_HDR_OPT_CB.
//C 	 *
//C 	 * Please refer to the comment in BPF_SOCK_OPS_PARSE_HDR_OPT_CB
//C 	 * for the header option related helpers that will be useful
//C 	 * to the bpf programs.
//C 	 */
//C 	BPF_SOCK_OPS_PARSE_UNKNOWN_HDR_OPT_CB_FLAG = (1<<5),
//C 	/* Call bpf when the kernel is writing header options for the
//C 	 * outgoing packet.  The bpf prog will first be called
//C 	 * to reserve space in a skb under
//C 	 * sock_ops->op == BPF_SOCK_OPS_HDR_OPT_LEN_CB.  Then
//C 	 * the bpf prog will be called to write the header option(s)
//C 	 * under sock_ops->op == BPF_SOCK_OPS_WRITE_HDR_OPT_CB.
//C 	 *
//C 	 * Please refer to the comment in BPF_SOCK_OPS_HDR_OPT_LEN_CB
//C 	 * and BPF_SOCK_OPS_WRITE_HDR_OPT_CB for the header option
//C 	 * related helpers that will be useful to the bpf programs.
//C 	 *
//C 	 * The kernel gets its chance to reserve space and write
//C 	 * options first before the BPF program does.
//C 	 */
//C 	BPF_SOCK_OPS_WRITE_HDR_OPT_CB_FLAG = (1<<6),
//C /* Mask of all currently supported cb flags */
//C 	BPF_SOCK_OPS_ALL_CB_FLAGS       = 0x7F,
//C };
//C 
//C enum {
//C 	SK_BPF_CB_TX_TIMESTAMPING	= 1<<0,
//C 	SK_BPF_CB_MASK			= (SK_BPF_CB_TX_TIMESTAMPING - 1) |
//C 					   SK_BPF_CB_TX_TIMESTAMPING
//C };
//C 
//C /* List of known BPF sock_ops operators.
//C  * New entries can only be added at the end
//C  */
//C enum {
//C 	BPF_SOCK_OPS_VOID,
//C 	BPF_SOCK_OPS_TIMEOUT_INIT,	/* Should return SYN-RTO value to use or
//C 					 * -1 if default value should be used
//C 					 */
//C 	BPF_SOCK_OPS_RWND_INIT,		/* Should return initial advertized
//C 					 * window (in packets) or -1 if default
//C 					 * value should be used
//C 					 */
//C 	BPF_SOCK_OPS_TCP_CONNECT_CB,	/* Calls BPF program right before an
//C 					 * active connection is initialized
//C 					 */
//C 	BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB,	/* Calls BPF program when an
//C 						 * active connection is
//C 						 * established
//C 						 */
//C 	BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB,	/* Calls BPF program when a
//C 						 * passive connection is
//C 						 * established
//C 						 */
//C 	BPF_SOCK_OPS_NEEDS_ECN,		/* If connection's congestion control
//C 					 * needs ECN
//C 					 */
//C 	BPF_SOCK_OPS_BASE_RTT,		/* Get base RTT. The correct value is
//C 					 * based on the path and may be
//C 					 * dependent on the congestion control
//C 					 * algorithm. In general it indicates
//C 					 * a congestion threshold. RTTs above
//C 					 * this indicate congestion
//C 					 */
//C 	BPF_SOCK_OPS_RTO_CB,		/* Called when an RTO has triggered.
//C 					 * Arg1: value of icsk_retransmits
//C 					 * Arg2: value of icsk_rto
//C 					 * Arg3: whether RTO has expired
//C 					 */
//C 	BPF_SOCK_OPS_RETRANS_CB,	/* Called when skb is retransmitted.
//C 					 * Arg1: sequence number of 1st byte
//C 					 * Arg2: # segments
//C 					 * Arg3: return value of
//C 					 *       tcp_transmit_skb (0 => success)
//C 					 */
//C 	BPF_SOCK_OPS_STATE_CB,		/* Called when TCP changes state.
//C 					 * Arg1: old_state
//C 					 * Arg2: new_state
//C 					 */
//C 	BPF_SOCK_OPS_TCP_LISTEN_CB,	/* Called on listen(2), right after
//C 					 * socket transition to LISTEN state.
//C 					 */
//C 	BPF_SOCK_OPS_RTT_CB,		/* Called on every RTT.
//C 					 * Arg1: measured RTT input (mrtt)
//C 					 * Arg2: updated srtt
//C 					 */
//C 	BPF_SOCK_OPS_PARSE_HDR_OPT_CB,	/* Parse the header option.
//C 					 * It will be called to handle
//C 					 * the packets received at
//C 					 * an already established
//C 					 * connection.
//C 					 *
//C 					 * sock_ops->skb_data:
//C 					 * Referring to the received skb.
//C 					 * It covers the TCP header only.
//C 					 *
//C 					 * bpf_load_hdr_opt() can also
//C 					 * be used to search for a
//C 					 * particular option.
//C 					 */
//C 	BPF_SOCK_OPS_HDR_OPT_LEN_CB,	/* Reserve space for writing the
//C 					 * header option later in
//C 					 * BPF_SOCK_OPS_WRITE_HDR_OPT_CB.
//C 					 * Arg1: bool want_cookie. (in
//C 					 *       writing SYNACK only)
//C 					 *
//C 					 * sock_ops->skb_data:
//C 					 * Not available because no header has
//C 					 * been	written yet.
//C 					 *
//C 					 * sock_ops->skb_tcp_flags:
//C 					 * The tcp_flags of the
//C 					 * outgoing skb. (e.g. SYN, ACK, FIN).
//C 					 *
//C 					 * bpf_reserve_hdr_opt() should
//C 					 * be used to reserve space.
//C 					 */
//C 	BPF_SOCK_OPS_WRITE_HDR_OPT_CB,	/* Write the header options
//C 					 * Arg1: bool want_cookie. (in
//C 					 *       writing SYNACK only)
//C 					 *
//C 					 * sock_ops->skb_data:
//C 					 * Referring to the outgoing skb.
//C 					 * It covers the TCP header
//C 					 * that has already been written
//C 					 * by the kernel and the
//C 					 * earlier bpf-progs.
//C 					 *
//C 					 * sock_ops->skb_tcp_flags:
//C 					 * The tcp_flags of the outgoing
//C 					 * skb. (e.g. SYN, ACK, FIN).
//C 					 *
//C 					 * bpf_store_hdr_opt() should
//C 					 * be used to write the
//C 					 * option.
//C 					 *
//C 					 * bpf_load_hdr_opt() can also
//C 					 * be used to search for a
//C 					 * particular option that
//C 					 * has already been written
//C 					 * by the kernel or the
//C 					 * earlier bpf-progs.
//C 					 */
//C 	BPF_SOCK_OPS_TSTAMP_SCHED_CB,	/* Called when skb is passing
//C 					 * through dev layer when
//C 					 * SK_BPF_CB_TX_TIMESTAMPING
//C 					 * feature is on.
//C 					 */
//C 	BPF_SOCK_OPS_TSTAMP_SND_SW_CB,	/* Called when skb is about to send
//C 					 * to the nic when SK_BPF_CB_TX_TIMESTAMPING
//C 					 * feature is on.
//C 					 */
//C 	BPF_SOCK_OPS_TSTAMP_SND_HW_CB,	/* Called in hardware phase when
//C 					 * SK_BPF_CB_TX_TIMESTAMPING feature
//C 					 * is on.
//C 					 */
//C 	BPF_SOCK_OPS_TSTAMP_ACK_CB,	/* Called when all the skbs in the
//C 					 * same sendmsg call are acked
//C 					 * when SK_BPF_CB_TX_TIMESTAMPING
//C 					 * feature is on.
//C 					 */
//C 	BPF_SOCK_OPS_TSTAMP_SENDMSG_CB,	/* Called when every sendmsg syscall
//C 					 * is triggered. It's used to correlate
//C 					 * sendmsg timestamp with corresponding
//C 					 * tskey.
//C 					 */
//C };
//C 
//C /* List of TCP states. There is a build check in net/ipv4/tcp.c to detect
//C  * changes between the TCP and BPF versions. Ideally this should never happen.
//C  * If it does, we need to add code to convert them before calling
//C  * the BPF sock_ops function.
//C  */
//C enum {
//C 	BPF_TCP_ESTABLISHED = 1,
//C 	BPF_TCP_SYN_SENT,
//C 	BPF_TCP_SYN_RECV,
//C 	BPF_TCP_FIN_WAIT1,
//C 	BPF_TCP_FIN_WAIT2,
//C 	BPF_TCP_TIME_WAIT,
//C 	BPF_TCP_CLOSE,
//C 	BPF_TCP_CLOSE_WAIT,
//C 	BPF_TCP_LAST_ACK,
//C 	BPF_TCP_LISTEN,
//C 	BPF_TCP_CLOSING,	/* Now a valid state */
//C 	BPF_TCP_NEW_SYN_RECV,
//C 	BPF_TCP_BOUND_INACTIVE,
//C 
//C 	BPF_TCP_MAX_STATES	/* Leave at the end! */
//C };
//C 
//C enum {
//C 	TCP_BPF_IW		= 1001,	/* Set TCP initial congestion window */
//C 	TCP_BPF_SNDCWND_CLAMP	= 1002,	/* Set sndcwnd_clamp */
//C 	TCP_BPF_DELACK_MAX	= 1003, /* Max delay ack in usecs */
//C 	TCP_BPF_RTO_MIN		= 1004, /* Min delay ack in usecs */
//C 	/* Copy the SYN pkt to optval
//C 	 *
//C 	 * BPF_PROG_TYPE_SOCK_OPS only.  It is similar to the
//C 	 * bpf_getsockopt(TCP_SAVED_SYN) but it does not limit
//C 	 * to only getting from the saved_syn.  It can either get the
//C 	 * syn packet from:
//C 	 *
//C 	 * 1. the just-received SYN packet (only available when writing the
//C 	 *    SYNACK).  It will be useful when it is not necessary to
//C 	 *    save the SYN packet for latter use.  It is also the only way
//C 	 *    to get the SYN during syncookie mode because the syn
//C 	 *    packet cannot be saved during syncookie.
//C 	 *
//C 	 * OR
//C 	 *
//C 	 * 2. the earlier saved syn which was done by
//C 	 *    bpf_setsockopt(TCP_SAVE_SYN).
//C 	 *
//C 	 * The bpf_getsockopt(TCP_BPF_SYN*) option will hide where the
//C 	 * SYN packet is obtained.
//C 	 *
//C 	 * If the bpf-prog does not need the IP[46] header,  the
//C 	 * bpf-prog can avoid parsing the IP header by using
//C 	 * TCP_BPF_SYN.  Otherwise, the bpf-prog can get both
//C 	 * IP[46] and TCP header by using TCP_BPF_SYN_IP.
//C 	 *
//C 	 *      >0: Total number of bytes copied
//C 	 * -ENOSPC: Not enough space in optval. Only optlen number of
//C 	 *          bytes is copied.
//C 	 * -ENOENT: The SYN skb is not available now and the earlier SYN pkt
//C 	 *	    is not saved by setsockopt(TCP_SAVE_SYN).
//C 	 */
//C 	TCP_BPF_SYN		= 1005, /* Copy the TCP header */
//C 	TCP_BPF_SYN_IP		= 1006, /* Copy the IP[46] and TCP header */
//C 	TCP_BPF_SYN_MAC         = 1007, /* Copy the MAC, IP[46], and TCP header */
//C 	TCP_BPF_SOCK_OPS_CB_FLAGS = 1008, /* Get or Set TCP sock ops flags */
//C 	SK_BPF_CB_FLAGS		= 1009, /* Get or set sock ops flags in socket */
//C 	SK_BPF_BYPASS_PROT_MEM	= 1010, /* Get or Set sk->sk_bypass_prot_mem */
//C 
//C };
//C 
//C enum {
//C 	BPF_LOAD_HDR_OPT_TCP_SYN = (1ULL << 0),
//C };
//C 
//C /* args[0] value during BPF_SOCK_OPS_HDR_OPT_LEN_CB and
//C  * BPF_SOCK_OPS_WRITE_HDR_OPT_CB.
//C  */
//C enum {
//C 	BPF_WRITE_HDR_TCP_CURRENT_MSS = 1,	/* Kernel is finding the
//C 						 * total option spaces
//C 						 * required for an established
//C 						 * sk in order to calculate the
//C 						 * MSS.  No skb is actually
//C 						 * sent.
//C 						 */
//C 	BPF_WRITE_HDR_TCP_SYNACK_COOKIE = 2,	/* Kernel is in syncookie mode
//C 						 * when sending a SYN.
//C 						 */
//C };
//C 
//C struct bpf_perf_event_value {
//C 	__u64 counter;
//C 	__u64 enabled;
//C 	__u64 running;
//C };
//C 
//C enum {
//C 	BPF_DEVCG_ACC_MKNOD	= (1ULL << 0),
//C 	BPF_DEVCG_ACC_READ	= (1ULL << 1),
//C 	BPF_DEVCG_ACC_WRITE	= (1ULL << 2),
//C };
//C 
//C enum {
//C 	BPF_DEVCG_DEV_BLOCK	= (1ULL << 0),
//C 	BPF_DEVCG_DEV_CHAR	= (1ULL << 1),
//C };
//C 
//C struct bpf_cgroup_dev_ctx {
//C 	/* access_type encoded as (BPF_DEVCG_ACC_* << 16) | BPF_DEVCG_DEV_* */
//C 	__u32 access_type;
//C 	__u32 major;
//C 	__u32 minor;
//C };
//C 
//C struct bpf_raw_tracepoint_args {
//C 	__u64 args[0];
//C };
//C 
//C /* DIRECT:  Skip the FIB rules and go to FIB table associated with device
//C  * OUTPUT:  Do lookup from egress perspective; default is ingress
//C  */
//C enum {
//C 	BPF_FIB_LOOKUP_DIRECT  = (1U << 0),
//C 	BPF_FIB_LOOKUP_OUTPUT  = (1U << 1),
//C 	BPF_FIB_LOOKUP_SKIP_NEIGH = (1U << 2),
//C 	BPF_FIB_LOOKUP_TBID    = (1U << 3),
//C 	BPF_FIB_LOOKUP_SRC     = (1U << 4),
//C 	BPF_FIB_LOOKUP_MARK    = (1U << 5),
//C 	BPF_FIB_LOOKUP_VLAN    = (1U << 6),
//C 	BPF_FIB_LOOKUP_VLAN_INPUT = (1U << 7),
//C };
//C 
//C enum {
//C 	BPF_FIB_LKUP_RET_SUCCESS,      /* lookup successful */
//C 	BPF_FIB_LKUP_RET_BLACKHOLE,    /* dest is blackholed; can be dropped */
//C 	BPF_FIB_LKUP_RET_UNREACHABLE,  /* dest is unreachable; can be dropped */
//C 	BPF_FIB_LKUP_RET_PROHIBIT,     /* dest not allowed; can be dropped */
//C 	BPF_FIB_LKUP_RET_NOT_FWDED,    /* packet is not forwarded */
//C 	BPF_FIB_LKUP_RET_FWD_DISABLED, /* fwding is not enabled on ingress */
//C 	BPF_FIB_LKUP_RET_UNSUPP_LWT,   /* fwd requires encapsulation */
//C 	BPF_FIB_LKUP_RET_NO_NEIGH,     /* no neighbor entry for nh */
//C 	BPF_FIB_LKUP_RET_FRAG_NEEDED,  /* fragmentation required to fwd */
//C 	BPF_FIB_LKUP_RET_NO_SRC_ADDR,  /* failed to derive IP src addr */
//C 	BPF_FIB_LKUP_RET_VLAN_FAILURE, /* VLAN egress, parent unresolvable */
//C };
//C 
//C struct bpf_fib_lookup {
//C 	/* input:  network family for lookup (AF_INET, AF_INET6)
//C 	 * output: network family of egress nexthop
//C 	 */
//C 	__u8	family;
//C 
//C 	/* set if lookup is to consider L4 data - e.g., FIB rules */
//C 	__u8	l4_protocol;
//C 	__be16	sport;
//C 	__be16	dport;
//C 
//C 	union {	/* used for MTU check */
//C 		/* input to lookup */
//C 		__u16	tot_len; /* L3 length from network hdr (iph->tot_len) */
//C 
//C 		/* output: MTU value */
//C 		__u16	mtu_result;
//C 	} __attribute__((packed, aligned(2)));
//C 	/* input: L3 device index for lookup
//C 	 * output: device index from FIB lookup
//C 	 */
//C 	__u32	ifindex;
//C 
//C 	union {
//C 		/* inputs to lookup */
//C 		__u8	tos;		/* AF_INET  */
//C 		__be32	flowinfo;	/* AF_INET6, flow_label + priority */
//C 
//C 		/* output: metric of fib result (IPv4/IPv6 only) */
//C 		__u32	rt_metric;
//C 	};
//C 
//C 	/* input: source address to consider for lookup
//C 	 * output: source address result from lookup
//C 	 */
//C 	union {
//C 		__be32		ipv4_src;
//C 		__u32		ipv6_src[4];  /* in6_addr; network order */
//C 	};
//C 
//C 	/* input to bpf_fib_lookup, ipv{4,6}_dst is destination address in
//C 	 * network header. output: bpf_fib_lookup sets to gateway address
//C 	 * if FIB lookup returns gateway route
//C 	 */
//C 	union {
//C 		__be32		ipv4_dst;
//C 		__u32		ipv6_dst[4];  /* in6_addr; network order */
//C 	};
//C 
//C 	union {
//C 		struct {
//C 			/*
//C 			 * output with BPF_FIB_LOOKUP_VLAN: set from the
//C 			 * resolved egress VLAN device (see the flag); zeroed
//C 			 * on other successful lookups. input with
//C 			 * BPF_FIB_LOOKUP_VLAN_INPUT: the VLAN tag to scope
//C 			 * the lookup by.
//C 			 */
//C 			__be16	h_vlan_proto;
//C 			__be16	h_vlan_TCI;
//C 		};
//C 		/* input: when accompanied with the
//C 		 * 'BPF_FIB_LOOKUP_DIRECT | BPF_FIB_LOOKUP_TBID` flags, a
//C 		 * specific routing table to use for the fib lookup.
//C 		 */
//C 		__u32	tbid;
//C 	};
//C 
//C 	union {
//C 		/* input */
//C 		struct {
//C 			__u32	mark;   /* policy routing */
//C 			/* 2 4-byte holes for input */
//C 		};
//C 
//C 		/* output: source and dest mac */
//C 		struct {
//C 			__u8	smac[6];	/* ETH_ALEN */
//C 			__u8	dmac[6];	/* ETH_ALEN */
//C 		};
//C 	};
//C };
//C 
//C struct bpf_redir_neigh {
//C 	/* network family for lookup (AF_INET, AF_INET6) */
//C 	__u32 nh_family;
//C 	/* network address of nexthop; skips fib lookup to find gateway */
//C 	union {
//C 		__be32		ipv4_nh;
//C 		__u32		ipv6_nh[4];  /* in6_addr; network order */
//C 	};
//C };
//C 
//C /* bpf_check_mtu flags*/
//C enum  bpf_check_mtu_flags {
//C 	BPF_MTU_CHK_SEGS  = (1U << 0),
//C };
//C 
//C enum bpf_check_mtu_ret {
//C 	BPF_MTU_CHK_RET_SUCCESS,      /* check and lookup successful */
//C 	BPF_MTU_CHK_RET_FRAG_NEEDED,  /* fragmentation required to fwd */
//C 	BPF_MTU_CHK_RET_SEGS_TOOBIG,  /* GSO re-segmentation needed to fwd */
//C };
//C 
//C enum bpf_task_fd_type {
//C 	BPF_FD_TYPE_RAW_TRACEPOINT,	/* tp name */
//C 	BPF_FD_TYPE_TRACEPOINT,		/* tp name */
//C 	BPF_FD_TYPE_KPROBE,		/* (symbol + offset) or addr */
//C 	BPF_FD_TYPE_KRETPROBE,		/* (symbol + offset) or addr */
//C 	BPF_FD_TYPE_UPROBE,		/* filename + offset */
//C 	BPF_FD_TYPE_URETPROBE,		/* filename + offset */
//C };
//C 
//C enum {
//C 	BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG		= (1U << 0),
//C 	BPF_FLOW_DISSECTOR_F_STOP_AT_FLOW_LABEL		= (1U << 1),
//C 	BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP		= (1U << 2),
//C };
//C 
//C struct bpf_flow_keys {
//C 	__u16	nhoff;
//C 	__u16	thoff;
//C 	__u16	addr_proto;			/* ETH_P_* of valid addrs */
//C 	__u8	is_frag;
//C 	__u8	is_first_frag;
//C 	__u8	is_encap;
//C 	__u8	ip_proto;
//C 	__be16	n_proto;
//C 	__be16	sport;
//C 	__be16	dport;
//C 	union {
//C 		struct {
//C 			__be32	ipv4_src;
//C 			__be32	ipv4_dst;
//C 		};
//C 		struct {
//C 			__u32	ipv6_src[4];	/* in6_addr; network order */
//C 			__u32	ipv6_dst[4];	/* in6_addr; network order */
//C 		};
//C 	};
//C 	__u32	flags;
//C 	__be32	flow_label;
//C };
//C 
//C struct bpf_func_info {
//C 	__u32	insn_off;
//C 	__u32	type_id;
//C };
//C 
//C #define BPF_LINE_INFO_LINE_NUM(line_col)	((line_col) >> 10)
//C #define BPF_LINE_INFO_LINE_COL(line_col)	((line_col) & 0x3ff)
//C 
//C struct bpf_line_info {
//C 	__u32	insn_off;
//C 	__u32	file_name_off;
//C 	__u32	line_off;
//C 	__u32	line_col;
//C };
//C 
//C struct bpf_spin_lock {
//C 	__u32	val;
//C };
//C 
//C struct bpf_timer {
//C 	__u64 __opaque[2];
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_task_work {
//C 	__u64 __opaque;
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_wq {
//C 	__u64 __opaque[2];
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_dynptr {
//C 	__u64 __opaque[2];
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_list_head {
//C 	__u64 __opaque[2];
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_list_node {
//C 	__u64 __opaque[3];
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_rb_root {
//C 	__u64 __opaque[2];
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_rb_node {
//C 	__u64 __opaque[4];
//C } __attribute__((aligned(8)));
//C 
//C struct bpf_refcount {
//C 	__u32 __opaque[1];
//C } __attribute__((aligned(4)));
//C 
//C struct bpf_sysctl {
//C 	__u32	write;		/* Sysctl is being read (= 0) or written (= 1).
//C 				 * Allows 1,2,4-byte read, but no write.
//C 				 */
//C 	__u32	file_pos;	/* Sysctl file position to read from, write to.
//C 				 * Allows 1,2,4-byte read an 4-byte write.
//C 				 */
//C };
//C 
//C struct bpf_sockopt {
//C 	__bpf_md_ptr(struct bpf_sock *, sk);
//C 	__bpf_md_ptr(void *, optval);
//C 	__bpf_md_ptr(void *, optval_end);
//C 
//C 	__s32	level;
//C 	__s32	optname;
//C 	__s32	optlen;
//C 	__s32	retval;
//C };
//C 
//C struct bpf_pidns_info {
//C 	__u32 pid;
//C 	__u32 tgid;
//C };
//C 
//C /* User accessible data for SK_LOOKUP programs. Add new fields at the end. */
//C struct bpf_sk_lookup {
//C 	union {
//C 		__bpf_md_ptr(struct bpf_sock *, sk); /* Selected socket */
//C 		__u64 cookie; /* Non-zero if socket was selected in PROG_TEST_RUN */
//C 	};
//C 
//C 	__u32 family;		/* Protocol family (AF_INET, AF_INET6) */
//C 	__u32 protocol;		/* IP protocol (IPPROTO_TCP, IPPROTO_UDP) */
//C 	__u32 remote_ip4;	/* Network byte order */
//C 	__u32 remote_ip6[4];	/* Network byte order */
//C 	__be16 remote_port;	/* Network byte order */
//C 	__u16 :16;		/* Zero padding */
//C 	__u32 local_ip4;	/* Network byte order */
//C 	__u32 local_ip6[4];	/* Network byte order */
//C 	__u32 local_port;	/* Host byte order */
//C 	__u32 ingress_ifindex;		/* The arriving interface. Determined by inet_iif. */
//C };
//C 
//C /*
//C  * struct btf_ptr is used for typed pointer representation; the
//C  * type id is used to render the pointer data as the appropriate type
//C  * via the bpf_snprintf_btf() helper described above.  A flags field -
//C  * potentially to specify additional details about the BTF pointer
//C  * (rather than its mode of display) - is included for future use.
//C  * Display flags - BTF_F_* - are passed to bpf_snprintf_btf separately.
//C  */
//C struct btf_ptr {
//C 	void *ptr;
//C 	__u32 type_id;
//C 	__u32 flags;		/* BTF ptr flags; unused at present. */
//C };
//C 
//C /*
//C  * Flags to control bpf_snprintf_btf() behaviour.
//C  *     - BTF_F_COMPACT: no formatting around type information
//C  *     - BTF_F_NONAME: no struct/union member names/types
//C  *     - BTF_F_PTR_RAW: show raw (unobfuscated) pointer values;
//C  *       equivalent to %px.
//C  *     - BTF_F_ZERO: show zero-valued struct/union members; they
//C  *       are not displayed by default
//C  */
//C enum {
//C 	BTF_F_COMPACT	=	(1ULL << 0),
//C 	BTF_F_NONAME	=	(1ULL << 1),
//C 	BTF_F_PTR_RAW	=	(1ULL << 2),
//C 	BTF_F_ZERO	=	(1ULL << 3),
//C };
//C 
//C /* bpf_core_relo_kind encodes which aspect of captured field/type/enum value
//C  * has to be adjusted by relocations. It is emitted by llvm and passed to
//C  * libbpf and later to the kernel.
//C  */
//C enum bpf_core_relo_kind {
//C 	BPF_CORE_FIELD_BYTE_OFFSET = 0,      /* field byte offset */
//C 	BPF_CORE_FIELD_BYTE_SIZE = 1,        /* field size in bytes */
//C 	BPF_CORE_FIELD_EXISTS = 2,           /* field existence in target kernel */
//C 	BPF_CORE_FIELD_SIGNED = 3,           /* field signedness (0 - unsigned, 1 - signed) */
//C 	BPF_CORE_FIELD_LSHIFT_U64 = 4,       /* bitfield-specific left bitshift */
//C 	BPF_CORE_FIELD_RSHIFT_U64 = 5,       /* bitfield-specific right bitshift */
//C 	BPF_CORE_TYPE_ID_LOCAL = 6,          /* type ID in local BPF object */
//C 	BPF_CORE_TYPE_ID_TARGET = 7,         /* type ID in target kernel */
//C 	BPF_CORE_TYPE_EXISTS = 8,            /* type existence in target kernel */
//C 	BPF_CORE_TYPE_SIZE = 9,              /* type size in bytes */
//C 	BPF_CORE_ENUMVAL_EXISTS = 10,        /* enum value existence in target kernel */
//C 	BPF_CORE_ENUMVAL_VALUE = 11,         /* enum value integer value */
//C 	BPF_CORE_TYPE_MATCHES = 12,          /* type match in target kernel */
//C };
//C 
//C /*
//C  * "struct bpf_core_relo" is used to pass relocation data form LLVM to libbpf
//C  * and from libbpf to the kernel.
//C  *
//C  * CO-RE relocation captures the following data:
//C  * - insn_off - instruction offset (in bytes) within a BPF program that needs
//C  *   its insn->imm field to be relocated with actual field info;
//C  * - type_id - BTF type ID of the "root" (containing) entity of a relocatable
//C  *   type or field;
//C  * - access_str_off - offset into corresponding .BTF string section. String
//C  *   interpretation depends on specific relocation kind:
//C  *     - for field-based relocations, string encodes an accessed field using
//C  *       a sequence of field and array indices, separated by colon (:). It's
//C  *       conceptually very close to LLVM's getelementptr ([0]) instruction's
//C  *       arguments for identifying offset to a field.
//C  *     - for type-based relocations, strings is expected to be just "0";
//C  *     - for enum value-based relocations, string contains an index of enum
//C  *       value within its enum type;
//C  * - kind - one of enum bpf_core_relo_kind;
//C  *
//C  * Example:
//C  *   struct sample {
//C  *       int a;
//C  *       struct {
//C  *           int b[10];
//C  *       };
//C  *   };
//C  *
//C  *   struct sample *s = ...;
//C  *   int *x = &s->a;     // encoded as "0:0" (a is field #0)
//C  *   int *y = &s->b[5];  // encoded as "0:1:0:5" (anon struct is field #1,
//C  *                       // b is field #0 inside anon struct, accessing elem #5)
//C  *   int *z = &s[10]->b; // encoded as "10:1" (ptr is used as an array)
//C  *
//C  * type_id for all relocs in this example will capture BTF type id of
//C  * `struct sample`.
//C  *
//C  * Such relocation is emitted when using __builtin_preserve_access_index()
//C  * Clang built-in, passing expression that captures field address, e.g.:
//C  *
//C  * bpf_probe_read(&dst, sizeof(dst),
//C  *		  __builtin_preserve_access_index(&src->a.b.c));
//C  *
//C  * In this case Clang will emit field relocation recording necessary data to
//C  * be able to find offset of embedded `a.b.c` field within `src` struct.
//C  *
//C  * [0] https://llvm.org/docs/LangRef.html#getelementptr-instruction
//C  */
//C struct bpf_core_relo {
//C 	__u32 insn_off;
//C 	__u32 type_id;
//C 	__u32 access_str_off;
//C 	enum bpf_core_relo_kind kind;
//C };
//C 
//C /*
//C  * Flags to control bpf_timer_start() behaviour.
//C  *     - BPF_F_TIMER_ABS: Timeout passed is absolute time, by default it is
//C  *       relative to current time.
//C  *     - BPF_F_TIMER_CPU_PIN: Timer will be pinned to the CPU of the caller.
//C  */
//C enum {
//C 	BPF_F_TIMER_ABS = (1ULL << 0),
//C 	BPF_F_TIMER_CPU_PIN = (1ULL << 1),
//C };
//C 
//C /* BPF numbers iterator state */
//C struct bpf_iter_num {
//C 	/* opaque iterator state; having __u64 here allows to preserve correct
//C 	 * alignment requirements in vmlinux.h, generated from BTF
//C 	 */
//C 	__u64 __opaque[1];
//C } __attribute__((aligned(8)));
//C 
//C /*
//C  * Flags to control BPF kfunc behaviour.
//C  *     - BPF_F_PAD_ZEROS: Pad destination buffer with zeros. (See the respective
//C  *       helper documentation for details.)
//C  */
//C enum bpf_kfunc_flags {
//C 	BPF_F_PAD_ZEROS = (1ULL << 0),
//C };
//C 
//C /*
//C  * Values of a BPF_MAP_TYPE_INSN_ARRAY entry must be of this type.
//C  *
//C  * Before the map is used the orig_off field should point to an
//C  * instruction inside the program being loaded. The other fields
//C  * must be set to 0.
//C  *
//C  * After the program is loaded, the xlated_off will be adjusted
//C  * by the verifier to point to the index of the original instruction
//C  * in the xlated program. If the instruction is deleted, it will
//C  * be set to (u32)-1. The jitted_off will be set to the corresponding
//C  * offset in the jitted image of the program.
//C  */
//C struct bpf_insn_array_value {
//C 	__u32 orig_off;
//C 	__u32 xlated_off;
//C 	__u32 jitted_off;
//C 	__u32 :32;
//C };
//C 
//C #endif /* _UAPI__LINUX_BPF_H__ */
//C 
