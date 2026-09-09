/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Generic I/O port emulation. C header translated literally. */

/* Dependencies supplied by the surrounding kernel translation. */
extern "C" {
    fn barrier();
    fn rmb();
    fn wmb();
    fn mmiowb_set_pending();
    fn __pa(x: usize) -> usize;
    fn __va(x: usize) -> *mut core::ffi::c_void;
    fn swab16(x: u16) -> u16;
    fn swab32(x: u32) -> u32;
    fn swab64(x: u64) -> u64;
}

pub type PhysAddr = usize;
pub const IO_SPACE_LIMIT: usize = 0xffff;
pub const PCI_IOBASE: *mut core::ffi::c_void = core::ptr::null_mut();

#[inline(always)] unsafe fn __io_br() { barrier(); }
#[inline(always)] unsafe fn __io_ar(_v: u64) { barrier(); }
#[inline(always)] unsafe fn __io_bw() { barrier(); }
#[inline(always)] unsafe fn __io_aw() { mmiowb_set_pending(); }
#[inline(always)] unsafe fn __io_pbw() { __io_bw(); }
#[inline(always)] unsafe fn __io_paw() { __io_aw(); }
#[inline(always)] unsafe fn __io_pbr() { __io_br(); }
#[inline(always)] unsafe fn __io_par(v: u64) { __io_ar(v); }

#[inline(always)] pub unsafe fn __raw_readb(addr: *const core::ffi::c_void) -> u8 { core::ptr::read_volatile(addr as *const u8) }
#[inline(always)] pub unsafe fn __raw_readw(addr: *const core::ffi::c_void) -> u16 { core::ptr::read_volatile(addr as *const u16) }
#[inline(always)] pub unsafe fn __raw_readl(addr: *const core::ffi::c_void) -> u32 { core::ptr::read_volatile(addr as *const u32) }
#[inline(always)] pub unsafe fn __raw_readq(addr: *const core::ffi::c_void) -> u64 { core::ptr::read_volatile(addr as *const u64) }
#[inline(always)] pub unsafe fn __raw_writeb(v: u8, addr: *mut core::ffi::c_void) { core::ptr::write_volatile(addr as *mut u8, v); }
#[inline(always)] pub unsafe fn __raw_writew(v: u16, addr: *mut core::ffi::c_void) { core::ptr::write_volatile(addr as *mut u16, v); }
#[inline(always)] pub unsafe fn __raw_writel(v: u32, addr: *mut core::ffi::c_void) { core::ptr::write_volatile(addr as *mut u32, v); }
#[inline(always)] pub unsafe fn __raw_writeq(v: u64, addr: *mut core::ffi::c_void) { core::ptr::write_volatile(addr as *mut u64, v); }

#[inline(always)] pub unsafe fn readb(a: *const core::ffi::c_void) -> u8 { __io_br(); let v=__raw_readb(a); __io_ar(v as u64); v }
#[inline(always)] pub unsafe fn readw(a: *const core::ffi::c_void) -> u16 { __io_br(); let v=u16::from_le(__raw_readw(a)); __io_ar(v as u64); v }
#[inline(always)] pub unsafe fn readl(a: *const core::ffi::c_void) -> u32 { __io_br(); let v=u32::from_le(__raw_readl(a)); __io_ar(v as u64); v }
#[inline(always)] pub unsafe fn readq(a: *const core::ffi::c_void) -> u64 { __io_br(); let v=u64::from_le(__raw_readq(a)); __io_ar(v); v }
#[inline(always)] pub unsafe fn writeb(v:u8,a:*mut core::ffi::c_void){__io_bw();__raw_writeb(v,a);__io_aw();}
#[inline(always)] pub unsafe fn writew(v:u16,a:*mut core::ffi::c_void){__io_bw();__raw_writew(v.to_le(),a);__io_aw();}
#[inline(always)] pub unsafe fn writel(v:u32,a:*mut core::ffi::c_void){__io_bw();__raw_writel(v.to_le(),a);__io_aw();}
#[inline(always)] pub unsafe fn writeq(v:u64,a:*mut core::ffi::c_void){__io_bw();__raw_writeq(v.to_le(),a);__io_aw();}
#[inline(always)] pub unsafe fn readb_relaxed(a:*const core::ffi::c_void)->u8{__raw_readb(a)}
#[inline(always)] pub unsafe fn readw_relaxed(a:*const core::ffi::c_void)->u16{u16::from_le(__raw_readw(a))}
#[inline(always)] pub unsafe fn readl_relaxed(a:*const core::ffi::c_void)->u32{u32::from_le(__raw_readl(a))}
#[inline(always)] pub unsafe fn readq_relaxed(a:*const core::ffi::c_void)->u64{u64::from_le(__raw_readq(a))}
#[inline(always)] pub unsafe fn writeb_relaxed(v:u8,a:*mut core::ffi::c_void){__raw_writeb(v,a)}
#[inline(always)] pub unsafe fn writew_relaxed(v:u16,a:*mut core::ffi::c_void){__raw_writew(v.to_le(),a)}
#[inline(always)] pub unsafe fn writel_relaxed(v:u32,a:*mut core::ffi::c_void){__raw_writel(v.to_le(),a)}
#[inline(always)] pub unsafe fn writeq_relaxed(v:u64,a:*mut core::ffi::c_void){__raw_writeq(v.to_le(),a)}

macro_rules! reads { ($n:ident,$t:ty,$r:ident) => { #[inline] pub unsafe fn $n(a:*const core::ffi::c_void,b:*mut core::ffi::c_void,mut c:u32){let mut p=b as *mut $t;while c!=0{*p=$r(a);p=p.add(1);c-=1;}} }; }
macro_rules! writes { ($n:ident,$t:ty,$w:ident) => { #[inline] pub unsafe fn $n(a:*mut core::ffi::c_void,b:*const core::ffi::c_void,mut c:u32){let mut p=b as *const $t;while c!=0{$w(*p,a);p=p.add(1);c-=1;}} }; }
reads!(readsb,u8,__raw_readb); reads!(readsw,u16,__raw_readw); reads!(readsl,u32,__raw_readl); reads!(readsq,u64,__raw_readq);
// The C header's writes{b,w,l,q} use native-endian raw accesses.
writes!(writesb,u8,__raw_writeb); writes!(writesw,u16,__raw_writew); writes!(writesl,u32,__raw_writel); writes!(writesq,u64,__raw_writeq);

#[inline] pub unsafe fn _inb(a:usize)->u8{__io_pbr();let v=__raw_readb((PCI_IOBASE as *mut u8).add(a) as *const _);__io_par(v as u64);v}
#[inline] pub unsafe fn _inw(a:usize)->u16{__io_pbr();let v=u16::from_le(__raw_readw((PCI_IOBASE as *mut u8).add(a) as *const _));__io_par(v as u64);v}
#[inline] pub unsafe fn _inl(a:usize)->u32{__io_pbr();let v=u32::from_le(__raw_readl((PCI_IOBASE as *mut u8).add(a) as *const _));__io_par(v as u64);v}
#[inline] pub unsafe fn _outb(v:u8,a:usize){__io_pbw();__raw_writeb(v,(PCI_IOBASE as *mut u8).add(a) as *mut _);__io_paw()}
#[inline] pub unsafe fn _outw(v:u16,a:usize){__io_pbw();__raw_writew(v.to_le(),(PCI_IOBASE as *mut u8).add(a) as *mut _);__io_paw()}
#[inline] pub unsafe fn _outl(v:u32,a:usize){__io_pbw();__raw_writel(v.to_le(),(PCI_IOBASE as *mut u8).add(a) as *mut _);__io_paw()}
pub use _inb as inb; pub use _inw as inw; pub use _inl as inl; pub use _outb as outb; pub use _outw as outw; pub use _outl as outl;
#[inline] pub unsafe fn inb_p(a:usize)->u8{inb(a)} #[inline] pub unsafe fn inw_p(a:usize)->u16{inw(a)} #[inline] pub unsafe fn inl_p(a:usize)->u32{inl(a)}
#[inline] pub unsafe fn outb_p(v:u8,a:usize){outb(v,a)} #[inline] pub unsafe fn outw_p(v:u16,a:usize){outw(v,a)} #[inline] pub unsafe fn outl_p(v:u32,a:usize){outl(v,a)}
#[inline] pub unsafe fn insb(a:usize,b:*mut core::ffi::c_void,c:u32){readsb((PCI_IOBASE as *mut u8).add(a) as *const _,b,c)}
#[inline] pub unsafe fn insw(a:usize,b:*mut core::ffi::c_void,c:u32){readsw((PCI_IOBASE as *mut u8).add(a) as *const _,b,c)}
#[inline] pub unsafe fn insl(a:usize,b:*mut core::ffi::c_void,c:u32){readsl((PCI_IOBASE as *mut u8).add(a) as *const _,b,c)}
#[inline] pub unsafe fn outsb(a:usize,b:*const core::ffi::c_void,c:u32){writesb((PCI_IOBASE as *mut u8).add(a) as *mut _,b,c)}
#[inline] pub unsafe fn outsw(a:usize,b:*const core::ffi::c_void,c:u32){writesw((PCI_IOBASE as *mut u8).add(a) as *mut _,b,c)}
#[inline] pub unsafe fn outsl(a:usize,b:*const core::ffi::c_void,c:u32){writesl((PCI_IOBASE as *mut u8).add(a) as *mut _,b,c)}
#[inline] pub unsafe fn insb_p(a:usize,b:*mut core::ffi::c_void,c:u32){insb(a,b,c)} #[inline] pub unsafe fn insw_p(a:usize,b:*mut core::ffi::c_void,c:u32){insw(a,b,c)} #[inline] pub unsafe fn insl_p(a:usize,b:*mut core::ffi::c_void,c:u32){insl(a,b,c)}
#[inline] pub unsafe fn outsb_p(a:usize,b:*const core::ffi::c_void,c:u32){outsb(a,b,c)} #[inline] pub unsafe fn outsw_p(a:usize,b:*const core::ffi::c_void,c:u32){outsw(a,b,c)} #[inline] pub unsafe fn outsl_p(a:usize,b:*const core::ffi::c_void,c:u32){outsl(a,b,c)}

#[inline] pub unsafe fn ioread8(a:*const core::ffi::c_void)->u8{readb(a)} #[inline] pub unsafe fn ioread16(a:*const core::ffi::c_void)->u16{readw(a)} #[inline] pub unsafe fn ioread32(a:*const core::ffi::c_void)->u32{readl(a)} #[inline] pub unsafe fn ioread64(a:*const core::ffi::c_void)->u64{readq(a)}
#[inline] pub unsafe fn iowrite8(v:u8,a:*mut core::ffi::c_void){writeb(v,a)} #[inline] pub unsafe fn iowrite16(v:u16,a:*mut core::ffi::c_void){writew(v,a)} #[inline] pub unsafe fn iowrite32(v:u32,a:*mut core::ffi::c_void){writel(v,a)} #[inline] pub unsafe fn iowrite64(v:u64,a:*mut core::ffi::c_void){writeq(v,a)}
#[inline] pub unsafe fn ioread16be(a:*const core::ffi::c_void)->u16{swab16(readw(a))} #[inline] pub unsafe fn ioread32be(a:*const core::ffi::c_void)->u32{swab32(readl(a))} #[inline] pub unsafe fn ioread64be(a:*const core::ffi::c_void)->u64{swab64(readq(a))}
#[inline] pub unsafe fn iowrite16be(v:u16,a:*mut core::ffi::c_void){writew(swab16(v),a)} #[inline] pub unsafe fn iowrite32be(v:u32,a:*mut core::ffi::c_void){writel(swab32(v),a)} #[inline] pub unsafe fn iowrite64be(v:u64,a:*mut core::ffi::c_void){writeq(swab64(v),a)}
#[inline] pub unsafe fn ioread8_rep(a:*const core::ffi::c_void,b:*mut core::ffi::c_void,c:u32){readsb(a,b,c)} #[inline] pub unsafe fn ioread16_rep(a:*const core::ffi::c_void,b:*mut core::ffi::c_void,c:u32){readsw(a,b,c)} #[inline] pub unsafe fn ioread32_rep(a:*const core::ffi::c_void,b:*mut core::ffi::c_void,c:u32){readsl(a,b,c)} #[inline] pub unsafe fn ioread64_rep(a:*const core::ffi::c_void,b:*mut core::ffi::c_void,c:u32){readsq(a,b,c)}
#[inline] pub unsafe fn iowrite8_rep(a:*mut core::ffi::c_void,b:*const core::ffi::c_void,c:u32){writesb(a,b,c)} #[inline] pub unsafe fn iowrite16_rep(a:*mut core::ffi::c_void,b:*const core::ffi::c_void,c:u32){writesw(a,b,c)} #[inline] pub unsafe fn iowrite32_rep(a:*mut core::ffi::c_void,b:*const core::ffi::c_void,c:u32){writesl(a,b,c)} #[inline] pub unsafe fn iowrite64_rep(a:*mut core::ffi::c_void,b:*const core::ffi::c_void,c:u32){writesq(a,b,c)}

#[inline] pub unsafe fn virt_to_phys(a:*mut core::ffi::c_void)->usize{__pa(a as usize)}
#[inline] pub unsafe fn phys_to_virt(a:usize)->*mut core::ffi::c_void{__va(a)}
#[inline] pub unsafe fn ioremap(offset:PhysAddr,_size:usize)->*mut core::ffi::c_void{offset as *mut _}
#[inline] pub unsafe fn iounmap(_addr:*mut core::ffi::c_void){}
#[inline] pub unsafe fn ioremap_uc(_offset:PhysAddr,_size:usize)->*mut core::ffi::c_void{core::ptr::null_mut()}
#[inline] pub unsafe fn ioremap_np(_offset:PhysAddr,_size:usize)->*mut core::ffi::c_void{core::ptr::null_mut()}
#[inline] pub unsafe fn xlate_dev_mem_ptr(a:PhysAddr)->*mut core::ffi::c_void{__va(a)}
#[inline] pub unsafe fn unxlate_dev_mem_ptr(_p:PhysAddr,_a:*mut core::ffi::c_void){}

extern "C" { pub fn memset_io(a:*mut core::ffi::c_void,v:i32,c:usize); pub fn memcpy_fromio(d:*mut core::ffi::c_void,s:*const core::ffi::c_void,c:usize); pub fn memcpy_toio(d:*mut core::ffi::c_void,s:*const core::ffi::c_void,c:usize); pub fn devmem_is_allowed(pfn:usize)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
