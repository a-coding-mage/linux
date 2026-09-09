// SPDX-License-Identifier: GPL-2.0
/*
 * vdso setup for s390
 *
 *  Copyright IBM Corp. 2008
 *  Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 */

// Kernel and architecture dependencies are supplied by the surrounding crate.

extern "C" {
    static mut vdso_start: u8;
    static mut vdso_end: u8;

    static mut current: *mut task_struct;
    static mut vdso_mapping: vm_special_mapping;

    fn set_tod_programmable_field(cpu: i32);
    fn smp_processor_id() -> i32;
    fn mmap_write_lock_killable(mm: *mut mm_struct) -> i32;
    fn get_unmapped_area(
        file: *mut core::ffi::c_void,
        addr: usize,
        len: usize,
        pgoff: usize,
        flags: usize,
    ) -> usize;
    fn vdso_install_vvar_mapping(mm: *mut mm_struct, addr: usize) -> *mut vm_area_struct;
    fn _install_special_mapping(
        mm: *mut mm_struct,
        addr: usize,
        len: usize,
        vm_flags: usize,
        sm: *mut vm_special_mapping,
    ) -> *mut vm_area_struct;
    fn do_munmap(
        mm: *mut mm_struct,
        start: usize,
        len: usize,
        uf: *mut core::ffi::c_void,
    ) -> i32;
    fn mmap_write_unlock(mm: *mut mm_struct);
    fn get_random_u32_below(n: u32) -> u32;
    fn kzalloc_objs<T>(count: usize) -> *mut T;
    fn panic(fmt: *const u8, ... ) -> !;
    fn virt_to_page(addr: *mut core::ffi::c_void) -> *mut page;
    fn find_section(
        hdr: *const elf64_hdr,
        shdr: *const elf64_shdr,
        name: *const u8,
    ) -> *const elf64_shdr;
    fn apply_alternatives(start: *mut alt_instr, end: *mut alt_instr);
}

#[repr(C)]
struct task_struct { mm: *mut mm_struct, flags: usize }
#[repr(C)]
struct mm_struct { context: mm_context }
#[repr(C)]
struct mm_context { vdso_base: usize, start_stack: usize }
#[repr(C)]
struct vm_area_struct { vm_start: usize }
#[repr(C)]
struct page;
#[repr(C)]
struct linux_binprm;
#[repr(C)]
struct elf64_hdr { e_shoff: usize }
#[repr(C)]
struct elf64_shdr { sh_offset: usize, sh_size: usize }
#[repr(C)]
struct alt_instr;
#[repr(C)]
struct vm_special_mapping {
    name: *const u8,
    mremap: Option<unsafe extern "C" fn(*const vm_special_mapping, *mut vm_area_struct) -> i32>,
    pages: *mut *mut page,
}

unsafe extern "C" fn vdso_mremap(
    _sm: *const vm_special_mapping,
    vma: *mut vm_area_struct,
) -> i32 {
    (*(*current).mm).context.vdso_base = (*vma).vm_start;
    0
}

static mut VDSO_MAPPING: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr(),
    mremap: Some(vdso_mremap),
    pages: core::ptr::null_mut(),
};

pub unsafe extern "C" fn vdso_getcpu_init() -> i32 {
    set_tod_programmable_field(smp_processor_id());
    0
}
// early_initcall(vdso_getcpu_init); /* Must be called before SMP init */

unsafe fn map_vdso(addr: usize, vdso_mapping_len: usize) -> i32 {
    let mm = (*current).mm;
    let mut vvar_start: usize;
    let vdso_text_len: usize = (&vdso_end as *const u8 as usize)
        .wrapping_sub(&vdso_start as *const u8 as usize);
    let mut vma: *mut vm_area_struct;

    // BUILD_BUG_ON(VDSO_NR_PAGES != __VDSO_PAGES);
    if mmap_write_lock_killable(mm) != 0 { return -4; /* -EINTR */ }

    vvar_start = get_unmapped_area(core::ptr::null_mut(), addr, vdso_mapping_len, 0, 0);
    let mut rc = vvar_start as i32;
    if (vvar_start as isize) < 0 { mmap_write_unlock(mm); return rc; }
    vma = vdso_install_vvar_mapping(mm, vvar_start);
    rc = vma as isize as i32;
    if (vma as isize) < 0 { mmap_write_unlock(mm); return rc; }
    let vdso_text_start = vvar_start + VDSO_NR_PAGES * PAGE_SIZE;
    // VM_MAYWRITE for COW so gdb can set breakpoints
    vma = _install_special_mapping(mm, vdso_text_start, vdso_text_len,
        VM_READ | VM_EXEC | VM_SEALED_SYSMAP | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC,
        &mut VDSO_MAPPING);
    if (vma as isize) < 0 {
        do_munmap(mm, vvar_start, PAGE_SIZE, core::ptr::null_mut());
        rc = vma as isize as i32;
    } else {
        (*(*current).mm).context.vdso_base = vdso_text_start;
        rc = 0;
    }
    mmap_write_unlock(mm);
    rc
}

unsafe fn vdso_addr(mut start: usize, len: usize) -> usize {
    let addr;
    start = (start + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    let mut end = (start + len + PMD_SIZE - 1) & PMD_MASK;
    if end >= VDSO_BASE { end = VDSO_BASE; }
    end -= len;
    if end > start {
        let offset = get_random_u32_below((((end - start) >> PAGE_SHIFT) + 1) as u32) as usize;
        addr = start + (offset << PAGE_SHIFT);
    } else { addr = start; }
    addr
}

pub unsafe extern "C" fn vdso_text_size() -> usize {
    ((&vdso_end as *const u8 as usize).wrapping_sub(&vdso_start as *const u8 as usize)
        + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

pub unsafe extern "C" fn vdso_size() -> usize { vdso_text_size() + VDSO_NR_PAGES * PAGE_SIZE }

pub unsafe extern "C" fn arch_setup_additional_pages(_bprm: *mut linux_binprm, _uses_interp: i32) -> i32 {
    let size = vdso_size();
    let mut addr = VDSO_BASE;
    if (*current).flags & PF_RANDOMIZE != 0 {
        addr = vdso_addr((*(*current).mm).context.start_stack + PAGE_SIZE, size);
    }
    map_vdso(addr, size)
}

unsafe fn vdso_setup_pages(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> *mut *mut page {
    let pages = (end as usize - start as usize) >> PAGE_SHIFT;
    let pagelist = kzalloc_objs::<*mut page>(pages + 1);
    if pagelist.is_null() { panic(b"%s: Cannot allocate page list for VDSO\0".as_ptr(), b"vdso_setup_pages\0".as_ptr()); }
    for i in 0..pages { *pagelist.add(i) = virt_to_page((start as usize + i * PAGE_SIZE) as *mut core::ffi::c_void); }
    pagelist
}

unsafe fn vdso_apply_alternatives() {
    let hdr = &vdso_start as *const u8 as *const elf64_hdr;
    let shdr = (hdr as *const u8).add((*hdr).e_shoff) as *const elf64_shdr;
    let alt = find_section(hdr, shdr, b".altinstructions\0".as_ptr());
    if alt.is_null() { return; }
    let start = (hdr as *const u8).add((*alt).sh_offset) as *mut alt_instr;
    let end = (start as *mut u8).add((*alt).sh_size) as *mut alt_instr;
    apply_alternatives(start, end);
}

pub unsafe extern "C" fn vdso_init() -> i32 {
    vdso_apply_alternatives();
    VDSO_MAPPING.pages = vdso_setup_pages(&mut vdso_start as *mut u8 as *mut _, &mut vdso_end as *mut u8 as *mut _);
    0
}
// arch_initcall(vdso_init);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
