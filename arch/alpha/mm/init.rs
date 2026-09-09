// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/alpha/mm/init.c
 *
 *  Copyright (C) 1995  Linus Torvalds
 */

/* 2.3.x zone allocator, 1999 Andrea Arcangeli <andrea@suse.de> */

// C header dependencies are supplied by the surrounding kernel translation.

static mut ORIGINAL_PCB: pcb_struct = pcb_struct { };

unsafe extern "C" {
    static mut init_mm: mm_struct;
    static mut swapper_pg_dir: [pgd_t; 1024];
    static mut hwrpb: *mut hwrpb_struct;
    static mut alpha_using_srm: bool;
    static mut init_thread_info: thread_info;
    static mut max_pfn: c_ulong;
    static mut max_low_pfn: c_ulong;
    static current_stack_pointer: c_ulong;

    fn __pgd_alloc(mm: *mut mm_struct, order: c_int) -> *mut pgd_t;
    fn pgd_offset(mm: *mut mm_struct, address: c_ulong) -> *mut pgd_t;
    fn virt_to_page(address: *mut c_void) -> *mut page;
    fn mk_pte(page: *mut page, prot: pgprot_t) -> pte_t;
    fn virt_to_phys(address: *const c_char) -> c_ulong;
    fn phys_to_virt(address: c_ulong) -> *mut c_void;
    fn __reload_thread(pcb: *mut pcb_struct) -> c_ulong;
    fn tbia();
    fn wrvptptr(address: c_ulong);
    fn hwrpb_update_checksum(hwrpb: *mut hwrpb_struct);
    fn srm_fixup(start: c_ulong, hwrpb: c_ulong) -> c_int;
    fn __halt() -> !;
    fn pgd_offset_k(address: c_ulong) -> *mut pgd_t;
    fn p4d_offset(pgd: *mut pgd_t, address: c_ulong) -> *mut p4d_t;
    fn pud_offset(p4d: *mut p4d_t, address: c_ulong) -> *mut pud_t;
    fn pud_set(pud: *mut pud_t, pmd: *mut pmd_t);
    fn pmd_offset(pud: *mut pud_t, address: c_ulong) -> *mut pmd_t;
    fn pmd_set(pmd: *mut pmd_t, pte: *mut pte_t);
    fn vm_area_register_early(vm: *mut vm_struct, align: c_ulong);
    fn pte_offset_kernel(pmd: *mut pmd_t, address: c_ulong) -> *mut pte_t;
    fn set_pte(pte: *mut pte_t, value: pte_t);
    fn pfn_pte(pfn: c_ulong, prot: pgprot_t) -> pte_t;
    fn absolute_pointer(address: c_ulong) -> *mut c_void;
}

unsafe fn load_PCB(pcb: *mut pcb_struct) -> c_ulong {
    (*pcb).ksp = current_stack_pointer;
    __reload_thread(pcb)
}

#[no_mangle]
pub unsafe extern "C" fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let ret = __pgd_alloc(mm, 0);
    let init = pgd_offset(&raw mut init_mm, 0);
    if !ret.is_null() {
        #[cfg(CONFIG_ALPHA_LARGE_VMALLOC)]
        core::ptr::copy_nonoverlapping(
            init.add(USER_PTRS_PER_PGD),
            ret.add(USER_PTRS_PER_PGD),
            PTRS_PER_PGD - USER_PTRS_PER_PGD - 1,
        );
        #[cfg(not(CONFIG_ALPHA_LARGE_VMALLOC))]
        {
            pgd_val(ret.add(PTRS_PER_PGD - 2), pgd_val(init.add(PTRS_PER_PGD - 2)));
        }
        pgd_val(
            ret.add(PTRS_PER_PGD - 1),
            pte_val(mk_pte(virt_to_page(ret.cast()), PAGE_KERNEL)),
        );
    }
    ret
}

unsafe fn switch_to_system_map() {
    let mut newptbr: c_ulong;
    let mut original_pcb_ptr: c_ulong;

    core::ptr::write_bytes(swapper_pg_dir.as_mut_ptr().cast::<c_void>(), 0, PAGE_SIZE);
    newptbr = (swapper_pg_dir.as_mut_ptr() as c_ulong - PAGE_OFFSET) >> PAGE_SHIFT;
    pgd_val(swapper_pg_dir.as_mut_ptr().add(1023),
            (newptbr << 32) | pgprot_val(PAGE_KERNEL));

    if (*hwrpb).vptb != 0xfffffffe00000000 {
        wrvptptr(0xfffffffe00000000);
        (*hwrpb).vptb = 0xfffffffe00000000;
        hwrpb_update_checksum(hwrpb);
    }

    (*init_thread_info.pcb).ptbr = newptbr;
    (*init_thread_info.pcb).flags = 1;
    original_pcb_ptr = load_PCB(&raw mut init_thread_info.pcb);
    tbia();

    if original_pcb_ptr < PAGE_OFFSET {
        original_pcb_ptr = phys_to_virt(original_pcb_ptr) as c_ulong;
    }
    ORIGINAL_PCB = *(original_pcb_ptr as *const pcb_struct);
}

#[no_mangle]
pub static mut callback_init_done: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn callback_init(mut kernel_end: *mut c_void) -> *mut c_void {
    let crb = (hwrpb as *mut u8).add((*hwrpb).crb_offset as usize) as *mut crb_struct;
    if alpha_using_srm {
        if srm_fixup(VMALLOC_START, hwrpb as c_ulong) != 0 { __halt(); }
        (*crb).dispatch_va = (VMALLOC_START + (*crb).dispatch_va as c_ulong - (*crb).map[0].va) as *mut procdesc_struct;
        (*crb).fixup_va = (VMALLOC_START + (*crb).fixup_va as c_ulong - (*crb).map[0].va) as *mut procdesc_struct;
    }
    switch_to_system_map();
    let two_pages = ((((kernel_end as c_ulong) + !PAGE_MASK) & PAGE_MASK) as *mut c_void);
    kernel_end = (two_pages as *mut u8).add(2 * PAGE_SIZE as usize).cast();
    core::ptr::write_bytes(two_pages, 0, 2 * PAGE_SIZE as usize);
    let pgd = pgd_offset_k(VMALLOC_START);
    let p4d = p4d_offset(pgd, VMALLOC_START);
    let pud = pud_offset(p4d, VMALLOC_START);
    pud_set(pud, two_pages.cast());
    let mut pmd = pmd_offset(pud, VMALLOC_START);
    pmd_set(pmd, (two_pages as *mut u8).add(PAGE_SIZE as usize).cast());
    if alpha_using_srm {
        static mut CONSOLE_REMAP_VM: vm_struct = vm_struct { };
        let mut nr_pages = 0;
        let mut vaddr;
        let mut i = 0;
        while i < (*crb).map_entries { nr_pages += (*crb).map[i].count; i += 1; }
        CONSOLE_REMAP_VM.flags = VM_ALLOC;
        CONSOLE_REMAP_VM.size = nr_pages << PAGE_SHIFT;
        vm_area_register_early(&raw mut CONSOLE_REMAP_VM, PAGE_SIZE);
        vaddr = CONSOLE_REMAP_VM.addr as c_ulong;
        i = 0;
        while i < (*crb).map_entries {
            let mut pfn = (*crb).map[i].pa >> PAGE_SHIFT;
            (*crb).map[i].va = vaddr;
            let mut j = 0;
            while j < (*crb).map[i].count {
                if pmd != pmd_offset(pud, vaddr) {
                    core::ptr::write_bytes(kernel_end, 0, PAGE_SIZE as usize);
                    pmd = pmd_offset(pud, vaddr);
                    pmd_set(pmd, kernel_end.cast());
                    kernel_end = (kernel_end as *mut u8).add(PAGE_SIZE as usize).cast();
                }
                set_pte(pte_offset_kernel(pmd, vaddr), pfn_pte(pfn, PAGE_KERNEL));
                pfn += 1; vaddr += PAGE_SIZE; j += 1;
            }
            i += 1;
        }
    }
    callback_init_done = 1;
    kernel_end
}

#[no_mangle]
pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfn: *mut c_ulong) {
    let dma_pfn = virt_to_phys(MAX_DMA_ADDRESS as *const c_char) >> PAGE_SHIFT;
    max_pfn = max_low_pfn;
    *max_zone_pfn.add(ZONE_DMA) = dma_pfn;
    *max_zone_pfn.add(ZONE_NORMAL) = max_pfn;
}

#[no_mangle]
pub unsafe extern "C" fn paging_init() { core::ptr::write_bytes(absolute_pointer(ZERO_PGE), 0, PAGE_SIZE as usize); }

#[cfg(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_SRM))]
#[no_mangle]
pub unsafe extern "C" fn srm_paging_stop() {
    swapper_pg_dir[1] = swapper_pg_dir[1023]; tbia();
    wrvptptr(0x200000000); (*hwrpb).vptb = 0x200000000; hwrpb_update_checksum(hwrpb);
    load_PCB(&raw mut ORIGINAL_PCB); tbia();
}

static PROTECTION_MAP: [pgprot_t; 16] = [
    _PAGE_P(_PAGE_FOE | _PAGE_FOW | _PAGE_FOR), _PAGE_P(_PAGE_FOE | _PAGE_FOW),
    _PAGE_P(_PAGE_FOE), _PAGE_P(_PAGE_FOE), _PAGE_P(_PAGE_FOW | _PAGE_FOR),
    _PAGE_P(_PAGE_FOW), _PAGE_P(0), _PAGE_P(0),
    _PAGE_S(_PAGE_FOE | _PAGE_FOW | _PAGE_FOR), _PAGE_S(_PAGE_FOE | _PAGE_FOW),
    _PAGE_S(_PAGE_FOE), _PAGE_S(_PAGE_FOE), _PAGE_S(_PAGE_FOW | _PAGE_FOR),
    _PAGE_S(_PAGE_FOW), _PAGE_S(0), _PAGE_S(0),
];

DECLARE_VM_GET_PAGE_PROT!();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
