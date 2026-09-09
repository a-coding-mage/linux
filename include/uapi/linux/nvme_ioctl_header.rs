/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Definitions for the NVM Express ioctl interface
 * Copyright (c) 2011-2014, Intel Corporation.
 */

// Dependency equivalent of: #include <linux/types.h>

#[repr(C)]
pub struct nvme_user_io {
    pub opcode: u8,
    pub flags: u8,
    pub control: u16,
    pub nblocks: u16,
    pub rsvd: u16,
    pub metadata: u64,
    pub addr: u64,
    pub slba: u64,
    pub dsmgmt: u32,
    pub reftag: u32,
    pub apptag: u16,
    pub appmask: u16,
}

#[repr(C)]
pub struct nvme_passthru_cmd {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd1: u16,
    pub nsid: u32,
    pub cdw2: u32,
    pub cdw3: u32,
    pub metadata: u64,
    pub addr: u64,
    pub metadata_len: u32,
    pub data_len: u32,
    pub cdw10: u32,
    pub cdw11: u32,
    pub cdw12: u32,
    pub cdw13: u32,
    pub cdw14: u32,
    pub cdw15: u32,
    pub timeout_ms: u32,
    pub result: u32,
}

#[repr(C)]
pub struct nvme_passthru_cmd64 {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd1: u16,
    pub nsid: u32,
    pub cdw2: u32,
    pub cdw3: u32,
    pub metadata: u64,
    pub addr: u64,
    pub metadata_len: u32,
    pub data: nvme_passthru_cmd64_data,
    pub cdw10: u32,
    pub cdw11: u32,
    pub cdw12: u32,
    pub cdw13: u32,
    pub cdw14: u32,
    pub cdw15: u32,
    pub timeout_ms: u32,
    pub rsvd2: u32,
    pub result: u64,
}

#[repr(C)]
pub union nvme_passthru_cmd64_data {
    /* for non-vectored io */
    pub data_len: u32,
    /* for vectored io */
    pub vec_cnt: u32,
}

/* same as struct nvme_passthru_cmd64, minus the 8b result field */
#[repr(C)]
pub struct nvme_uring_cmd {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd1: u16,
    pub nsid: u32,
    pub cdw2: u32,
    pub cdw3: u32,
    pub metadata: u64,
    pub addr: u64,
    pub metadata_len: u32,
    pub data_len: u32,
    pub cdw10: u32,
    pub cdw11: u32,
    pub cdw12: u32,
    pub cdw13: u32,
    pub cdw14: u32,
    pub cdw15: u32,
    pub timeout_ms: u32,
    pub rsvd2: u32,
}

pub type nvme_admin_cmd = nvme_passthru_cmd;

pub const NVME_IOCTL_ID: _ = _IO(b'N' as _, 0x40);
pub const NVME_IOCTL_ADMIN_CMD: _ = _IOWR(b'N' as _, 0x41, nvme_admin_cmd);
pub const NVME_IOCTL_SUBMIT_IO: _ = _IOW(b'N' as _, 0x42, nvme_user_io);
pub const NVME_IOCTL_IO_CMD: _ = _IOWR(b'N' as _, 0x43, nvme_passthru_cmd);
pub const NVME_IOCTL_RESET: _ = _IO(b'N' as _, 0x44);
pub const NVME_IOCTL_SUBSYS_RESET: _ = _IO(b'N' as _, 0x45);
pub const NVME_IOCTL_RESCAN: _ = _IO(b'N' as _, 0x46);
pub const NVME_IOCTL_ADMIN64_CMD: _ = _IOWR(b'N' as _, 0x47, nvme_passthru_cmd64);
pub const NVME_IOCTL_IO64_CMD: _ = _IOWR(b'N' as _, 0x48, nvme_passthru_cmd64);
pub const NVME_IOCTL_IO64_CMD_VEC: _ = _IOWR(b'N' as _, 0x49, nvme_passthru_cmd64);

/* io_uring async commands: */
pub const NVME_URING_CMD_IO: _ = _IOWR(b'N' as _, 0x80, nvme_uring_cmd);
pub const NVME_URING_CMD_IO_VEC: _ = _IOWR(b'N' as _, 0x81, nvme_uring_cmd);
pub const NVME_URING_CMD_ADMIN: _ = _IOWR(b'N' as _, 0x82, nvme_uring_cmd);
pub const NVME_URING_CMD_ADMIN_VEC: _ = _IOWR(b'N' as _, 0x83, nvme_uring_cmd);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
