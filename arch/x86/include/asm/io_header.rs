/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of the x86 I/O header. C includes and preprocessor-only
 * build conditions are represented by comments or cfg attributes below. */

/* x86 I/O instructions and their string/pausing variants. */

#[inline]
pub unsafe fn readb(addr: *const u8) -> u8 { core::ptr::read_volatile(addr) }
#[inline]
pub unsafe fn readw(addr: *const u16) -> u16 { core::ptr::read_volatile(addr) }
#[inline]
pub unsafe fn readl(addr: *const u32) -> u32 { core::ptr::read_volatile(addr) }
#[inline]
pub unsafe fn __readb(addr: *const u8) -> u8 { core::ptr::read(addr) }
#[inline]
pub unsafe fn __readw(addr: *const u16) -> u16 { core::ptr::read(addr) }
#[inline]
pub unsafe fn __readl(addr: *const u32) -> u32 { core::ptr::read(addr) }

#[inline]
pub unsafe fn writeb(val: u8, addr: *mut u8) { core::ptr::write_volatile(addr, val) }
#[inline]
pub unsafe fn writew(val: u16, addr: *mut u16) { core::ptr::write_volatile(addr, val) }
#[inline]
pub unsafe fn writel(val: u32, addr: *mut u32) { core::ptr::write_volatile(addr, val) }
#[inline]
pub unsafe fn __writeb(val: u8, addr: *mut u8) { core::ptr::write(addr, val) }
#[inline]
pub unsafe fn __writew(val: u16, addr: *mut u16) { core::ptr::write(addr, val) }
#[inline]
pub unsafe fn __writel(val: u32, addr: *mut u32) { core::ptr::write(addr, val) }

#[inline] pub unsafe fn readb_relaxed(a: *const u8) -> u8 { __readb(a) }
#[inline] pub unsafe fn readw_relaxed(a: *const u16) -> u16 { __readw(a) }
#[inline] pub unsafe fn readl_relaxed(a: *const u32) -> u32 { __readl(a) }
#[inline] pub unsafe fn writeb_relaxed(v: u8, a: *mut u8) { __writeb(v, a) }
#[inline] pub unsafe fn writew_relaxed(v: u16, a: *mut u16) { __writew(v, a) }
#[inline] pub unsafe fn writel_relaxed(v: u32, a: *mut u32) { __writel(v, a) }

#[cfg(target_pointer_width = "64")]
extern "C" {
    pub fn readq(addr: *const u64) -> u64;
    pub fn __readq(addr: *const u64) -> u64;
    pub fn writeq(val: u64, addr: *mut u64);
    pub fn __writeq(val: u64, addr: *mut u64);
}

pub const ARCH_HAS_VALID_PHYS_ADDR_RANGE: bool = true;
extern "C" {
    pub fn valid_phys_addr_range(addr: usize, size: usize) -> i32;
    pub fn valid_mmap_phys_addr_range(pfn: usize, size: usize) -> i32;
    pub fn ioremap_uc(offset: usize, size: u64) -> *mut u8;
    pub fn ioremap_cache(offset: usize, size: u64) -> *mut u8;
    pub fn ioremap_prot(offset: usize, size: u64, prot: usize) -> *mut u8;
    pub fn ioremap_encrypted(phys_addr: usize, size: u64) -> *mut u8;
    pub fn arch_memremap_wb(phys_addr: usize, size: usize, flags: u64) -> *mut u8;
    pub fn ioremap(offset: usize, size: u64) -> *mut u8;
    pub fn iounmap(addr: *mut u8);
    pub fn native_io_delay();
    pub static mut io_delay_type: i32;
    pub fn io_delay_init();
    pub fn xlate_dev_mem_ptr(phys: usize) -> *mut core::ffi::c_void;
    pub fn unxlate_dev_mem_ptr(phys: usize, addr: *mut core::ffi::c_void);
    pub fn ioremap_change_attr(vaddr: u64, size: u64, pcm: i32) -> i32;
    pub fn ioremap_wc(offset: usize, size: u64) -> *mut u8;
    pub fn ioremap_wt(offset: usize, size: u64) -> *mut u8;
    pub fn is_early_ioremap_ptep(ptep: *mut core::ffi::c_void) -> bool;
}

#[inline]
pub unsafe fn virt_to_phys(address: *mut core::ffi::c_void) -> usize { __pa(address) }
#[inline]
pub unsafe fn phys_to_virt(address: usize) -> *mut core::ffi::c_void { __va(address) }
#[inline]
pub unsafe fn isa_virt_to_bus(address: *mut core::ffi::c_void) -> u32 { virt_to_phys(address) as u32 }

extern "C" {
    fn __pa(address: *mut core::ffi::c_void) -> usize;
    fn __va(address: usize) -> *mut core::ffi::c_void;
}

extern "C" {
    pub fn outb(value: u8, port: u16);
    pub fn outw(value: u16, port: u16);
    pub fn outl(value: u32, port: u16);
    pub fn inb(port: u16) -> u8;
    pub fn inw(port: u16) -> u16;
    pub fn inl(port: u16) -> u32;
    pub fn cc_platform_has(attr: i32) -> bool;
    pub fn call_io_delay() -> bool;
}

#[inline] pub unsafe fn slow_down_io() { if call_io_delay() { native_io_delay(); } }
#[inline] pub unsafe fn outb_p(value: u8, port: u16) { outb(value, port); slow_down_io(); }
#[inline] pub unsafe fn outw_p(value: u16, port: u16) { outw(value, port); slow_down_io(); }
#[inline] pub unsafe fn outl_p(value: u32, port: u16) { outl(value, port); slow_down_io(); }
#[inline] pub unsafe fn inb_p(port: u16) -> u8 { let v = inb(port); slow_down_io(); v }
#[inline] pub unsafe fn inw_p(port: u16) -> u16 { let v = inw(port); slow_down_io(); v }
#[inline] pub unsafe fn inl_p(port: u16) -> u32 { let v = inl(port); slow_down_io(); v }

#[inline] pub unsafe fn outsb(port: u16, addr: *const u8, count: usize) { for i in 0..count { outb(*addr.add(i), port); } }
#[inline] pub unsafe fn outsw(port: u16, addr: *const u16, count: usize) { for i in 0..count { outw(*addr.add(i), port); } }
#[inline] pub unsafe fn outsl(port: u16, addr: *const u32, count: usize) { for i in 0..count { outl(*addr.add(i), port); } }
#[inline] pub unsafe fn insb(port: u16, addr: *mut u8, count: usize) { for i in 0..count { addr.add(i).write(inb(port)); } }
#[inline] pub unsafe fn insw(port: u16, addr: *mut u16, count: usize) { for i in 0..count { addr.add(i).write(inw(port)); } }
#[inline] pub unsafe fn insl(port: u16, addr: *mut u32, count: usize) { for i in 0..count { addr.add(i).write(inl(port)); } }

#[cfg(target_pointer_width = "64")]
#[inline] pub unsafe fn __iowrite32_copy(mut to: *mut u8, mut from: *const u8, mut count: usize) {
    while count != 0 { core::ptr::copy_nonoverlapping(from, to, 4); to = to.add(4); from = from.add(4); count -= 1; }
}

extern "C" {
    pub fn arch_memremap_can_ram_remap(offset: u64, size: u64, flags: u64) -> bool;
    pub fn arch_phys_wc_index(handle: i32) -> i32;
    pub fn arch_phys_wc_add(base: u64, size: u64) -> i32;
    pub fn arch_phys_wc_del(handle: i32);
    pub fn arch_io_reserve_memtype_wc(start: u64, size: u64) -> i32;
    pub fn arch_io_free_memtype_wc(start: u64, size: u64);
}

pub const IO_SPACE_LIMIT: u16 = 0xffff;

#[cfg(feature = "CONFIG_AMD_MEM_ENCRYPT")]
extern "C" { pub fn phys_mem_access_encrypted(phys_addr: u64, size: u64) -> bool; }
#[cfg(not(feature = "CONFIG_AMD_MEM_ENCRYPT"))]
#[inline] pub fn phys_mem_access_encrypted(_phys_addr: u64, _size: u64) -> bool { true }

extern "C" {
    pub fn memcpy_fromio(to: *mut core::ffi::c_void, from: *const u8, size: usize);
    pub fn memcpy_toio(to: *mut u8, from: *const core::ffi::c_void, size: usize);
    pub fn memset_io(to: *mut u8, value: i32, size: usize);
    pub fn movdir64b_io(dst: *mut u8, src: *const u8);
}

#[inline]
pub unsafe fn iosubmit_cmds512(mut dst: *mut u8, mut src: *const u8, count: usize) {
    let end = src.add(count.wrapping_mul(64));
    while src < end {
        movdir64b_io(dst, src);
        src = src.add(64);
    }
}

/* __ISA_IO_base is PAGE_OFFSET cast to an I/O-space pointer when __KERNEL__ is enabled. */
/* CONFIG_PARAVIRT, CONFIG_MTRR, CONFIG_X86_PAT, and asm-generic/io.h supply
 * additional declarations and conditional helpers in the original header. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
