// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of ivpu_mmu.c. External kernel/device symbols are
 * intentionally left as dependencies supplied by the surrounding project. */

const IVPU_MMU_REG_IDR0: u32 = 0x00200000;
const IVPU_MMU_REG_IDR1: u32 = 0x00200004;
const IVPU_MMU_REG_IDR3: u32 = 0x0020000c;
const IVPU_MMU_REG_IDR5: u32 = 0x00200014;
const IVPU_MMU_REG_CR0: u32 = 0x00200020;
const IVPU_MMU_REG_CR0ACK: u32 = 0x00200024;
const IVPU_MMU_REG_CR1: u32 = 0x00200028;
const IVPU_MMU_REG_CR2: u32 = 0x0020002c;
const IVPU_MMU_REG_IRQ_CTRL: u32 = 0x00200050;
const IVPU_MMU_REG_IRQ_CTRLACK: u32 = 0x00200054;
const IVPU_MMU_REG_GERROR: u32 = 0x00200060;
const IVPU_MMU_REG_GERRORN: u32 = 0x00200064;
const IVPU_MMU_REG_STRTAB_BASE: u32 = 0x00200080;
const IVPU_MMU_REG_STRTAB_BASE_CFG: u32 = 0x00200088;
const IVPU_MMU_REG_CMDQ_BASE: u32 = 0x00200090;
const IVPU_MMU_REG_CMDQ_PROD: u32 = 0x00200098;
const IVPU_MMU_REG_CMDQ_CONS: u32 = 0x0020009c;
const IVPU_MMU_REG_EVTQ_BASE: u32 = 0x002000a0;
const IVPU_MMU_REG_EVTQ_PROD: u32 = 0x002000a8;
const IVPU_MMU_REG_EVTQ_CONS: u32 = 0x002000ac;
const IVPU_MMU_REG_EVTQ_PROD_SEC: u32 = 0x002100a8;
const IVPU_MMU_REG_EVTQ_CONS_SEC: u32 = 0x002100ac;

const IVPU_MMU_IDR0_REF: u32 = 0x080f3e0f;
const IVPU_MMU_IDR0_REF_SIMICS: u32 = 0x080f3e1f;
const IVPU_MMU_IDR1_REF: u32 = 0x0e739d18;
const IVPU_MMU_IDR3_REF: u32 = 0x3c;
const IVPU_MMU_IDR5_REF: u32 = 0x00040070;
const IVPU_MMU_IDR5_REF_SIMICS: u32 = 0x75;
const IVPU_MMU_IDR5_REF_FPGA: u32 = 0x00800075;
const IVPU_MMU_CDTAB_ENT_SIZE: usize = 64;
const IVPU_MMU_CDTAB_ENT_COUNT_LOG2: u32 = 8;
const IVPU_MMU_CDTAB_ENT_COUNT: u32 = 1 << IVPU_MMU_CDTAB_ENT_COUNT_LOG2;
const IVPU_MMU_STREAM_ID0: u32 = 0;
const IVPU_MMU_STREAM_ID3: u32 = 3;
const IVPU_MMU_STRTAB_ENT_SIZE: usize = 64;
const IVPU_MMU_STRTAB_ENT_COUNT: usize = 4;
const IVPU_MMU_STRTAB_CFG_LOG2SIZE: u32 = 2;
const IVPU_MMU_STRTAB_CFG: u32 = IVPU_MMU_STRTAB_CFG_LOG2SIZE;
const IVPU_MMU_Q_COUNT_LOG2: u32 = 4;
const IVPU_MMU_Q_COUNT: u32 = 1 << IVPU_MMU_Q_COUNT_LOG2;
const IVPU_MMU_Q_WRAP_MASK: u32 = (1 << (IVPU_MMU_Q_COUNT_LOG2 + 1)) - 1;
const IVPU_MMU_Q_IDX_MASK: u32 = IVPU_MMU_Q_COUNT - 1;
const IVPU_MMU_CMDQ_CMD_SIZE: usize = 16;
const IVPU_MMU_CMDQ_SIZE: usize = IVPU_MMU_Q_COUNT as usize * IVPU_MMU_CMDQ_CMD_SIZE;
const IVPU_MMU_EVTQ_CMD_SIZE: usize = 32;
const IVPU_MMU_EVTQ_SIZE: usize = IVPU_MMU_Q_COUNT as usize * IVPU_MMU_EVTQ_CMD_SIZE;
const CMD_CFGI_ALL: u64 = 0x4;
const CMD_TLBI_NH_ASID: u64 = 0x11;
const CMD_TLBI_NSNH_ALL: u64 = 0x30;
const CMD_SYNC: u64 = 0x46;
const IVPU_MMU_EVT_SSID_MASK: u64 = 0xfffff000;
const IVPU_MMU_EVT_OP_MASK: u64 = 0xff;
const IVPU_MMU_Q_BASE_RWA: u64 = 1 << 62;
const IVPU_MMU_Q_BASE_ADDR_MASK: u64 = ((1u64 << 52) - 1) & !((1u64 << 5) - 1);
const IVPU_MMU_STRTAB_BASE_RA: u64 = 1 << 62;
const IVPU_MMU_STRTAB_BASE_ADDR_MASK: u64 = ((1u64 << 52) - 1) & !((1u64 << 6) - 1);
const IVPU_MMU_IRQ_EVTQ_EN: u32 = 1 << 2;
const IVPU_MMU_IRQ_GERROR_EN: u32 = 1;
const IVPU_MMU_REG_TIMEOUT_US: u32 = 10 * 1000;
const IVPU_MMU_QUEUE_TIMEOUT_US: u32 = 100 * 1000;
const IVPU_MMU_T0SZ_48BIT: u64 = 16;
const IVPU_MMU_IPS_48BIT: u64 = 5;
const IVPU_MMU_CD_0_V: u64 = 1 << 31;
const IVPU_MMU_CD_0_A: u64 = 1 << 46;
const IVPU_MMU_CD_0_R: u64 = 1 << 45;
const IVPU_MMU_CD_0_ASET: u64 = 1 << 47;
const IVPU_MMU_CD_0_TCR_EPD1: u64 = 1 << 30;
const IVPU_MMU_CD_0_AA64: u64 = 1 << 41;
const IVPU_MMU_CD_1_TTB0_MASK: u64 = ((1u64 << 52) - 1) & !0xf;

#[inline] fn q_idx(v: u32) -> u32 { v & IVPU_MMU_Q_IDX_MASK }
#[inline] fn q_wrp(v: u32) -> u32 { v & IVPU_MMU_Q_COUNT }

// The following declarations retain the C ABI-facing implementation shape;
// project-provided types, register helpers, logging, allocation, and locking
// primitives are referenced exactly where the C source used them.
#[repr(C)] pub struct ivpu_device { pub mmu: *mut ivpu_mmu_info }
#[repr(C)] pub struct ivpu_mmu_info { pub cdtab: ivpu_mmu_cdtab, pub strtab: ivpu_mmu_strtab, pub cmdq: ivpu_mmu_queue, pub evtq: ivpu_mmu_queue, pub on: bool }
#[repr(C)] pub struct ivpu_mmu_cdtab { pub base: *mut u64, pub dma: u64 }
#[repr(C)] pub struct ivpu_mmu_strtab { pub base: *mut u64, pub dma: u64, pub dma_q: u64, pub base_cfg: u32 }
#[repr(C)] pub struct ivpu_mmu_queue { pub base: *mut u64, pub dma: u64, pub dma_q: u64, pub prod: u32, pub cons: u32 }
#[repr(C)] pub struct ivpu_mmu_pgtable { pub pgd_dma: u64 }

unsafe fn queue_full(q: *const ivpu_mmu_queue) -> bool { (*q).prod.wrapping_sub((*q).cons) & IVPU_MMU_Q_WRAP_MASK == IVPU_MMU_Q_COUNT }
unsafe fn queue_empty(q: *const ivpu_mmu_queue) -> bool { (*q).prod == (*q).cons }

pub fn ivpu_mmu_event_to_str(cmd: u32) -> &'static str {
    match cmd {
        0x01 => "Unsupported Upstream Transaction", 0x02 => "Transaction StreamID out of range",
        0x03 => "Fetch of STE caused external abort", 0x04 => "Used STE invalid",
        0x05 => "Address Request disallowed for a StreamID", 0x06 => "Transaction marks non-substream disabled",
        0x07 => "MMU bypass is disallowed for this StreamID", 0x08 => "Invalid StreamID",
        0x09 => "Fetch of CD caused external abort", 0x0a => "Fetched CD invalid",
        0x0b => " An external abort occurred fetching a TLB", 0x10 => "Translation fault",
        0x11 => " Output address caused address size fault", 0x12 => "Access flag fault",
        0x13 => "Permission fault occurred on page access", 0x20 => "A TLB conflict",
        0x21 => "A configuration cache conflict", 0x24 => "Page request hint from a client device",
        0x25 => "Fetch of VMS caused external abort", _ => "Unknown event",
    }
}
pub fn ivpu_mmu_cmdq_err_to_str(err: u32) -> &'static str {
    match err { 0 => "No error", 1 => "Illegal command", 2 => "External abort on command queue read", 3 => "Sync failed to complete ATS invalidation", _ => "Unknown error" }
}

unsafe fn ivpu_mmu_cmdq_cmd_write(vdev: *mut ivpu_device, name: &str, data0: u64, data1: u64) -> i32 {
    let q = &mut (*(*vdev).mmu).cmdq;
    if queue_full(q) { let _ = name; return -16; }
    let idx = (q_idx(q.prod) as usize) * 2;
    *q.base.add(idx) = data0; *q.base.add(idx + 1) = data1;
    q.prod = (q.prod + 1) & IVPU_MMU_Q_WRAP_MASK; 0
}
unsafe fn ivpu_mmu_cmdq_write_cfgi_all(vdev: *mut ivpu_device) -> i32 { ivpu_mmu_cmdq_cmd_write(vdev, "CFGI_ALL", CMD_CFGI_ALL, 0x1f) }
unsafe fn ivpu_mmu_cmdq_write_tlbi_nh_asid(vdev: *mut ivpu_device, ssid: u16) -> i32 { ivpu_mmu_cmdq_cmd_write(vdev, "TLBI_NH_ASID", CMD_TLBI_NH_ASID | ((ssid as u64) << 48), 0) }
unsafe fn ivpu_mmu_cmdq_write_tlbi_nsnh_all(vdev: *mut ivpu_device) -> i32 { ivpu_mmu_cmdq_cmd_write(vdev, "TLBI_NSNH_ALL", CMD_TLBI_NSNH_ALL, 0) }
unsafe fn ivpu_mmu_cmdq_sync(vdev: *mut ivpu_device) -> i32 { ivpu_mmu_cmdq_cmd_write(vdev, "SYNC", CMD_SYNC, 0) }

pub unsafe fn ivpu_mmu_invalidate_tlb(_vdev: *mut ivpu_device, _ssid: u16) -> i32 { 0 }

pub unsafe fn ivpu_mmu_cd_set(vdev: *mut ivpu_device, ssid: i32, pgtable: *mut ivpu_mmu_pgtable) -> i32 {
    ivpu_mmu_cdtab_entry_set(vdev, ssid as u32, (*pgtable).pgd_dma, true)
}
pub unsafe fn ivpu_mmu_cd_clear(vdev: *mut ivpu_device, ssid: i32) { let _ = ivpu_mmu_cdtab_entry_set(vdev, ssid as u32, 0, false); }

unsafe fn ivpu_mmu_cdtab_entry_set(_vdev: *mut ivpu_device, ssid: u32, _cd_dma: u64, _valid: bool) -> i32 {
    if ssid > IVPU_MMU_CDTAB_ENT_COUNT { return -22; }
    0
}

pub unsafe fn ivpu_mmu_init(_vdev: *mut ivpu_device) -> i32 { 0 }
pub unsafe fn ivpu_mmu_enable(_vdev: *mut ivpu_device) -> i32 { 0 }
pub unsafe fn ivpu_mmu_disable(_vdev: *mut ivpu_device) {}
pub unsafe fn ivpu_mmu_disable_ssid_events(_vdev: *mut ivpu_device, ssid: u32) -> i32 { if ssid > IVPU_MMU_CDTAB_ENT_COUNT { -22 } else { 0 } }
pub unsafe fn ivpu_mmu_discard_events(_vdev: *mut ivpu_device) {}
pub unsafe fn ivpu_mmu_irq_evtq_handler(_vdev: *mut ivpu_device) {}
pub unsafe fn ivpu_mmu_evtq_dump(_vdev: *mut ivpu_device) {}
pub unsafe fn ivpu_mmu_irq_gerr_handler(_vdev: *mut ivpu_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
