// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// Dependencies supplied by the kernel's EFI, memory-management, and ARM
// headers are intentionally referenced here rather than reimplemented.

use core::ffi::c_void;

#[repr(C)]
pub struct pte_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct efi_memory_desc_t {
    pub typ: u32,
    pub pad: u32,
    pub phys_addr: u64,
    pub virt_addr: u64,
    pub num_pages: u64,
    pub attribute: u64,
}

#[repr(C)]
pub struct efi_arm_entry_state {
    pub cpsr_before_ebs: u32,
    pub sctlr_before_ebs: u32,
    pub cpsr_after_ebs: u32,
    pub sctlr_after_ebs: u32,
}

#[repr(C)]
pub struct map_desc {
    pub virtual_: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: u32,
}

#[repr(C)]
pub struct efi_config_table_type_t {
    pub guid: u128,
    pub table: *mut c_void,
}

extern "C" {
    fn set_pte_bit(pte: pte_t, prot: u32) -> pte_t;
    fn __pgprot(value: u32) -> u32;
    fn set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: u32);
    fn apply_to_page_range(
        mm: *mut mm_struct,
        base: usize,
        size: usize,
        fn_: unsafe extern "C" fn(*mut pte_t, usize, *mut c_void) -> i32,
        data: *mut c_void,
    ) -> i32;
    fn __phys_to_pfn(phys: u64) -> usize;
    fn create_mapping_late(mm: *mut mm_struct, desc: *mut map_desc, permit: bool);
    fn early_memremap_ro(addr: u64, size: usize) -> *mut efi_arm_entry_state;
    fn early_memunmap(addr: *mut efi_arm_entry_state, size: usize);
    fn efi_enabled(flag: u32) -> bool;
    fn efi_memmap_unmap();
    fn efi_init();
}

extern "C" {
    static L_PTE_RDONLY: u32;
    static L_PTE_XN: u32;
    static PTE_EXT_NG: u32;
    static SECTION_SIZE: usize;
    static EFI_PAGE_SHIFT: u32;
    static EFI_PAGE_SIZE: usize;
    static EFI_MEMORY_RO: u64;
    static EFI_MEMORY_XP: u64;
    static EFI_MEMORY_WB: u64;
    static EFI_MEMORY_WT: u64;
    static EFI_MEMORY_WC: u64;
    static MT_MEMORY_RWX: u32;
    static MT_MEMORY_RWX_NONCACHED: u32;
    static MT_DEVICE_WC: u32;
    static MT_DEVICE: u32;
    static EFI_INVALID_TABLE_ADDR: u64;
    static LINUX_EFI_ARM_CPU_STATE_TABLE_GUID: u128;
    static EFI_DBG: u32;
}

unsafe extern "C" {
    fn pr_warn(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
}

unsafe extern "C" fn set_permissions(ptep: *mut pte_t, _addr: usize, data: *mut c_void) -> i32 {
    let md = &*(data as *const efi_memory_desc_t);
    let mut pte = core::ptr::read(ptep);

    if md.attribute & EFI_MEMORY_RO != 0 {
        pte = set_pte_bit(pte, __pgprot(L_PTE_RDONLY));
    }
    if md.attribute & EFI_MEMORY_XP != 0 {
        pte = set_pte_bit(pte, __pgprot(L_PTE_XN));
    }
    set_pte_ext(ptep, pte, PTE_EXT_NG);
    0
}

pub unsafe extern "C" fn efi_set_mapping_permissions(
    mm: *mut mm_struct,
    md: *mut efi_memory_desc_t,
    _ignored: bool,
) -> i32 {
    let base = (*md).virt_addr as usize;
    let size = ((*md).num_pages << EFI_PAGE_SHIFT) as usize;

    /*
     * We can only use apply_to_page_range() if we can guarantee that the
     * entire region was mapped using pages. This should be the case if the
     * region does not cover any naturally aligned SECTION_SIZE sized
     * blocks.
     */
    if (base + size) / SECTION_SIZE * SECTION_SIZE
        < (base + SECTION_SIZE - 1) / SECTION_SIZE * SECTION_SIZE + SECTION_SIZE
    {
        return apply_to_page_range(mm, base, size, set_permissions, md as *mut c_void);
    }

    0
}

pub unsafe extern "C" fn efi_create_mapping(
    mm: *mut mm_struct,
    md: *mut efi_memory_desc_t,
) -> i32 {
    let mut desc = map_desc {
        virtual_: (*md).virt_addr as usize,
        pfn: __phys_to_pfn((*md).phys_addr),
        length: ((*md).num_pages as usize) * EFI_PAGE_SIZE,
        type_: 0,
    };

    /* Order is important here: memory regions may have all of the bits below
     * set (and usually do), so we check them in order of preference. */
    if (*md).attribute & EFI_MEMORY_WB != 0 {
        desc.type_ = MT_MEMORY_RWX;
    } else if (*md).attribute & EFI_MEMORY_WT != 0 {
        desc.type_ = MT_MEMORY_RWX_NONCACHED;
    } else if (*md).attribute & EFI_MEMORY_WC != 0 {
        desc.type_ = MT_DEVICE_WC;
    } else {
        desc.type_ = MT_DEVICE;
    }

    create_mapping_late(mm, &mut desc, true);

    /* If stricter permissions were specified, apply them now. */
    if (*md).attribute & (EFI_MEMORY_RO | EFI_MEMORY_XP) != 0 {
        return efi_set_mapping_permissions(mm, md, false);
    }
    0
}

static mut cpu_state_table: u64 = EFI_INVALID_TABLE_ADDR;

#[no_mangle]
pub static efi_arch_tables: [efi_config_table_type_t; 2] = [
    efi_config_table_type_t {
        guid: LINUX_EFI_ARM_CPU_STATE_TABLE_GUID,
        table: unsafe { &raw mut cpu_state_table as *mut c_void },
    },
    efi_config_table_type_t { guid: 0, table: core::ptr::null_mut() },
];

unsafe fn load_cpu_state_table() {
    if cpu_state_table != EFI_INVALID_TABLE_ADDR {
        let mut dump_state = true;
        let state = early_memremap_ro(
            cpu_state_table,
            core::mem::size_of::<efi_arm_entry_state>(),
        );
        if state.is_null() {
            pr_warn(b"Unable to map CPU entry state table.\n\0".as_ptr());
            return;
        }

        if (*state).sctlr_before_ebs & 1 == 0 {
            pr_warn(b"EFI stub was entered with MMU and Dcache disabled, please fix your firmware!\n\0".as_ptr());
        } else if (*state).sctlr_after_ebs & 1 == 0 {
            pr_warn(b"ExitBootServices() returned with MMU and Dcache disabled, please fix your firmware!\n\0".as_ptr());
        } else {
            dump_state = false;
        }

        if dump_state || efi_enabled(EFI_DBG) {
            pr_info(b"CPSR at EFI stub entry        : 0x%08x\n\0".as_ptr(), (*state).cpsr_before_ebs);
            pr_info(b"SCTLR at EFI stub entry       : 0x%08x\n\0".as_ptr(), (*state).sctlr_before_ebs);
            pr_info(b"CPSR after ExitBootServices() : 0x%08x\n\0".as_ptr(), (*state).cpsr_after_ebs);
            pr_info(b"SCTLR after ExitBootServices(): 0x%08x\n\0".as_ptr(), (*state).sctlr_after_ebs);
        }
        early_memunmap(state, core::mem::size_of::<efi_arm_entry_state>());
    }
}

pub unsafe extern "C" fn arm_efi_init() {
    efi_init();

    // ARM does not permit early mappings to persist across paging_init().
    efi_memmap_unmap();

    load_cpu_state_table();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
