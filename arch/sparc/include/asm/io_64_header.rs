/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from sparc/include/asm/io_64.h. */

/* External kernel types and ASI constants are supplied by other headers. */

extern "C" {
    pub static mut kern_base: ::core::ffi::c_ulong;
    pub static mut kern_size: ::core::ffi::c_ulong;
}

/* BIO layer definitions. */

#[inline(always)]
pub unsafe fn __raw_readb(addr: *const u8) -> u8 { let mut ret: u8; ::core::arch::asm!("lduba [{addr}] {asi}, {ret} /* pci_raw_readb */", addr = in(reg) addr, asi = const ASI_PHYS_BYPASS_EC_E, ret = lateout(reg_byte) ret); ret }
#[inline(always)]
pub unsafe fn __raw_readw(addr: *const u8) -> u16 { let mut ret: u16; ::core::arch::asm!("lduha [{addr}] {asi}, {ret} /* pci_raw_readw */", addr = in(reg) addr, asi = const ASI_PHYS_BYPASS_EC_E, ret = lateout(reg) ret); ret }
#[inline(always)]
pub unsafe fn __raw_readl(addr: *const u8) -> u32 { let mut ret: u32; ::core::arch::asm!("lduwa [{addr}] {asi}, {ret} /* pci_raw_readl */", addr = in(reg) addr, asi = const ASI_PHYS_BYPASS_EC_E, ret = lateout(reg) ret); ret }
#[inline(always)]
pub unsafe fn __raw_readq(addr: *const u8) -> u64 { let mut ret: u64; ::core::arch::asm!("ldxa [{addr}] {asi}, {ret} /* pci_raw_readq */", addr = in(reg) addr, asi = const ASI_PHYS_BYPASS_EC_E, ret = lateout(reg) ret); ret }
#[inline(always)]
pub unsafe fn __raw_writeb(b: u8, addr: *const u8) { ::core::arch::asm!("stba {b}, [{addr}] {asi} /* pci_raw_writeb */", b = in(reg_byte) b, addr = in(reg) addr, asi = const ASI_PHYS_BYPASS_EC_E); }
#[inline(always)]
pub unsafe fn __raw_writew(w: u16, addr: *const u8) { ::core::arch::asm!("stha {w}, [{addr}] {asi} /* pci_raw_writew */", w = in(reg) w, addr = in(reg) addr, asi = const ASI_PHYS_BYPASS_EC_E); }
#[inline(always)]
pub unsafe fn __raw_writel(l: u32, addr: *const u8) { ::core::arch::asm!("stwa {l}, [{addr}] {asi} /* pci_raw_writel */", l = in(reg) l, addr = in(reg) addr, asi = const ASI_PHYS_BYPASS_EC_E); }
#[inline(always)]
pub unsafe fn __raw_writeq(q: u64, addr: *const u8) { ::core::arch::asm!("stxa {q}, [{addr}] {asi} /* pci_raw_writeq */", q = in(reg) q, addr = in(reg) addr, asi = const ASI_PHYS_BYPASS_EC_E); }

#[inline(always)] pub unsafe fn readb(addr: *const u8) -> u8 { let mut r:u8; ::core::arch::asm!("lduba [{a}] {asi}, {r} /* pci_readb */", a=in(reg)addr, asi=const ASI_PHYS_BYPASS_EC_E_L, r=lateout(reg_byte)r, options(preserves_flags)); r }
#[inline(always)] pub unsafe fn readw(addr:*const u8)->u16 { let mut r:u16; ::core::arch::asm!("lduha [{a}] {asi}, {r} /* pci_readw */", a=in(reg)addr, asi=const ASI_PHYS_BYPASS_EC_E_L, r=lateout(reg)r); r }
#[inline(always)] pub unsafe fn readl(addr:*const u8)->u32 { let mut r:u32; ::core::arch::asm!("lduwa [{a}] {asi}, {r} /* pci_readl */", a=in(reg)addr, asi=const ASI_PHYS_BYPASS_EC_E_L, r=lateout(reg)r); r }
#[inline(always)] pub unsafe fn readq(addr:*const u8)->u64 { let mut r:u64; ::core::arch::asm!("ldxa [{a}] {asi}, {r} /* pci_readq */", a=in(reg)addr, asi=const ASI_PHYS_BYPASS_EC_E_L, r=lateout(reg)r); r }
#[inline(always)] pub unsafe fn writeb(b:u8,addr:*mut u8){::core::arch::asm!("stba {b}, [{a}] {asi} /* pci_writeb */",b=in(reg_byte)b,a=in(reg)addr,asi=const ASI_PHYS_BYPASS_EC_E_L);}
#[inline(always)] pub unsafe fn writew(w:u16,addr:*mut u8){::core::arch::asm!("stha {w}, [{a}] {asi} /* pci_writew */",w=in(reg)w,a=in(reg)addr,asi=const ASI_PHYS_BYPASS_EC_E_L);}
#[inline(always)] pub unsafe fn writel(l:u32,addr:*mut u8){::core::arch::asm!("stwa {l}, [{a}] {asi} /* pci_writel */",l=in(reg)l,a=in(reg)addr,asi=const ASI_PHYS_BYPASS_EC_E_L);}
#[inline(always)] pub unsafe fn writeq(q:u64,addr:*mut u8){::core::arch::asm!("stxa {q}, [{a}] {asi} /* pci_writeq */",q=in(reg)q,a=in(reg)addr,asi=const ASI_PHYS_BYPASS_EC_E_L);}

#[inline(always)] pub unsafe fn inb(addr:usize)->u8{readb(addr as *const u8)}
#[inline(always)] pub unsafe fn inw(addr:usize)->u16{readw(addr as *const u8)}
#[inline(always)] pub unsafe fn inl(addr:usize)->u32{readl(addr as *const u8)}
#[inline(always)] pub unsafe fn outb(b:u8,addr:usize){writeb(b,addr as *mut u8)}
#[inline(always)] pub unsafe fn outw(w:u16,addr:usize){writew(w,addr as *mut u8)}
#[inline(always)] pub unsafe fn outl(l:u32,addr:usize){writel(l,addr as *mut u8)}

extern "C" {
    pub fn outsb(port: usize, buf: *const core::ffi::c_void, count: usize);
    pub fn outsw(port: usize, buf: *const core::ffi::c_void, count: usize);
    pub fn outsl(port: usize, buf: *const core::ffi::c_void, count: usize);
    pub fn insb(port: usize, buf: *mut core::ffi::c_void, count: usize);
    pub fn insw(port: usize, buf: *mut core::ffi::c_void, count: usize);
    pub fn insl(port: usize, buf: *mut core::ffi::c_void, count: usize);
}

#[inline(always)] pub unsafe fn readsb(port:*const u8,buf:*mut core::ffi::c_void,count:usize){insb(port as usize,buf,count)}
#[inline(always)] pub unsafe fn readsw(port:*const u8,buf:*mut core::ffi::c_void,count:usize){insw(port as usize,buf,count)}
#[inline(always)] pub unsafe fn readsl(port:*const u8,buf:*mut core::ffi::c_void,count:usize){insl(port as usize,buf,count)}
#[inline(always)] pub unsafe fn writesb(port:*mut u8,buf:*const core::ffi::c_void,count:usize){outsb(port as usize,buf,count)}
#[inline(always)] pub unsafe fn writesw(port:*mut u8,buf:*const core::ffi::c_void,count:usize){outsw(port as usize,buf,count)}
#[inline(always)] pub unsafe fn writesl(port:*mut u8,buf:*const core::ffi::c_void,count:usize){outsl(port as usize,buf,count)}

pub const IO_SPACE_LIMIT: u64 = 0xffff_ffff_ffff_ffff;

#[inline(always)] pub unsafe fn sbus_readb(a:*const u8)->u8{__raw_readb(a)}
#[inline(always)] pub unsafe fn sbus_readw(a:*const u8)->u16{__raw_readw(a)}
#[inline(always)] pub unsafe fn sbus_readl(a:*const u8)->u32{__raw_readl(a)}
#[inline(always)] pub unsafe fn sbus_readq(a:*const u8)->u64{__raw_readq(a)}
#[inline(always)] pub unsafe fn sbus_writeb(v:u8,a:*const u8){__raw_writeb(v,a)}
#[inline(always)] pub unsafe fn sbus_writew(v:u16,a:*const u8){__raw_writew(v,a)}
#[inline(always)] pub unsafe fn sbus_writel(v:u32,a:*const u8){__raw_writel(v,a)}
#[inline(always)] pub unsafe fn sbus_writeq(v:u64,a:*const u8){__raw_writeq(v,a)}

#[inline(always)] pub unsafe fn sbus_memset_io(mut dst:*mut u8,c:i32,mut n:usize){while n!=0{n-=1;sbus_writeb(c as u8,dst);dst=dst.add(1);}}
#[inline(always)] pub unsafe fn memset_io(mut dst:*mut u8,c:i32,mut n:usize){while n!=0{n-=1;writeb(c as u8,dst);dst=dst.add(1);}}
#[inline(always)] pub unsafe fn sbus_memcpy_fromio(mut dst:*mut u8,mut src:*const u8,mut n:usize){while n!=0{n-=1;*dst=sbus_readb(src);dst=dst.add(1);src=src.add(1);}}
#[inline(always)] pub unsafe fn memcpy_fromio(mut dst:*mut u8,mut src:*const u8,mut n:usize){while n!=0{n-=1;*dst=readb(src);dst=dst.add(1);src=src.add(1);}}
#[inline(always)] pub unsafe fn sbus_memcpy_toio(mut dst:*mut u8,mut src:*const u8,mut n:usize){while n!=0{n-=1;sbus_writeb(*src,dst);dst=dst.add(1);src=src.add(1);}}
#[inline(always)] pub unsafe fn memcpy_toio(mut dst:*mut u8,mut src:*const u8,mut n:usize){while n!=0{n-=1;writeb(*src,dst);dst=dst.add(1);src=src.add(1);}}

/* The following aliases are the C header's direct macro aliases. */
#[inline(always)] pub unsafe fn ioread8(p:*const u8)->u8{readb(p)}
#[inline(always)] pub unsafe fn ioread16(p:*const u8)->u16{readw(p)}
#[inline(always)] pub unsafe fn ioread16be(p:*const u8)->u16{__raw_readw(p)}
#[inline(always)] pub unsafe fn ioread32(p:*const u8)->u32{readl(p)}
#[inline(always)] pub unsafe fn ioread32be(p:*const u8)->u32{__raw_readl(p)}
#[inline(always)] pub unsafe fn iowrite8(v:u8,p:*mut u8){writeb(v,p)}
#[inline(always)] pub unsafe fn iowrite16(v:u16,p:*mut u8){writew(v,p)}
#[inline(always)] pub unsafe fn iowrite16be(v:u16,p:*mut u8){__raw_writew(v,p)}
#[inline(always)] pub unsafe fn iowrite32(v:u32,p:*mut u8){writel(v,p)}
#[inline(always)] pub unsafe fn iowrite32be(v:u32,p:*mut u8){__raw_writel(v,p)}

#[inline(always)] pub unsafe fn ioremap(offset:usize,_size:usize)->*mut u8{offset as *mut u8}
#[inline(always)] pub unsafe fn ioremap_np(_offset:usize,_size:usize)->*mut u8{core::ptr::null_mut()}
#[inline(always)] pub unsafe fn iounmap(_addr:*mut u8){}

#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
extern "C" { pub fn ioport_map(port:usize,nr:u32)->*mut u8; pub fn ioport_unmap(addr:*mut u8); pub fn pci_iounmap(dev:*mut pci_dev,addr:*mut u8); }
#[inline(always)] pub const fn sbus_can_dma_64bit()->i32{1}
#[inline(always)] pub const fn sbus_can_burst64()->i32{1}
#[repr(C)] pub struct device { _private: [u8; 0] }
extern "C" { pub fn sbus_set_sbus64(dev:*mut device, arg:i32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
