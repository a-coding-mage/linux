// Translated from mock_file.h.
// Dependency intent: __u32 and __u64 correspond to Linux kernel fixed-width types.

pub const IORING_MOCK_FEAT_CMD_COPY: u32 = 0;
pub const IORING_MOCK_FEAT_RW_ZERO: u32 = 1;
pub const IORING_MOCK_FEAT_RW_NOWAIT: u32 = 2;
pub const IORING_MOCK_FEAT_RW_ASYNC: u32 = 3;
pub const IORING_MOCK_FEAT_POLL: u32 = 4;
pub const IORING_MOCK_FEAT_END: u32 = 5;

#[repr(C)]
pub struct io_uring_mock_probe {
    pub features: u64,
    pub __resv: [u64; 9],
}

pub const IORING_MOCK_CREATE_F_SUPPORT_NOWAIT: u32 = 1;
pub const IORING_MOCK_CREATE_F_POLL: u32 = 2;

#[repr(C)]
pub struct io_uring_mock_create {
    pub out_fd: u32,
    pub flags: u32,
    pub file_size: u64,
    pub rw_delay_ns: u64,
    pub __resv: [u64; 13],
}

pub const IORING_MOCK_MGR_CMD_PROBE: u32 = 0;
pub const IORING_MOCK_MGR_CMD_CREATE: u32 = 1;

pub const IORING_MOCK_CMD_COPY_REGBUF: u32 = 0;

pub const IORING_MOCK_COPY_FROM: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
