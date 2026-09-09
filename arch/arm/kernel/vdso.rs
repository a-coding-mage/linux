// SPDX-License-Identifier: GPL-2.0-only
/*
 * Adapted from arm64 version.
 *
 * Copyright (C) 2012 ARM Limited
 * Copyright (C) 2015 Mentor Graphics Corporation.
 */

// Kernel and architecture dependencies supplied by other translation units.

const MAX_SYMNAME: usize = 64;

static mut vdso_text_pagelist: *mut *mut page = core::ptr::null_mut();

extern "C" {
    static mut vdso_start: u8;
    static mut vdso_end: u8;
}

/* Total number of pages needed for the data and text portions of the VDSO. */
#[no_mangle]
static mut vdso_total_pages: u32 = 0;

unsafe fn vdso_mremap(
    _sm: *const vm_special_mapping,
    new_vma: *mut vm_area_struct,
) -> i32 {
    (*current).mm.context.vdso = (*new_vma).vm_start;
    0
}

static mut vdso_text_mapping: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const i8,
    mremap: Some(vdso_mremap),
    pages: core::ptr::null_mut(),
};

#[repr(C)]
struct elfinfo {
    hdr: *mut Elf32_Ehdr,       /* ptr to ELF */
    dynsym: *mut Elf32_Sym,     /* ptr to .dynsym section */
    dynsymsize: usize,          /* size of .dynsym section */
    dynstr: *mut i8,            /* ptr to .dynstr section */
}

/* Boot-time check for whether the arch timer exists, and if so,
 * whether the virtual counter is usable.
 */
unsafe fn cntvct_functional() -> bool {
    let mut np: *mut device_node;
    let mut ret = false;

    if !IS_ENABLED_CONFIG_ARM_ARCH_TIMER {
        return ret;
    }

    /* The arm_arch_timer core should export
     * arch_timer_use_virtual or similar so we don't have to do
     * this.
     */
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"arm,armv7-timer\0".as_ptr() as *const i8);
    if np.is_null() {
        np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"arm,armv8-timer\0".as_ptr() as *const i8);
    }
    if np.is_null() {
        return ret;
    }

    if of_property_read_bool(np, b"arm,cpu-registers-not-fw-configured\0".as_ptr() as *const i8) {
        of_node_put(np);
        return ret;
    }

    ret = true;
    of_node_put(np);
    ret
}

unsafe fn find_section(ehdr: *mut Elf32_Ehdr, name: *const i8, size: *mut usize) -> *mut core::ffi::c_void {
    let sechdrs = (ehdr as *mut u8).add((*ehdr).e_shoff as usize) as *mut Elf32_Shdr;
    let secnames = (ehdr as *mut u8).add((*sechdrs.add((*ehdr).e_shstrndx as usize)).sh_offset as usize);

    for i in 1..(*ehdr).e_shnum {
        let sh = sechdrs.add(i as usize);
        if strcmp(secnames.add((*sh).sh_name as usize) as *const i8, name) == 0 {
            if !size.is_null() { *size = (*sh).sh_size as usize; }
            return (ehdr as *mut u8).add((*sh).sh_offset as usize) as *mut core::ffi::c_void;
        }
    }
    if !size.is_null() { *size = 0; }
    core::ptr::null_mut()
}

unsafe fn find_symbol(lib: *mut elfinfo, symname: *const i8) -> *mut Elf32_Sym {
    for i in 0..((*lib).dynsymsize / core::mem::size_of::<Elf32_Sym>()) {
        let mut name = [0i8; MAX_SYMNAME];
        if (*(*lib).dynsym.add(i)).st_name == 0 { continue; }
        strscpy(name.as_mut_ptr(), (*lib).dynstr.add((*(*lib).dynsym.add(i)).st_name as usize), MAX_SYMNAME);
        let c = strchr(name.as_mut_ptr(), b'@' as i32);
        if !c.is_null() { *c = 0; }
        if strcmp(symname, name.as_ptr()) == 0 { return (*lib).dynsym.add(i); }
    }
    core::ptr::null_mut()
}

unsafe fn vdso_nullpatch_one(lib: *mut elfinfo, symname: *const i8) {
    let sym = find_symbol(lib, symname);
    if !sym.is_null() { (*sym).st_name = 0; }
}

unsafe fn patch_vdso(ehdr: *mut core::ffi::c_void) {
    let mut einfo = elfinfo { hdr: ehdr as *mut Elf32_Ehdr, dynsym: core::ptr::null_mut(), dynsymsize: 0, dynstr: core::ptr::null_mut() };
    einfo.dynsym = find_section(einfo.hdr, b".dynsym\0".as_ptr() as *const i8, &mut einfo.dynsymsize) as *mut Elf32_Sym;
    einfo.dynstr = find_section(einfo.hdr, b".dynstr\0".as_ptr() as *const i8, core::ptr::null_mut()) as *mut i8;
    if !cntvct_functional() {
        for s in [b"__vdso_gettimeofday\0", b"__vdso_clock_gettime\0", b"__vdso_clock_gettime64\0", b"__vdso_clock_getres\0", b"__vdso_clock_getres_time64\0"] {
            vdso_nullpatch_one(&mut einfo, s.as_ptr() as *const i8);
        }
    }
}

unsafe fn vdso_init() -> i32 {
    if core::slice::from_raw_parts(&vdso_start, 4) != b"\x7fELF" { pr_err(b"VDSO is not a valid ELF object!\n\0".as_ptr() as *const i8); return -ENOEXEC; }
    let text_pages = (vdso_end as usize - vdso_start as usize) >> PAGE_SHIFT;
    vdso_text_pagelist = kzalloc_objs::<*mut page>(text_pages);
    if vdso_text_pagelist.is_null() { return -ENOMEM; }
    for i in 0..text_pages { *vdso_text_pagelist.add(i) = virt_to_page(vdso_start.add(i * PAGE_SIZE)); }
    vdso_text_mapping.pages = vdso_text_pagelist;
    vdso_total_pages = VDSO_NR_PAGES + text_pages as u32;
    patch_vdso(&mut vdso_start as *mut u8 as *mut core::ffi::c_void);
    0
}

/* arch_initcall(vdso_init); */
static_assert!(__VDSO_PAGES == VDSO_NR_PAGES);

/* assumes mmap_lock is write-locked */
#[no_mangle]
pub unsafe fn arm_install_vdso(mm: *mut mm_struct, mut addr: usize) {
    let mut vma: *mut vm_area_struct;
    let len: usize;
    (*mm).context.vdso = 0;
    if vdso_text_pagelist.is_null() { return; }
    if IS_ERR(vdso_install_vvar_mapping(mm, addr)) { return; }
    addr += VDSO_NR_PAGES * PAGE_SIZE;
    len = (vdso_total_pages as usize - VDSO_NR_PAGES) << PAGE_SHIFT;
    vma = _install_special_mapping(mm, addr, len, VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC, &mut vdso_text_mapping);
    if !IS_ERR(vma) { (*mm).context.vdso = addr; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
