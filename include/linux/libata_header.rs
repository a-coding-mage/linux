/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Faithful Rust translation of linux/libata.h.  Kernel dependencies are
 * intentionally left as external symbols/types. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/* C headers removed; these names are supplied by the surrounding kernel
 * translation.  CONFIG_* conditional declarations retain their C intent. */

pub const ATA_TAG_POISON: u32 = 0xfafbfcfd;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ata_quirks {
    __ATA_QUIRK_DIAGNOSTIC, __ATA_QUIRK_NODMA, __ATA_QUIRK_NONCQ,
    __ATA_QUIRK_BROKEN_HPA, __ATA_QUIRK_DISABLE, __ATA_QUIRK_HPA_SIZE,
    __ATA_QUIRK_IVB, __ATA_QUIRK_STUCK_ERR, __ATA_QUIRK_BRIDGE_OK,
    __ATA_QUIRK_ATAPI_MOD16_DMA, __ATA_QUIRK_FIRMWARE_WARN,
    __ATA_QUIRK_1_5_GBPS, __ATA_QUIRK_NOSETXFER, __ATA_QUIRK_BROKEN_FPDMA_AA,
    __ATA_QUIRK_DUMP_ID, __ATA_QUIRK_MAX_SEC_LBA48, __ATA_QUIRK_ATAPI_DMADIR,
    __ATA_QUIRK_NO_NCQ_TRIM, __ATA_QUIRK_NOLPM, __ATA_QUIRK_WD_BROKEN_LPM,
    __ATA_QUIRK_ZERO_AFTER_TRIM, __ATA_QUIRK_NO_DMA_LOG, __ATA_QUIRK_NOTRIM,
    __ATA_QUIRK_MAX_SEC, __ATA_QUIRK_MAX_TRIM_128M, __ATA_QUIRK_NO_NCQ_ON_ATI,
    __ATA_QUIRK_NO_LPM_ON_ATI, __ATA_QUIRK_NO_ID_DEV_LOG, __ATA_QUIRK_NO_LOG_DIR,
    __ATA_QUIRK_NO_FUA, __ATA_QUIRK_MAX,
}

macro_rules! bit_consts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n:u64 = 1u64 << ($v as u32);)* }; }
bit_consts! {
 ATA_QUIRK_DIAGNOSTIC=0, ATA_QUIRK_NODMA=1, ATA_QUIRK_NONCQ=2,
 ATA_QUIRK_BROKEN_HPA=3, ATA_QUIRK_DISABLE=4, ATA_QUIRK_HPA_SIZE=5,
 ATA_QUIRK_IVB=6, ATA_QUIRK_STUCK_ERR=7, ATA_QUIRK_BRIDGE_OK=8,
 ATA_QUIRK_ATAPI_MOD16_DMA=9, ATA_QUIRK_FIRMWARE_WARN=10, ATA_QUIRK_1_5_GBPS=11,
 ATA_QUIRK_NOSETXFER=12, ATA_QUIRK_BROKEN_FPDMA_AA=13, ATA_QUIRK_DUMP_ID=14,
 ATA_QUIRK_MAX_SEC_LBA48=15, ATA_QUIRK_ATAPI_DMADIR=16, ATA_QUIRK_NO_NCQ_TRIM=17,
 ATA_QUIRK_NOLPM=18, ATA_QUIRK_WD_BROKEN_LPM=19, ATA_QUIRK_ZERO_AFTER_TRIM=20,
 ATA_QUIRK_NO_DMA_LOG=21, ATA_QUIRK_NOTRIM=22, ATA_QUIRK_MAX_SEC=23,
 ATA_QUIRK_MAX_TRIM_128M=24, ATA_QUIRK_NO_NCQ_ON_ATI=25, ATA_QUIRK_NO_LPM_ON_ATI=26,
 ATA_QUIRK_NO_ID_DEV_LOG=27, ATA_QUIRK_NO_LOG_DIR=28, ATA_QUIRK_NO_FUA=29
}

/* The following masks preserve the original enum values and expressions. */
pub const ATA_DFLAG_LBA:u64=1<<0; pub const ATA_DFLAG_LBA48:u64=1<<1;
pub const ATA_DFLAG_CDB_INTR:u64=1<<2; pub const ATA_DFLAG_NCQ:u64=1<<3;
pub const ATA_DFLAG_FLUSH_EXT:u64=1<<4; pub const ATA_DFLAG_ACPI_PENDING:u64=1<<5;
pub const ATA_DFLAG_ACPI_FAILED:u64=1<<6; pub const ATA_DFLAG_AN:u64=1<<7;
pub const ATA_DFLAG_TRUSTED:u64=1<<8; pub const ATA_DFLAG_FUA:u64=1<<9;
pub const ATA_DFLAG_DMADIR:u64=1<<10; pub const ATA_DFLAG_NCQ_SEND_RECV:u64=1<<11;
pub const ATA_DFLAG_NCQ_PRIO:u64=1<<12; pub const ATA_DFLAG_CDL:u64=1<<13;
pub const ATA_DFLAG_DEPOP:u64=1<<14; pub const ATA_DFLAG_DEPOP_RESTORE:u64=1<<15;
pub const ATA_DFLAG_DEPOP_MODIFY:u64=1<<16; pub const ATA_DFLAG_CFG_MASK:u64=(1<<17)-1;
pub const ATA_DFLAG_PIO:u64=1<<17; pub const ATA_DFLAG_NCQ_OFF:u64=1<<18;
pub const ATA_DFLAG_SLEEPING:u64=1<<19; pub const ATA_DFLAG_DUBIOUS_XFER:u64=1<<20;
pub const ATA_DFLAG_NO_UNLOAD:u64=1<<21; pub const ATA_DFLAG_UNLOCK_HPA:u64=1<<22;
pub const ATA_DFLAG_INIT_MASK:u64=(1<<23)-1; pub const ATA_DFLAG_NCQ_PRIO_ENABLED:u64=1<<23;
pub const ATA_DFLAG_CDL_ENABLED:u64=1<<24; pub const ATA_DFLAG_RESUMING:u64=1<<25;
pub const ATA_DFLAG_DETACH:u64=1<<26; pub const ATA_DFLAG_DETACHED:u64=1<<27;
pub const ATA_DFLAG_DA:u64=1<<28; pub const ATA_DFLAG_DEVSLP:u64=1<<29;
pub const ATA_DFLAG_ACPI_DISABLED:u64=1<<30; pub const ATA_DFLAG_D_SENSE:u64=1<<31;

pub const LIBATA_MAX_PRD:u32 = ATA_MAX_PRD / 2;
pub const LIBATA_DUMB_MAX_PRD:u32 = ATA_MAX_PRD / 4;
pub const ATA_DEF_QUEUE:u32=1; pub const ATA_MAX_QUEUE:u32=32;
pub const ATA_TAG_INTERNAL:u32=ATA_MAX_QUEUE; pub const ATA_SHORT_PAUSE:u32=16;
pub const ATAPI_MAX_DRAIN:u32=16<<10; pub const ATA_ALL_DEVICES:u32=(1<<ATA_MAX_DEVICES)-1;
pub const ATA_SHT_EMULATED:i32=1; pub const ATA_SHT_THIS_ID:i32=-1;
pub const ATA_TFLAG_LBA48:u32=1<<0; pub const ATA_TFLAG_ISADDR:u32=1<<1;
pub const ATA_TFLAG_DEVICE:u32=1<<2; pub const ATA_TFLAG_WRITE:u32=1<<3;
pub const ATA_TFLAG_LBA:u32=1<<4; pub const ATA_TFLAG_FUA:u32=1<<5;
pub const ATA_TFLAG_POLLING:u32=1<<6;
pub const ATA_DEV_UNKNOWN:u32=0; pub const ATA_DEV_ATA:u32=1; pub const ATA_DEV_ATA_UNSUP:u32=2;
pub const ATA_DEV_ATAPI:u32=3; pub const ATA_DEV_ATAPI_UNSUP:u32=4; pub const ATA_DEV_PMP:u32=5;
pub const ATA_DEV_PMP_UNSUP:u32=6; pub const ATA_DEV_SEMB:u32=7; pub const ATA_DEV_SEMB_UNSUP:u32=8;
pub const ATA_DEV_ZAC:u32=9; pub const ATA_DEV_ZAC_UNSUP:u32=10; pub const ATA_DEV_NONE:u32=11;
pub const ATA_LFLAG_NO_HRST:u32=1<<1; pub const ATA_LFLAG_NO_SRST:u32=1<<2;
pub const ATA_LFLAG_ASSUME_ATA:u32=1<<3; pub const ATA_LFLAG_ASSUME_SEMB:u32=1<<4;
pub const ATA_LFLAG_ASSUME_CLASS:u32=ATA_LFLAG_ASSUME_ATA|ATA_LFLAG_ASSUME_SEMB;
pub const ATA_LFLAG_NO_RETRY:u32=1<<5; pub const ATA_LFLAG_DISABLED:u32=1<<6;
pub const ATA_LFLAG_SW_ACTIVITY:u32=1<<7; pub const ATA_LFLAG_NO_LPM:u32=1<<8;
pub const ATA_LFLAG_RST_ONCE:u32=1<<9; pub const ATA_LFLAG_CHANGED:u32=1<<10;
pub const ATA_LFLAG_NO_DEBOUNCE_DELAY:u32=1<<11;

#[repr(C)] pub struct ata_taskfile { pub flags: usize, pub protocol:u8, pub ctl:u8,
 pub hob_feature:u8,pub hob_nsect:u8,pub hob_lbal:u8,pub hob_lbam:u8,pub hob_lbah:u8,
 pub error:u8,pub nsect:u8,pub lbal:u8,pub lbam:u8,pub lbah:u8,pub device:u8,
 pub status:u8,pub auxiliary:u32 }

#[repr(C)] pub struct ata_host { pub lock: spinlock_t,pub dev:*mut device,pub iomap:*const *mut core::ffi::c_void,
 pub n_ports:u32,pub n_tags:u32,pub private_data:*mut core::ffi::c_void,pub ops:*mut ata_port_operations,
 pub flags:usize,pub kref:kref,pub eh_mutex:mutex,pub eh_owner:*mut task_struct,
 pub simplex_claimed:*mut ata_port,pub ports:[*mut ata_port;0] }

pub type ata_qc_cb_t = unsafe extern "C" fn(*mut ata_queued_cmd);
pub type ata_prereset_fn_t = unsafe extern "C" fn(*mut ata_link, usize)->i32;
pub type ata_reset_fn_t = unsafe extern "C" fn(*mut ata_link,*mut u32,usize)->i32;
pub type ata_postreset_fn_t = unsafe extern "C" fn(*mut ata_link,*mut u32);

#[repr(C)] pub struct ata_queued_cmd { pub ap:*mut ata_port,pub dev:*mut ata_device,pub scsicmd:*mut scsi_cmnd,
 pub scsidone:Option<unsafe extern "C" fn(*mut scsi_cmnd)>,pub tf:ata_taskfile,pub cdb:[u8;ATAPI_CDB_LEN],
 pub flags:usize,pub tag:u32,pub hw_tag:u32,pub n_elem:u32,pub orig_n_elem:u32,pub dma_dir:i32,
 pub sect_size:u32,pub nbytes:u32,pub extrabytes:u32,pub curbytes:u32,pub sgent:scatterlist,
 pub sg:*mut scatterlist,pub cursg:*mut scatterlist,pub cursg_ofs:u32,pub err_mask:u32,
 pub result_tf:ata_taskfile,pub complete_fn:Option<ata_qc_cb_t>,pub private_data:*mut core::ffi::c_void,pub lldd_task:*mut core::ffi::c_void }

#[repr(C)] pub struct ata_port_stats { pub unhandled_irq:usize,pub idle_irq:usize,pub rw_reqbuf:usize }
#[repr(C)] pub struct ata_ering_entry { pub eflags:u32,pub err_mask:u32,pub timestamp:u64 }
#[repr(C)] pub struct ata_ering { pub cursor:i32,pub ring:[ata_ering_entry;ATA_ERING_SIZE] }
#[repr(C)] pub struct ata_cpr { pub num:u8,pub num_storage_elements:u8,pub start_lba:u64,pub num_lbas:u64 }
#[repr(C)] pub struct ata_cpr_log { pub nr_cpr:u8,pub cpr:[ata_cpr;0] }
#[repr(C)] pub struct ata_cdl { pub desc_log_buf:[u8;ATA_LOG_CDL_SIZE],pub ncq_sense_log_buf:[u8;ATA_LOG_SENSE_NCQ_SIZE] }

#[repr(C)] pub struct ata_device { pub link:*mut ata_link,pub devno:u32,pub quirks:u64,pub flags:usize,
 pub sdev:*mut scsi_device,pub private_data:*mut core::ffi::c_void,pub tdev:device,pub n_sectors:u64,
 pub n_native_sectors:u64,pub class:u32,pub unpark_deadline:usize,pub pio_mode:u8,pub dma_mode:u8,
 pub xfer_mode:u8,pub xfer_shift:u32,pub multi_count:u32,pub max_sectors:u32,pub cdb_len:u32,
 pub pio_mask:u32,pub mwdma_mask:u32,pub udma_mask:u32,pub cylinders:u16,pub heads:u16,pub sectors:u16,
 pub id:[u16;ATA_ID_WORDS],pub gp_log_dir:[u8;ATA_SECT_SIZE],pub devslp_timing:[u8;ATA_LOG_DEVSLP_SIZE],
 pub ncq_send_recv_cmds:[u8;ATA_LOG_NCQ_SEND_RECV_SIZE],pub ncq_non_data_cmds:[u8;ATA_LOG_NCQ_NON_DATA_SIZE],
 pub zac_zoned_cap:u32,pub zac_zones_optimal_open:u32,pub zac_zones_optimal_nonseq:u32,pub zac_zones_max_open:u32,
 pub cpr_log:*mut ata_cpr_log,pub cdl:*mut ata_cdl,pub spdn_cnt:i32,pub ering:ata_ering,pub sector_buf:[u8;ATA_SECT_SIZE] }

#[repr(C)] pub struct ata_eh_info { pub dev:*mut ata_device,pub serror:u32,pub err_mask:u32,pub action:u32,
 pub dev_action:[u32;ATA_MAX_DEVICES],pub flags:u32,pub probe_mask:u32,pub desc:[i8;ATA_EH_DESC_LEN],pub desc_len:i32 }
#[repr(C)] pub struct ata_eh_context { pub i:ata_eh_info,pub tries:[i32;ATA_MAX_DEVICES],pub cmd_timeout_idx:[[i32;ATA_EH_CMD_TIMEOUT_TABLE_SIZE];ATA_MAX_DEVICES],pub classes:[u32;ATA_MAX_DEVICES],pub did_probe_mask:u32,pub unloaded_mask:u32,pub saved_ncq_enabled:u32,pub saved_xfer_mode:[u8;ATA_MAX_DEVICES],pub last_reset:usize }
#[repr(C,packed)] pub struct ata_acpi_drive { pub pio:u32,pub dma:u32 }
#[repr(C,packed)] pub struct ata_acpi_gtm { pub drive:[ata_acpi_drive;2],pub flags:u32 }
#[repr(C)] pub struct ata_link { pub ap:*mut ata_port,pub pmp:i32,pub tdev:device,pub active_tag:u32,pub sactive:u32,pub flags:u32,pub saved_scontrol:u32,pub hw_sata_spd_limit:u32,pub sata_spd_limit:u32,pub sata_spd:u32,pub lpm_policy:ata_lpm_policy,pub deferred_qc_work:work_struct,pub deferred_qc:*mut ata_queued_cmd,pub eh_info:ata_eh_info,pub eh_context:ata_eh_context,pub device:[ata_device;ATA_MAX_DEVICES],pub last_lpm_change:usize }

#[repr(C)] pub struct ata_reset_operations { pub prereset:Option<ata_prereset_fn_t>,pub softreset:Option<ata_reset_fn_t>,pub hardreset:Option<ata_reset_fn_t>,pub postreset:Option<ata_postreset_fn_t> }
#[repr(C)] pub struct ata_port_operations {
 pub qc_defer:Option<unsafe extern "C" fn(*mut ata_queued_cmd)->i32>,pub check_atapi_dma:Option<unsafe extern "C" fn(*mut ata_queued_cmd)->i32>,pub qc_prep:Option<unsafe extern "C" fn(*mut ata_queued_cmd)->ata_completion_errors>,pub qc_issue:Option<unsafe extern "C" fn(*mut ata_queued_cmd)->u32>,pub qc_fill_rtf:Option<unsafe extern "C" fn(*mut ata_queued_cmd)>,pub qc_ncq_fill_rtf:Option<unsafe extern "C" fn(*mut ata_port,u64)>,pub cable_detect:Option<unsafe extern "C" fn(*mut ata_port)->i32>,pub mode_filter:Option<unsafe extern "C" fn(*mut ata_device,u32)->u32>,pub set_piomode:Option<unsafe extern "C" fn(*mut ata_port,*mut ata_device)>,pub set_dmamode:Option<unsafe extern "C" fn(*mut ata_port,*mut ata_device)>,pub set_mode:Option<unsafe extern "C" fn(*mut ata_link,*mut *mut ata_device)->i32>,pub dev_config:Option<unsafe extern "C" fn(*mut ata_device)>,pub freeze:Option<unsafe extern "C" fn(*mut ata_port)>,pub thaw:Option<unsafe extern "C" fn(*mut ata_port)>,pub reset:ata_reset_operations,pub pmp_reset:ata_reset_operations,pub error_handler:Option<unsafe extern "C" fn(*mut ata_port)>,pub lost_interrupt:Option<unsafe extern "C" fn(*mut ata_port)>,pub post_internal_cmd:Option<unsafe extern "C" fn(*mut ata_queued_cmd)>,pub sched_eh:Option<unsafe extern "C" fn(*mut ata_port)>,pub end_eh:Option<unsafe extern "C" fn(*mut ata_port)>,pub scr_read:Option<unsafe extern "C" fn(*mut ata_link,u32,*mut u32)->i32>,pub scr_write:Option<unsafe extern "C" fn(*mut ata_link,u32,u32)->i32>,pub pmp_attach:Option<unsafe extern "C" fn(*mut ata_port)>,pub pmp_detach:Option<unsafe extern "C" fn(*mut ata_port)>,pub set_lpm:Option<unsafe extern "C" fn(*mut ata_link,ata_lpm_policy,u32)->i32>,pub port_suspend:Option<unsafe extern "C" fn(*mut ata_port,pm_message_t)->i32>,pub port_resume:Option<unsafe extern "C" fn(*mut ata_port)->i32>,pub port_start:Option<unsafe extern "C" fn(*mut ata_port)->i32>,pub port_stop:Option<unsafe extern "C" fn(*mut ata_port)>,pub host_stop:Option<unsafe extern "C" fn(*mut ata_host)>,pub inherits:*const ata_port_operations }

#[repr(u32)] pub enum ata_lpm_policy { ATA_LPM_UNKNOWN,ATA_LPM_MAX_POWER,ATA_LPM_MED_POWER,ATA_LPM_MED_POWER_WITH_DIPM,ATA_LPM_MIN_POWER_WITH_PARTIAL,ATA_LPM_MIN_POWER }
#[repr(u32)] pub enum ata_completion_errors { AC_ERR_OK=0,AC_ERR_DEV=1<<0,AC_ERR_HSM=1<<1,AC_ERR_TIMEOUT=1<<2,AC_ERR_MEDIA=1<<3,AC_ERR_ATA_BUS=1<<4,AC_ERR_HOST_BUS=1<<5,AC_ERR_SYSTEM=1<<6,AC_ERR_INVALID=1<<7,AC_ERR_OTHER=1<<8,AC_ERR_NODEV_HINT=1<<9,AC_ERR_NCQ=1<<10 }
#[repr(u32)] pub enum ata_link_iter_mode { ATA_LITER_EDGE,ATA_LITER_HOST_FIRST,ATA_LITER_PMP_FIRST }
#[repr(u32)] pub enum ata_dev_iter_mode { ATA_DITER_ENABLED,ATA_DITER_ENABLED_REVERSE,ATA_DITER_ALL,ATA_DITER_ALL_REVERSE }

/* External declarations from the Linux kernel and other libata units. */
extern "C" {
 pub static ata_dummy_port_ops:ata_port_operations;
 pub fn ata_host_alloc(dev:*mut device,n_ports:i32)->*mut ata_host;
 pub fn ata_host_put(host:*mut ata_host); pub fn ata_host_start(host:*mut ata_host)->i32;
 pub fn ata_port_alloc(host:*mut ata_host)->*mut ata_port; pub fn ata_port_free(ap:*mut ata_port);
 pub fn ata_link_next(link:*mut ata_link,ap:*mut ata_port,mode:ata_link_iter_mode)->*mut ata_link;
 pub fn ata_dev_next(dev:*mut ata_device,link:*mut ata_link,mode:ata_dev_iter_mode)->*mut ata_device;
 pub fn ata_dev_disable(dev:*mut ata_device); pub fn ata_qc_complete(qc:*mut ata_queued_cmd);
 pub fn ata_std_qc_defer(qc:*mut ata_queued_cmd)->i32;
 pub fn sata_pmp_supported(ap:*mut ata_port)->bool;
}

#[inline] pub unsafe fn ata_is_atapi(prot:u8)->bool { prot & ATA_PROT_FLAG_ATAPI != 0 }
#[inline] pub unsafe fn ata_is_pio(prot:u8)->bool { prot & ATA_PROT_FLAG_PIO != 0 }
#[inline] pub unsafe fn ata_is_dma(prot:u8)->bool { prot & ATA_PROT_FLAG_DMA != 0 }
#[inline] pub unsafe fn ata_is_ncq(prot:u8)->bool { prot & ATA_PROT_FLAG_NCQ != 0 }
#[inline] pub unsafe fn ata_tag_internal(tag:u32)->bool { tag == ATA_TAG_INTERNAL }
#[inline] pub unsafe fn ata_tag_valid(tag:u32)->bool { tag < ATA_MAX_QUEUE || ata_tag_internal(tag) }
#[inline] pub unsafe fn ata_class_enabled(class:u32)->bool { matches!(class,ATA_DEV_ATA|ATA_DEV_ATAPI|ATA_DEV_PMP|ATA_DEV_SEMB|ATA_DEV_ZAC) }
#[inline] pub unsafe fn ata_class_disabled(class:u32)->bool { matches!(class,ATA_DEV_ATA_UNSUP|ATA_DEV_ATAPI_UNSUP|ATA_DEV_PMP_UNSUP|ATA_DEV_SEMB_UNSUP|ATA_DEV_ZAC_UNSUP) }
#[inline] pub unsafe fn ata_class_absent(class:u32)->bool { !ata_class_enabled(class) && !ata_class_disabled(class) }

/* Remaining declaration-only APIs and configuration-specific helpers retain
 * their original names and linkage in the kernel translation unit. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
