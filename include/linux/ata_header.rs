/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/ata.h. C includes and build configuration are
 * supplied by the surrounding kernel translation. */

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

pub const ATA_DMA_BOUNDARY: u64 = 0xffff;
pub const ATA_DMA_MASK: u64 = 0xffff_ffff;

macro_rules! ata_consts { ($( $n:ident = $v:expr ),* $(,)?) => { $(pub const $n: u64 = $v;)* }; }
ata_consts! {
 ATA_MAX_DEVICES=2, ATA_MAX_PRD=256, ATA_SECT_SIZE=512, ATA_MAX_SECTORS=256,
 ATA_MAX_SECTORS_LBA48=65535, ATA_MAX_SECTORS_TAPE=65535, ATA_MAX_TRIM_RNUM=64,
 ATA_ID_WORDS=256, ATA_ID_CONFIG=0, ATA_ID_CYLS=1, ATA_ID_HEADS=3, ATA_ID_SECTORS=6,
 ATA_ID_SERNO=10, ATA_ID_BUF_SIZE=21, ATA_ID_FW_REV=23, ATA_ID_PROD=27,
 ATA_ID_MAX_MULTSECT=47, ATA_ID_DWORD_IO=48, ATA_ID_TRUSTED=48, ATA_ID_CAPABILITY=49,
 ATA_ID_OLD_PIO_MODES=51, ATA_ID_OLD_DMA_MODES=52, ATA_ID_FIELD_VALID=53,
 ATA_ID_CUR_CYLS=54, ATA_ID_CUR_HEADS=55, ATA_ID_CUR_SECTORS=56, ATA_ID_MULTSECT=59,
 ATA_ID_LBA_CAPACITY=60, ATA_ID_SWDMA_MODES=62, ATA_ID_MWDMA_MODES=63, ATA_ID_PIO_MODES=64,
 ATA_ID_EIDE_DMA_MIN=65, ATA_ID_EIDE_DMA_TIME=66, ATA_ID_EIDE_PIO=67, ATA_ID_EIDE_PIO_IORDY=68,
 ATA_ID_ADDITIONAL_SUPP=69, ATA_ID_QUEUE_DEPTH=75, ATA_ID_SATA_CAPABILITY=76,
 ATA_ID_SATA_CAPABILITY_2=77, ATA_ID_FEATURE_SUPP=78, ATA_ID_MAJOR_VER=80,
 ATA_ID_COMMAND_SET_1=82, ATA_ID_COMMAND_SET_2=83, ATA_ID_CFSSE=84, ATA_ID_CFS_ENABLE_1=85,
 ATA_ID_CFS_ENABLE_2=86, ATA_ID_CSF_DEFAULT=87, ATA_ID_UDMA_MODES=88, ATA_ID_HW_CONFIG=93,
 ATA_ID_SPG=98, ATA_ID_LBA_CAPACITY_2=100, ATA_ID_MAX_PAGES_PER_DSM=105, ATA_ID_SECTOR_SIZE=106,
 ATA_ID_WWN=108, ATA_ID_LOGICAL_SECTOR_SIZE=117, ATA_ID_COMMAND_SET_3=119, ATA_ID_COMMAND_SET_4=120,
 ATA_ID_LAST_LUN=126, ATA_ID_DLF=128, ATA_ID_CSFO=129, ATA_ID_CFA_POWER=160,
 ATA_ID_CFA_KEY_MGMT=162, ATA_ID_CFA_MODES=163, ATA_ID_DATA_SET_MGMT=169, ATA_ID_SCT_CMD_XPORT=206,
 ATA_ID_ROT_SPEED=217, ATA_ID_PIO4=1<<1, ATA_ID_SERNO_LEN=20, ATA_ID_FW_REV_LEN=8,
 ATA_ID_PROD_LEN=40, ATA_ID_WWN_LEN=8, ATA_PCI_CTL_OFS=2,
 ATA_PIO0=1<<0, ATA_PIO1=ATA_PIO0|1<<1, ATA_PIO2=ATA_PIO1|1<<2, ATA_PIO3=ATA_PIO2|1<<3,
 ATA_PIO4=ATA_PIO3|1<<4, ATA_PIO5=ATA_PIO4|1<<5, ATA_PIO6=ATA_PIO5|1<<6, ATA_PIO4_ONLY=1<<4,
 ATA_SWDMA0=1<<0, ATA_SWDMA1=ATA_SWDMA0|1<<1, ATA_SWDMA2=ATA_SWDMA1|1<<2, ATA_SWDMA2_ONLY=1<<2,
 ATA_MWDMA0=1<<0, ATA_MWDMA1=ATA_MWDMA0|1<<1, ATA_MWDMA2=ATA_MWDMA1|1<<2, ATA_MWDMA3=ATA_MWDMA2|1<<3,
 ATA_MWDMA4=ATA_MWDMA3|1<<4, ATA_MWDMA12_ONLY=(1<<1)|(1<<2), ATA_MWDMA2_ONLY=1<<2,
 ATA_UDMA0=1<<0, ATA_UDMA1=ATA_UDMA0|1<<1, ATA_UDMA2=ATA_UDMA1|1<<2, ATA_UDMA3=ATA_UDMA2|1<<3,
 ATA_UDMA4=ATA_UDMA3|1<<4, ATA_UDMA5=ATA_UDMA4|1<<5, ATA_UDMA6=ATA_UDMA5|1<<6, ATA_UDMA7=ATA_UDMA6|1<<7,
 ATA_UDMA24_ONLY=(1<<2)|(1<<4), ATA_UDMA_MASK_40C=ATA_UDMA2, ATA_PRD_SZ=8,
 ATA_PRD_TBL_SZ=ATA_MAX_PRD*ATA_PRD_SZ, ATA_PRD_EOT=1<<31, ATA_DMA_TABLE_OFS=4,
 ATA_DMA_STATUS=2, ATA_DMA_CMD=0, ATA_DMA_WR=1<<3, ATA_DMA_START=1, ATA_DMA_INTR=1<<2,
 ATA_DMA_ERR=1<<1, ATA_DMA_ACTIVE=1, ATA_HOB=1<<7, ATA_NIEN=1<<1, ATA_LBA=1<<6,
 ATA_DEV1=1<<4, ATA_DEVICE_OBS=(1<<7)|(1<<5), ATA_DEVCTL_OBS=1<<3, ATA_BUSY=1<<7,
 ATA_DRDY=1<<6, ATA_DF=1<<5, ATA_DSC=1<<4, ATA_DRQ=1<<3, ATA_CORR=1<<2, ATA_SENSE=1<<1,
 ATA_ERR=1, ATA_SRST=1<<2, ATA_ICRC=1<<7, ATA_BBK=ATA_ICRC, ATA_UNC=1<<6, ATA_MC=1<<5,
 ATA_IDNF=1<<4, ATA_MCR=1<<3, ATA_ABORTED=1<<2, ATA_TRK0NF=1<<1, ATA_AMNF=1,
 ATAPI_LFS=0xf0, ATAPI_EOM=ATA_TRK0NF, ATAPI_ILI=ATA_AMNF, ATAPI_IO=1<<1, ATAPI_COD=1,
 ATA_REG_DATA=0, ATA_REG_ERR=1, ATA_REG_NSECT=2, ATA_REG_LBAL=3, ATA_REG_LBAM=4, ATA_REG_LBAH=5,
 ATA_REG_DEVICE=6, ATA_REG_STATUS=7, ATA_REG_FEATURE=ATA_REG_ERR, ATA_REG_CMD=ATA_REG_STATUS,
 ATA_REG_BYTEL=ATA_REG_LBAM, ATA_REG_BYTEH=ATA_REG_LBAH, ATA_REG_DEVSEL=ATA_REG_DEVICE, ATA_REG_IRQ=ATA_REG_NSECT,
 ATA_CMD_DEV_RESET=0x08, ATA_CMD_CHK_POWER=0xe5, ATA_CMD_STANDBY=0xe2, ATA_CMD_IDLE=0xe3,
 ATA_CMD_EDD=0x90, ATA_CMD_NOP=0, ATA_CMD_FLUSH=0xe7, ATA_CMD_FLUSH_EXT=0xea, ATA_CMD_ID_ATA=0xec,
 ATA_CMD_ID_ATAPI=0xa1, ATA_CMD_SERVICE=0xa2, ATA_CMD_READ=0xc8, ATA_CMD_READ_EXT=0x25,
 ATA_CMD_READ_QUEUED=0x26, ATA_CMD_WRITE=0xca, ATA_CMD_WRITE_EXT=0x35, ATA_CMD_WRITE_QUEUED=0x36,
 ATA_CMD_FPDMA_READ=0x60, ATA_CMD_FPDMA_WRITE=0x61, ATA_CMD_PIO_READ=0x20, ATA_CMD_PIO_READ_EXT=0x24,
 ATA_CMD_PIO_WRITE=0x30, ATA_CMD_PIO_WRITE_EXT=0x34, ATA_CMD_SET_FEATURES=0xef, ATA_CMD_PACKET=0xa0,
 ATA_CMD_VERIFY=0x40, ATA_CMD_VERIFY_EXT=0x42, ATA_CMD_DSM=6, ATA_CMD_SMART=0xb0,
 ATA_SMART_ENABLE=0xd8, ATA_SMART_READ_VALUES=0xd0, ATA_SMART_READ_THRESHOLDS=0xd1,
 ATA_DSM_TRIM=1, ATAPI_PKT_DMA=1, ATAPI_DMADIR=1<<2, ATAPI_CDB_LEN=16,
 SATA_PMP_MAX_PORTS=15, SATA_PMP_CTRL_PORT=15, SATA_PMP_GSCR_DWORDS=128,
 SATA_PMP_GSCR_PROD_ID=0, SATA_PMP_GSCR_REV=1, SATA_PMP_GSCR_PORT_INFO=2,
 SATA_PMP_GSCR_ERROR=32, SATA_PMP_GSCR_ERROR_EN=33, SATA_PMP_GSCR_FEAT=64, SATA_PMP_GSCR_FEAT_EN=96,
 SATA_PMP_PSCR_STATUS=0, SATA_PMP_PSCR_ERROR=1, SATA_PMP_PSCR_CONTROL=2,
 SCR_STATUS=0, SCR_ERROR=1, SCR_CONTROL=2, SCR_ACTIVE=3, SCR_NOTIFICATION=4
}

#[repr(C)]
pub struct ata_bmdma_prd { pub addr: u32, pub flags_len: u32 }

pub const fn ata_id_is_ata(id: &[u16; 256]) -> bool { id[ATA_ID_CONFIG as usize] & (1<<15) == 0 }
pub const fn ata_id_has_lba(id: &[u16; 256]) -> u16 { id[ATA_ID_CAPABILITY as usize] & (1<<9) }
pub const fn ata_id_has_dma(id: &[u16; 256]) -> u16 { id[ATA_ID_CAPABILITY as usize] & (1<<8) }
pub const fn ata_id_has_ncq(id: &[u16; 256]) -> u16 { id[ATA_ID_SATA_CAPABILITY as usize] & (1<<8) }
pub const fn ata_id_queue_depth(id: &[u16; 256]) -> u16 { (id[ATA_ID_QUEUE_DEPTH as usize] & 0x1f)+1 }
pub const fn ata_id_removable(id: &[u16; 256]) -> u16 { id[ATA_ID_CONFIG as usize] & (1<<7) }
pub const fn ata_id_is_locked(id: &[u16; 256]) -> bool { id[ATA_ID_DLF as usize] & 7 == 7 }
pub const fn ata_id_u32(id: &[u16; 256], n: usize) -> u32 { ((id[n+1] as u32)<<16) | id[n] as u32 }
pub const fn ata_id_u64(id: &[u16; 256], n: usize) -> u64 { ((id[n+3] as u64)<<48)|((id[n+2] as u64)<<32)|((id[n+1] as u64)<<16)|id[n] as u64 }
pub fn ata_id_has_hipm(id: &[u16;256]) -> bool { let v=id[76]; v!=0 && v!=0xffff && v&(1<<9)!=0 }
pub fn ata_id_has_lba48(id: &[u16;256]) -> bool { id[83]&0xc000==0x4000 && ata_id_u64(id,100)!=0 && id[83]&(1<<10)!=0 }
pub fn ata_id_major_version(id: &[u16;256]) -> u32 { if id[80]==0xffff {return 0} for m in (1..=14).rev(){if id[80]&(1<<m)!=0{return m}} 0 }
pub fn ata_id_is_sata(id: &[u16;256]) -> bool { id[93]==0 && (id[80] as i16)>=0x20 }
pub fn ata_id_has_trim(id: &[u16;256]) -> bool { ata_id_major_version(id)>=7 && id[169]&1!=0 }
pub fn ata_ok(status: u8) -> bool { status & ((ATA_BUSY|ATA_DRDY|ATA_DF|ATA_DRQ|ATA_ERR) as u8) == ATA_DRDY as u8 }
pub fn lba_28_ok(block:u64,n_block:u32)->bool { block.wrapping_add(n_block as u64)<((1u64<<28)-1) && n_block<=ATA_MAX_SECTORS as u32 }
pub fn lba_48_ok(block:u64,n_block:u32)->bool { block.wrapping_add(n_block as u64).wrapping_sub(1)<(1u64<<48) && n_block<=ATA_MAX_SECTORS_LBA48 as u32 }
pub const fn sata_pmp_gscr_vendor(gscr: &[u32]) -> u32 { gscr[0]&0xffff }
pub const fn sata_pmp_gscr_devid(gscr: &[u32]) -> u32 { gscr[0]>>16 }
pub const fn sata_pmp_gscr_rev(gscr: &[u32]) -> u32 { (gscr[1]>>8)&0xff }
pub const fn sata_pmp_gscr_ports(gscr: &[u32]) -> u32 { gscr[2]&0xf }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
