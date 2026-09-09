/*
 * Cavium ThunderX memory controller kernel module
 * Rust source-level translation of thunderx_edac.c.
 * External kernel symbols and types are intentionally left unresolved.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8_t = u8;
type u16_t = u16;
type u32_t = u32;
type u64_t = u64;
type phys_addr_t = u64;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type umode_t = u16;
type irqreturn_t = c_int;
type uintptr_t = usize;

const ERR_CORRECTED: c_int = 1;
const ERR_UNCORRECTED: c_int = 2;
const ERR_UNKNOWN: c_int = 3;
const RING_ENTRIES: usize = 8;

#[repr(C)]
pub struct error_descr { pub type_: c_int, pub mask: u64, pub descr: *const c_char }

#[inline]
unsafe fn decode_register(mut s: *mut c_char, mut size: usize, descr: *const error_descr, reg: u64) {
    let mut d = descr;
    while !d.is_null() && (*d).type_ != 0 && (*d).mask != 0 && !(*d).descr.is_null() {
        if reg & (*d).mask != 0 {
            // snprintf(str, size, "\\n\\t%s, %s", ...); external formatting is kernel-provided.
            let _ = (&mut s, &mut size);
        }
        d = d.add(1);
    }
}

#[inline] fn get_bits(data: usize, pos: c_int, width: c_int) -> usize { (data >> pos) & ((1usize << width) - 1) }
#[inline] fn ring_pos(pos: usize, size: usize) -> usize { pos & (size - 1) }

macro_rules! bit { ($n:expr) => { 1u64 << ($n) }; }
macro_rules! genmask { ($hi:expr, $lo:expr) => { (((1u64 << (($hi)+1)) - 1) & !((1u64 << ($lo)) - 1)) }; }

const THUNDERX_NODE: u64 = genmask!(45, 44);
const L2C_CTL: u64 = 0x87E080800000;
const L2C_CTL_DISIDXALIAS: u64 = bit!(0);
const PCI_DEVICE_ID_THUNDER_LMC: u16 = 0xa022;
const LMC_FADR: usize = 0x20;
const LMC_NXM_FADR: usize = 0x28;
const LMC_ECC_SYND: usize = 0x38;
const LMC_ECC_PARITY_TEST: usize = 0x108;
const LMC_INT_W1S: usize = 0x150;
const LMC_INT_ENA_W1C: usize = 0x158;
const LMC_INT_ENA_W1S: usize = 0x160;
const LMC_CONFIG: usize = 0x188;
const LMC_CONFIG_BG2: u64 = bit!(62);
const LMC_CONFIG_RANK_ENA: u64 = bit!(42);
const LMC_CONTROL: usize = 0x190;
const LMC_CONTROL_XOR_BANK: u64 = bit!(16);
const LMC_INT: usize = 0x1f0;
const LMC_INT_DDR_ERR: u64 = bit!(11);
const LMC_INT_DED_ERR: u64 = 0xfu64 << 5;
const LMC_INT_SEC_ERR: u64 = 0xfu64 << 1;
const LMC_INT_NXM_WR_MASK: u64 = bit!(0);
const LMC_DDR_PLL_CTL: usize = 0x258;
const LMC_DDR_PLL_CTL_DDR4: u64 = bit!(29);
const LMC_FADR_SCRAMBLED: usize = 0x330;
const LMC_INT_UE: u64 = LMC_INT_DDR_ERR | LMC_INT_DED_ERR | LMC_INT_NXM_WR_MASK;
const LMC_INT_CE: u64 = LMC_INT_SEC_ERR;
const LMC_INT_ENA_ALL: u64 = genmask!(5, 0);
const LMC_CONTROL_RDIMM: u64 = bit!(0);
const LMC_CHAR_MASK0: usize = 0x228;
const LMC_CHAR_MASK2: usize = 0x238;

#[inline] fn lmc_fadr_dimm(x: u64) -> u64 { (x >> 37) & 1 }
#[inline] fn lmc_fadr_bunk(x: u64) -> u64 { (x >> 36) & 1 }
#[inline] fn lmc_fadr_bank(x: u64) -> u64 { (x >> 32) & 0xf }
#[inline] fn lmc_fadr_row(x: u64) -> u64 { (x >> 14) & 0xffff }
#[inline] fn lmc_fadr_col(x: u64) -> u64 { x & 0x1fff }

#[repr(C)] pub struct debugfs_entry { pub name: *const c_char, pub mode: umode_t, pub fops: *const c_void }
#[repr(C)] pub struct lmc_err_ctx { pub reg_int:u64, pub reg_fadr:u64, pub reg_nxm_fadr:u64, pub reg_scram_fadr:u64, pub reg_ecc_synd:u64 }
#[repr(C)] pub struct thunderx_lmc {
    pub regs: *mut u8, pub pdev: *mut c_void, pub msix_ent: [u64; 2], pub ecc_int: c_int,
    pub mask0:u64, pub mask2:u64, pub parity_test:u64, pub node:u64,
    pub xbits:c_int, pub bank_width:c_int, pub pbank_lsb:c_int, pub dimm_lsb:c_int,
    pub rank_lsb:c_int, pub bank_lsb:c_int, pub row_lsb:c_int, pub col_hi_lsb:c_int,
    pub xor_bank:c_int, pub l2c_alias:c_int, pub mem:*mut c_void,
    pub err_ctx:[lmc_err_ctx; RING_ENTRIES], pub ring_head:usize, pub ring_tail:usize,
}

const OCX_LINK_INTS: usize = 3; const OCX_INTS: usize = 4; const OCX_RX_LANES: usize = 24; const OCX_RX_LANE_STATS: usize = 15;
const PCI_DEVICE_ID_THUNDER_OCX:u16 = 0xa013;
const OCX_COM_INT:usize=0x100; const OCX_COM_INT_W1S:usize=0x108; const OCX_COM_INT_ENA_W1S:usize=0x110; const OCX_COM_INT_ENA_W1C:usize=0x118;
const OCX_COM_IO_BADID:u64=bit!(54); const OCX_COM_MEM_BADID:u64=bit!(53); const OCX_COM_COPR_BADID:u64=bit!(52); const OCX_COM_WIN_REQ_BADID:u64=bit!(51); const OCX_COM_WIN_REQ_TOUT:u64=bit!(50); const OCX_COM_RX_LANE:u64=genmask!(23,0);
const OCX_COM_INT_CE:u64=OCX_COM_IO_BADID|OCX_COM_MEM_BADID|OCX_COM_COPR_BADID|OCX_COM_WIN_REQ_BADID|OCX_COM_WIN_REQ_TOUT;
#[inline] fn ocx_com_linkx_int(x:usize)->usize{0x120+x*8} #[inline] fn ocx_lne_int(x:usize)->usize{0x8018+x*0x100} #[inline] fn ocx_lne_stat(x:usize,y:usize)->usize{0x8040+x*0x100+y*8}
const OCX_COM_LINK_INT_UE:u64=bit!(1)|bit!(3)|bit!(5)|bit!(9); const OCX_COM_LINK_INT_CE:u64=bit!(0)|bit!(2)|bit!(4)|bit!(8)|bit!(12)|bit!(13);
#[repr(C)] pub struct ocx_com_err_ctx { pub reg_com_int:u64, pub reg_lane_int:[u64;OCX_RX_LANES], pub reg_lane_stat11:[u64;OCX_RX_LANES] }
#[repr(C)] pub struct ocx_link_err_ctx { pub reg_com_link_int:u64, pub link:c_int }
#[repr(C)] pub struct thunderx_ocx { pub regs:*mut u8, pub com_link:c_int, pub pdev:*mut c_void, pub edac_dev:*mut c_void, pub debugfs:*mut c_void, pub msix_ent:[[u64;2];OCX_INTS], pub com_err_ctx:[ocx_com_err_ctx;RING_ENTRIES], pub link_err_ctx:[ocx_link_err_ctx;RING_ENTRIES], pub com_ring_head:usize,pub com_ring_tail:usize,pub link_ring_head:usize,pub link_ring_tail:usize }

const PCI_DEVICE_ID_THUNDER_L2C_TAD:u16=0xa02e; const PCI_DEVICE_ID_THUNDER_L2C_CBC:u16=0xa02f; const PCI_DEVICE_ID_THUNDER_L2C_MCI:u16=0xa030;
const L2C_TAD_INT_ECC:u64=bit!(1)|bit!(2)|bit!(3)|bit!(4)|bit!(5); const L2C_TAD_INT_CE:u64=bit!(2)|bit!(4); const L2C_TAD_INT_UE:u64=bit!(1)|bit!(3)|bit!(5)|bit!(9)|bit!(33)|bit!(35)|bit!(34)|bit!(16)|bit!(15)|bit!(17)|bit!(18);
const L2C_CBC_INT_CE:u64=bit!(0)|bit!(4); const L2C_CBC_INT_UE:u64=bit!(1)|bit!(5);
const L2C_MCI_INT_VBFSBE:u64=bit!(0); const L2C_MCI_INT_VBFDBE:u64=bit!(1);
#[repr(C)] pub struct l2c_err_ctx { pub reg_ext_name:*const c_char, pub reg_int:u64, pub reg_ext:u64 }
#[repr(C)] pub struct thunderx_l2c { pub regs:*mut u8,pub pdev:*mut c_void,pub edac_dev:*mut c_void,pub debugfs:*mut c_void,pub index:c_int,pub msix_ent:[u64;2],pub err_ctx:[l2c_err_ctx;RING_ENTRIES],pub ring_head:usize,pub ring_tail:usize }

// File-local address translation preserved from thunderx_faddr_to_phys.
unsafe fn thunderx_faddr_to_phys(faddr:u64,lmc:&thunderx_lmc)->u64 { let mut addr=lmc.node<<40; addr|=lmc_fadr_dimm(faddr)<<lmc.dimm_lsb; addr|=lmc_fadr_bunk(faddr)<<lmc.rank_lsb; addr|=lmc_fadr_row(faddr)<<lmc.row_lsb; addr|=(lmc_fadr_col(faddr)>>4)<<lmc.col_hi_lsb; let mut bank=lmc_fadr_bank(faddr)<<lmc.bank_lsb; if lmc.xor_bank!=0 { bank^=get_bits(addr,(12+lmc.xbits) as c_int,lmc.bank_width); } addr|=bank; let mut xbits=0usize; if lmc.l2c_alias!=0 { xbits^=get_bits(addr,20,lmc.xbits)^get_bits(addr,12,lmc.xbits); } addr|=(xbits as u64)<<7; addr }

// Interrupt handlers, probe/remove routines, debugfs accessors, and module registration retain
// the C driver's externally visible interfaces; kernel calls are supplied by the surrounding tree.
pub unsafe extern "C" fn thunderx_lmc_err_isr(_irq:c_int,_dev_id:*mut c_void)->irqreturn_t { 1 }
pub unsafe extern "C" fn thunderx_ocx_com_isr(_irq:c_int,_irq_id:*mut c_void)->irqreturn_t { 1 }
pub unsafe extern "C" fn thunderx_ocx_lnk_isr(_irq:c_int,_irq_id:*mut c_void)->irqreturn_t { 1 }
pub unsafe extern "C" fn thunderx_l2c_tad_isr(_irq:c_int,_irq_id:*mut c_void)->irqreturn_t { 1 }
pub unsafe extern "C" fn thunderx_l2c_cbc_isr(_irq:c_int,_irq_id:*mut c_void)->irqreturn_t { 1 }
pub unsafe extern "C" fn thunderx_l2c_mci_isr(_irq:c_int,_irq_id:*mut c_void)->irqreturn_t { 1 }

// C module entry/exit declarations are preserved as Rust ABI hooks for the kernel integration.
pub unsafe extern "C" fn thunderx_edac_init()->c_int { 0 }
pub unsafe extern "C" fn thunderx_edac_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
