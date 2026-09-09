/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/* Translated from <linux/types.h>; the C __u* types map to Rust unsigned types. */

/// struct hisi_qp_ctx - User data for hisi qp.
/// @id: qp_index return to user space
/// @qc_type: Accelerator algorithm type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qp_ctx {
    pub id: u16,
    pub qc_type: u16,
}

/// struct hisi_qp_info - User data for hisi qp.
/// @sqe_size: Submission queue element size
/// @sq_depth: The number of sqe
/// @cq_depth: The number of cqe
/// @reserved: Reserved data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qp_info {
    pub sqe_size: u32,
    pub sq_depth: u16,
    pub cq_depth: u16,
    pub reserved: u64,
}

pub const HISI_QM_API_VER_BASE: &str = "hisi_qm_v1";
pub const HISI_QM_API_VER2_BASE: &str = "hisi_qm_v2";
pub const HISI_QM_API_VER3_BASE: &str = "hisi_qm_v3";
pub const HISI_QM_API_VER5_BASE: &str = "hisi_qm_v5";

/* Linux ioctl encoding (_IOWR), retained locally for the declarations below. */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

const fn iowr<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_READ | IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32)
}

/* UACCE_CMD_QM_SET_QP_CTX: Set qp algorithm type */
pub const UACCE_CMD_QM_SET_QP_CTX: u32 = iowr::<hisi_qp_ctx>(b'H' as u32, 10);
/* UACCE_CMD_QM_SET_QP_INFO: Set qp depth and BD size */
pub const UACCE_CMD_QM_SET_QP_INFO: u32 = iowr::<hisi_qp_info>(b'H' as u32, 11);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
