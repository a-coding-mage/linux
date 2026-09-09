// SPDX-License-Identifier: GPL-2.0
/* iomap.c - Implement iomap interface for PA-RISC */

// Kernel-provided types and functions are intentionally referenced as externals.
type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;

#[cfg(target_pointer_width = "64")]
const IOPORT_MAP_BASE: usize = 8usize << 60;
#[cfg(not(target_pointer_width = "64"))]
const IOPORT_MAP_BASE: usize = 8usize << 28;

#[cfg(target_pointer_width = "64")]
#[inline]
fn indirect_addr(addr: *const core::ffi::c_void) -> bool { (addr as usize & (1usize << 63)) != 0 }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
fn indirect_addr(addr: *const core::ffi::c_void) -> bool { (addr as usize & (1usize << 31)) != 0 }
#[cfg(target_pointer_width = "64")]
#[inline]
fn addr_to_region(addr: *const core::ffi::c_void) -> usize { (addr as usize >> 60) & 7 }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
fn addr_to_region(addr: *const core::ffi::c_void) -> usize { (addr as usize >> 28) & 7 }

extern "C" {
    fn inb(port: usize) -> U32; fn inw(port: usize) -> U32; fn inl(port: usize) -> U32;
    fn outb(v: U8, port: usize); fn outw(v: U16, port: usize); fn outl(v: U32, port: usize);
    fn insb(port: usize, dst: *mut core::ffi::c_void, n: usize);
    fn insw(port: usize, dst: *mut core::ffi::c_void, n: usize);
    fn insl(port: usize, dst: *mut core::ffi::c_void, n: usize);
    fn outsb(port: usize, src: *const core::ffi::c_void, n: usize);
    fn outsw(port: usize, src: *const core::ffi::c_void, n: usize);
    fn outsl(port: usize, src: *const core::ffi::c_void, n: usize);
    fn readb(a: *const core::ffi::c_void) -> U8; fn readw(a: *const core::ffi::c_void) -> U16;
    fn readl(a: *const core::ffi::c_void) -> U32; fn readq(a: *const core::ffi::c_void) -> U64;
    fn __raw_readb(a: *const core::ffi::c_void) -> U8; fn __raw_readw(a: *const core::ffi::c_void) -> U16;
    fn __raw_readl(a: *const core::ffi::c_void) -> U32; fn __raw_readq(a: *const core::ffi::c_void) -> U64;
    fn writeb(v: U8, a: *mut core::ffi::c_void); fn writew(v: U16, a: *mut core::ffi::c_void);
    fn writel(v: U32, a: *mut core::ffi::c_void); fn writeq(v: U64, a: *mut core::ffi::c_void);
    fn __raw_writeb(v: U8, a: *mut core::ffi::c_void); fn __raw_writew(v: U16, a: *mut core::ffi::c_void);
    fn __raw_writel(v: U32, a: *mut core::ffi::c_void); fn __raw_writeq(v: U64, a: *mut core::ffi::c_void);
    fn iounmap(a: *mut core::ffi::c_void);
}

#[repr(C)]
struct IomapOps {
    read8: unsafe extern "C" fn(*const core::ffi::c_void) -> U32,
    read16: unsafe extern "C" fn(*const core::ffi::c_void) -> U32,
    read16be: unsafe extern "C" fn(*const core::ffi::c_void) -> U32,
    read32: unsafe extern "C" fn(*const core::ffi::c_void) -> U32,
    read32be: unsafe extern "C" fn(*const core::ffi::c_void) -> U32,
    #[cfg(target_pointer_width = "64")] read64: unsafe extern "C" fn(*const core::ffi::c_void) -> U64,
    #[cfg(target_pointer_width = "64")] read64be: unsafe extern "C" fn(*const core::ffi::c_void) -> U64,
    write8: unsafe extern "C" fn(U8, *mut core::ffi::c_void), write16: unsafe extern "C" fn(U16, *mut core::ffi::c_void),
    write16be: unsafe extern "C" fn(U16, *mut core::ffi::c_void), write32: unsafe extern "C" fn(U32, *mut core::ffi::c_void),
    write32be: unsafe extern "C" fn(U32, *mut core::ffi::c_void),
    #[cfg(target_pointer_width = "64")] write64: unsafe extern "C" fn(U64, *mut core::ffi::c_void),
    #[cfg(target_pointer_width = "64")] write64be: unsafe extern "C" fn(U64, *mut core::ffi::c_void),
    read8r: unsafe extern "C" fn(*const core::ffi::c_void, *mut core::ffi::c_void, usize),
    read16r: unsafe extern "C" fn(*const core::ffi::c_void, *mut core::ffi::c_void, usize),
    read32r: unsafe extern "C" fn(*const core::ffi::c_void, *mut core::ffi::c_void, usize),
    write8r: unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize),
    write16r: unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize),
    write32r: unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize),
}

#[inline] unsafe fn port(a: *const core::ffi::c_void) -> usize { a as usize & 0xffffff }
unsafe extern "C" fn ioport_read8(a:*const core::ffi::c_void)->U32{inb(port(a))}
unsafe extern "C" fn ioport_read16(a:*const core::ffi::c_void)->U32{inw(port(a))}
unsafe extern "C" fn ioport_read32(a:*const core::ffi::c_void)->U32{inl(port(a))}
unsafe extern "C" fn ioport_write8(v:U8,a:*mut core::ffi::c_void){outb(v,port(a))}
unsafe extern "C" fn ioport_write16(v:U16,a:*mut core::ffi::c_void){outw(v,port(a))}
unsafe extern "C" fn ioport_write32(v:U32,a:*mut core::ffi::c_void){outl(v,port(a))}
unsafe extern "C" fn ioport_read8r(a:*const core::ffi::c_void,d:*mut core::ffi::c_void,n:usize){insb(port(a),d,n)}
unsafe extern "C" fn ioport_read16r(a:*const core::ffi::c_void,d:*mut core::ffi::c_void,n:usize){insw(port(a),d,n)}
unsafe extern "C" fn ioport_read32r(a:*const core::ffi::c_void,d:*mut core::ffi::c_void,n:usize){insl(port(a),d,n)}
unsafe extern "C" fn ioport_write8r(a:*mut core::ffi::c_void,s:*const core::ffi::c_void,n:usize){outsb(port(a),s,n)}
unsafe extern "C" fn ioport_write16r(a:*mut core::ffi::c_void,s:*const core::ffi::c_void,n:usize){outsw(port(a),s,n)}
unsafe extern "C" fn ioport_write32r(a:*mut core::ffi::c_void,s:*const core::ffi::c_void,n:usize){outsl(port(a),s,n)}

unsafe extern "C" fn iomem_read8(a:*const core::ffi::c_void)->U32{readb(a) as U32}
unsafe extern "C" fn iomem_read16(a:*const core::ffi::c_void)->U32{readw(a) as U32}
unsafe extern "C" fn iomem_read16be(a:*const core::ffi::c_void)->U32{__raw_readw(a) as U32}
unsafe extern "C" fn iomem_read32(a:*const core::ffi::c_void)->U32{readl(a)}
unsafe extern "C" fn iomem_read32be(a:*const core::ffi::c_void)->U32{__raw_readl(a)}
#[cfg(target_pointer_width="64")] unsafe extern "C" fn iomem_read64(a:*const core::ffi::c_void)->U64{readq(a)}
#[cfg(target_pointer_width="64")] unsafe extern "C" fn iomem_read64be(a:*const core::ffi::c_void)->U64{__raw_readq(a)}
unsafe extern "C" fn iomem_write8(v:U8,a:*mut core::ffi::c_void){writeb(v,a)}
unsafe extern "C" fn iomem_write16(v:U16,a:*mut core::ffi::c_void){writew(v,a)}
unsafe extern "C" fn iomem_write16be(v:U16,a:*mut core::ffi::c_void){__raw_writew(v,a)}
unsafe extern "C" fn iomem_write32(v:U32,a:*mut core::ffi::c_void){writel(v,a)}
unsafe extern "C" fn iomem_write32be(v:U32,a:*mut core::ffi::c_void){__raw_writel(v,a)}
#[cfg(target_pointer_width="64")] unsafe extern "C" fn iomem_write64(v:U64,a:*mut core::ffi::c_void){writeq(v,a)}
#[cfg(target_pointer_width="64")] unsafe extern "C" fn iomem_write64be(v:U64,a:*mut core::ffi::c_void){__raw_writeq(v,a)}

unsafe extern "C" fn iomem_read8r(a:*const core::ffi::c_void,d:*mut core::ffi::c_void,mut n:usize){while n!=0{*(d as *mut U8)=__raw_readb(a);d=d.add(1);n-=1}}
unsafe extern "C" fn iomem_read16r(a:*const core::ffi::c_void,d:*mut core::ffi::c_void,mut n:usize){while n!=0{*(d as *mut U16)=__raw_readw(a);d=d.add(2);n-=1}}
unsafe extern "C" fn iomem_read32r(a:*const core::ffi::c_void,d:*mut core::ffi::c_void,mut n:usize){while n!=0{*(d as *mut U32)=__raw_readl(a);d=d.add(4);n-=1}}
unsafe extern "C" fn iomem_write8r(a:*mut core::ffi::c_void,mut s:*const core::ffi::c_void,mut n:usize){while n!=0{__raw_writeb(*(s as *const U8),a);s=s.add(1);n-=1}}
unsafe extern "C" fn iomem_write16r(a:*mut core::ffi::c_void,mut s:*const core::ffi::c_void,mut n:usize){while n!=0{__raw_writew(*(s as *const U16),a);s=s.add(2);n-=1}}
unsafe extern "C" fn iomem_write32r(a:*mut core::ffi::c_void,mut s:*const core::ffi::c_void,mut n:usize){while n!=0{__raw_writel(*(s as *const U32),a);s=s.add(4);n-=1}}

static IOport_OPS: IomapOps = IomapOps { read8:ioport_read8,read16:ioport_read16,read16be:ioport_read16,read32:ioport_read32,read32be:ioport_read32,
    #[cfg(target_pointer_width="64")] read64:iomem_read64, #[cfg(target_pointer_width="64")] read64be:iomem_read64be,
    write8:ioport_write8,write16:ioport_write16,write16be:ioport_write16,write32:ioport_write32,write32be:ioport_write32,
    #[cfg(target_pointer_width="64")] write64:iomem_write64, #[cfg(target_pointer_width="64")] write64be:iomem_write64be,
    read8r:ioport_read8r,read16r:ioport_read16r,read32r:ioport_read32r,write8r:ioport_write8r,write16r:ioport_write16r,write32r:ioport_write32r };
static IOMEM_OPS: IomapOps = IomapOps { read8:iomem_read8,read16:iomem_read16,read16be:iomem_read16be,read32:iomem_read32,read32be:iomem_read32be,
    #[cfg(target_pointer_width="64")] read64:iomem_read64, #[cfg(target_pointer_width="64")] read64be:iomem_read64be,
    write8:iomem_write8,write16:iomem_write16,write16be:iomem_write16be,write32:iomem_write32,write32be:iomem_write32be,
    #[cfg(target_pointer_width="64")] write64:iomem_write64, #[cfg(target_pointer_width="64")] write64be:iomem_write64be,
    read8r:iomem_read8r,read16r:iomem_read16r,read32r:iomem_read32r,write8r:iomem_write8r,write16r:iomem_write16r,write32r:iomem_write32r };
static IOMAP_OPS: [Option<&'static IomapOps>;8] = [Some(&IOport_OPS),None,None,None,None,None,None,Some(&IOMEM_OPS)];

#[inline] unsafe fn ops(a:*const core::ffi::c_void)->&'static IomapOps{IOMAP_OPS[addr_to_region(a)].unwrap()}
unsafe fn load<T:Copy>(a:*const core::ffi::c_void)->T{*(a as *const T)}
unsafe fn store<T>(a:*mut core::ffi::c_void,v:T){*(a as *mut T)=v}

#[no_mangle] pub unsafe extern "C" fn ioread8(a:*const core::ffi::c_void)->U32{if indirect_addr(a){(ops(a).read8)(a)}else{load::<U8>(a) as U32}}
#[no_mangle] pub unsafe extern "C" fn ioread16(a:*const core::ffi::c_void)->U32{if indirect_addr(a){(ops(a).read16)(a)}else{U16::from_le(load(a)) as U32}}
#[no_mangle] pub unsafe extern "C" fn ioread16be(a:*const core::ffi::c_void)->U32{if indirect_addr(a){(ops(a).read16be)(a)}else{load::<U16>(a) as U32}}
#[no_mangle] pub unsafe extern "C" fn ioread32(a:*const core::ffi::c_void)->U32{if indirect_addr(a){(ops(a).read32)(a)}else{U32::from_le(load(a))}}
#[no_mangle] pub unsafe extern "C" fn ioread32be(a:*const core::ffi::c_void)->U32{if indirect_addr(a){(ops(a).read32be)(a)}else{load(a)}}
#[cfg(target_pointer_width="64")] #[no_mangle] pub unsafe extern "C" fn ioread64(a:*const core::ffi::c_void)->U64{if indirect_addr(a){(ops(a).read64)(a)}else{U64::from_le(load(a))}}
#[cfg(target_pointer_width="64")] #[no_mangle] pub unsafe extern "C" fn ioread64be(a:*const core::ffi::c_void)->U64{if indirect_addr(a){(ops(a).read64be)(a)}else{load(a)}}

#[no_mangle] pub unsafe extern "C" fn iowrite8(v:U8,a:*mut core::ffi::c_void){if indirect_addr(a){(ops(a).write8)(v,a)}else{store(a,v)}}
#[no_mangle] pub unsafe extern "C" fn iowrite16(v:U16,a:*mut core::ffi::c_void){if indirect_addr(a){(ops(a).write16)(v,a)}else{store(a,v.to_le())}}
#[no_mangle] pub unsafe extern "C" fn iowrite16be(v:U16,a:*mut core::ffi::c_void){if indirect_addr(a){(ops(a).write16be)(v,a)}else{store(a,v)}}
#[no_mangle] pub unsafe extern "C" fn iowrite32(v:U32,a:*mut core::ffi::c_void){if indirect_addr(a){(ops(a).write32)(v,a)}else{store(a,v.to_le())}}
#[no_mangle] pub unsafe extern "C" fn iowrite32be(v:U32,a:*mut core::ffi::c_void){if indirect_addr(a){(ops(a).write32be)(v,a)}else{store(a,v)}}
#[cfg(target_pointer_width="64")] #[no_mangle] pub unsafe extern "C" fn iowrite64(v:U64,a:*mut core::ffi::c_void){if indirect_addr(a){(ops(a).write64)(v,a)}else{store(a,v.to_le())}}
#[cfg(target_pointer_width="64")] #[no_mangle] pub unsafe extern "C" fn iowrite64be(v:U64,a:*mut core::ffi::c_void){if indirect_addr(a){(ops(a).write64be)(v,a)}else{store(a,v)}}

#[no_mangle] pub unsafe extern "C" fn ioread8_rep(a:*const core::ffi::c_void,mut d:*mut core::ffi::c_void,mut n:usize){if indirect_addr(a){(ops(a).read8r)(a,d,n)}else{while n!=0{*(d as *mut U8)=load(a);d=d.add(1);n-=1}}}
#[no_mangle] pub unsafe extern "C" fn ioread16_rep(a:*const core::ffi::c_void,mut d:*mut core::ffi::c_void,mut n:usize){if indirect_addr(a){(ops(a).read16r)(a,d,n)}else{while n!=0{*(d as *mut U16)=load(a);d=d.add(2);n-=1}}}
#[no_mangle] pub unsafe extern "C" fn ioread32_rep(a:*const core::ffi::c_void,mut d:*mut core::ffi::c_void,mut n:usize){if indirect_addr(a){(ops(a).read32r)(a,d,n)}else{while n!=0{*(d as *mut U32)=load(a);d=d.add(4);n-=1}}}
#[no_mangle] pub unsafe extern "C" fn iowrite8_rep(a:*mut core::ffi::c_void,mut s:*const core::ffi::c_void,mut n:usize){if indirect_addr(a){(ops(a).write8r)(a,s,n)}else{while n!=0{store(a,load(s));s=s.add(1);n-=1}}}
#[no_mangle] pub unsafe extern "C" fn iowrite16_rep(a:*mut core::ffi::c_void,mut s:*const core::ffi::c_void,mut n:usize){if indirect_addr(a){(ops(a).write16r)(a,s,n)}else{while n!=0{store(a,load(s));s=s.add(2);n-=1}}}
#[no_mangle] pub unsafe extern "C" fn iowrite32_rep(a:*mut core::ffi::c_void,mut s:*const core::ffi::c_void,mut n:usize){if indirect_addr(a){(ops(a).write32r)(a,s,n)}else{while n!=0{store(a,load(s));s=s.add(4);n-=1}}}

#[no_mangle] pub unsafe extern "C" fn ioport_map(port:usize,_nr:U32)->*mut core::ffi::c_void{(IOPORT_MAP_BASE|port) as *mut _}
#[no_mangle] pub unsafe extern "C" fn ioport_unmap(a:*mut core::ffi::c_void){if !indirect_addr(a){iounmap(a)}}
#[cfg(feature="CONFIG_PCI")] #[no_mangle] pub unsafe extern "C" fn pci_iounmap(_dev:*mut core::ffi::c_void,a:*mut core::ffi::c_void){if !indirect_addr(a){iounmap(a)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
