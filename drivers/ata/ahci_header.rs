/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of ahci.h; kernel dependencies are supplied externally. */

pub const EM_CTRL_MSG_TYPE: u32 = 0x000f0000;
pub const EM_MSG_LED_HBA_PORT: u32 = 0x0000000f;
pub const EM_MSG_LED_PMP_SLOT: u32 = 0x0000ff00;
pub const EM_MSG_LED_VALUE: u32 = 0xffff0000;
pub const EM_MSG_LED_VALUE_ACTIVITY: u32 = 0x00070000;
pub const EM_MSG_LED_VALUE_OFF: u32 = 0xfff80000;
pub const EM_MSG_LED_VALUE_ON: u32 = 0x00010000;

pub const AHCI_MAX_PORTS: usize = 32;
pub const AHCI_MAX_SG: usize = 168;
pub const AHCI_DMA_BOUNDARY: u32 = 0xffff_ffff;
pub const AHCI_MAX_CMDS: usize = 32;
pub const AHCI_CMD_SZ: usize = 32;
pub const AHCI_CMD_SLOT_SZ: usize = AHCI_MAX_CMDS * AHCI_CMD_SZ;
pub const AHCI_RX_FIS_SZ: usize = 256;
pub const AHCI_CMD_TBL_CDB: usize = 0x40;
pub const AHCI_CMD_TBL_HDR_SZ: usize = 0x80;
pub const AHCI_CMD_TBL_SZ: usize = AHCI_CMD_TBL_HDR_SZ + AHCI_MAX_SG * 16;
pub const AHCI_CMD_TBL_AR_SZ: usize = AHCI_CMD_TBL_SZ * AHCI_MAX_CMDS;
pub const AHCI_PORT_PRIV_DMA_SZ: usize = AHCI_CMD_SLOT_SZ + AHCI_CMD_TBL_AR_SZ + AHCI_RX_FIS_SZ;
pub const AHCI_PORT_PRIV_FBS_DMA_SZ: usize = AHCI_CMD_SLOT_SZ + AHCI_CMD_TBL_AR_SZ + AHCI_RX_FIS_SZ * 16;

macro_rules! bit { ($n:expr) => { 1u32 << $n }; }

pub const AHCI_IRQ_ON_SG: u32 = bit!(31); pub const AHCI_CMD_ATAPI: u32 = bit!(5);
pub const AHCI_CMD_WRITE: u32 = bit!(6); pub const AHCI_CMD_PREFETCH: u32 = bit!(7);
pub const AHCI_CMD_RESET: u32 = bit!(8); pub const AHCI_CMD_CLR_BUSY: u32 = bit!(10);
pub const RX_FIS_PIO_SETUP: u32 = 0x20; pub const RX_FIS_D2H_REG: u32 = 0x40;
pub const RX_FIS_SDB: u32 = 0x58; pub const RX_FIS_UNK: u32 = 0x60;

pub const HOST_CAP: u32=0x00; pub const HOST_CTL: u32=0x04; pub const HOST_IRQ_STAT:u32=0x08;
pub const HOST_PORTS_IMPL:u32=0x0c; pub const HOST_VERSION:u32=0x10; pub const HOST_EM_LOC:u32=0x1c;
pub const HOST_EM_CTL:u32=0x20; pub const HOST_CAP2:u32=0x24;
pub const HOST_RESET:u32=bit!(0); pub const HOST_IRQ_EN:u32=bit!(1); pub const HOST_MRSM:u32=bit!(2); pub const HOST_AHCI_EN:u32=bit!(31);
pub const HOST_CAP_SXS:u32=bit!(5); pub const HOST_CAP_EMS:u32=bit!(6); pub const HOST_CAP_CCC:u32=bit!(7);
pub const HOST_CAP_PART:u32=bit!(13); pub const HOST_CAP_SSC:u32=bit!(14); pub const HOST_CAP_PIO_MULTI:u32=bit!(15);
pub const HOST_CAP_FBS:u32=bit!(16); pub const HOST_CAP_PMP:u32=bit!(17); pub const HOST_CAP_ONLY:u32=bit!(18);
pub const HOST_CAP_CLO:u32=bit!(24); pub const HOST_CAP_LED:u32=bit!(25); pub const HOST_CAP_ALPM:u32=bit!(26);
pub const HOST_CAP_SSS:u32=bit!(27); pub const HOST_CAP_MPS:u32=bit!(28); pub const HOST_CAP_SNTF:u32=bit!(29);
pub const HOST_CAP_NCQ:u32=bit!(30); pub const HOST_CAP_64:u32=bit!(31);
pub const HOST_CAP2_BOH:u32=bit!(0); pub const HOST_CAP2_NVMHCI:u32=bit!(1); pub const HOST_CAP2_APST:u32=bit!(2);
pub const HOST_CAP2_SDS:u32=bit!(3); pub const HOST_CAP2_SADM:u32=bit!(4); pub const HOST_CAP2_DESO:u32=bit!(5);

pub const PORT_LST_ADDR:u32=0x00; pub const PORT_LST_ADDR_HI:u32=0x04; pub const PORT_FIS_ADDR:u32=0x08; pub const PORT_FIS_ADDR_HI:u32=0x0c;
pub const PORT_IRQ_STAT:u32=0x10; pub const PORT_IRQ_MASK:u32=0x14; pub const PORT_CMD:u32=0x18; pub const PORT_TFDATA:u32=0x20; pub const PORT_SIG:u32=0x24;
pub const PORT_CMD_ISSUE:u32=0x38; pub const PORT_SCR_STAT:u32=0x28; pub const PORT_SCR_CTL:u32=0x2c; pub const PORT_SCR_ERR:u32=0x30; pub const PORT_SCR_ACT:u32=0x34; pub const PORT_SCR_NTF:u32=0x3c; pub const PORT_FBS:u32=0x40; pub const PORT_DEVSLP:u32=0x44;
pub const PORT_IRQ_COLD_PRES:u32=bit!(31); pub const PORT_IRQ_TF_ERR:u32=bit!(30); pub const PORT_IRQ_HBUS_ERR:u32=bit!(29); pub const PORT_IRQ_HBUS_DATA_ERR:u32=bit!(28); pub const PORT_IRQ_IF_ERR:u32=bit!(27); pub const PORT_IRQ_IF_NONFATAL:u32=bit!(26); pub const PORT_IRQ_OVERFLOW:u32=bit!(24); pub const PORT_IRQ_BAD_PMP:u32=bit!(23);
pub const PORT_IRQ_PHYRDY:u32=bit!(22); pub const PORT_IRQ_DMPS:u32=bit!(7); pub const PORT_IRQ_CONNECT:u32=bit!(6); pub const PORT_IRQ_SG_DONE:u32=bit!(5); pub const PORT_IRQ_UNK_FIS:u32=bit!(4); pub const PORT_IRQ_SDB_FIS:u32=bit!(3); pub const PORT_IRQ_DMAS_FIS:u32=bit!(2); pub const PORT_IRQ_PIOS_FIS:u32=bit!(1); pub const PORT_IRQ_D2H_REG_FIS:u32=bit!(0);
pub const PORT_IRQ_FREEZE:u32=PORT_IRQ_HBUS_ERR|PORT_IRQ_IF_ERR|PORT_IRQ_CONNECT|PORT_IRQ_PHYRDY|PORT_IRQ_UNK_FIS|PORT_IRQ_BAD_PMP;
pub const PORT_IRQ_ERROR:u32=PORT_IRQ_FREEZE|PORT_IRQ_TF_ERR|PORT_IRQ_HBUS_DATA_ERR;
pub const DEF_PORT_IRQ:u32=PORT_IRQ_ERROR|PORT_IRQ_SG_DONE|PORT_IRQ_SDB_FIS|PORT_IRQ_DMAS_FIS|PORT_IRQ_PIOS_FIS|PORT_IRQ_D2H_REG_FIS;

pub const PORT_CMD_ASP:u32=bit!(27); pub const PORT_CMD_ALPE:u32=bit!(26); pub const PORT_CMD_ATAPI:u32=bit!(24); pub const PORT_CMD_FBSCP:u32=bit!(22); pub const PORT_CMD_ESP:u32=bit!(21); pub const PORT_CMD_CPD:u32=bit!(20); pub const PORT_CMD_MPSP:u32=bit!(19); pub const PORT_CMD_HPCP:u32=bit!(18); pub const PORT_CMD_PMP:u32=bit!(17); pub const PORT_CMD_LIST_ON:u32=bit!(15); pub const PORT_CMD_FIS_ON:u32=bit!(14); pub const PORT_CMD_FIS_RX:u32=bit!(4); pub const PORT_CMD_CLO:u32=bit!(3); pub const PORT_CMD_POWER_ON:u32=bit!(2); pub const PORT_CMD_SPIN_UP:u32=bit!(1); pub const PORT_CMD_START:u32=bit!(0);
pub const PORT_CMD_ICC_MASK:u32=0xf<<28; pub const PORT_CMD_ICC_ACTIVE:u32=1<<28; pub const PORT_CMD_ICC_PARTIAL:u32=2<<28; pub const PORT_CMD_ICC_SLUMBER:u32=6<<28;
pub const PORT_CMD_CAP:u32=PORT_CMD_HPCP|PORT_CMD_MPSP|PORT_CMD_CPD|PORT_CMD_ESP|PORT_CMD_FBSCP;
pub const PORT_FBS_DWE_OFFSET:u32=16; pub const PORT_FBS_ADO_OFFSET:u32=12; pub const PORT_FBS_DEV_OFFSET:u32=8; pub const PORT_FBS_DEV_MASK:u32=0xf<<8; pub const PORT_FBS_SDE:u32=bit!(2); pub const PORT_FBS_DEC:u32=bit!(1); pub const PORT_FBS_EN:u32=bit!(0);
pub const PORT_DEVSLP_DM_OFFSET:u32=25; pub const PORT_DEVSLP_DM_MASK:u32=0xf<<25; pub const PORT_DEVSLP_DITO_OFFSET:u32=15; pub const PORT_DEVSLP_MDAT_OFFSET:u32=10; pub const PORT_DEVSLP_DETO_OFFSET:u32=2; pub const PORT_DEVSLP_DSP:u32=bit!(1); pub const PORT_DEVSLP_ADSE:u32=bit!(0);

pub const AHCI_HFLAG_NO_NCQ:u32=bit!(0); pub const AHCI_HFLAG_IGN_IRQ_IF_ERR:u32=bit!(1); pub const AHCI_HFLAG_IGN_SERR_INTERNAL:u32=bit!(2); pub const AHCI_HFLAG_32BIT_ONLY:u32=bit!(3); pub const AHCI_HFLAG_MV_PATA:u32=bit!(4); pub const AHCI_HFLAG_NO_MSI:u32=bit!(5); pub const AHCI_HFLAG_NO_PMP:u32=bit!(6); pub const AHCI_HFLAG_SECT255:u32=bit!(8); pub const AHCI_HFLAG_YES_NCQ:u32=bit!(9); pub const AHCI_HFLAG_NO_SUSPEND:u32=bit!(10); pub const AHCI_HFLAG_SRST_TOUT_IS_OFFLINE:u32=bit!(11); pub const AHCI_HFLAG_NO_SNTF:u32=bit!(12); pub const AHCI_HFLAG_NO_FPDMA_AA:u32=bit!(13); pub const AHCI_HFLAG_YES_FBS:u32=bit!(14); pub const AHCI_HFLAG_DELAY_ENGINE:u32=bit!(15); pub const AHCI_HFLAG_NO_DEVSLP:u32=bit!(17); pub const AHCI_HFLAG_NO_FBS:u32=bit!(18);
pub const AHCI_HFLAG_MULTI_MSI:u32=bit!(20); pub const AHCI_HFLAG_WAKE_BEFORE_STOP:u32=bit!(22); pub const AHCI_HFLAG_YES_ALPM:u32=bit!(23); pub const AHCI_HFLAG_NO_WRITE_TO_RO:u32=bit!(24); pub const AHCI_HFLAG_SUSPEND_PHYS:u32=bit!(25); pub const AHCI_HFLAG_NO_SXS:u32=bit!(26); pub const AHCI_HFLAG_43BIT_ONLY:u32=bit!(27); pub const AHCI_HFLAG_INTEL_PCS_QUIRK:u32=bit!(28); pub const AHCI_HFLAG_ATAPI_DMA_QUIRK:u32=bit!(29);
pub const ICH_MAP:u32=0x90; pub const PCS_6:u32=0x92; pub const PCS_7:u32=0x94; pub const EM_MAX_SLOTS:u32=SATA_PMP_MAX_PORTS; pub const EM_MAX_RETRY:u32=5; pub const EM_CTL_RST:u32=bit!(9); pub const EM_CTL_TM:u32=bit!(8); pub const EM_CTL_MR:u32=bit!(0); pub const EM_CTL_ALHD:u32=bit!(26); pub const EM_CTL_XMT:u32=bit!(25); pub const EM_CTL_SMB:u32=bit!(24); pub const EM_CTL_SGPIO:u32=bit!(19); pub const EM_CTL_SES:u32=bit!(18); pub const EM_CTL_SAFTE:u32=bit!(17); pub const EM_CTL_LED:u32=bit!(16); pub const EM_MSG_TYPE_LED:u32=bit!(0); pub const EM_MSG_TYPE_SAFTE:u32=bit!(1); pub const EM_MSG_TYPE_SES2:u32=bit!(2); pub const EM_MSG_TYPE_SGPIO:u32=bit!(3);
pub const AHCI_FLAG_COMMON:u32=ATA_FLAG_SATA|ATA_FLAG_PIO_DMA|ATA_FLAG_ACPI_SATA|ATA_FLAG_AN;
#[macro_export] macro_rules! AHCI_HFLAGS { ($flags:expr) => { private_data = $flags as *mut core::ffi::c_void }; }

#[repr(C)] pub struct ahci_cmd_hdr { pub opts:u32, pub status:u32, pub tbl_addr:u32, pub tbl_addr_hi:u32, pub reserved:[u32;4] }
#[repr(C)] pub struct ahci_sg { pub addr:u32, pub addr_hi:u32, pub reserved:u32, pub flags_size:u32 }
#[repr(C)] pub struct ahci_em_priv { pub blink_policy: sw_activity, pub timer: timer_list, pub saved_activity: usize, pub activity: usize, pub led_state: usize, pub link:*mut ata_link }
#[repr(C)] pub struct ahci_port_priv { pub active_link:*mut ata_link, pub cmd_slot:*mut ahci_cmd_hdr, pub cmd_slot_dma:dma_addr_t, pub cmd_tbl:*mut core::ffi::c_void, pub cmd_tbl_dma:dma_addr_t, pub rx_fis:*mut core::ffi::c_void, pub rx_fis_dma:dma_addr_t, pub ncq_saw_d2h:u32, pub ncq_saw_dmas:u32, pub ncq_saw_sdb:u32, pub lock:spinlock_t, pub intr_mask:u32, pub fbs_supported:bool, pub fbs_enabled:bool, pub fbs_last_dev:i32, pub em_priv:[ahci_em_priv; EM_MAX_SLOTS as usize], pub irq_desc:*mut i8 }
#[repr(C)] pub struct ahci_host_priv { pub flags:u32, pub mask_port_map:u32, pub mask_port_ext:u32, pub mmio:*mut core::ffi::c_void, pub cap:u32, pub cap2:u32, pub version:u32, pub port_map:u32, pub saved_cap:u32, pub saved_cap2:u32, pub saved_port_map:u32, pub saved_port_cap:[u32;AHCI_MAX_PORTS], pub em_loc:u32, pub em_buf_sz:u32, pub em_msg_type:u32, pub remapped_nvme:u32, pub got_runtime_pm:bool, pub n_clks:u32, pub clks:*mut clk_bulk_data, pub f_rsts:u32, pub rsts:*mut reset_control, pub target_pwrs:*mut *mut regulator, pub ahci_regulator:*mut regulator, pub phy_regulator:*mut regulator, pub nports:u32, pub plat_data:*mut core::ffi::c_void, pub irq:u32, pub start_engine:Option<unsafe extern "C" fn(*mut ata_port)>, pub stop_engine:Option<unsafe extern "C" fn(*mut ata_port)->i32>, pub irq_handler:Option<unsafe extern "C" fn(i32,*mut core::ffi::c_void)->irqreturn_t>, pub get_irq_vector:Option<unsafe extern "C" fn(*mut ata_host,i32)->i32>, pub phys:[*mut phy;0] }

#[inline] pub unsafe fn ahci_ignore_port(hpriv:*mut ahci_host_priv, portid:u32)->bool { if portid >= (*hpriv).nports { return true; } if (*hpriv).mask_port_map == 0 { return false; } ((*hpriv).mask_port_map & (1u32 << portid)) == 0 }
pub static mut ahci_ignore_sss:i32 = 0;
pub static mut ahci_shost_groups:*const *const attribute_group = core::ptr::null();
pub static mut ahci_sdev_groups:*const *const attribute_group = core::ptr::null();
pub static mut ahci_ops: ata_port_operations; pub static mut ahci_platform_ops: ata_port_operations; pub static mut ahci_pmp_retry_srst_ops: ata_port_operations;

extern "C" { pub fn ahci_dev_classify(*mut ata_port)->u32; pub fn ahci_fill_cmd_slot(*mut ahci_port_priv,u32,u32); pub fn ahci_save_initial_config(*mut device,*mut ahci_host_priv); pub fn ahci_init_controller(*mut ata_host); pub fn ahci_reset_controller(*mut ata_host)->i32; pub fn ahci_do_softreset(*mut ata_link,*mut u32,i32,usize,Option<unsafe extern "C" fn(*mut ata_link)->i32>)->i32; pub fn ahci_do_hardreset(*mut ata_link,*mut u32,usize,*mut bool)->i32; pub fn ahci_qc_issue(*mut ata_queued_cmd)->u32; pub fn ahci_stop_engine(*mut ata_port)->i32; pub fn ahci_start_fis_rx(*mut ata_port); pub fn ahci_start_engine(*mut ata_port); pub fn ahci_check_ready(*mut ata_link)->i32; pub fn ahci_kick_engine(*mut ata_port)->i32; pub fn ahci_port_resume(*mut ata_port)->i32; pub fn ahci_set_em_messages(*mut ahci_host_priv,*mut ata_port_info); pub fn ahci_reset_em(*mut ata_host)->i32; pub fn ahci_print_info(*mut ata_host,*const i8); pub fn ahci_host_activate(*mut ata_host,*const scsi_host_template)->i32; pub fn ahci_error_handler(*mut ata_port); pub fn ahci_handle_port_intr(*mut ata_host,u32)->u32; }

#[inline] pub unsafe fn __ahci_port_base(hpriv:*mut ahci_host_priv, port_no:u32)->*mut core::ffi::c_void { (*hpriv).mmio.add(0x100 + (port_no as usize * 0x80)) }
#[inline] pub unsafe fn ahci_port_base(ap:*mut ata_port)->*mut core::ffi::c_void { __ahci_port_base((*(*ap).host).private_data as *mut ahci_host_priv, (*ap).port_no) }
#[inline] pub fn ahci_nr_ports(cap:u32)->i32 { ((cap & 0x1f) + 1) as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
