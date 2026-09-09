// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Imagination Technologies
 * Author: Alex Smith <alex.smith@imgtec.com>
 */

// Kernel headers and build-time configuration supplied by the surrounding
// translation unit are intentionally not reproduced here.

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct mips_vdso_image {
    pub data: *const core::ffi::c_void,
    pub size: usize,
    pub mapping: vm_special_mapping,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct vm_special_mapping {
    pub name: *const core::ffi::c_char,
    pub pages: *mut *mut page,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct page;
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct linux_binprm;
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct mm_struct {
    pub context: mm_context,
}
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct mm_context {
    pub vdso: *mut core::ffi::c_void,
}
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct vm_area_struct {
    pub vm_page_prot: usize,
}

extern "C" {
    static mut vdso_image: mips_vdso_image;
    #[cfg(CONFIG_MIPS32_O32)]
    static mut vdso_image_o32: mips_vdso_image;
    #[cfg(CONFIG_MIPS32_N32)]
    static mut vdso_image_n32: mips_vdso_image;
    static mut current: *mut task_struct;
    static cpu_has_dc_aliases: bool;
    static shm_align_mask: usize;
    static vdso_k_time_data: *const core::ffi::c_void;
    static mips_gic_base: *mut core::ffi::c_void;

    fn __phys_to_pfn(addr: usize) -> usize;
    fn __pa_symbol(addr: *const core::ffi::c_void) -> usize;
    fn pfn_to_page(pfn: usize) -> *mut page;
    fn get_random_u32_below(n: u32) -> u32;
    fn mmap_write_lock_killable(mm: *mut mm_struct) -> i32;
    fn mmap_write_unlock(mm: *mut mm_struct);
    fn do_mmap(file: *mut core::ffi::c_void, addr: usize, len: usize, prot: usize,
               flags: usize, vm_flags: usize, pgoff: usize,
               populate: *mut usize, uf: *mut core::ffi::c_void) -> usize;
    fn get_unmapped_area(file: *mut core::ffi::c_void, addr: usize, len: usize,
                         pgoff: usize, flags: usize) -> usize;
    fn mips_gic_present() -> bool;
    fn vdso_install_vvar_mapping(mm: *mut mm_struct, addr: usize) -> *mut vm_area_struct;
    fn _install_special_mapping(mm: *mut mm_struct, addr: usize, len: usize,
                                vm_flags: usize, spec: *const vm_special_mapping)
                                -> *mut vm_area_struct;
    fn __pa(addr: usize) -> usize;
    fn io_remap_pfn_range(vma: *mut vm_area_struct, addr: usize, pfn: usize,
                          size: usize, prot: usize) -> i32;
    fn pgprot_noncached(prot: usize) -> usize;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct task_struct {
    flags: usize,
    thread: thread_struct,
    mm: *mut mm_struct,
}
#[repr(C)]
struct thread_struct {
    abi: *mut abi_struct,
}
#[repr(C)]
struct abi_struct {
    vdso: *mut mips_vdso_image,
}

unsafe fn init_vdso_image(image: *mut mips_vdso_image) {
    // BUG_ON(!PAGE_ALIGNED(image->data));
    // BUG_ON(!PAGE_ALIGNED(image->size));
    let num_pages = (*image).size / PAGE_SIZE;
    let data_pfn = __phys_to_pfn(__pa_symbol((*image).data));
    for i in 0..num_pages {
        let pages = (*image).mapping.pages;
        *pages.add(i) = pfn_to_page(data_pfn + i);
    }
}

unsafe fn init_vdso() -> i32 {
    init_vdso_image(&raw mut vdso_image);
    #[cfg(CONFIG_MIPS32_O32)]
    init_vdso_image(&raw mut vdso_image_o32);
    #[cfg(CONFIG_MIPS32_N32)]
    init_vdso_image(&raw mut vdso_image_n32);
    0
}

// subsys_initcall(init_vdso);

unsafe fn vdso_base() -> usize {
    let mut base = STACK_TOP;
    if cfg!(CONFIG_MIPS_FP_SUPPORT) {
        // Skip the delay slot emulation page
        base += PAGE_SIZE;
    }
    if (*current).flags & PF_RANDOMIZE != 0 {
        base += get_random_u32_below(VDSO_RANDOMIZE_SIZE as u32) as usize;
        base = (base + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    }
    base
}

pub unsafe fn arch_setup_additional_pages(_bprm: *mut linux_binprm, _uses_interp: i32) -> i32 {
    let image = (*(*current).thread.abi).vdso;
    let mm = (*current).mm;
    let (mut gic_size, mut size, mut base, mut data_addr, mut vdso_addr, mut gic_pfn, mut gic_base):
        (usize, usize, usize, usize, usize, usize, usize);
    let mut vma: *mut vm_area_struct;
    let mut ret: i32;

    if mmap_write_lock_killable(mm) != 0 { return -EINTR; }
    if cfg!(CONFIG_MIPS_FP_SUPPORT) {
        let mut unused = 0usize;
        base = do_mmap(core::ptr::null_mut(), STACK_TOP, PAGE_SIZE, PROT_READ | PROT_EXEC,
                       MAP_ANONYMOUS | MAP_PRIVATE | MAP_FIXED, EMPTY_VMA_FLAGS, 0,
                       &mut unused, core::ptr::null_mut());
        if base >= (-4095isize as usize) { ret = base as i32; goto_out(mm, ret); }
    }
    gic_size = if mips_gic_present() { PAGE_SIZE } else { 0 };
    size = gic_size + VDSO_NR_PAGES * PAGE_SIZE + (*image).size;
    if cpu_has_dc_aliases { size += shm_align_mask + 1; }
    base = get_unmapped_area(core::ptr::null_mut(), vdso_base(), size, 0, 0);
    if base >= (-4095isize as usize) { ret = base as i32; goto_out(mm, ret); }
    if cpu_has_dc_aliases && cfg!(CONFIG_MIPS_GENERIC_GETTIMEOFDAY) {
        base &= !shm_align_mask;
        base += (vdso_k_time_data as usize - gic_size) & shm_align_mask;
    }
    data_addr = base + gic_size;
    vdso_addr = data_addr + VDSO_NR_PAGES * PAGE_SIZE;
    if cfg!(CONFIG_MIPS_GENERIC_GETTIMEOFDAY) {
        vma = vdso_install_vvar_mapping(mm, data_addr);
        if vma.is_null() { ret = -1; goto_out(mm, ret); }
    }
    if gic_size != 0 {
        gic_base = mips_gic_base as usize + MIPS_GIC_USER_OFS;
        gic_pfn = PFN_DOWN(__pa(gic_base));
        let gic_mapping = vm_special_mapping { name: b"[gic]\0".as_ptr() as _, pages: core::ptr::null_mut() };
        vma = _install_special_mapping(mm, base, gic_size, VM_READ | VM_MAYREAD, &gic_mapping);
        if vma.is_null() { ret = -1; goto_out(mm, ret); }
        ret = io_remap_pfn_range(vma, base, gic_pfn, gic_size, pgprot_noncached((*vma).vm_page_prot));
        if ret != 0 { goto_out(mm, ret); }
    }
    vma = _install_special_mapping(mm, vdso_addr, (*image).size,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC, &(*image).mapping);
    if vma.is_null() { ret = -1; goto_out(mm, ret); }
    (*mm).context.vdso = vdso_addr as *mut _;
    ret = 0;
    mmap_write_unlock(mm);
    ret
}

unsafe fn goto_out(mm: *mut mm_struct, ret: i32) -> i32 {
    mmap_write_unlock(mm);
    ret
}

// Constants and macros below are provided by the corresponding kernel headers.
extern "C" {
    static PAGE_SIZE: usize;
}
const STACK_TOP: usize = 0;
const PF_RANDOMIZE: usize = 0;
const VDSO_RANDOMIZE_SIZE: usize = 0;
const VDSO_NR_PAGES: usize = 0;
const MIPS_GIC_USER_OFS: usize = 0;
const PFN_DOWN: usize = 0;
const PROT_READ: usize = 0;
const PROT_EXEC: usize = 0;
const MAP_ANONYMOUS: usize = 0;
const MAP_PRIVATE: usize = 0;
const MAP_FIXED: usize = 0;
const EMPTY_VMA_FLAGS: usize = 0;
const VM_READ: usize = 0;
const VM_EXEC: usize = 0;
const VM_MAYREAD: usize = 0;
const VM_MAYWRITE: usize = 0;
const VM_MAYEXEC: usize = 0;
const EINTR: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
