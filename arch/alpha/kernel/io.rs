// SPDX-License-Identifier: GPL-2.0
/* Alpha IO and memory functions. */

/* IO_CONCAT(__IO_PREFIX, ...) denotes the platform-specific implementation. */

extern "C" {
    fn mb();
    fn ioportmap(port: libc::c_ulong) -> *mut core::ffi::c_void;
    fn platform_ioread8(addr: *const core::ffi::c_void) -> u32;
    fn platform_ioread16(addr: *const core::ffi::c_void) -> u32;
    fn platform_ioread32(addr: *const core::ffi::c_void) -> u32;
    fn platform_ioread64(addr: *const core::ffi::c_void) -> u64;
    fn platform_iowrite8(b: u8, addr: *mut core::ffi::c_void);
    fn platform_iowrite16(b: u16, addr: *mut core::ffi::c_void);
    fn platform_iowrite32(b: u32, addr: *mut core::ffi::c_void);
    fn platform_iowrite64(b: u64, addr: *mut core::ffi::c_void);
    fn platform_readb(addr: *const core::ffi::c_void) -> u8;
    fn platform_readw(addr: *const core::ffi::c_void) -> u16;
    fn platform_readl(addr: *const core::ffi::c_void) -> u32;
    fn platform_readq(addr: *const core::ffi::c_void) -> u64;
    fn platform_writeb(b: u8, addr: *mut core::ffi::c_void);
    fn platform_writew(b: u16, addr: *mut core::ffi::c_void);
    fn platform_writel(b: u32, addr: *mut core::ffi::c_void);
    fn platform_writeq(b: u64, addr: *mut core::ffi::c_void);
}

#[inline(never)] pub unsafe fn ioread8(addr: *const core::ffi::c_void) -> u32 { mb(); let ret = platform_ioread8(addr); mb(); ret }
#[inline(never)] pub unsafe fn ioread16(addr: *const core::ffi::c_void) -> u32 { mb(); let ret = platform_ioread16(addr); mb(); ret }
#[inline(never)] pub unsafe fn ioread32(addr: *const core::ffi::c_void) -> u32 { mb(); let ret = platform_ioread32(addr); mb(); ret }
#[inline(never)] pub unsafe fn ioread64(addr: *const core::ffi::c_void) -> u64 { mb(); let ret = platform_ioread64(addr); mb(); ret }
pub unsafe fn iowrite8(b: u8, addr: *mut core::ffi::c_void) { mb(); platform_iowrite8(b, addr); }
pub unsafe fn iowrite16(b: u16, addr: *mut core::ffi::c_void) { mb(); platform_iowrite16(b, addr); }
pub unsafe fn iowrite32(b: u32, addr: *mut core::ffi::c_void) { mb(); platform_iowrite32(b, addr); }
pub unsafe fn iowrite64(b: u64, addr: *mut core::ffi::c_void) { mb(); platform_iowrite64(b, addr); }

pub unsafe fn inb(port: libc::c_ulong) -> u8 { ioread8(ioport_map(port)). as u8 }
pub unsafe fn inw(port: libc::c_ulong) -> u16 { ioread16(ioport_map(port)). as u16 }
pub unsafe fn inl(port: libc::c_ulong) -> u32 { ioread32(ioport_map(port)). as u32 }
pub unsafe fn outb(b: u8, port: libc::c_ulong) { iowrite8(b, ioport_map(port)); }
pub unsafe fn outw(b: u16, port: libc::c_ulong) { iowrite16(b, ioport_map(port)); }
pub unsafe fn outl(b: u32, port: libc::c_ulong) { iowrite32(b, ioport_map(port)); }

pub unsafe fn __raw_readb(a: *const core::ffi::c_void) -> u8 { platform_readb(a) }
pub unsafe fn __raw_readw(a: *const core::ffi::c_void) -> u16 { platform_readw(a) }
pub unsafe fn __raw_readl(a: *const core::ffi::c_void) -> u32 { platform_readl(a) }
pub unsafe fn __raw_readq(a: *const core::ffi::c_void) -> u64 { platform_readq(a) }
pub unsafe fn __raw_writeb(b: u8, a: *mut core::ffi::c_void) { platform_writeb(b, a) }
pub unsafe fn __raw_writew(b: u16, a: *mut core::ffi::c_void) { platform_writew(b, a) }
pub unsafe fn __raw_writel(b: u32, a: *mut core::ffi::c_void) { platform_writel(b, a) }
pub unsafe fn __raw_writeq(b: u64, a: *mut core::ffi::c_void) { platform_writeq(b, a) }

pub unsafe fn readb(a: *const core::ffi::c_void) -> u8 { mb(); let r=__raw_readb(a); mb(); r }
pub unsafe fn readw(a: *const core::ffi::c_void) -> u16 { mb(); let r=__raw_readw(a); mb(); r }
pub unsafe fn readl(a: *const core::ffi::c_void) -> u32 { mb(); let r=__raw_readl(a); mb(); r }
pub unsafe fn readq(a: *const core::ffi::c_void) -> u64 { mb(); let r=__raw_readq(a); mb(); r }
pub unsafe fn writeb(b:u8,a:*mut core::ffi::c_void){mb();__raw_writeb(b,a)}
pub unsafe fn writew(b:u16,a:*mut core::ffi::c_void){mb();__raw_writew(b,a)}
pub unsafe fn writel(b:u32,a:*mut core::ffi::c_void){mb();__raw_writel(b,a)}
pub unsafe fn writeq(b:u64,a:*mut core::ffi::c_void){mb();__raw_writeq(b,a)}

/* The relaxed functions are ordered with respect to each other. */
pub unsafe fn readb_relaxed(a:*const core::ffi::c_void)->u8{mb();__raw_readb(a)}
pub unsafe fn readw_relaxed(a:*const core::ffi::c_void)->u16{mb();__raw_readw(a)}
pub unsafe fn readl_relaxed(a:*const core::ffi::c_void)->u32{mb();__raw_readl(a)}
pub unsafe fn readq_relaxed(a:*const core::ffi::c_void)->u64{mb();__raw_readq(a)}

pub unsafe fn ioread8_rep(port:*const core::ffi::c_void, mut dst:*mut u8, mut count:libc::c_ulong){
    while (dst as usize)&3 != 0 { if count==0{return} count-=1; *dst=ioread8(port) as u8; dst=dst.add(1); }
    while count>=4 { count-=4; let mut w=ioread8(port); w|=ioread8(port)<<8; w|=ioread8(port)<<16; w|=ioread8(port)<<24; (dst as *mut u32).write_unaligned(w); dst=dst.add(4); }
    while count!=0 { count-=1; *dst=ioread8(port) as u8; dst=dst.add(1); }
}
pub unsafe fn insb(p:libc::c_ulong,d:*mut u8,c:libc::c_ulong){ioread8_rep(ioport_map(p),d,c)}

pub unsafe fn ioread16_rep(port:*const core::ffi::c_void, mut dst:*mut u8, mut count:libc::c_ulong){
    if (dst as usize)&3 != 0 { if count==0{return} assert!((dst as usize)&1==0); count-=1; (dst as *mut u16).write_unaligned(ioread16(port) as u16); dst=dst.add(2); }
    while count>=2 { count-=2; let w=ioread16(port)|(ioread16(port)<<16); (dst as *mut u32).write_unaligned(w); dst=dst.add(4); }
    if count!=0 {(dst as *mut u16).write_unaligned(ioread16(port) as u16)}
}
pub unsafe fn insw(p:libc::c_ulong,d:*mut u8,c:libc::c_ulong){ioread16_rep(ioport_map(p),d,c)}

pub unsafe fn ioread32_rep(port:*const core::ffi::c_void, mut dst:*mut u8, mut count:libc::c_ulong){while count!=0{(dst as *mut u32).write_unaligned(ioread32(port));dst=dst.add(4);count-=1}}
pub unsafe fn insl(p:libc::c_ulong,d:*mut u8,c:libc::c_ulong){ioread32_rep(ioport_map(p),d,c)}
pub unsafe fn iowrite8_rep(port:*mut core::ffi::c_void,mut src:*const u8,mut count:libc::c_ulong){while count!=0{iowrite8(*src,port);src=src.add(1);count-=1}}
pub unsafe fn outsb(p:libc::c_ulong,s:*const u8,c:libc::c_ulong){iowrite8_rep(ioport_map(p),s,c)}
pub unsafe fn iowrite16_rep(port:*mut core::ffi::c_void,mut src:*const u8,mut count:libc::c_ulong){if (src as usize)&3!=0{if count==0{return}assert!((src as usize)&1==0);iowrite16((src as *const u16).read_unaligned(),port);src=src.add(2);count-=1}while count>=2{let w=(src as *const u32).read_unaligned();src=src.add(4);iowrite16(w as u16,port);iowrite16((w>>16) as u16,port);count-=2}if count!=0{iowrite16((src as *const u16).read_unaligned(),port)}}
pub unsafe fn outsw(p:libc::c_ulong,s:*const u8,c:libc::c_ulong){iowrite16_rep(ioport_map(p),s,c)}
pub unsafe fn iowrite32_rep(port:*mut core::ffi::c_void,mut src:*const u8,mut count:libc::c_ulong){while count!=0{iowrite32((src as *const u32).read_unaligned(),port);src=src.add(4);count-=1}}
pub unsafe fn outsl(p:libc::c_ulong,s:*const u8,c:libc::c_ulong){iowrite32_rep(ioport_map(p),s,c)}

pub unsafe fn memcpy_fromio(mut to:*mut u8,mut from:*const core::ffi::c_void,mut count:libc::c_long){while count>=8&&((to as u64)&7)==((from as u64)&7){(to as *mut u64).write_unaligned(__raw_readq(from));to=to.add(8);from=from.add(8);count-=8}while count>=4&&((to as u64)&3)==((from as u64)&3){(to as *mut u32).write_unaligned(__raw_readl(from));to=to.add(4);from=from.add(4);count-=4}while count>=2&&((to as u64)&1)==((from as u64)&1){(to as *mut u16).write_unaligned(__raw_readw(from));to=to.add(2);from=from.add(2);count-=2}while count>0{*to=__raw_readb(from);to=to.add(1);from=from.add(1);count-=1}mb()}
pub unsafe fn memcpy_toio(mut to:*mut core::ffi::c_void,mut from:*const u8,mut count:libc::c_long){while count>=8&&((to as u64)&7)==((from as u64)&7){__raw_writeq((from as *const u64).read_unaligned(),to);to=to.add(8);from=from.add(8);count-=8}while count>=4&&((to as u64)&3)==((from as u64)&3){__raw_writel((from as *const u32).read_unaligned(),to);to=to.add(4);from=from.add(4);count-=4}while count>=2&&((to as u64)&1)==((from as u64)&1){__raw_writew((from as *const u16).read_unaligned(),to);to=to.add(2);from=from.add(2);count-=2}while count>0{__raw_writeb(*from,to);to=to.add(1);from=from.add(1);count-=1}mb()}
pub unsafe fn _memset_c_io(mut to:*mut core::ffi::c_void,c:libc::c_ulong,mut count:libc::c_long){while count>0&&(to as u64)&1!=0{__raw_writeb(c as u8,to);to=to.add(1);count-=1}while count>=2&&(to as u64)&2!=0{__raw_writew(c as u16,to);to=to.add(2);count-=2}while count>=4&&(to as u64)&4!=0{__raw_writel(c as u32,to);to=to.add(4);count-=4}while count>=8{__raw_writeq(c as u64,to);to=to.add(8);count-=8}if count>=4{__raw_writel(c as u32,to);to=to.add(4);count-=4}if count>=2{__raw_writew(c as u16,to);to=to.add(2);count-=2}if count!=0{__raw_writeb(c as u8,to)}mb()}

/* CONFIG_VGA_CONSOLE conditional: VGA console helpers. */
extern "C" { fn __is_ioaddr(p:*const core::ffi::c_void)->i32; fn memcpy(d:*mut u8,s:*const u8,n:usize); fn scr_readw(p:*const u16)->u16; fn scr_writew(v:u16,p:*mut u16); }
pub unsafe fn scr_memcpyw(d:*mut u16,s:*const u16,mut count:u32){let si=__is_ioaddr(s as *const _)!=0;let di=__is_ioaddr(d as *const _)!=0;if si{if di{count/=2;while count!=0{__raw_writew(__raw_readw(s as *const _),d as *mut _);s=s.add(1);d=d.add(1);count-=1}}else{memcpy_fromio(d as *mut u8,s as *const _,count as libc::c_long)}}else if di{memcpy_toio(d as *mut _,s as *const u8,count as libc::c_long)}else{memcpy(d as *mut u8,s as *const u8,count as usize)}}
pub unsafe fn scr_memmovew(mut d:*mut u16,mut s:*const u16,count:u32){if (d as usize)<(s as usize){scr_memcpyw(d,s,count)}else{let mut n=count/2;d=d.add(n as usize);s=s.add(n as usize);while n!=0{n-=1;d=d.sub(1);s=s.sub(1);scr_writew(scr_readw(s),d)}}}

pub unsafe fn ioport_map(port:libc::c_ulong,_size:u32)->*mut core::ffi::c_void{ioportmap(port)}
pub unsafe fn ioport_unmap(_addr:*mut core::ffi::c_void){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
