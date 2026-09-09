/* Rust translation of the MIPS asm/io.h header. */

// C includes are supplied by the surrounding kernel translation unit.

pub type U8 = u8;
pub type U16 = u16;
pub type U32 = u32;
pub type U64 = u64;
pub type PhysAddr = usize;
pub type PgProt = usize;

pub static mut mips_io_port_base: usize = 0;

#[inline]
pub unsafe fn set_io_port_base(base: usize) { mips_io_port_base = base; }

// Raw operations are never swapped in software.
#[inline] pub fn __raw_ioswabb<T>(_: *const T, x: u8) -> u8 { x }
#[inline] pub fn __raw_ioswabw<T>(_: *const T, x: u16) -> u16 { x }
#[inline] pub fn __raw_ioswabl<T>(_: *const T, x: u32) -> u32 { x }
#[inline] pub fn __raw_ioswabq<T>(_: *const T, x: u64) -> u64 { x }
#[inline] pub fn ____raw_ioswabq<T>(_: *const T, x: u64) -> u64 { x }

// iobarrier_* map to the platform barrier functions supplied by asm/barrier.h.
#[inline] pub unsafe fn iobarrier_rw() { mb(); }
#[inline] pub unsafe fn iobarrier_r() { rmb(); }
#[inline] pub unsafe fn iobarrier_w() { wmb(); }
#[inline] pub unsafe fn iobarrier_sync() { iob(); }

extern "C" {
    pub fn mb(); pub fn rmb(); pub fn wmb(); pub fn iob();
    pub fn barrier();
    pub fn __pa(address: *const core::ffi::c_void) -> usize;
    pub fn __pgprot(value: usize) -> PgProt;
    pub fn __swizzle_addr_b(address: usize) -> usize;
    pub fn __swizzle_addr_w(address: usize) -> usize;
    pub fn __swizzle_addr_l(address: usize) -> usize;
    pub fn __swizzle_addr_q(address: usize) -> usize;
    pub fn phys_to_virt(address: usize) -> *mut core::ffi::c_void;
    pub fn BUG() -> !;
}

#[inline]
pub unsafe fn __virt_to_phys_nodebug(address: *const core::ffi::c_void) -> usize { __pa(address) }

#[inline]
pub unsafe fn virt_to_phys(x: *const core::ffi::c_void) -> usize { __virt_to_phys_nodebug(x) }

#[inline]
pub unsafe fn isa_virt_to_bus(address: *mut core::ffi::c_void) -> usize { virt_to_phys(address) }

extern "C" {
    pub fn ioremap_prot(offset: usize, size: usize, prot: PgProt) -> *mut core::ffi::c_void;
    pub fn iounmap(addr: *const core::ffi::c_void);
}

#[inline] pub unsafe fn ioremap(offset: usize, size: usize) -> *mut core::ffi::c_void {
    ioremap_prot(offset, size, __pgprot(_CACHE_UNCACHED))
}
#[inline] pub unsafe fn ioremap_cache(offset: usize, size: usize) -> *mut core::ffi::c_void {
    ioremap_prot(offset, size, __pgprot(_page_cachable_default))
}
#[inline] pub unsafe fn ioremap_wc(offset: usize, size: usize) -> *mut core::ffi::c_void {
    ioremap_prot(offset, size, __pgprot(boot_cpu_data_writecombine()))
}

// Build-time architecture constants and CPU data are supplied externally.
extern "C" { pub static boot_cpu_data: CpuData; }
#[repr(C)] pub struct CpuData { pub writecombine: usize }
extern "C" fn boot_cpu_data_writecombine() -> usize { unsafe { boot_cpu_data.writecombine } }
extern "C" { pub static _CACHE_UNCACHED: usize; pub static _page_cachable_default: usize; }

macro_rules! build_memory_single {
    ($write:ident, $read:ident, $ty:ty, $swizzle:ident, $swap:ident, $relaxed:expr) => {
        #[inline] pub unsafe fn $write(val: $ty, mem: *mut core::ffi::c_void) {
            if !$relaxed { iobarrier_rw(); } else { barrier(); }
            let p = __swizzle_addr_b(mem as usize) as *mut $ty;
            core::ptr::write_volatile(p, $swap(p, val));
        }
        #[inline] pub unsafe fn $read(mem: *const core::ffi::c_void) -> $ty {
            let p = __swizzle_addr_b(mem as usize) as *const $ty;
            if !$relaxed { iobarrier_rw(); }
            let v = core::ptr::read_volatile(p);
            if !$relaxed { rmb(); }
            $swap(p, v)
        }
    };
}

// Locally generated families corresponding to BUILDIO_MEM/BUILDIO_IOPORT.
macro_rules! memory_io { ($b:ident, $w:ident, $l:ident) => {
    build_memory_single!(__raw_writeb, __raw_readb, u8, $b, __raw_ioswabb, false);
    build_memory_single!(__raw_writew, __raw_readw, u16, $w, __raw_ioswabw, false);
    build_memory_single!(__raw_writel, __raw_readl, u32, $l, __raw_ioswabl, false);
}; }

#[inline] pub unsafe fn __raw_writeb(v: u8, p: *mut core::ffi::c_void) { core::ptr::write_volatile(p as *mut u8, v); }
#[inline] pub unsafe fn __raw_readb(p: *const core::ffi::c_void) -> u8 { core::ptr::read_volatile(p as *const u8) }
#[inline] pub unsafe fn __raw_writew(v: u16, p: *mut core::ffi::c_void) { core::ptr::write_volatile(p as *mut u16, v); }
#[inline] pub unsafe fn __raw_readw(p: *const core::ffi::c_void) -> u16 { core::ptr::read_volatile(p as *const u16) }
#[inline] pub unsafe fn __raw_writel(v: u32, p: *mut core::ffi::c_void) { core::ptr::write_volatile(p as *mut u32, v); }
#[inline] pub unsafe fn __raw_readl(p: *const core::ffi::c_void) -> u32 { core::ptr::read_volatile(p as *const u32) }

#[inline] pub unsafe fn __mem_writeb(v: u8, p: *mut core::ffi::c_void) { __raw_writeb(v, p) }
#[inline] pub unsafe fn __mem_readb(p: *const core::ffi::c_void) -> u8 { __raw_readb(p) }
#[inline] pub unsafe fn __mem_writew(v: u16, p: *mut core::ffi::c_void) { __raw_writew(v, p) }
#[inline] pub unsafe fn __mem_readw(p: *const core::ffi::c_void) -> u16 { __raw_readw(p) }
#[inline] pub unsafe fn __mem_writel(v: u32, p: *mut core::ffi::c_void) { __raw_writel(v, p) }
#[inline] pub unsafe fn __mem_readl(p: *const core::ffi::c_void) -> u32 { __raw_readl(p) }

#[inline] pub unsafe fn writesb(mem: *mut core::ffi::c_void, addr: *const u8, mut count: u32) {
    while count != 0 { __mem_writeb(core::ptr::read(addr), mem); addr = addr.add(1); count -= 1; }
}
#[inline] pub unsafe fn readsb(mem: *const core::ffi::c_void, addr: *mut u8, mut count: u32) {
    while count != 0 { core::ptr::write(addr, __mem_readb(mem)); addr = addr.add(1); count -= 1; }
}
#[inline] pub unsafe fn writesw(mem: *mut core::ffi::c_void, addr: *const u16, mut count: u32) {
    while count != 0 { __mem_writew(core::ptr::read(addr), mem); addr = addr.add(1); count -= 1; }
}
#[inline] pub unsafe fn readsw(mem: *const core::ffi::c_void, addr: *mut u16, mut count: u32) {
    while count != 0 { core::ptr::write(addr, __mem_readw(mem)); addr = addr.add(1); count -= 1; }
}
#[inline] pub unsafe fn writesl(mem: *mut core::ffi::c_void, addr: *const u32, mut count: u32) {
    while count != 0 { __mem_writel(core::ptr::read(addr), mem); addr = addr.add(1); count -= 1; }
}
#[inline] pub unsafe fn readsl(mem: *const core::ffi::c_void, addr: *mut u32, mut count: u32) {
    while count != 0 { core::ptr::write(addr, __mem_readl(mem)); addr = addr.add(1); count -= 1; }
}

#[cfg(feature = "dma_noncoherent")]
extern "C" {
    pub static mut _dma_cache_wback_inv: Option<unsafe extern "C" fn(usize, usize)>;
    pub static mut _dma_cache_wback: Option<unsafe extern "C" fn(usize, usize)>;
    pub static mut _dma_cache_inv: Option<unsafe extern "C" fn(usize, usize)>;
}
#[inline] pub unsafe fn dma_cache_wback_inv(start: usize, size: usize) {
    #[cfg(feature = "dma_noncoherent")] { if let Some(f) = _dma_cache_wback_inv { f(start, size); } }
    #[cfg(not(feature = "dma_noncoherent"))] { let _ = (start, size); }
}
#[inline] pub unsafe fn dma_cache_wback(start: usize, size: usize) {
    #[cfg(feature = "dma_noncoherent")] { if let Some(f) = _dma_cache_wback { f(start, size); } }
    #[cfg(not(feature = "dma_noncoherent"))] { let _ = (start, size); }
}
#[inline] pub unsafe fn dma_cache_inv(start: usize, size: usize) {
    #[cfg(feature = "dma_noncoherent")] { if let Some(f) = _dma_cache_inv { f(start, size); } }
    #[cfg(not(feature = "dma_noncoherent"))] { let _ = (start, size); }
}

pub const CSR_32_ADJUST: usize = 0; // __MIPSEB__ builds use 4.
#[inline] pub unsafe fn csr_out32(v: u32, a: *mut core::ffi::c_void) {
    core::ptr::write_volatile((a as usize + CSR_32_ADJUST) as *mut u32, v);
}
#[inline] pub unsafe fn csr_in32(a: *const core::ffi::c_void) -> u32 {
    core::ptr::read_volatile((a as usize + CSR_32_ADJUST) as *const u32)
}

extern "C" { pub fn __ioread64_copy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize); }

#[inline] pub unsafe fn isa_bus_to_virt(address: usize) -> *mut core::ffi::c_void { phys_to_virt(address) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
