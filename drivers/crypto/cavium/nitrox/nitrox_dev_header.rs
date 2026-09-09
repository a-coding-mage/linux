/* SPDX-License-Identifier: GPL-2.0 */

// Kernel includes from the original header are external Rust dependencies.

pub const VERSION_LEN: usize = 32;
pub const MAX_PF_QUEUES: usize = 64;
pub const MAX_DEV_QUEUES: usize = MAX_PF_QUEUES;
pub const CNN55XX_MAX_UCD_BLOCKS: usize = 8;
pub const IRQ_NAMESZ: usize = 32;

#[repr(C)]
pub struct nitrox_cmdq {
    pub cmd_qlock: spinlock_t,
    pub resp_qlock: spinlock_t,
    pub backlog_qlock: spinlock_t,
    pub ndev: *mut nitrox_device,
    pub response_head: list_head,
    pub backlog_head: list_head,
    pub dbell_csr_addr: *mut u8,
    pub compl_cnt_csr_addr: *mut u8,
    pub base: *mut u8,
    pub dma: dma_addr_t,
    pub backlog_qflush: work_struct,
    pub pending_count: atomic_t,
    pub backlog_count: atomic_t,
    pub write_idx: ::core::ffi::c_int,
    pub instr_size: u8,
    pub qno: u8,
    pub qsize: u32,
    pub unalign_base: *mut u8,
    pub unalign_dma: dma_addr_t,
}

#[repr(C)]
pub struct nitrox_hw {
    pub partname: [::core::ffi::c_char; IFNAMSIZ * 2],
    pub fw_name: [[::core::ffi::c_char; VERSION_LEN]; CNN55XX_MAX_UCD_BLOCKS],
    pub freq: ::core::ffi::c_int,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision_id: u8,
    pub se_cores: u8,
    pub ae_cores: u8,
    pub zip_cores: u8,
}

#[repr(C)]
pub struct nitrox_stats {
    pub posted: atomic64_t,
    pub completed: atomic64_t,
    pub dropped: atomic64_t,
}

#[repr(C)]
pub struct nitrox_q_vector {
    pub name: [::core::ffi::c_char; IRQ_NAMESZ],
    pub valid: bool,
    pub ring: ::core::ffi::c_int,
    pub resp_tasklet: tasklet_struct,
    pub data: nitrox_q_vector_data,
}

#[repr(C)]
pub union nitrox_q_vector_data {
    pub cmdq: *mut nitrox_cmdq,
    pub ndev: *mut nitrox_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mcode_type {
    MCODE_TYPE_INVALID,
    MCODE_TYPE_AE,
    MCODE_TYPE_SE_SSL,
    MCODE_TYPE_SE_IPSEC,
}

#[repr(C)]
pub union mbox_msg {
    pub value: u64,
    pub fields: mbox_msg_fields,
    pub id: mbox_msg_id,
    pub mcode_info: mbox_msg_mcode_info,
}

// C bitfields are represented by their containing 64-bit storage word.
#[repr(C)]
pub struct mbox_msg_fields { pub bits: u64 }
#[repr(C)]
pub struct mbox_msg_id { pub bits: u64 }
#[repr(C)]
pub struct mbox_msg_mcode_info { pub bits: u64 }

#[repr(C)]
pub struct nitrox_vfdev {
    pub state: atomic_t,
    pub vfno: ::core::ffi::c_int,
    pub nr_queues: ::core::ffi::c_int,
    pub ring: ::core::ffi::c_int,
    pub msg: mbox_msg,
    pub mbx_resp: atomic64_t,
}

#[repr(C)]
pub struct nitrox_iov {
    pub num_vfs: ::core::ffi::c_int,
    pub max_vf_queues: ::core::ffi::c_int,
    pub vfdev: *mut nitrox_vfdev,
    pub pf2vf_wq: *mut workqueue_struct,
    pub msix: msix_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ndev_state {
    __NDEV_NOT_READY,
    __NDEV_READY,
    __NDEV_IN_RESET,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vf_mode {
    __NDEV_MODE_PF,
    __NDEV_MODE_VF16,
    __NDEV_MODE_VF32,
    __NDEV_MODE_VF64,
    __NDEV_MODE_VF128,
}

pub const __NDEV_SRIOV_BIT: u32 = 0;
pub const DEFAULT_CMD_QLEN: u32 = 2048;
pub const CMD_TIMEOUT: u32 = 2000;

#[repr(C)]
pub struct nitrox_device {
    pub list: list_head,
    pub bar_addr: *mut u8,
    pub pdev: *mut pci_dev,
    pub state: atomic_t,
    pub flags: ::core::ffi::c_ulong,
    pub timeout: ::core::ffi::c_ulong,
    pub refcnt: refcount_t,
    pub idx: u8,
    pub node: ::core::ffi::c_int,
    pub qlen: u16,
    pub nr_queues: u16,
    pub mode: vf_mode,
    pub ctx_pool: *mut dma_pool,
    pub pkt_inq: *mut nitrox_cmdq,
    pub aqmq: [*mut nitrox_cmdq; MAX_DEV_QUEUES],
    pub qvec: *mut nitrox_q_vector,
    pub iov: nitrox_iov,
    pub num_vecs: ::core::ffi::c_int,
    pub stats: nitrox_stats,
    pub hw: nitrox_hw,
    // Present only when CONFIG_DEBUG_FS is enabled in the C build.
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_dir: *mut dentry,
}

#[inline]
pub unsafe fn nitrox_read_csr(ndev: *mut nitrox_device, offset: u64) -> u64 {
    readq((*ndev).bar_addr.add(offset as usize))
}

#[inline]
pub unsafe fn nitrox_write_csr(ndev: *mut nitrox_device, offset: u64, value: u64) {
    writeq(value, (*ndev).bar_addr.add(offset as usize));
}

#[inline]
pub unsafe fn nitrox_ready(ndev: *mut nitrox_device) -> bool {
    atomic_read(&(*ndev).state) == __NDEV_READY as ::core::ffi::c_int
}

#[inline]
pub unsafe fn nitrox_vfdev_ready(vfdev: *mut nitrox_vfdev) -> bool {
    atomic_read(&(*vfdev).state) == __NDEV_READY as ::core::ffi::c_int
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
