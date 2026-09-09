/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of declarations shared between arch/ppc/mm files. */

// Dependencies supplied by other translation units:
// linux/mm.h, asm/mmu.h, asm/trace.h

#[cfg(feature = "CONFIG_PPC_MMU_NOHASH")]
mod mmu_nohash {
    #[cfg(feature = "CONFIG_PPC_8xx")]
    #[inline]
    pub unsafe fn _tlbil_all() {
        core::arch::asm!("sync; tlbia; isync", options(nostack, preserves_flags));
        trace_tlbia(MMU_NO_CONTEXT);
    }

    #[cfg(feature = "CONFIG_PPC_8xx")]
    #[inline]
    pub unsafe fn _tlbil_pid(pid: u32) {
        core::arch::asm!("sync; tlbia; isync", options(nostack, preserves_flags));
        trace_tlbia(pid);
    }

    #[cfg(not(feature = "CONFIG_PPC_8xx"))]
    unsafe extern "C" {
        pub fn _tlbil_all();
        pub fn _tlbil_pid(pid: u32);
    }

    #[cfg(feature = "CONFIG_PPC_BOOK3E_64")]
    unsafe extern "C" {
        pub fn _tlbil_pid_noind(pid: u32);
    }

    #[cfg(not(feature = "CONFIG_PPC_BOOK3E_64"))]
    #[inline]
    pub unsafe fn _tlbil_pid_noind(pid: u32) { _tlbil_pid(pid); }

    #[cfg(feature = "CONFIG_PPC_8xx")]
    #[inline]
    pub unsafe fn _tlbil_va(address: usize, pid: u32, _tsize: u32, _ind: u32) {
        core::arch::asm!("tlbie {0}; sync", in(reg) address, options(nostack, preserves_flags));
        trace_tlbie(0, 0, address, pid, 0, 0, 0);
    }

    #[cfg(all(not(feature = "CONFIG_PPC_8xx"), feature = "CONFIG_PPC_BOOK3E_64"))]
    unsafe extern "C" { pub fn _tlbil_va(address: usize, pid: u32, tsize: u32, ind: u32); }

    #[cfg(all(not(feature = "CONFIG_PPC_8xx"), not(feature = "CONFIG_PPC_BOOK3E_64")))]
    unsafe extern "C" { pub fn __tlbil_va(address: usize, pid: u32); }

    #[cfg(all(not(feature = "CONFIG_PPC_8xx"), not(feature = "CONFIG_PPC_BOOK3E_64")))]
    #[inline]
    pub unsafe fn _tlbil_va(address: usize, pid: u32, _tsize: u32, _ind: u32) { __tlbil_va(address, pid); }

    #[cfg(any(feature = "CONFIG_PPC_BOOK3E_64", feature = "CONFIG_PPC_47x"))]
    unsafe extern "C" { pub fn _tlbivax_bcast(address: usize, pid: u32, tsize: u32, ind: u32); }

    #[cfg(not(any(feature = "CONFIG_PPC_BOOK3E_64", feature = "CONFIG_PPC_47x")))]
    #[inline]
    pub unsafe fn _tlbivax_bcast(_address: usize, _pid: u32, _tsize: u32, _ind: u32) { BUG(); }

    #[inline] pub fn print_system_hash_info() {}
}

#[cfg(not(feature = "CONFIG_PPC_MMU_NOHASH"))]
unsafe extern "C" { pub fn print_system_hash_info(); }

#[cfg(feature = "CONFIG_PPC32")]
unsafe extern "C" {
    pub fn mapin_ram();
    pub fn setbat(index: i32, virt: usize, phys: phys_addr_t, size: u32, prot: pgprot_t);
    pub static mut early_hash: u8;
}

unsafe extern "C" {
    pub static mut __max_low_memory: usize;
    pub static mut total_memory: phys_addr_t;
    pub static mut total_lowmem: phys_addr_t;
    pub static mut memstart_addr: phys_addr_t;
    pub static mut lowmem_end_addr: phys_addr_t;
}

#[cfg(feature = "CONFIG_PPC32")]
unsafe extern "C" {
    pub fn MMU_init_hw();
    pub fn MMU_init_hw_patch();
    pub fn mmu_mapin_ram(base: usize, top: usize) -> usize;
}
unsafe extern "C" { pub fn mmu_init_secondary(cpu: i32); }

#[cfg(feature = "CONFIG_PPC_E500")]
unsafe extern "C" {
    pub fn map_mem_in_cams(ram: usize, max_cam_idx: i32, dryrun: bool, init: bool) -> usize;
    pub fn loadcam_entry(index: u32);
    pub fn loadcam_multi(first_idx: i32, num: i32, tmp_idx: i32);
}

#[cfg(all(feature = "CONFIG_PPC_E500", feature = "CONFIG_PPC32"))]
unsafe extern "C" {
    pub fn adjust_total_lowmem();
    pub fn switch_to_as1() -> i32;
    pub fn restore_to_as0(esel: i32, offset: i32, dt_ptr: *mut core::ffi::c_void, bootcpu: i32);
    pub fn create_kaslr_tlb_entry(entry: i32, virt: usize, phys: phys_addr_t);
    pub fn reloc_kernel_entry(fdt: *mut core::ffi::c_void, addr: i32);
    pub fn relocate_init(dt_ptr: u64, start: phys_addr_t);
    pub static mut is_second_reloc: i32;
}

#[cfg(all(feature = "CONFIG_PPC_E500", feature = "CONFIG_RANDOMIZE_BASE"))]
unsafe extern "C" { pub fn kaslr_early_init(dt_ptr: *mut core::ffi::c_void, size: phys_addr_t); pub fn kaslr_late_init(); }
#[cfg(all(feature = "CONFIG_PPC_E500", not(feature = "CONFIG_RANDOMIZE_BASE")))]
#[inline] pub fn kaslr_early_init(_dt_ptr: *mut core::ffi::c_void, _size: phys_addr_t) {}
#[cfg(all(feature = "CONFIG_PPC_E500", not(feature = "CONFIG_RANDOMIZE_BASE")))]
#[inline] pub fn kaslr_late_init() {}

#[cfg(feature = "CONFIG_PPC_E500")]
#[repr(C)] pub struct tlbcam { pub MAS0: u32, pub MAS1: u32, pub MAS2: usize, pub MAS3: u32, pub MAS7: u32 }
pub const NUM_TLBCAMS: usize = 64;
#[cfg(feature = "CONFIG_PPC_E500")]
unsafe extern "C" { pub static mut TLBCAM: [tlbcam; NUM_TLBCAMS]; }

#[cfg(any(feature = "CONFIG_PPC_BOOK3S_32", feature = "CONFIG_PPC_85xx", feature = "CONFIG_PPC_8xx"))]
unsafe extern "C" { pub fn v_block_mapped(va: usize) -> phys_addr_t; pub fn p_block_mapped(pa: phys_addr_t) -> usize; }
#[cfg(not(any(feature = "CONFIG_PPC_BOOK3S_32", feature = "CONFIG_PPC_85xx", feature = "CONFIG_PPC_8xx")))]
#[inline] pub fn v_block_mapped(_va: usize) -> phys_addr_t { 0 }
#[cfg(not(any(feature = "CONFIG_PPC_BOOK3S_32", feature = "CONFIG_PPC_85xx", feature = "CONFIG_PPC_8xx")))]
#[inline] pub fn p_block_mapped(_pa: phys_addr_t) -> usize { 0 }

#[cfg(any(feature = "CONFIG_PPC_BOOK3S_32", feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_E500"))]
unsafe extern "C" { pub fn mmu_mark_initmem_nx() -> i32; pub fn mmu_mark_rodata_ro() -> i32; }
#[cfg(not(any(feature = "CONFIG_PPC_BOOK3S_32", feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_E500")))]
#[inline] pub fn mmu_mark_initmem_nx() -> i32 { 0 }
#[cfg(not(any(feature = "CONFIG_PPC_BOOK3S_32", feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_E500")))]
#[inline] pub fn mmu_mark_rodata_ro() -> i32 { 0 }

#[cfg(feature = "CONFIG_PPC_8xx")]
unsafe extern "C" { pub fn mmu_mapin_immr(); }

#[inline]
pub unsafe fn debug_pagealloc_enabled_or_kfence() -> bool {
    cfg!(feature = "CONFIG_KFENCE") || debug_pagealloc_enabled()
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
unsafe extern "C" { pub fn create_section_mapping(start: usize, end: usize, nid: i32, prot: pgprot_t) -> i32; }

unsafe extern "C" { pub fn hash__kernel_map_pages(page: *mut page, numpages: i32, enable: i32) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
