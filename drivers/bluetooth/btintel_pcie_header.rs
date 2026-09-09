/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Bluetooth support for Intel PCIe devices. Rust translation of btintel_pcie.h. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

const fn BIT(n: u32) -> u32 { 1u32 << n }

pub const BTINTEL_PCIE_CSR_BASE: u32 = 0x000;
pub const BTINTEL_PCIE_CSR_FUNC_CTRL_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x024;
pub const BTINTEL_PCIE_CSR_HW_REV_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x028;
pub const BTINTEL_PCIE_CSR_RF_ID_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x09C;
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x108;
pub const BTINTEL_PCIE_CSR_IPC_CONTROL_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x10C;
pub const BTINTEL_PCIE_CSR_IPC_STATUS_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x110;
pub const BTINTEL_PCIE_CSR_IPC_SLEEP_CTL_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x114;
pub const BTINTEL_PCIE_CSR_CI_ADDR_LSB_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x118;
pub const BTINTEL_PCIE_CSR_CI_ADDR_MSB_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x11C;
pub const BTINTEL_PCIE_CSR_IMG_RESPONSE_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x12C;
pub const BTINTEL_PCIE_CSR_MBOX_1_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x170;
pub const BTINTEL_PCIE_CSR_MBOX_2_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x174;
pub const BTINTEL_PCIE_CSR_MBOX_3_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x178;
pub const BTINTEL_PCIE_CSR_MBOX_4_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x17C;
pub const BTINTEL_PCIE_CSR_MBOX_STATUS_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x180;
pub const BTINTEL_PCIE_PRPH_DEV_ADDR_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x440;
pub const BTINTEL_PCIE_PRPH_DEV_RD_REG: u32 = BTINTEL_PCIE_CSR_BASE + 0x458;
pub const BTINTEL_PCIE_CSR_HBUS_TARG_WRPTR: u32 = BTINTEL_PCIE_CSR_BASE + 0x460;

pub const BTINTEL_PCIE_CSR_FUNC_CTRL_FUNC_ENA: u32 = BIT(0);
pub const BTINTEL_PCIE_CSR_FUNC_CTRL_MAC_INIT: u32 = BIT(6);
pub const BTINTEL_PCIE_CSR_FUNC_CTRL_FUNC_INIT: u32 = BIT(7);
pub const BTINTEL_PCIE_CSR_FUNC_CTRL_MAC_ACCESS_STS: u32 = BIT(20);
pub const BTINTEL_PCIE_CSR_FUNC_CTRL_MAC_ACCESS_REQ: u32 = BIT(21);
pub const BTINTEL_PCIE_CSR_FUNC_CTRL_BUS_MASTER_STS: u32 = BIT(28);
pub const BTINTEL_PCIE_CSR_FUNC_CTRL_BUS_MASTER_DISCON: u32 = BIT(29);
pub const BTINTEL_PCIE_CSR_FUNC_CTRL_SW_RESET: u32 = BIT(31);

pub const BTINTEL_PCIE_CSR_BOOT_STAGE_ROM: u32 = BIT(0);
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_IML: u32 = BIT(1);
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_OPFW: u32 = BIT(2);
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_ROM_LOCKDOWN: u32 = BIT(10);
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_IML_LOCKDOWN: u32 = BIT(11);
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_DEVICE_WARNING: u32 = BIT(12);
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_ABORT_HANDLER: u32 = BIT(13);
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_DEVICE_HALTED: u32 = BIT(14);
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_MAC_ACCESS_ON: u32 = BIT(16);
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_ALIVE: u32 = BIT(23);
pub const BTINTEL_PCIE_CSR_BOOT_STAGE_D3_STATE_READY: u32 = BIT(24);

pub const BTINTEL_PCIE_CSR_MSIX_BASE: u32 = 0x2000;
pub const BTINTEL_PCIE_CSR_MSIX_FH_INT_CAUSES: u32 = BTINTEL_PCIE_CSR_MSIX_BASE + 0x0800;
pub const BTINTEL_PCIE_CSR_MSIX_FH_INT_MASK: u32 = BTINTEL_PCIE_CSR_MSIX_BASE + 0x0804;
pub const BTINTEL_PCIE_CSR_MSIX_HW_INT_CAUSES: u32 = BTINTEL_PCIE_CSR_MSIX_BASE + 0x0808;
pub const BTINTEL_PCIE_CSR_MSIX_HW_INT_MASK: u32 = BTINTEL_PCIE_CSR_MSIX_BASE + 0x080C;
pub const BTINTEL_PCIE_CSR_MSIX_AUTOMASK_ST: u32 = BTINTEL_PCIE_CSR_MSIX_BASE + 0x0810;
pub const BTINTEL_PCIE_CSR_MSIX_AUTOMASK_EN: u32 = BTINTEL_PCIE_CSR_MSIX_BASE + 0x0814;
pub const BTINTEL_PCIE_CSR_MSIX_IVAR_BASE: u32 = BTINTEL_PCIE_CSR_MSIX_BASE + 0x0880;
pub const fn BTINTEL_PCIE_CSR_MSIX_IVAR(cause: u32) -> u32 { BTINTEL_PCIE_CSR_MSIX_IVAR_BASE + cause }

pub const BTINTEL_PCIE_DBGC_BASE_ADDR: u32 = 0xf3800300;
pub const BTINTEL_PCIE_DBGC_CUR_DBGBUFF_STATUS: u32 = BTINTEL_PCIE_DBGC_BASE_ADDR + 0x1C;
pub const BTINTEL_PCIE_DBGC_DBGBUFF_WRAP_ARND: u32 = BTINTEL_PCIE_DBGC_BASE_ADDR + 0x2C;
pub const BTINTEL_PCIE_DBGC_BASE_ADDR_SCP: u32 = 0xf0d5d500;
pub const BTINTEL_PCIE_DBGC_CUR_DBGBUFF_STATUS_SCP: u32 = BTINTEL_PCIE_DBGC_BASE_ADDR_SCP + 0x1C;
pub const BTINTEL_PCIE_DBGC_DBGBUFF_WRAP_ARND_SCP: u32 = BTINTEL_PCIE_DBGC_BASE_ADDR_SCP + 0x2C;
pub const BTINTEL_PCIE_DBG_IDX_BIT_MASK: u32 = 0x0F;
pub const fn BTINTEL_PCIE_DBGC_DBG_BUF_IDX(data: u32) -> u32 { (data >> 24) & BTINTEL_PCIE_DBG_IDX_BIT_MASK }
pub const BTINTEL_PCIE_DBG_OFFSET_BIT_MASK: u32 = 0xFFFFFF;
pub const BTINTEL_PCIE_DBGC_BUFFER_COUNT: u32 = 16;
pub const BTINTEL_PCIE_DBGC_BUFFER_SIZE: u32 = 256 * 1024;
pub const BTINTEL_PCIE_DBGC_FRAG_VERSION: u32 = 1;
pub const BTINTEL_PCIE_DBGC_FRAG_BUFFER_COUNT: u32 = BTINTEL_PCIE_DBGC_BUFFER_COUNT;
pub const BTINTEL_PCIE_DBGC_FRAG_HEADER_SIZE: u32 = 12;
pub const BTINTEL_PCIE_DBGC_FRAG_PAYLOAD_SIZE: u32 = 196;

pub const BTINTEL_PCIE_MSIX_FH_INT_CAUSES_0: u32 = BIT(0);
pub const BTINTEL_PCIE_MSIX_FH_INT_CAUSES_1: u32 = BIT(1);
pub const BTINTEL_PCIE_MSIX_HW_INT_CAUSES_GP0: u32 = BIT(0);
pub const BTINTEL_PCIE_MSIX_HW_INT_CAUSES_GP1: u32 = BIT(1);
pub const BTINTEL_PCIE_MSIX_HW_INT_CAUSES_HWEXP: u32 = BIT(3);
pub const BTINTEL_PCIE_MSIX_HW_INT_CAUSES_FWTRIG: u32 = BIT(5);
pub const BTINTEL_PCIE_STATE_D0: u32 = 0;
pub const BTINTEL_PCIE_STATE_D3_HOT: u32 = 2;
pub const BTINTEL_PCIE_STATE_D3_COLD: u32 = 3;
pub const BTINTEL_PCIE_CORE_HALTED: u32 = 0;
pub const BTINTEL_PCIE_COREDUMP_INPROGRESS: u32 = 1;
pub const BTINTEL_PCIE_FWTRIGGER_DUMP_INPROGRESS: u32 = 2;
pub const BTINTEL_PCIE_RECOVERY_IN_PROGRESS: u32 = 3;
pub const BTINTEL_PCIE_SETUP_DONE: u32 = 4;

pub const BTINTEL_PCIE_CSR_MBOX_STATUS_MBOX1: u32 = BIT(0);
pub const BTINTEL_PCIE_CSR_MBOX_STATUS_MBOX2: u32 = BIT(1);
pub const BTINTEL_PCIE_CSR_MBOX_STATUS_MBOX3: u32 = BIT(2);
pub const BTINTEL_PCIE_CSR_MBOX_STATUS_MBOX4: u32 = BIT(3);
#[repr(u32)] pub enum btintel_pcie_reset_type { BTINTEL_PCIE_IOSF_PRR_FLR = 0, BTINTEL_PCIE_IOSF_PRR_PLDR = 1 }
pub const BTINTEL_PCIE_MSIX_NON_AUTO_CLEAR_CAUSE: u32 = BIT(7);
pub const BTINTEL_PCIE_MSIX_VEC_MAX: usize = 1;
pub const BTINTEL_PCIE_MSIX_VEC_MIN: usize = 1;
pub const BTINTEL_DEFAULT_MAC_ACCESS_TIMEOUT_US: u32 = 200000;
pub const BTINTEL_DEFAULT_INTR_TIMEOUT_MS: u32 = 3000;
pub const BTINTEL_PCIE_DX_TRANSITION_MAX_RETRIES: u32 = 3;
pub const BTINTEL_PCIE_TX_DESCS_COUNT: u32 = 32;
pub const BTINTEL_PCIE_RX_DESCS_COUNT: u32 = 64;
pub const BTINTEL_PCIE_TXQ_NUM: u32 = 0;
pub const BTINTEL_PCIE_RXQ_NUM: u32 = 1;
pub const BTINTEL_PCIE_NUM_QUEUES: u32 = 2;
pub const BTINTEL_PCIE_BUFFER_SIZE: u32 = 4096;
pub const BTINTEL_PCIE_TX_WAIT_TIMEOUT_MS: u32 = 500;
pub const BTINTEL_PCIE_TX_DB_VEC: u32 = 0;
pub const BTINTEL_PCIE_RX_DB_VEC: u32 = 513;
pub const BTINTEL_PCIE_RBD_SIZE_4K: u32 = 0x04;

#[repr(C, packed)] pub struct ctx_info { pub version:u16, pub size:u16, pub config:u32, pub reserved_dw02:u32, pub reserved_dw03:u32, pub addr_tr_hia:u64, pub addr_tr_tia:u64, pub addr_cr_hia:u64, pub addr_cr_tia:u64, pub num_tr_ia:u16, pub num_cr_ia:u16, pub rbd_size_reserved_dw13:u32, pub addr_tfdq:u64, pub addr_urbdq0:u64, pub num_tfdq:u16, pub num_urbdq0:u16, pub tfdq_db_vec:u16, pub urbdq0_db_vec:u16, pub addr_frbdq:u64, pub addr_urbdq1:u64, pub num_frbdq:u16, pub frbdq_db_vec:u16, pub num_urbdq1:u16, pub urbdq_db_vec:u16, pub tr_msi_vec:u16, pub cr_msi_vec:u16, pub reserved_dw27:u32, pub dbgc_addr:u64, pub dbgc_size:u32, pub debug_control_dw31:u32, pub ext_addr:u64, pub ext_size:u32, pub test_param:u32, pub reserved_dw36:u32, pub reserved_dw37:u32 }
#[repr(C, packed)] pub struct tfd { pub type_:u8, pub size:u16, pub reserved:u8, pub addr:u64, pub reserved1:u32 }
#[repr(C, packed)] pub struct urbd0 { pub control:u32 }
#[repr(C, packed)] pub struct frbd { pub tag_reserved:u32, pub reserved2:u32, pub addr:u64 }
#[repr(C, packed)] pub struct urbd1 { pub status_control:u32 }
#[repr(C, packed)] pub struct rfh_hdr { pub packet_control:u64 }
#[repr(C)] pub struct data_buf { pub data:*mut u8, pub data_p_addr:dma_addr_t }
#[repr(C)] pub struct ia { pub tr_hia_p_addr:dma_addr_t, pub tr_hia:*mut u16, pub tr_tia_p_addr:dma_addr_t, pub tr_tia:*mut u16, pub cr_hia_p_addr:dma_addr_t, pub cr_hia:*mut u16, pub cr_tia_p_addr:dma_addr_t, pub cr_tia:*mut u16 }
#[repr(C)] pub struct txq { pub count:u16, pub tfds_p_addr:dma_addr_t, pub tfds:*mut tfd, pub urbd0s_p_addr:dma_addr_t, pub urbd0s:*mut urbd0, pub buf_p_addr:dma_addr_t, pub buf_v_addr:*mut core::ffi::c_void, pub bufs:*mut data_buf }
#[repr(C)] pub struct rxq { pub count:u16, pub frbds_p_addr:dma_addr_t, pub frbds:*mut frbd, pub urbd1s_p_addr:dma_addr_t, pub urbd1s:*mut urbd1, pub buf_p_addr:dma_addr_t, pub buf_v_addr:*mut core::ffi::c_void, pub bufs:*mut data_buf }
#[repr(C)] pub struct btintel_pcie_dbgc { pub count:u16, pub frag_v_addr:*mut core::ffi::c_void, pub frag_p_addr:dma_addr_t, pub frag_size:u16, pub buf_p_addr:dma_addr_t, pub buf_v_addr:*mut core::ffi::c_void, pub bufs:*mut data_buf }
#[repr(C)] pub struct btintel_pcie_dump_header { pub driver_name:*const core::ffi::c_char, pub cnvi_top:u32, pub cnvr_top:u32, pub fw_timestamp:u16, pub fw_build_type:u8, pub fw_build_num:u32, pub fw_git_sha1:u32, pub cnvi_bt:u32, pub write_ptr:u32, pub wrap_ctr:u32, pub trigger_reason:u16, pub state:i32, pub event_type:u8, pub event_id:u16 }

#[repr(C)] pub struct btintel_pcie_data {
    pub pdev:*mut pci_dev, pub hdev:*mut hci_dev, pub flags: c_ulong, pub irq_lock: spinlock_t, pub hci_rx_lock: spinlock_t, pub base_addr:*mut core::ffi::c_void,
    pub msix_entries:[msix_entry; BTINTEL_PCIE_MSIX_VEC_MAX], pub msix_enabled:bool, pub alloc_vecs:u32, pub def_irq:u32, pub fh_init_mask:u32, pub hw_init_mask:u32,
    pub boot_stage_cache:u32, pub img_resp_cache:u32, pub cnvi:u32, pub cnvr:u32, pub gp0_received:bool, pub gp0_wait_q:wait_queue_head_t, pub tx_wait_done:bool, pub tx_wait_q:wait_queue_head_t,
    pub workqueue:*mut workqueue_struct, pub rx_skb_q:sk_buff_head, pub rx_work:work_struct, pub reset_work:work_struct, pub dump_workqueue:*mut workqueue_struct, pub coredump_work:work_struct, pub hwexp_work:work_struct, pub fwtrigger_work:work_struct,
    pub dma_pool:*mut dma_pool, pub dma_p_addr:dma_addr_t, pub dma_v_addr:*mut core::ffi::c_void, pub ci_p_addr:dma_addr_t, pub ci:*mut ctx_info, pub ia:ia, pub txq:txq, pub rxq:rxq, pub alive_intr_ctxt:u32, pub reset_type:btintel_pcie_reset_type, pub dbgc:btintel_pcie_dbgc, pub dmp_hdr:btintel_pcie_dump_header, pub pm_sx_event:u8, pub debug_evt_addr:u32, pub debug_evt_size:u32
}

extern "C" { fn ioread32(addr:*mut core::ffi::c_void)->u32; fn iowrite8(val:u8, addr:*mut core::ffi::c_void); fn iowrite32(val:u32, addr:*mut core::ffi::c_void); }
#[inline] pub unsafe fn btintel_pcie_rd_reg32(data:*mut btintel_pcie_data, offset:u32)->u32 { ioread32((*data).base_addr.add(offset as usize)) }
#[inline] pub unsafe fn btintel_pcie_wr_reg8(data:*mut btintel_pcie_data, offset:u32, val:u8) { iowrite8(val, (*data).base_addr.add(offset as usize)); }
#[inline] pub unsafe fn btintel_pcie_wr_reg32(data:*mut btintel_pcie_data, offset:u32, val:u32) { iowrite32(val, (*data).base_addr.add(offset as usize)); }
#[inline] pub unsafe fn btintel_pcie_set_reg_bits(data:*mut btintel_pcie_data, offset:u32, bits:u32) { let r=ioread32((*data).base_addr.add(offset as usize)); iowrite32(r|bits, (*data).base_addr.add(offset as usize)); }
#[inline] pub unsafe fn btintel_pcie_clr_reg_bits(data:*mut btintel_pcie_data, offset:u32, bits:u32) { let r=ioread32((*data).base_addr.add(offset as usize)); iowrite32(r & !bits, (*data).base_addr.add(offset as usize)); }
#[inline] pub unsafe fn btintel_pcie_rd_dev_mem(data:*mut btintel_pcie_data, addr:u32)->u32 { btintel_pcie_wr_reg32(data, BTINTEL_PCIE_PRPH_DEV_ADDR_REG, addr); btintel_pcie_rd_reg32(data, BTINTEL_PCIE_PRPH_DEV_RD_REG) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
