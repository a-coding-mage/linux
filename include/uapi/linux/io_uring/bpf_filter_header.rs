/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT */
/*
 * Header file for the io_uring BPF filters.
 */

/* Dependency supplied by the Linux type definitions. */

/*
 * Struct passed to filters.
 */
#[repr(C)]
pub struct io_uring_bpf_ctx {
    pub user_data: __u64,
    pub opcode: __u8,
    pub sqe_flags: __u8,
    pub pdu_size: __u8, /* size of aux data for filter */
    pub pad: [__u8; 5],
    pub data: io_uring_bpf_ctx__bindgen_ty_1,
}

#[repr(C)]
pub union io_uring_bpf_ctx__bindgen_ty_1 {
    pub socket: io_uring_bpf_ctx__bindgen_ty_1__socket,
    pub open: io_uring_bpf_ctx__bindgen_ty_1__open,
    pub connect: io_uring_bpf_ctx__bindgen_ty_1__connect,
}

#[repr(C)]
pub struct io_uring_bpf_ctx__bindgen_ty_1__socket {
    pub family: __u32,
    pub type_: __u32,
    pub protocol: __u32,
}

#[repr(C)]
pub struct io_uring_bpf_ctx__bindgen_ty_1__open {
    pub flags: __u64,
    pub mode: __u64,
    pub resolve: __u64,
}

/*
 * For CONNECT: fields are populated only when addr_len covers
 * them; unpopulated fields are zero from the caller-side memset
 * in io_uring_populate_bpf_ctx(). port and v4_addr are network
 * byte order. Filters may only issue BPF_LD|BPF_W|BPF_ABS at
 * 4-byte aligned offsets; load + mask for sub-word fields.
 */
#[repr(C)]
pub struct io_uring_bpf_ctx__bindgen_ty_1__connect {
    pub family: __u32, /* sa_family_t zero-extended */
    pub port: __be16,
    pub pad: [__u8; 2],
    pub addr: io_uring_bpf_ctx__bindgen_ty_1__connect__bindgen_ty_1,
}

#[repr(C)]
pub union io_uring_bpf_ctx__bindgen_ty_1__connect__bindgen_ty_1 {
    pub v4_addr: __be32,
    pub v6_addr: [__u8; 16],
}

/*
 * If set, any currently unset opcode will have a deny filter attached
 */
pub const IO_URING_BPF_FILTER_DENY_REST: u32 = 1;
/*
 * If set, if kernel and application don't agree on pdu_size for
 * the given opcode, fail the registration of the filter.
 */
pub const IO_URING_BPF_FILTER_SZ_STRICT: u32 = 2;

#[repr(C)]
pub struct io_uring_bpf_filter {
    pub opcode: __u32, /* io_uring opcode to filter */
    pub flags: __u32,
    pub filter_len: __u32, /* number of BPF instructions */
    pub pdu_size: __u8, /* expected pdu size for opcode */
    pub resv: [__u8; 3],
    pub filter_ptr: __u64, /* pointer to BPF filter */
    pub resv2: [__u64; 5],
}

pub const IO_URING_BPF_CMD_FILTER: u32 = 1;

#[repr(C)]
pub struct io_uring_bpf {
    pub cmd_type: __u16, /* IO_URING_BPF_* values */
    pub cmd_flags: __u16, /* none so far */
    pub resv: __u32,
    pub filter: io_uring_bpf_filter,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
