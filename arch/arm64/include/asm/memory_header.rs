/* SPDX-License-Identifier: GPL-2.0-only */
/* Based on arch/arm/include/asm/memory.h */

// C header dependencies: linux/const.h, linux/sizes.h, asm/page-def.h.

pub const PCI_IO_SIZE: usize = SZ_16M;
pub const VMEMMAP_RANGE: usize = _PAGE_END(VA_BITS_MIN) - PAGE_OFFSET;
pub const VMEMMAP_SIZE: usize = (VMEMMAP_RANGE >> PAGE_SHIFT) * core::mem::size_of::<Page>();

pub const VA_BITS: usize = CONFIG_ARM64_VA_BITS;
#[inline] pub const fn _PAGE_OFFSET(va: usize) -> usize { (!0usize) << va }
pub const PAGE_OFFSET: usize = _PAGE_OFFSET(VA_BITS);
pub const KIMAGE_VADDR: usize = MODULES_END;
pub const MODULES_END: usize = MODULES_VADDR + MODULES_VSIZE;
pub const MODULES_VADDR: usize = _PAGE_END(VA_BITS_MIN);
pub const MODULES_VSIZE: usize = SZ_2G;
pub const VMEMMAP_START: usize = VMEMMAP_END - VMEMMAP_SIZE;
pub const VMEMMAP_END: usize = (!0usize) - (SZ_1G - 1);
pub const PCI_IO_START: usize = VMEMMAP_END + SZ_8M;
pub const PCI_IO_END: usize = PCI_IO_START + PCI_IO_SIZE;
pub const FIXADDR_TOP: usize = (!0usize) - (SZ_8M - 1);

// VA_BITS_MIN is selected by CONFIG_ARM64_16K_PAGES when VA_BITS > 48.
pub const VA_BITS_MIN: usize = if VA_BITS > 48 {
    if cfg!(feature = "CONFIG_ARM64_16K_PAGES") { 47 } else { 48 }
} else { VA_BITS };
#[inline] pub const fn _PAGE_END(va: usize) -> usize { (!0usize) << (va - 1) }

pub const KERNEL_START: usize = _text as usize;
pub const KERNEL_END: usize = _end as usize;

// KASAN shadow layout is selected by CONFIG_KASAN_GENERIC/CONFIG_KASAN_SW_TAGS.
#[cfg(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS"))]
pub const KASAN_SHADOW_OFFSET: u64 = CONFIG_KASAN_SHADOW_OFFSET;
#[cfg(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS"))]
pub const KASAN_SHADOW_END: u64 = (1u64 << (64 - KASAN_SHADOW_SCALE_SHIFT)) + KASAN_SHADOW_OFFSET;
#[cfg(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS"))]
pub const KASAN_SHADOW_START: u64 = KASAN_SHADOW_END - (1u64 << (vabits_actual() - KASAN_SHADOW_SCALE_SHIFT));
#[cfg(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS"))]
pub const PAGE_END: u64 = KASAN_SHADOW_START;
#[cfg(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS"))]
pub const KASAN_THREAD_SHIFT: usize = 1;
#[cfg(not(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS")))]
pub const KASAN_THREAD_SHIFT: usize = 0;
#[cfg(not(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS")))]
pub const PAGE_END: usize = _PAGE_END(VA_BITS_MIN);

pub const DIRECT_MAP_PHYSMEM_END: usize = __pa(PAGE_END - 1);
pub const MIN_THREAD_SHIFT: usize = 14 + KASAN_THREAD_SHIFT;
pub const THREAD_SHIFT: usize = if MIN_THREAD_SHIFT < PAGE_SHIFT { PAGE_SHIFT } else { MIN_THREAD_SHIFT };
pub const THREAD_SIZE_ORDER: usize = THREAD_SHIFT - PAGE_SHIFT;
pub const THREAD_SIZE: usize = 1usize << THREAD_SHIFT;
pub const THREAD_ALIGN: usize = 2 * THREAD_SIZE;
pub const IRQ_STACK_SIZE: usize = THREAD_SIZE;
pub const OVERFLOW_STACK_SIZE: usize = SZ_4K;
pub const NVHE_STACK_SHIFT: usize = PAGE_SHIFT;
pub const NVHE_STACK_SIZE: usize = 1usize << NVHE_STACK_SHIFT;
pub const NVHE_STACKTRACE_SIZE: usize = (OVERFLOW_STACK_SIZE + NVHE_STACK_SIZE) / 2 + core::mem::size_of::<usize>();
pub const SEGMENT_ALIGN: usize = SZ_64K;

pub const MT_NORMAL: u32 = 0;
pub const MT_NORMAL_TAGGED: u32 = 1;
pub const MT_NORMAL_NC: u32 = 2;
pub const MT_DEVICE_nGnRnE: u32 = 3;
pub const MT_DEVICE_nGnRE: u32 = 4;
pub const MT_S2_NORMAL: u32 = 0xf;
pub const MT_S2_NORMAL_NC: u32 = 0x5;
pub const MT_S2_DEVICE_nGnRE: u32 = 0x1;
pub const MT_S2_AS_S1: u32 = MT_S2_NORMAL;
pub const MT_S2_FWB_NORMAL: u32 = 6;
pub const MT_S2_FWB_NORMAL_NC: u32 = 5;
pub const MT_S2_FWB_DEVICE_nGnRE: u32 = 1;
pub const MT_S2_FWB_AS_S1: u32 = 7;

pub const IOREMAP_MAX_ORDER: usize = if cfg!(feature = "CONFIG_ARM64_4K_PAGES") { PUD_SHIFT } else { PMD_SHIFT };
pub const RESERVED_SWAPPER_OFFSET: usize = PAGE_SIZE;
pub const TRAMP_SWAPPER_OFFSET: usize = 2 * PAGE_SIZE;

#[inline]
pub unsafe fn read_tcr() -> u64 { let tcr: u64; core::arch::asm!("mrs {0}, tcr_el1", out(reg) tcr); tcr }
#[inline]
pub fn vabits_actual() -> u64 { if VA_BITS > 48 { 64 - ((unsafe { read_tcr() } >> 16) & 63) } else { VA_BITS as u64 } }

extern "C" {
    pub static mut memstart_addr: i64;
    pub static kimage_voffset: u64;
    pub static _text: u8;
    pub static _end: u8;
}
#[inline] pub unsafe fn PHYS_OFFSET() -> i64 { debug_assert!((memstart_addr & 1) == 0); memstart_addr }
#[inline] pub unsafe fn kaslr_offset() -> u64 { (&_text as *const u8 as u64) - KIMAGE_VADDR as u64 }
#[cfg(feature = "CONFIG_RANDOMIZE_BASE")] extern "C" { pub fn kaslr_init(); pub static __kaslr_is_enabled: bool; }
#[cfg(feature = "CONFIG_RANDOMIZE_BASE")] pub unsafe fn kaslr_enabled() -> bool { __kaslr_is_enabled }
#[cfg(not(feature = "CONFIG_RANDOMIZE_BASE"))] pub unsafe fn kaslr_init() {}
#[cfg(not(feature = "CONFIG_RANDOMIZE_BASE"))] pub fn kaslr_enabled() -> bool { false }

pub const MIN_MEMBLOCK_ADDR: u64 = 0;
pub const MAX_MEMBLOCK_ADDR: u64 = u64::MAX;
pub const PHYS_PFN_OFFSET: usize = (PHYS_OFFSET() as usize) >> PAGE_SHIFT;

#[inline] pub unsafe fn __untagged_addr(addr: u64) -> u64 { ((addr as i64) << 8 >> 8) as u64 }
#[inline] pub unsafe fn untagged_addr(addr: u64) -> u64 { addr & __untagged_addr(addr) }
#[cfg(any(feature = "CONFIG_KASAN_SW_TAGS", feature = "CONFIG_KASAN_HW_TAGS"))]
#[inline] pub fn __tag_shifted(tag: u8) -> u64 { (tag as u64) << 56 }
#[cfg(not(any(feature = "CONFIG_KASAN_SW_TAGS", feature = "CONFIG_KASAN_HW_TAGS")))]
#[inline] pub fn __tag_shifted(_tag: u8) -> u64 { 0 }
#[inline] pub unsafe fn __tag_reset(addr: u64) -> u64 { __untagged_addr(addr) }
#[inline] pub fn __tag_get(addr: u64) -> u8 { (addr >> 56) as u8 }
#[inline] pub fn __tag_set(addr: *const core::ffi::c_void, tag: u8) -> *const core::ffi::c_void { ((addr as u64 & !__tag_shifted(0xff)) | __tag_shifted(tag)) as *const _ }

pub const IOREMAP_MAX_ORDER_NOTE: &str = "CONFIG_ARM64_4K_PAGES selects PUD_SHIFT; otherwise PMD_SHIFT";

pub type PhysAddr = u64;
pub type Page = core::ffi::c_void;
extern "C" {
    pub fn __virt_to_phys(x: usize) -> PhysAddr;
    pub fn __phys_addr_symbol(x: usize) -> PhysAddr;
    pub fn __phys_to_pfn(x: PhysAddr) -> usize;
    pub fn pfn_is_map_memory(pfn: usize) -> bool;
    pub fn pfn_to_page(pfn: usize) -> *mut Page;
    pub fn page_to_phys(page: *const Page) -> PhysAddr;
    pub fn page_kasan_tag(page: *const Page) -> u8;
    pub fn dump_mem_limit();
}
#[inline] pub unsafe fn __phys_to_virt(x: PhysAddr) -> usize { ((x - PHYS_OFFSET() as u64) as usize) | PAGE_OFFSET }
#[inline] pub unsafe fn __phys_to_kimg(x: PhysAddr) -> usize { (x + kimage_voffset) as usize }
#[inline] pub unsafe fn virt_to_phys(x: *const core::ffi::c_void) -> PhysAddr { __virt_to_phys(x as usize) }
#[inline] pub unsafe fn phys_to_virt(x: PhysAddr) -> *mut core::ffi::c_void { __phys_to_virt(x) as *mut _ }
#[inline] pub unsafe fn virt_to_pfn(x: *const core::ffi::c_void) -> usize { __phys_to_pfn(virt_to_phys(x)) }
#[inline] pub unsafe fn __pa(x: usize) -> PhysAddr { __virt_to_phys(x) }
#[inline] pub unsafe fn __pa_nodebug(x: usize) -> PhysAddr { __virt_to_phys(x) }
#[inline] pub unsafe fn __va(x: PhysAddr) -> *mut core::ffi::c_void { phys_to_virt(x) }
#[inline] pub unsafe fn pfn_to_kaddr(pfn: usize) -> *mut core::ffi::c_void { __va((pfn << PAGE_SHIFT) as PhysAddr) }
#[inline] pub unsafe fn sym_to_pfn(x: usize) -> usize { __phys_to_pfn(__pa(x)) }
#[inline] pub unsafe fn virt_addr_valid(addr: u64) -> bool {
    let a = __tag_reset(addr);
    (a.wrapping_sub(PAGE_OFFSET as u64) < (PAGE_END as u64).wrapping_sub(PAGE_OFFSET as u64)) && pfn_is_map_memory(virt_to_pfn(a as *const _))
}

// CONFIG_EFI && CONFIG_ARM_GIC_V3_ITS: INIT_MEMBLOCK_RESERVED_REGIONS = INIT_MEMBLOCK_REGIONS + NR_CPUS + 1
// CONFIG_EFI: INIT_MEMBLOCK_MEMORY_REGIONS = INIT_MEMBLOCK_REGIONS * 8

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
