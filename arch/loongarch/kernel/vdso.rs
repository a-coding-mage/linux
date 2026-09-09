// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the kernel and architecture headers.

extern "C" {
    static mut vdso_start: u8;
    static mut vdso_end: u8;
}

unsafe extern "C" {
    fn kzalloc_objs<T>(count: usize) -> *mut T;
    fn __phys_to_pfn(phys: usize) -> usize;
    fn __pa_symbol(addr: *const u8) -> usize;
    fn pfn_to_page(pfn: usize) -> *mut page;
    fn get_random_u32_below(max: u32) -> u32;
    fn mmap_write_lock_killable(mm: *mut mm_struct) -> bool;
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
        flags: usize,
        sm: *mut vm_special_mapping,
    ) -> *mut vm_area_struct;
    fn mmap_write_unlock(mm: *mut mm_struct);
    fn cpu_to_node(cpu: usize) -> i32;
}

#[repr(C)]
pub struct loongarch_vdso_info {
    pub vdso: *mut u8,
    pub code_mapping: vm_special_mapping,
    pub offset_sigreturn: usize,
    pub size: usize,
}

extern "C" {
    static mut vdso_k_arch_data: *mut vdso_arch_data;
    static mut current: *mut task_struct;
}

#[repr(C)]
pub struct vdso_arch_data {
    pub pdata: [vdso_arch_data_cpu; 0],
}

#[repr(C)]
pub struct vdso_arch_data_cpu {
    pub node: i32,
}

#[repr(C)]
pub struct vm_special_mapping {
    pub name: *const u8,
    pub mremap: Option<unsafe extern "C" fn(*const vm_special_mapping, *mut vm_area_struct) -> i32>,
    pub pages: *mut *mut page,
}

#[repr(C)]
pub struct page;
#[repr(C)]
pub struct vm_area_struct;
#[repr(C)]
pub struct linux_binprm;
#[repr(C)]
pub struct mm_struct {
    pub context: mm_context,
}
#[repr(C)]
pub struct mm_context {
    pub vdso: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct task_struct {
    pub mm: *mut mm_struct,
    pub flags: usize,
    pub thread: thread_struct,
}
#[repr(C)]
pub struct thread_struct {
    pub vdso: *mut loongarch_vdso_info,
}

extern "C" {
    static vdso_offset_sigreturn: usize;
}

unsafe extern "C" fn vdso_mremap(_sm: *const vm_special_mapping, new_vma: *mut vm_area_struct) -> i32 {
    (*(*current).mm).context.vdso = (*new_vma).vm_start as *mut core::ffi::c_void;
    0
}

pub static mut vdso_info: loongarch_vdso_info = loongarch_vdso_info {
    vdso: core::ptr::addr_of_mut!(vdso_start),
    code_mapping: vm_special_mapping {
        name: b"[vdso]\0".as_ptr(),
        mremap: Some(vdso_mremap),
        pages: core::ptr::null_mut(),
    },
    offset_sigreturn: 0,
    size: 0,
};

unsafe extern "C" fn init_vdso() -> i32 {
    let mut i: usize;
    let mut cpu: usize;
    let mut pfn: usize;

    BUG_ON(!PAGE_ALIGNED((*addr_of!(vdso_info)).vdso));

    for_each_possible_cpu!(cpu, {
        (*vdso_k_arch_data).pdata[cpu].node = cpu_to_node(cpu);
    });

    vdso_info.size = PAGE_ALIGN(addr_of!(vdso_end) as usize - addr_of!(vdso_start) as usize);
    vdso_info.code_mapping.pages = kzalloc_objs::<*mut page>(vdso_info.size / PAGE_SIZE);

    if vdso_info.code_mapping.pages.is_null() {
        return -ENOMEM;
    }

    pfn = __phys_to_pfn(__pa_symbol(vdso_info.vdso));
    i = 0;
    while i < vdso_info.size / PAGE_SIZE {
        *vdso_info.code_mapping.pages.add(i) = pfn_to_page(pfn + i);
        i += 1;
    }

    0
}

// subsys_initcall(init_vdso);

unsafe fn vdso_base() -> usize {
    let mut base = STACK_TOP;

    if (*current).flags & PF_RANDOMIZE != 0 {
        base = base.wrapping_add(get_random_u32_below(VDSO_RANDOMIZE_SIZE) as usize);
        base = PAGE_ALIGN(base);
    }

    base
}

pub unsafe extern "C" fn arch_setup_additional_pages(
    _bprm: *mut linux_binprm,
    _uses_interp: i32,
) -> i32 {
    let ret: i32;
    let size: usize;
    let data_addr: usize;
    let vdso_addr: usize;
    let mm = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let info = (*current).thread.vdso;

    if mmap_write_lock_killable(mm) {
        return -EINTR;
    }

    /*
     * Determine total area size. This includes the VDSO data itself
     * and the data pages.
     */
    size = VVAR_SIZE + (*info).size;

    data_addr = get_unmapped_area(core::ptr::null_mut(), vdso_base(), size, 0, 0);
    if IS_ERR_VALUE!(data_addr) {
        ret = data_addr as i32;
        goto_out!(mmap_write_unlock(mm), ret);
    }

    vma = vdso_install_vvar_mapping(mm, data_addr);
    if IS_ERR!(vma) {
        ret = PTR_ERR!(vma);
        goto_out!(mmap_write_unlock(mm), ret);
    }

    vdso_addr = data_addr + VVAR_SIZE;
    vma = _install_special_mapping(
        mm,
        vdso_addr,
        (*info).size,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC | VM_SEALED_SYSMAP,
        &mut (*info).code_mapping,
    );
    if IS_ERR!(vma) {
        ret = PTR_ERR!(vma);
        goto_out!(mmap_write_unlock(mm), ret);
    }

    (*mm).context.vdso = vdso_addr as *mut core::ffi::c_void;
    ret = 0;

    mmap_write_unlock(mm);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
