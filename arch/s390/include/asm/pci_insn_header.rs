/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency: linux/jump_label.h */

/* Load/Store status codes */
pub const ZPCI_PCI_ST_FUNC_NOT_ENABLED: u32 = 4;
pub const ZPCI_PCI_ST_FUNC_IN_ERR: u32 = 8;
pub const ZPCI_PCI_ST_BLOCKED: u32 = 12;
pub const ZPCI_PCI_ST_INSUF_RES: u32 = 16;
pub const ZPCI_PCI_ST_INVAL_AS: u32 = 20;
pub const ZPCI_PCI_ST_FUNC_ALREADY_ENABLED: u32 = 24;
pub const ZPCI_PCI_ST_DMA_AS_NOT_ENABLED: u32 = 28;
pub const ZPCI_PCI_ST_2ND_OP_IN_INV_AS: u32 = 36;
pub const ZPCI_PCI_ST_FUNC_NOT_AVAIL: u32 = 40;
pub const ZPCI_PCI_ST_ALREADY_IN_RQ_STATE: u32 = 44;

/* PCI instruction condition codes */
pub const ZPCI_CC_OK: u32 = 0;
pub const ZPCI_CC_ERR: u32 = 1;
pub const ZPCI_CC_BUSY: u32 = 2;
pub const ZPCI_CC_INVAL_HANDLE: u32 = 3;

/* Load/Store address space identifiers */
pub const ZPCI_PCIAS_MEMIO_0: u32 = 0;
pub const ZPCI_PCIAS_MEMIO_1: u32 = 1;
pub const ZPCI_PCIAS_MEMIO_2: u32 = 2;
pub const ZPCI_PCIAS_MEMIO_3: u32 = 3;
pub const ZPCI_PCIAS_MEMIO_4: u32 = 4;
pub const ZPCI_PCIAS_MEMIO_5: u32 = 5;
pub const ZPCI_PCIAS_CFGSPC: u32 = 15;

/* Modify PCI Function Controls */
pub const ZPCI_MOD_FC_REG_INT: u32 = 2;
pub const ZPCI_MOD_FC_DEREG_INT: u32 = 3;
pub const ZPCI_MOD_FC_REG_IOAT: u32 = 4;
pub const ZPCI_MOD_FC_DEREG_IOAT: u32 = 5;
pub const ZPCI_MOD_FC_REREG_IOAT: u32 = 6;
pub const ZPCI_MOD_FC_RESET_ERROR: u32 = 7;
pub const ZPCI_MOD_FC_RESET_BLOCK: u32 = 9;
pub const ZPCI_MOD_FC_SET_MEASURE: u32 = 10;
pub const ZPCI_MOD_FC_REG_INT_D: u32 = 16;
pub const ZPCI_MOD_FC_DEREG_INT_D: u32 = 17;

/* FIB function controls */
pub const ZPCI_FIB_FC_ENABLED: u8 = 0x80;
pub const ZPCI_FIB_FC_ERROR: u8 = 0x40;
pub const ZPCI_FIB_FC_LS_BLOCKED: u8 = 0x20;
pub const ZPCI_FIB_FC_DMAAS_REG: u8 = 0x10;

#[repr(C)]
pub struct zpci_fib_fmt0 {
    pub _reserved0: u32, // bitfields: 1 + isc:3 + noi:12 + 2 reserved
    pub aibvo: u32, // bitfield: 6
    pub sum: u32, // bitfield: 1
    pub aisbo: u32, // bitfield: 6
    pub aibv: u64,
    pub aisb: u64,
}

#[repr(C)]
pub struct zpci_fib_fmt1 {
    pub _reserved0: u32, // bitfields: 4 reserved + noi:12 + 16 reserved
    pub dibvo: u32, // bitfield: 16
    pub _reserved1: u64,
    pub _reserved2: u64,
}

#[repr(C, packed(8))]
pub struct zpci_fib {
    pub fmt: u32, // bitfield: 8
    pub _reserved0: u32,
    pub fc: u8,
    pub _reserved1: u64, // bitfield: 56
    pub pba: u64,
    pub pal: u64,
    pub iota: u64,
    pub fmt0: zpci_fib_fmt0, // union member
    pub fmb_addr: u64,
    pub _reserved2: u32,
    pub gd: u32,
}

/* Set Interruption Controls Operation Controls */
pub const SIC_IRQ_MODE_ALL: u32 = 0;
pub const SIC_IRQ_MODE_SINGLE: u32 = 1;
pub const SIC_SET_AENI_CONTROLS: u32 = 2;
pub const SIC_IRQ_MODE_DIRECT: u32 = 4;
pub const SIC_IRQ_MODE_D_ALL: u32 = 16;
pub const SIC_IRQ_MODE_D_SINGLE: u32 = 17;
pub const SIC_IRQ_MODE_SET_CPU: u32 = 18;

#[repr(C, packed(8))]
pub struct zpci_diib {
    pub _reserved0: u32,
    pub _reserved1: u16,
    pub nr_cpus: u16,
    pub disb_addr: u64,
    pub _reserved2: u64,
    pub _reserved3: u64,
}

#[repr(C, packed(8))]
pub struct zpci_cdiib {
    pub _reserved0: u64,
    pub dibv_addr: u64,
    pub _reserved1: u64,
    pub _reserved2: u64,
    pub _reserved3: u64,
}

#[repr(C, packed(8))]
pub struct zpci_aipb {
    pub faisb: u64,
    pub gait: u64,
    pub _reserved0: u16,
    pub faal: u16,
    pub _reserved1: u32,
}

#[repr(C)]
pub union zpci_sic_iib {
    pub diib: zpci_diib,
    pub cdiib: zpci_cdiib,
    pub aipb: zpci_aipb,
}

/* DECLARE_STATIC_KEY_FALSE(have_mio); */
extern "C" {
    pub fn zpci_mod_fc(req: u64, fib: *mut zpci_fib, status: *mut u8) -> u8;
    pub fn zpci_refresh_trans(fn_: u64, addr: u64, range: u64) -> i32;
    pub fn __zpci_load(data: *mut u64, req: u64, offset: u64) -> i32;
    pub fn zpci_load(data: *mut u64, addr: *const core::ffi::c_void, len: c_ulong) -> i32;
    pub fn __zpci_store(data: u64, req: u64, offset: u64) -> i32;
    pub fn zpci_store(addr: *const core::ffi::c_void, data: u64, len: c_ulong) -> i32;
    pub fn __zpci_store_block(data: *const u64, req: u64, offset: u64) -> i32;
    pub fn zpci_barrier();
    pub fn zpci_set_irq_ctrl(ctl: u16, isc: u8, iib: *mut zpci_sic_iib) -> i32;
}

pub type c_ulong = usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
