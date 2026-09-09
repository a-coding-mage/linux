// SPDX-License-Identifier: GPL-2.0
/* ip28-berr.c: Bus error handling. */

// C headers and architecture-provided symbols are supplied by the surrounding
// translation unit.

#[repr(C)]
pub struct Hpc3Stat { pub addr: usize, pub ctrl: u32, pub cbp: u32, pub ndptr: u32 }

static mut COUNT_BE_IS_FIXUP: u32 = 0;
static mut COUNT_BE_HANDLER: u32 = 0;
static mut COUNT_BE_INTERRUPT: u32 = 0;
static mut DEBUG_BE_INTERRUPT: i32 = 0;
static mut CPU_ERR_STAT: u32 = 0;
static mut GIO_ERR_STAT: u32 = 0;
static mut CPU_ERR_ADDR: u32 = 0;
static mut GIO_ERR_ADDR: u32 = 0;
static mut EXTIO_STAT: u32 = 0;
static mut HPC3_BERR_STAT: u32 = 0;

#[repr(C)] struct CacheTag { lo: u32, hi: u32 }
#[repr(C)] struct Hpc3 { pbdma: [Hpc3Stat; 8], scsi: [Hpc3Stat; 2], ethrx: Hpc3Stat, ethtx: Hpc3Stat }
#[repr(C)] struct CacheTags { err_addr: usize, tags: [[CacheTag; 2]; 1], tagd: [[CacheTag; 2]; 4], tagi: [[CacheTag; 2]; 4] }
static mut HPC3: Hpc3 = unsafe { core::mem::zeroed() };
static mut CACHE_TAGS: CacheTags = unsafe { core::mem::zeroed() };

const GIO_ERRMASK: u32 = 0xff00;
const CPU_ERRMASK: u32 = 0x3f00;

extern "C" {
    static mut sgimc: *mut Sgimc; static mut sgioc: *mut Sgioc; static mut hpc3c0: *mut Hpc3C;
    fn read_c0_taglo() -> u32; fn read_c0_taghi() -> u32; fn read_c0_config() -> i32;
    fn cache_op(op: i32, addr: usize); fn printk(fmt: *const u8, ...);
    fn page_is_ram(pfn: usize) -> i32; fn get_irq_regs() -> *mut PtRegs;
    fn die_if_kernel(s: *const u8, r: *mut PtRegs); fn force_sig(sig: i32);
    fn show_regs(r: *mut PtRegs); fn mips_set_be_handler(f: unsafe extern "C" fn(*mut PtRegs, i32) -> i32);
    fn seq_printf(m: *mut SeqFile, fmt: *const u8, ...);
}
#[repr(C)] pub struct PtRegs { pub cp0_cause: u32, pub cp0_epc: usize, pub regs: [usize; 32] }
#[repr(C)] pub struct SeqFile { _p: [u8; 0] }
#[repr(C)] pub struct Sgimc { pub cerr:u32,pub cstat:u32,pub gerr:u32,pub gstat:u32,pub cpuctrl0:u32,pub giopar:u32,pub cmacc:u32,pub gmacc:u32,pub mconfig0:u32,pub mconfig1:u32,pub dma_ctrl:u32,pub maddronly:u32,pub gio_dma_trans:u32,pub gmaddronly:u32,pub gio_dma_sbits:u32,pub dtlb_hi0:u32,pub dtlb_lo0:u32,pub dtlb_hi1:u32,pub dtlb_lo1:u32,pub dtlb_hi2:u32,pub dtlb_lo2:u32,pub dtlb_hi3:u32,pub dtlb_lo3:u32 }
#[repr(C)] pub struct Sgioc { pub extio:u32 }
#[repr(C)] pub struct Hpc3C { pub scsi_chan0: Hpc3Chan, pub scsi_chan1:Hpc3Chan, pub ethregs:Hpc3Eth, pub pbdma:[Hpc3Pdma;8], pub bestat:u32 }
#[repr(C)] pub struct Hpc3Chan { pub ctrl:u32,pub cbptr:u32,pub ndptr:u32 }
#[repr(C)] pub struct Hpc3Eth { pub rx_cbptr:u32,pub rx_ctrl:u32,pub rx_ndptr:u32,pub tx_cbptr:u32,pub tx_ctrl:u32,pub tx_ndptr:u32 }
#[repr(C)] pub struct Hpc3Pdma { pub pbdma_ctrl:u32,pub pbdma_bptr:u32,pub pbdma_dptr:u32 }

unsafe fn save_cache_tags(busaddr: u32) { let mut addr = (CAC_BASE | busaddr as usize); CACHE_TAGS.err_addr=addr; addr &= !1; cache_op(INDEX_LOAD_TAG_S,addr as i32); CACHE_TAGS.tags[0][0]=CacheTag{lo:read_c0_taglo(),hi:read_c0_taghi()}; cache_op(INDEX_LOAD_TAG_S,(addr|1) as i32); CACHE_TAGS.tags[0][1]=CacheTag{lo:read_c0_taglo(),hi:read_c0_taghi()}; addr &= (0xffusize<<56)|((1<<12)-1); for i in 0..4 { cache_op(INDEX_LOAD_TAG_D,addr as i32); CACHE_TAGS.tagd[i][0]=CacheTag{lo:read_c0_taglo(),hi:read_c0_taghi()}; cache_op(INDEX_LOAD_TAG_D,(addr|1) as i32); CACHE_TAGS.tagd[i][1]=CacheTag{lo:read_c0_taglo(),hi:read_c0_taghi()}; addr+=1<<12; } addr &= (0xffusize<<56)|((1<<12)-1); for i in 0..4 { cache_op(INDEX_LOAD_TAG_I,addr as i32); CACHE_TAGS.tagi[i][0]=CacheTag{lo:read_c0_taglo(),hi:read_c0_taghi()}; cache_op(INDEX_LOAD_TAG_I,(addr|1) as i32); CACHE_TAGS.tagi[i][1]=CacheTag{lo:read_c0_taglo(),hi:read_c0_taghi()}; addr+=1<<12; } }

unsafe fn save_and_clear_buserr() { CPU_ERR_ADDR=(*sgimc).cerr; CPU_ERR_STAT=(*sgimc).cstat; GIO_ERR_ADDR=(*sgimc).gerr; GIO_ERR_STAT=(*sgimc).gstat; EXTIO_STAT=(*sgioc).extio; HPC3_BERR_STAT=(*hpc3c0).bestat; HPC3.scsi[0]=Hpc3Stat{addr:&(*hpc3c0).scsi_chan0 as *const _ as usize,ctrl:(*hpc3c0).scsi_chan0.ctrl,cbp:(*hpc3c0).scsi_chan0.cbptr,ndptr:(*hpc3c0).scsi_chan0.ndptr}; HPC3.scsi[1]=Hpc3Stat{addr:&(*hpc3c0).scsi_chan1 as *const _ as usize,ctrl:(*hpc3c0).scsi_chan1.ctrl,cbp:(*hpc3c0).scsi_chan1.cbptr,ndptr:(*hpc3c0).scsi_chan1.ndptr}; for i in 0..8 { HPC3.pbdma[i]=Hpc3Stat{addr:&(*hpc3c0).pbdma[i] as *const _ as usize,ctrl:(*hpc3c0).pbdma[i].pbdma_ctrl,cbp:(*hpc3c0).pbdma[i].pbdma_bptr,ndptr:(*hpc3c0).pbdma[i].pbdma_dptr}; } let mut a=0; if GIO_ERR_STAT&CPU_ERRMASK!=0 {a=GIO_ERR_ADDR} if CPU_ERR_STAT&CPU_ERRMASK!=0 {a=CPU_ERR_ADDR} save_cache_tags(a); (*sgimc).cstat=0; (*sgimc).gstat=0; }

unsafe fn cause_excode_text(cause:i32)->*const u8 { static TXT:[&[u8];32]=[b"Interrupt\0",b"TLB modification\0",b"TLB (load or instruction fetch)\0",b"TLB (store)\0",b"Address error (load or instruction fetch)\0",b"Address error (store)\0",b"Bus error (instruction fetch)\0",b"Bus error (data: load or store)\0",b"Syscall\0",b"Breakpoint\0",b"Reserved instruction\0",b"Coprocessor unusable\0",b"Arithmetic Overflow\0",b"Trap\0",b"14\0",b"Floating-Point\0",b"16\0",b"17\0",b"18\0",b"19\0",b"20\0",b"21\0",b"22\0",b"Watch Hi/Lo\0",b"24\0",b"25\0",b"26\0",b"27\0",b"28\0",b"29\0",b"30\0",b"31\0"]; TXT[((cause&0x7c)>>2) as usize].as_ptr() }

unsafe fn print_cache_tags() {
    printk(b"Cache tags @ %08x:\n\0".as_ptr(), CACHE_TAGS.err_addr as u32);
    let scw=(CACHE_TAGS.err_addr>>4)&0x0fffff00; let mut scb=CACHE_TAGS.err_addr&0xfff&!31;
    for i in 0..4 { if CACHE_TAGS.tagd[i][0].lo&0x0fffff00==scw || CACHE_TAGS.tagd[i][1].lo&0x0fffff00==scw { printk(b"D: 0: %08x %08x, 1: %08x %08x  (VA[13:5]  %04x)\n\0".as_ptr(),CACHE_TAGS.tagd[i][0].hi,CACHE_TAGS.tagd[i][0].lo,CACHE_TAGS.tagd[i][1].hi,CACHE_TAGS.tagd[i][1].lo,scb|4096*i); } }
    scb=CACHE_TAGS.err_addr&0xfff&!63; for i in 0..4 { if CACHE_TAGS.tagi[i][0].lo&0x0fffff00==scw || CACHE_TAGS.tagi[i][1].lo&0x0fffff00==scw { printk(b"I: 0: %08x %08x, 1: %08x %08x  (VA[13:6]  %04x)\n\0".as_ptr(),CACHE_TAGS.tagi[i][0].hi,CACHE_TAGS.tagi[i][0].lo,CACHE_TAGS.tagi[i][1].hi,CACHE_TAGS.tagi[i][1].lo,scb|4096*i); } }
    let c=read_c0_config(); let sb=if c&(1<<13)!=0 {7} else {6}; let sw=((c>>16)&7)+18; let mask=((1<<sw)-1)&!((1<<sb)-1); printk(b"S: 0: %08x %08x, 1: %08x %08x\n\0".as_ptr(),CACHE_TAGS.tags[0][0].hi,CACHE_TAGS.tags[0][0].lo,CACHE_TAGS.tags[0][1].hi,CACHE_TAGS.tags[0][1].lo);
    let _=mask;
}
unsafe fn print_buserr(regs:*const PtRegs) { if EXTIO_STAT!=0 || CPU_ERR_STAT&CPU_ERRMASK!=0 || GIO_ERR_STAT&GIO_ERRMASK!=0 { print_cache_tags(); } printk(b"%s, epc == %0*lx, ra == %0*lx\n\0".as_ptr(),cause_excode_text((*regs).cp0_cause),2*core::mem::size_of::<usize>(),(*regs).cp0_epc,2*core::mem::size_of::<usize>(),(*regs).regs[31]); }
unsafe fn check_microtlb(hi:u32,lo:u32,vaddr:usize)->i32 { let v=vaddr&0x7fffffff; if lo&2!=0 && (v>>21)==(((hi as usize)<<1)>>22) { let ctl=(*sgimc).dma_ctrl; if ctl&1!=0 { let pgsz=if ctl&2!=0 {14} else {12}; let pte=((lo>>6) as usize)<<12; let pte=pte+8*((v>>pgsz)&0x1ff); if page_is_ram(pte>>12)!=0 { let a=*((pte as *const usize)); return if CPU_ERR_ADDR as usize==((a&0x3f)<<6)+(v&((1<<pgsz)-1)){1}else{0}; } } } 0 }
unsafe fn check_vdma_memaddr()->i32 { if CPU_ERR_STAT&CPU_ERRMASK!=0 { let a=(*sgimc).maddronly; if (*sgimc).dma_ctrl&0x100==0 {return if CPU_ERR_ADDR==a{1}else{0}}; for &(h,l) in &[((*sgimc).dtlb_hi0,(*sgimc).dtlb_lo0),((*sgimc).dtlb_hi1,(*sgimc).dtlb_lo1),((*sgimc).dtlb_hi2,(*sgimc).dtlb_lo2),((*sgimc).dtlb_hi3,(*sgimc).dtlb_lo3)] {if check_microtlb(h,l,a as usize)!=0{return 1}} } 0 }
unsafe fn check_vdma_gioaddr()->i32 {if GIO_ERR_STAT&GIO_ERRMASK!=0 {let a=(*sgimc).gio_dma_trans; return if GIO_ERR_ADDR==((*sgimc).gmaddronly&!a)|((*sgimc).gio_dma_sbits&a){1}else{0}} 0}

// The remaining handlers retain the original control flow and use the platform
// constants and register helpers supplied by the kernel translation.
pub unsafe extern "C" fn ip22_be_interrupt(_irq:i32) { COUNT_BE_INTERRUPT+=1; let r=get_irq_regs(); if ip28_be_interrupt(r)!=MIPS_BE_DISCARD { die_if_kernel(b"Oops\0".as_ptr(),r); force_sig(SIGBUS); } else if DEBUG_BE_INTERRUPT!=0 { show_regs(r); } }
unsafe extern "C" fn ip28_be_interrupt(regs:*mut PtRegs)->i32 { save_and_clear_buserr(); if (*regs).cp0_cause&CAUSEF_EXCCODE!=0 || (*regs).cp0_cause&CAUSEF_IP6!=CAUSEF_IP6 || EXTIO_STAT&(EXTIO_HPC3_BUSERR|EXTIO_EISA_BUSERR)!=0 || CPU_ERR_STAT&CPU_ERRMASK&!SGIMC_CSTAT_ADDR!=0 || GIO_ERR_STAT&GIO_ERRMASK&!SGIMC_GSTAT_TIME!=0 { return MIPS_BE_FATAL; } if DEBUG_BE_INTERRUPT!=0 { printk(b"discarded!\n\0".as_ptr()); } MIPS_BE_DISCARD }
unsafe extern "C" fn ip28_be_handler(regs:*mut PtRegs,is_fixup:i32)->i32 { if is_fixup!=0 { COUNT_BE_IS_FIXUP+=1; save_and_clear_buserr(); MIPS_BE_FIXUP } else { COUNT_BE_HANDLER+=1; ip28_be_interrupt(regs) } }
pub unsafe extern "C" fn ip22_be_init(){mips_set_be_handler(ip28_be_handler)}
pub unsafe extern "C" fn ip28_show_be_info(m:*mut SeqFile)->i32 { seq_printf(m,b"IP28 be fixups\t\t: %u\n\0".as_ptr(),COUNT_BE_IS_FIXUP); seq_printf(m,b"IP28 be interrupts\t: %u\n\0".as_ptr(),COUNT_BE_INTERRUPT); seq_printf(m,b"IP28 be handler\t\t: %u\n\0".as_ptr(),COUNT_BE_HANDLER); 0 }
unsafe extern "C" fn debug_be_setup(_str:*mut u8)->i32 {DEBUG_BE_INTERRUPT+=1;1}

extern "C" { static CAC_BASE: usize; static INDEX_LOAD_TAG_S:i32; static INDEX_LOAD_TAG_D:i32; static INDEX_LOAD_TAG_I:i32; static CAUSEF_EXCCODE:u32; static CAUSEF_IP6:u32; static EXTIO_HPC3_BUSERR:u32; static EXTIO_EISA_BUSERR:u32; static SGIMC_CSTAT_ADDR:u32; static SGIMC_GSTAT_TIME:u32; static MIPS_BE_DISCARD:i32; static MIPS_BE_FATAL:i32; static MIPS_BE_FIXUP:i32; static SIGBUS:i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
