// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Modifications by Kumar Gala (galak@kernel.crashing.org) to support
 * E500 Book E processors.
 *
 * Copyright 2004,2010 Freescale Semiconductor, Inc.
 *
 * This file contains the routines for initializing the MMU
 * on the 4xx series of chips.
 *  -- paulus
 *
 *  Derived from arch/ppc/mm/init.c:
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 *  and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *    Copyright (C) 1996 Paul Mackerras
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 */

extern "C" {
    static mut tlbcam_index: u32;
    static mut TLBCAM: [tlbcam; NUM_TLBCAMS];
    static mut memstart_addr: phys_addr_t;
    static mut _sinittext: usize;
    static mut _stext: usize;
    static mut __max_low_memory: usize;
    static mut total_lowmem: usize;
    static mut kernstart_virt_addr: usize;
    static mut kernstart_addr: phys_addr_t;
    static mut virt_phys_offset: usize;
    static mut is_second_reloc: i32;

    fn __ilog2(value: usize) -> u32;
    fn __ffs(value: usize) -> u32;
    fn mmu_has_feature(feature: usize) -> bool;
    fn is_kernel_addr(addr: usize) -> bool;
    fn mfspr(reg: usize) -> usize;
    fn mtspr(reg: usize, value: usize);
    fn isync();
    fn strict_kernel_rwx_enabled() -> bool;
    fn pgprot_val(prot: pgprot_t) -> usize;
    fn loadcam_multi(first: i32, count: i32, max: i32);
    fn switch_to_as1() -> i32;
    fn restore_to_as0(asid: i32, offset: isize, ptr: *mut core::ffi::c_void, value: i32);
    fn memblock_set_current_limit(limit: phys_addr_t);
    fn kaslr_late_init();
    fn early_get_first_memblock_info(ptr: *mut core::ffi::c_void, size: *mut phys_addr_t);
    fn kaslr_early_init(ptr: *mut core::ffi::c_void, size: phys_addr_t);
    fn panic(msg: *const u8) -> !;
    fn pr_info(fmt: *const u8, ...);
    fn pr_cont(fmt: *const u8, ...);
}

static mut tlbcam_addrs: [TlbcamAddrs; NUM_TLBCAMS] = [TlbcamAddrs { start: 0, limit: 0, phys: 0 }; NUM_TLBCAMS];

#[repr(C)]
#[derive(Copy, Clone)]
struct TlbcamAddrs { start: usize, limit: usize, phys: phys_addr_t }

#[cfg(CONFIG_PPC_85xx)]
pub unsafe fn v_block_mapped(va: usize) -> phys_addr_t {
    let mut b = 0;
    while b < tlbcam_index {
        if va >= tlbcam_addrs[b as usize].start && va < tlbcam_addrs[b as usize].limit {
            return tlbcam_addrs[b as usize].phys + (va - tlbcam_addrs[b as usize].start) as phys_addr_t;
        }
        b += 1;
    }
    0
}

#[cfg(CONFIG_PPC_85xx)]
pub unsafe fn p_block_mapped(pa: phys_addr_t) -> usize {
    let mut b = 0;
    while b < tlbcam_index {
        let a = &tlbcam_addrs[b as usize];
        if pa >= a.phys && pa < (a.limit - a.start) as phys_addr_t + a.phys {
            return a.start + (pa - a.phys) as usize;
        }
        b += 1;
    }
    0
}

unsafe fn settlbcam(index: usize, virt: usize, phys: phys_addr_t, size: usize, mut flags: usize, pid: u32) {
    let tsize = __ilog2(size) - 10;

    // CONFIG_SMP || CONFIG_PPC_E500MC
    if (flags & _PAGE_NO_CACHE) == 0 { flags |= _PAGE_COHERENT; }

    TLBCAM[index].MAS0 = MAS0_TLBSEL(1) | MAS0_ESEL(index) | MAS0_NV(index + 1);
    TLBCAM[index].MAS1 = MAS1_VALID | MAS1_IPROT | MAS1_TSIZE(tsize) | MAS1_TID(pid);
    TLBCAM[index].MAS2 = virt & PAGE_MASK;
    TLBCAM[index].MAS2 |= if flags & _PAGE_WRITETHRU != 0 { MAS2_W } else { 0 };
    TLBCAM[index].MAS2 |= if flags & _PAGE_NO_CACHE != 0 { MAS2_I } else { 0 };
    TLBCAM[index].MAS2 |= if flags & _PAGE_COHERENT != 0 { MAS2_M } else { 0 };
    TLBCAM[index].MAS2 |= if flags & _PAGE_GUARDED != 0 { MAS2_G } else { 0 };
    TLBCAM[index].MAS2 |= if flags & _PAGE_ENDIAN != 0 { MAS2_E } else { 0 };
    TLBCAM[index].MAS3 = (phys & MAS3_RPN) | MAS3_SR;
    TLBCAM[index].MAS3 |= if flags & _PAGE_WRITE != 0 { MAS3_SW } else { 0 };
    if mmu_has_feature(MMU_FTR_BIG_PHYS) { TLBCAM[index].MAS7 = (phys as u64 >> 32) as usize; }
    if !is_kernel_addr(virt) {
        TLBCAM[index].MAS3 |= MAS3_UR;
        TLBCAM[index].MAS3 |= if flags & _PAGE_EXEC != 0 { MAS3_UX } else { 0 };
        TLBCAM[index].MAS3 |= if flags & _PAGE_WRITE != 0 { MAS3_UW } else { 0 };
    } else { TLBCAM[index].MAS3 |= if flags & _PAGE_EXEC != 0 { MAS3_SX } else { 0 }; }
    tlbcam_addrs[index] = TlbcamAddrs { start: virt, limit: virt + size - 1, phys };
}

unsafe fn calc_cam_sz(ram: usize, virt: usize, phys: phys_addr_t) -> usize {
    let mut camsize = __ilog2(ram);
    let mut align = __ffs(virt | phys as usize);
    let max_cam;
    if (mfspr(SPRN_MMUCFG) & MMUCFG_MAVN) == MMUCFG_MAVN_V1 {
        max_cam = ((mfspr(SPRN_TLB1CFG) >> 16) & 0xf) * 2 + 10;
        camsize &= !1;
        align &= !1;
    } else { max_cam = __ilog2(mfspr(SPRN_TLB1PS)) + 10; }
    if camsize > align { camsize = align; }
    if camsize > max_cam as u32 { camsize = max_cam as u32; }
    1usize << camsize
}

unsafe fn map_mem_in_cams_addr(mut phys: phys_addr_t, mut virt: usize, mut ram: usize, max_cam_idx: i32, dryrun: bool, init: bool) -> usize {
    let mut i = 0;
    let mut amount_mapped = 0;
    let boundary = if strict_kernel_rwx_enabled() { _sinittext - _stext } else { ram };
    let mut boundary = boundary;
    while boundary != 0 && i < max_cam_idx {
        let prot = if init { PAGE_KERNEL_X } else { PAGE_KERNEL_ROX };
        let cam_sz = calc_cam_sz(boundary, virt, phys);
        if !dryrun { settlbcam(i as usize, virt, phys, cam_sz, pgprot_val(prot), 0); }
        boundary -= cam_sz; amount_mapped += cam_sz; virt += cam_sz; phys += cam_sz as phys_addr_t; i += 1;
    }
    ram -= amount_mapped;
    while ram != 0 && i < max_cam_idx {
        let prot = if init { PAGE_KERNEL_X } else { PAGE_KERNEL };
        let cam_sz = calc_cam_sz(ram, virt, phys);
        if !dryrun { settlbcam(i as usize, virt, phys, cam_sz, pgprot_val(prot), 0); }
        ram -= cam_sz; amount_mapped += cam_sz; virt += cam_sz; phys += cam_sz as phys_addr_t; i += 1;
    }
    if dryrun { return amount_mapped; }
    if init { loadcam_multi(0, i, max_cam_idx); tlbcam_index = i as u32; }
    else { loadcam_multi(0, i, 0); /* WARN_ON(i > tlbcam_index) */ }
    amount_mapped
}

pub unsafe fn map_mem_in_cams(ram: usize, max_cam_idx: i32, dryrun: bool, init: bool) -> usize {
    map_mem_in_cams_addr(memstart_addr, PAGE_OFFSET, ram, max_cam_idx, dryrun, init)
}

#[cfg(CONFIG_PPC32)]
pub unsafe fn mmu_mapin_ram(_base: usize, _top: usize) -> usize { tlbcam_addrs[(tlbcam_index - 1) as usize].limit - PAGE_OFFSET + 1 }

pub unsafe fn flush_instruction_cache() { let mut tmp = mfspr(SPRN_L1CSR1); tmp |= L1CSR1_ICFI | L1CSR1_ICLFR; mtspr(SPRN_L1CSR1, tmp); isync(); }
pub unsafe fn MMU_init_hw() { flush_instruction_cache(); }
unsafe fn tlbcam_sz(idx: usize) -> usize { tlbcam_addrs[idx].limit - tlbcam_addrs[idx].start + 1 }

pub unsafe fn adjust_total_lowmem() {
    let ram = core::cmp::min(__max_low_memory, total_lowmem);
    let i = switch_to_as1();
    __max_low_memory = map_mem_in_cams(ram, CONFIG_LOWMEM_CAM_NUM, false, true);
    restore_to_as0(i, 0, core::ptr::null_mut(), 1);
    pr_info(b"Memory CAM mapping: \0".as_ptr());
    let mut j = 0;
    while j < tlbcam_index - 1 { pr_cont(b"%lu/\0".as_ptr(), (tlbcam_sz(j as usize) >> 20) as u64); j += 1; }
    pr_cont(b"%lu Mb, residual: %dMb\n\0".as_ptr(), (tlbcam_sz((tlbcam_index - 1) as usize) >> 20) as u64, ((total_lowmem - __max_low_memory) >> 20) as u32);
    memblock_set_current_limit(memstart_addr + __max_low_memory as phys_addr_t);
}

pub unsafe fn mmu_mark_initmem_nx() -> i32 { 0 }
pub unsafe fn setup_initial_memory_limit(first_memblock_base: phys_addr_t, first_memblock_size: phys_addr_t) { memblock_set_current_limit(core::cmp::min(first_memblock_base + first_memblock_size, 0x04000000)); }

#[cfg(CONFIG_STRICT_KERNEL_RWX)]
pub unsafe fn mmu_mark_rodata_ro() -> i32 {
    let remapped = map_mem_in_cams(__max_low_memory, CONFIG_LOWMEM_CAM_NUM, false, false);
    if __max_low_memory != remapped { return -EINVAL; }
    0
}

#[cfg(CONFIG_RELOCATABLE)]
pub unsafe fn relocate_init(dt_ptr: u64, start: phys_addr_t) {
    let mut base = kernstart_virt_addr;
    let mut size: phys_addr_t = 0;
    kernstart_addr = start;
    if is_second_reloc != 0 {
        virt_phys_offset = PAGE_OFFSET - memstart_addr as usize;
        kaslr_late_init();
        return;
    }
    start &= !0x3ffffff;
    base &= !0x3ffffff;
    virt_phys_offset = base - start as usize;
    early_get_first_memblock_info(__va(dt_ptr) as *mut core::ffi::c_void, &mut size);
    if start != memstart_addr {
        let offset = start as isize - memstart_addr as isize;
        is_second_reloc = 1;
        let n = switch_to_as1();
        if memstart_addr > start {
            map_mem_in_cams(0x4000000, CONFIG_LOWMEM_CAM_NUM, false, true);
        } else {
            map_mem_in_cams_addr(start, PAGE_OFFSET.wrapping_add(offset as usize), 0x4000000, CONFIG_LOWMEM_CAM_NUM, false, true);
        }
        restore_to_as0(n, offset, __va(dt_ptr) as *mut core::ffi::c_void, 1);
        panic(b"Relocation error\0".as_ptr());
    }
    kaslr_early_init(__va(dt_ptr) as *mut core::ffi::c_void, size);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
