/* SPDX-License-Identifier: GPL-2.0 */
/* Marvel / EV7 system definitions, translated from core_marvel.h. */

pub const MARVEL_MAX_PIDS: usize = 32;
pub const MARVEL_IRQ_VEC_PE_SHIFT: usize = 10;
pub const MARVEL_IRQ_VEC_IRQ_MASK: usize = (1usize << MARVEL_IRQ_VEC_PE_SHIFT) - 1;
pub const MARVEL_NR_IRQS: usize = 16 + MARVEL_MAX_PIDS * (1usize << MARVEL_IRQ_VEC_PE_SHIFT);

#[repr(C, align(16))]
pub struct ev7_csr { pub csr: core::cell::UnsafeCell<usize> }

#[repr(C)]
pub struct ev7_csrs {
    pub RBOX_CFG: ev7_csr, pub RBOX_NSVC: ev7_csr, pub RBOX_EWVC: ev7_csr,
    pub RBOX_WHAMI: ev7_csr, pub RBOX_TCTL: ev7_csr, pub RBOX_INT: ev7_csr,
    pub RBOX_IMASK: ev7_csr, pub RBOX_IREQ: ev7_csr, pub RBOX_INTQ: ev7_csr,
    pub RBOX_INTA: ev7_csr, pub RBOX_IT: ev7_csr, pub RBOX_SCRATCH1: ev7_csr,
    pub RBOX_SCRATCH2: ev7_csr, pub RBOX_L_ERR: ev7_csr,
}

#[inline] pub fn EV7_MASK40(addr: usize) -> usize { addr & ((1usize << 41) - 1) }
#[inline] pub fn EV7_KERN_ADDR(addr: usize) -> *mut core::ffi::c_void { (IDENT_ADDR | EV7_MASK40(addr)) as *mut core::ffi::c_void }
pub const EV7_PE_MASK: usize = 0x1ff;
#[inline] pub fn EV7_IPE(pe: isize) -> usize { ((!pe as usize) & EV7_PE_MASK) << 35 }
#[inline] pub fn EV7_CSR_PHYS(pe: isize, off: usize) -> usize { EV7_IPE(pe) | (0x7ffcusize << 20) | off }
#[inline] pub fn EV7_CSRS_PHYS(pe: isize) -> usize { EV7_CSR_PHYS(pe, 0) }
#[inline] pub fn EV7_CSR_KERN(pe: isize, off: usize) -> *mut core::ffi::c_void { EV7_KERN_ADDR(EV7_CSR_PHYS(pe, off)) }
#[inline] pub fn EV7_CSRS_KERN(pe: isize) -> *mut core::ffi::c_void { EV7_KERN_ADDR(EV7_CSRS_PHYS(pe)) }

#[repr(C, align(64))]
pub struct io7_csr { pub csr: core::cell::UnsafeCell<usize> }

#[repr(C)]
pub struct io7_ioport_csrs {
    pub POx_CTRL: io7_csr, pub POx_CACHE_CTL: io7_csr, pub POx_TIMER: io7_csr, pub POx_IO_ADR_EXT: io7_csr,
    pub POx_MEM_ADR_EXT: io7_csr, pub POx_XCAL_CTRL: io7_csr, pub rsvd1: [io7_csr; 2],
    pub POx_DM_SOURCE: io7_csr, pub POx_DM_DEST: io7_csr, pub POx_DM_SIZE: io7_csr, pub POx_DM_CTRL: io7_csr, pub rsvd2: [io7_csr; 4],
    pub AGP_CAP_ID: io7_csr, pub AGP_STAT: io7_csr, pub AGP_CMD: io7_csr, pub rsvd3: io7_csr,
    pub POx_MONCTL: io7_csr, pub POx_CTRA: io7_csr, pub POx_CTRB: io7_csr, pub POx_CTR56: io7_csr,
    pub POx_SCRATCH: io7_csr, pub POx_XTRA_A: io7_csr, pub POx_XTRA_TS: io7_csr, pub POx_XTRA_Z: io7_csr,
    pub rsvd4: io7_csr, pub POx_THRESHA: io7_csr, pub POx_THRESHB: io7_csr, pub rsvd5: [io7_csr; 33],
    pub POx_WBASE: [io7_csr;4], pub POx_WMASK: [io7_csr;4], pub POx_TBASE: [io7_csr;4], pub POx_SG_TBIA: io7_csr, pub POx_MSI_WBASE: io7_csr, pub rsvd6: [io7_csr;50],
    pub POx_ERR_SUM: io7_csr, pub POx_FIRST_ERR: io7_csr, pub POx_MSK_HEI: io7_csr, pub POx_TLB_ERR: io7_csr, pub POx_SPL_COMPLT: io7_csr, pub POx_TRANS_SUM: io7_csr, pub POx_FRC_PCI_ERR: io7_csr, pub POx_MULT_ERR: io7_csr, pub rsvd7: [io7_csr;8],
    pub EOI_DAT: io7_csr, pub rsvd8: [io7_csr;7], pub POx_IACK_SPECIAL: io7_csr, pub rsvd9: [io7_csr;103],
}

/* The large IO7 port-7 register block is represented with its exact C field order. */
#[repr(C)] pub struct io7_port7_csrs {
    pub IO_ASIC_REV: io7_csr, pub IO_SYS_REV: io7_csr, pub SER_CHAIN3: io7_csr, pub PO7_RST1: io7_csr, pub PO7_RST2: io7_csr, pub POx_RST: [io7_csr;4], pub IO7_DWNH: io7_csr, pub IO7_MAF: io7_csr, pub IO7_MAF_TO: io7_csr, pub IO7_ACC_CLUMP: io7_csr, pub IO7_PMASK: io7_csr, pub IO7_IOMASK: io7_csr, pub IO7_UPH: io7_csr, pub IO7_UPH_TO: io7_csr, pub RBX_IREQ_OFF: io7_csr, pub RBX_INTA_OFF: io7_csr, pub INT_RTY: io7_csr, pub PO7_MONCTL: io7_csr, pub PO7_CTRA: io7_csr, pub PO7_CTRB: io7_csr, pub PO7_CTR56: io7_csr, pub PO7_SCRATCH: io7_csr, pub PO7_XTRA_A: io7_csr, pub PO7_XTRA_TS: io7_csr, pub PO7_XTRA_Z: io7_csr, pub PO7_PMASK: io7_csr, pub PO7_THRESHA: io7_csr, pub PO7_THRESHB: io7_csr, pub rsvd1: [io7_csr;97], pub PO7_ERROR_SUM: io7_csr, pub PO7_BHOLE_MASK: io7_csr, pub PO7_HEI_MSK: io7_csr, pub PO7_CRD_MSK: io7_csr, pub PO7_UNCRR_SYM: io7_csr, pub PO7_CRRCT_SYM: io7_csr, pub PO7_ERR_PKT: [io7_csr;2], pub PO7_UGBGE_SYM: io7_csr, pub rsbv2: [io7_csr;887], pub PO7_LSI_CTL: [io7_csr;128], pub rsvd3: [io7_csr;123], pub HLT_CTL: io7_csr, pub HPI_CTL: io7_csr, pub CRD_CTL: io7_csr, pub STV_CTL: io7_csr, pub HEI_CTL: io7_csr, pub PO7_MSI_CTL: [io7_csr;16], pub rsvd4: [io7_csr;240],
    pub INT_DIAG: [io7_int_diag;4], pub rsvd5: [io7_csr;125], pub MISC_PND: io7_csr, pub rsvd6: [io7_csr;31], pub MSI_PND: [io7_csr;16], pub rsvd7: [io7_csr;16], pub MSI_CLR: [io7_csr;16],
}
#[repr(C)] pub struct io7_int_diag { pub INT_PND: io7_csr, pub INT_CLR: io7_csr, pub INT_EOI: io7_csr, pub rsvd: [io7_csr;29] }

pub const wbase_m_ena: u64 = 0x1; pub const wbase_m_sg: u64 = 0x2; pub const wbase_m_dac: u64 = 0x4; pub const wbase_m_addr: u64 = 0xFFF00000;
#[repr(C)] pub union IO7_POx_WBASE { pub bits: u64, pub as_long: [u32;2], pub as_quad: u64 }
#[repr(C)] pub union IO7_IID { pub bits: u64, pub as_long: [u32;2], pub as_quad: u64 }

#[inline] pub fn IO7_KERN_ADDR(addr: usize) -> *mut core::ffi::c_void { EV7_KERN_ADDR(addr) }
pub const IO7_NUM_PORTS: usize = 4; pub const IO7_AGP_PORT: usize = 3; pub const IO7_PORT_MASK: usize = 7;
#[inline] pub fn IO7_IPE(pe: isize) -> usize { EV7_IPE(pe) }
#[inline] pub fn IO7_IPORT(port: isize) -> usize { ((!port as usize) & IO7_PORT_MASK) << 32 }
#[inline] pub fn IO7_HOSE(pe:isize, port:isize)->usize { IO7_IPE(pe)|IO7_IPORT(port) }
#[inline] pub fn IO7_MEM_PHYS(pe:isize,port:isize)->usize{IO7_HOSE(pe,port)}
#[inline] pub fn IO7_CONF_PHYS(pe:isize,port:isize)->usize{IO7_HOSE(pe,port)|0xFE000000}
#[inline] pub fn IO7_IO_PHYS(pe:isize,port:isize)->usize{IO7_HOSE(pe,port)|0xFF000000}
#[inline] pub fn IO7_CSR_PHYS(pe:isize,port:isize,off:usize)->usize{IO7_HOSE(pe,port)|0xFF800000|off}
#[inline] pub fn IO7_CSRS_PHYS(pe:isize,port:isize)->usize{IO7_CSR_PHYS(pe,port,0)}
#[inline] pub fn IO7_PORT7_CSRS_PHYS(pe:isize)->usize{IO7_CSR_PHYS(pe,7,0x300000)}
#[inline] pub fn IO7_MEM_KERN(pe:isize,p:isize)->*mut core::ffi::c_void{IO7_KERN_ADDR(IO7_MEM_PHYS(pe,p))}
#[inline] pub fn IO7_CONF_KERN(pe:isize,p:isize)->*mut core::ffi::c_void{IO7_KERN_ADDR(IO7_CONF_PHYS(pe,p))}
#[inline] pub fn IO7_IO_KERN(pe:isize,p:isize)->*mut core::ffi::c_void{IO7_KERN_ADDR(IO7_IO_PHYS(pe,p))}
#[inline] pub fn IO7_CSR_KERN(pe:isize,p:isize,o:usize)->*mut core::ffi::c_void{IO7_KERN_ADDR(IO7_CSR_PHYS(pe,p,o))}
#[inline] pub fn IO7_CSRS_KERN(pe:isize,p:isize)->*mut core::ffi::c_void{IO7_KERN_ADDR(IO7_CSRS_PHYS(pe,p))}
#[inline] pub fn IO7_PORT7_CSRS_KERN(pe:isize)->*mut core::ffi::c_void{IO7_KERN_ADDR(IO7_PORT7_CSRS_PHYS(pe))}
#[inline] pub fn IO7_PLL_RNGA(pll:usize)->usize{(pll>>3)&7} #[inline] pub fn IO7_PLL_RNGB(pll:usize)->usize{(pll>>6)&7}
pub const IO7_MEM_SPACE: usize=2*1024*1024*1024; pub const IO7_IO_SPACE:usize=8*1024*1024; pub const IO7_DAC_OFFSET:usize=1usize<<49;

#[cfg(feature="kernel")]
#[repr(C)] pub struct io7_port { pub io7:*mut io7, pub hose:*mut pci_controller, pub enabled:i32, pub port:u32, pub csrs:*mut io7_ioport_csrs, pub saved_wbase:[usize;4], pub saved_wmask:[usize;4], pub saved_tbase:[usize;4] }
#[cfg(feature="kernel")]
#[repr(C)] pub struct io7 { pub next:*mut io7, pub pe:u32, pub csrs:*mut io7_port7_csrs, pub ports:[io7_port;IO7_NUM_PORTS], pub irq_lock:raw_spinlock_t }

#[cfg(feature="kernel")]
extern "C" { pub static IDENT_ADDR: usize; }

/* Kernel-only structs and inline I/O declarations are retained as declarations. */
#[cfg(feature="kernel")]
extern "C" { pub fn marvel_ioread8(addr:*const core::ffi::c_void)->u8; pub fn marvel_iowrite8(b:u8,addr:*mut core::ffi::c_void); pub fn marvel_ioread16(addr:*const core::ffi::c_void)->u16; pub fn marvel_iowrite16(b:u16,addr:*mut core::ffi::c_void); pub fn marvel_ioremap(addr:usize,size:usize)->*mut core::ffi::c_void; pub fn marvel_iounmap(addr:*mut core::ffi::c_void); pub fn marvel_ioportmap(addr:usize)->*mut core::ffi::c_void; pub fn marvel_is_ioaddr(addr:usize)->i32; pub fn marvel_is_mmio(addr:*const core::ffi::c_void)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
