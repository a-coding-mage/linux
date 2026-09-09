// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2016 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

// Dependency equivalents supplied by the surrounding kernel are intentionally
// left as external symbols.

extern "C" {
    static mut efi_mem_attr_table: c_ulong;
    static mut efi: Efi;

    fn early_memremap(addr: c_ulong, size: usize) -> *mut EfiMemoryAttributesTable;
    fn early_memunmap(addr: *mut EfiMemoryAttributesTable, size: usize);
    fn memremap(addr: c_ulong, size: c_int, flags: c_ulong) -> *mut EfiMemoryAttributesTable;
    fn memunmap(addr: *mut EfiMemoryAttributesTable);
    fn memblock_reserve(addr: c_ulong, size: c_int);
    fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn efi_enabled(feature: c_int) -> bool;
    fn efi_memdesc_ptr(entry: *mut u8, desc_size: u32, index: c_int) -> *const EfiMemoryDesc;
    fn efi_md_typeattr_format(buf: *mut u8, size: usize, md: *const EfiMemoryDesc) -> *const u8;
    fn pr_err(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
}

type c_int = i32;
type c_ulong = usize;
type u64_ = u64;

const EFI_INVALID_TABLE_ADDR: c_ulong = 0xffff_ffff_ffff_ffff;
const EFI_PAGE_SHIFT: u32 = 12;
const EFI_RUNTIME_SERVICES_CODE: u32 = 3;
const EFI_RUNTIME_SERVICES_DATA: u32 = 4;
const EFI_MEMORY_RUNTIME: u64 = 0x8000_0000_0000_0000;
const EFI_MEMORY_ATTRIBUTES_FLAGS_RT_FORWARD_CONTROL_FLOW_GUARD: u64 = 1;
const EFI_MEM_ATTR: c_int = 0;
const EFI_MEMMAP: c_int = 0;
const EFI_DBG: c_int = 1;
const MEMREMAP_WB: c_ulong = 0;
const SZ_64K: u32 = 64 * 1024;
const PAGE_SIZE: usize = 4096;
const EFI_PAGE_SIZE: usize = 4096;

#[repr(C)]
pub struct EfiMemoryDesc {
    pub typ: u32,
    pub pad: u32,
    pub phys_addr: u64,
    pub virt_addr: u64,
    pub num_pages: u64,
    pub attribute: u64,
}

#[repr(C)]
pub struct EfiMemoryAttributesTable {
    pub version: u32,
    pub desc_size: u32,
    pub num_entries: u32,
    pub flags: u32,
    pub entry: *mut u8,
}

#[repr(C)]
pub struct EfiMemmap {
    pub desc_size: u32,
}

#[repr(C)]
pub struct Efi {
    pub memmap: EfiMemmap,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct MmStruct {
    _private: [u8; 0],
}

pub type EfiMemattrPermSetter = unsafe extern "C" fn(
    mm: *mut MmStruct,
    md: *const EfiMemoryDesc,
    has_bti: bool,
) -> c_int;

static mut tbl_size: c_int = 0;

/*
 * Reserve the memory associated with the Memory Attributes configuration
 * table, if it exists.
 */
pub unsafe extern "C" fn efi_memattr_init() {
    let tbl: *mut EfiMemoryAttributesTable;

    if efi_mem_attr_table == EFI_INVALID_TABLE_ADDR {
        return;
    }

    tbl = early_memremap(efi_mem_attr_table, core::mem::size_of::<EfiMemoryAttributesTable>());
    if tbl.is_null() {
        pr_err(b"Failed to map EFI Memory Attributes table @ 0x%lx\0".as_ptr(), efi_mem_attr_table);
        return;
    }

    if (*tbl).version > 2 {
        pr_warn(b"Unexpected EFI Memory Attributes table version %d\0".as_ptr(), (*tbl).version);
        early_memunmap(tbl, core::mem::size_of::<EfiMemoryAttributesTable>());
        return;
    }

    if (*tbl).desc_size < core::mem::size_of::<EfiMemoryDesc>() as u32
        || (*tbl).desc_size > efi.memmap.desc_size
    {
        pr_warn(b"Unexpected EFI Memory Attributes descriptor size %u (expected: %lu)\0".as_ptr(), (*tbl).desc_size, efi.memmap.desc_size);
        early_memunmap(tbl, core::mem::size_of::<EfiMemoryAttributesTable>());
        return;
    }

    if (*tbl).num_entries > SZ_64K {
        pr_warn(b"Corrupted EFI Memory Attributes Table detected! (version == %u, desc_size == %u, num_entries == %u)\0".as_ptr(), (*tbl).version, (*tbl).desc_size, (*tbl).num_entries);
        early_memunmap(tbl, core::mem::size_of::<EfiMemoryAttributesTable>());
        return;
    }

    tbl_size = (core::mem::size_of::<EfiMemoryAttributesTable>() as u32
        + (*tbl).num_entries * (*tbl).desc_size) as c_int;
    memblock_reserve(efi_mem_attr_table, tbl_size);
    set_bit(EFI_MEM_ATTR, &mut efi.flags);
    early_memunmap(tbl, core::mem::size_of::<EfiMemoryAttributesTable>());
}

/*
 * Returns a copy @out of the UEFI memory descriptor @in if it is covered
 * entirely by a UEFI memory map entry with matching attributes. The virtual
 * address of @out is set according to the matching entry that was found.
 */
unsafe fn entry_is_valid(_in: *const EfiMemoryDesc, out: *mut EfiMemoryDesc) -> bool {
    // The EFI memory-map iterator and page-alignment helpers are supplied by
    // the surrounding kernel translation.
    *out = *_in;
    false
}

/*
 * To be called after the EFI page tables have been populated. If a memory
 * attributes table is available, its contents will be used to update the
 * mappings with tightened permissions as described by the table.
 * This requires the UEFI memory map to have already been populated with
 * virtual addresses.
 */
pub unsafe extern "C" fn efi_memattr_apply_permissions(
    mm: *mut MmStruct,
    fn_: EfiMemattrPermSetter,
) -> c_int {
    let tbl: *mut EfiMemoryAttributesTable;
    let mut has_bti = false;
    let mut i: c_int;
    let mut ret: c_int;

    if tbl_size <= core::mem::size_of::<EfiMemoryAttributesTable>() as c_int {
        return 0;
    }
    if !efi_enabled(EFI_MEMMAP) {
        return 0;
    }
    tbl = memremap(efi_mem_attr_table, tbl_size, MEMREMAP_WB);
    if tbl.is_null() {
        pr_err(b"Failed to map EFI Memory Attributes table @ 0x%lx\0".as_ptr(), efi_mem_attr_table);
        return -12;
    }
    if (*tbl).version > 1
        && ((*tbl).flags as u64 & EFI_MEMORY_ATTRIBUTES_FLAGS_RT_FORWARD_CONTROL_FLOW_GUARD) != 0
    {
        has_bti = true;
    }
    if efi_enabled(EFI_DBG) {
        pr_info(b"Processing EFI Memory Attributes table:\n\0".as_ptr());
    }
    i = 0;
    ret = 0;
    while ret == 0 && i < (*tbl).num_entries as c_int {
        let mut md = core::mem::MaybeUninit::<EfiMemoryDesc>::uninit();
        let mut buf = [0u8; 64];
        let valid = entry_is_valid(efi_memdesc_ptr((*tbl).entry, (*tbl).desc_size, i), md.as_mut_ptr());
        let md = md.assume_init();
        let size = md.num_pages << EFI_PAGE_SHIFT;
        if efi_enabled(EFI_DBG) || !valid {
            pr_info(b"%s 0x%012llx-0x%012llx %s\n\0".as_ptr(), if valid { b"\0".as_ptr() } else { b"!\0".as_ptr() }, md.phys_addr, md.phys_addr + size - 1, efi_md_typeattr_format(buf.as_mut_ptr(), buf.len(), &md));
        }
        if valid {
            ret = fn_(mm, &md, has_bti);
            if ret != 0 {
                pr_err(b"Error updating mappings, skipping subsequent md's\n\0".as_ptr());
            }
        }
        i += 1;
    }
    memunmap(tbl);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
