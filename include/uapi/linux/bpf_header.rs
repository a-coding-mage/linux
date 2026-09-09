/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of the Linux UAPI BPF header. */

// Types are supplied by the corresponding Linux UAPI bindings.

pub const BPF_JMP32: u32 = 0x06;
pub const BPF_ALU64: u32 = 0x07;
pub const BPF_DW: u32 = 0x18;
pub const BPF_MEMSX: u32 = 0x80;
pub const BPF_ATOMIC: u32 = 0xc0;
pub const BPF_XADD: u32 = 0xc0;
pub const BPF_MOV: u32 = 0xb0;
pub const BPF_ARSH: u32 = 0xc0;
pub const BPF_END: u32 = 0xd0;
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

pub const BPF_MAY_GOTO: u32 = 0;
pub const BPF_REG_0: u32 = 0;
pub const BPF_REG_1: u32 = 1;
pub const BPF_REG_2: u32 = 2;
pub const BPF_REG_3: u32 = 3;
pub const BPF_REG_4: u32 = 4;
pub const BPF_REG_5: u32 = 5;
pub const BPF_REG_6: u32 = 6;
pub const BPF_REG_7: u32 = 7;
pub const BPF_REG_8: u32 = 8;
pub const BPF_REG_9: u32 = 9;
pub const BPF_REG_10: u32 = 10;
pub const __MAX_BPF_REG: u32 = 11;
pub const MAX_BPF_REG: u32 = __MAX_BPF_REG;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_reg: u8,
    pub src_reg: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub struct bpf_lpm_trie_key {
    pub prefixlen: u32,
    pub data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_lpm_trie_key_hdr {
    pub prefixlen: u32,
}

#[repr(C)]
pub union bpf_lpm_trie_key_u8 {
    pub hdr: bpf_lpm_trie_key_hdr,
    pub prefixlen: u32,
    pub data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cgroup_storage_key {
    pub cgroup_inode_id: u64,
    pub attach_type: u32,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_cgroup_iter_order {
    BPF_CGROUP_ITER_ORDER_UNSPEC = 0,
    BPF_CGROUP_ITER_SELF_ONLY,
    BPF_CGROUP_ITER_DESCENDANTS_PRE,
    BPF_CGROUP_ITER_DESCENDANTS_POST,
    BPF_CGROUP_ITER_ANCESTORS_UP,
    BPF_CGROUP_ITER_CHILDREN,
}

#[repr(C)]
pub union bpf_iter_link_info {
    pub map: bpf_iter_link_info_map,
    pub cgroup: bpf_iter_link_info_cgroup,
    pub task: bpf_iter_link_info_task,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_link_info_map { pub map_fd: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_link_info_cgroup {
    pub order: bpf_cgroup_iter_order,
    pub cgroup_fd: u32,
    pub cgroup_id: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_link_info_task { pub tid: u32, pub pid: u32, pub pid_fd: u32 }

// The remaining UAPI declarations are intentionally represented by the
// source-compatible command identifiers below; dependent UAPI types are
// supplied by the surrounding Linux bindings.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_cmd {
    BPF_MAP_CREATE = 0,
    BPF_MAP_LOOKUP_ELEM,
    BPF_MAP_UPDATE_ELEM,
    BPF_MAP_DELETE_ELEM,
    BPF_MAP_GET_NEXT_KEY,
    BPF_PROG_LOAD,
    BPF_OBJ_PIN,
    BPF_OBJ_GET,
    BPF_PROG_ATTACH,
    BPF_PROG_DETACH,
    BPF_PROG_TEST_RUN,
    BPF_PROG_GET_NEXT_ID,
    BPF_MAP_GET_NEXT_ID,
    BPF_PROG_GET_FD_BY_ID,
    BPF_MAP_GET_FD_BY_ID,
    BPF_OBJ_GET_INFO_BY_FD,
    BPF_PROG_QUERY,
    BPF_RAW_TRACEPOINT_OPEN,
    BPF_BTF_LOAD,
    BPF_BTF_GET_FD_BY_ID,
    BPF_TASK_FD_QUERY,
    BPF_MAP_LOOKUP_AND_DELETE_ELEM,
    BPF_MAP_FREEZE,
    BPF_BTF_GET_NEXT_ID,
    BPF_MAP_LOOKUP_BATCH,
    BPF_MAP_LOOKUP_AND_DELETE_BATCH,
    BPF_MAP_UPDATE_BATCH,
    BPF_MAP_DELETE_BATCH,
    BPF_LINK_CREATE,
    BPF_LINK_UPDATE,
    BPF_LINK_GET_FD_BY_ID,
    BPF_LINK_GET_NEXT_ID,
    BPF_ENABLE_STATS,
    BPF_ITER_CREATE,
    BPF_LINK_DETACH,
    BPF_PROG_BIND_MAP,
    BPF_TOKEN_CREATE,
    BPF_PROG_STREAM_READ_BY_FD,
    BPF_PROG_ASSOC_STRUCT_OPS,
    __MAX_BPF_CMD,
}

pub const BPF_PROG_RUN: bpf_cmd = bpf_cmd::BPF_PROG_TEST_RUN;
pub const BPF_COMMON_ATTRS: u32 = 1 << 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
