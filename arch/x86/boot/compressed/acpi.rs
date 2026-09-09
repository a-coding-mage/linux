// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding boot environment are intentionally
// left external, corresponding to the C includes.

const MAX_ACPI_ARG_LENGTH: usize = 10;

// Immovable memory regions representation. Max amount of memory regions is
// MAX_NUMNODES*2.
#[no_mangle]
pub static mut immovable_mem: [mem_vector; MAX_NUMNODES * 2] =
    [mem_vector { start: 0, size: 0 }; MAX_NUMNODES * 2];

unsafe fn __efi_get_rsdp_addr(cfg_tbl_pa: c_ulong, cfg_tbl_len: c_uint) -> acpi_physical_address {
    // CONFIG_EFI
    let rsdp_addr = efi_find_vendor_table(boot_params_ptr, cfg_tbl_pa, cfg_tbl_len, ACPI_20_TABLE_GUID);
    if rsdp_addr != 0 {
        return rsdp_addr as acpi_physical_address;
    }

    let rsdp_addr = efi_find_vendor_table(boot_params_ptr, cfg_tbl_pa, cfg_tbl_len, ACPI_TABLE_GUID);
    if rsdp_addr != 0 {
        return rsdp_addr as acpi_physical_address;
    }

    debug_putstr(b"Error getting RSDP address.\0".as_ptr() as *const c_char);
    0
}

unsafe fn efi_get_rsdp_addr() -> acpi_physical_address {
    // CONFIG_EFI
    let mut cfg_tbl_pa: c_ulong = 0;
    let mut cfg_tbl_len: c_uint = 0;
    let et = efi_get_type(boot_params_ptr);
    if et == EFI_TYPE_NONE {
        return 0;
    }

    let systab_pa = efi_get_system_table(boot_params_ptr);
    if systab_pa == 0 {
        error(b"EFI support advertised, but unable to locate system table.\0".as_ptr() as *const c_char);
    }

    let ret = efi_get_conf_table(boot_params_ptr, &mut cfg_tbl_pa, &mut cfg_tbl_len);
    if ret != 0 || cfg_tbl_pa == 0 {
        error(b"EFI config table not found.\0".as_ptr() as *const c_char);
    }

    __efi_get_rsdp_addr(cfg_tbl_pa, cfg_tbl_len)
}

unsafe fn compute_checksum(mut buffer: *mut u8, length: u32) -> u8 {
    let end = buffer.add(length as usize);
    let mut sum: u8 = 0;
    while buffer < end {
        sum = sum.wrapping_add(*buffer);
        buffer = buffer.add(1);
    }
    sum
}

/* Search a block of memory for the RSDP signature. */
unsafe fn scan_mem_for_rsdp(start: *mut u8, length: u32) -> *mut u8 {
    let end = start.add(length as usize);
    let mut address = start;
    while address < end {
        let rsdp = address as *mut acpi_table_rsdp;
        if !ACPI_VALIDATE_RSDP_SIG((*rsdp).signature) {
            address = address.add(ACPI_RSDP_SCAN_STEP as usize);
            continue;
        }
        if compute_checksum(rsdp as *mut u8, ACPI_RSDP_CHECKSUM_LENGTH) != 0 {
            address = address.add(ACPI_RSDP_SCAN_STEP as usize);
            continue;
        }
        if (*rsdp).revision >= 2
            && compute_checksum(rsdp as *mut u8, ACPI_RSDP_XCHECKSUM_LENGTH) != 0
        {
            address = address.add(ACPI_RSDP_SCAN_STEP as usize);
            continue;
        }
        return address;
    }
    core::ptr::null_mut()
}

/* Search RSDP address in EBDA. */
unsafe fn bios_get_rsdp_addr() -> acpi_physical_address {
    let mut address = *(ACPI_EBDA_PTR_LOCATION as *const u16) as c_ulong;
    address <<= 4;
    if address > 0x400 {
        let rsdp = scan_mem_for_rsdp(address as *mut u8, ACPI_EBDA_WINDOW_SIZE);
        if !rsdp.is_null() {
            return rsdp as c_ulong as acpi_physical_address;
        }
    }
    let rsdp = scan_mem_for_rsdp(ACPI_HI_RSDP_WINDOW_BASE as *mut u8, ACPI_HI_RSDP_WINDOW_SIZE);
    if !rsdp.is_null() {
        return rsdp as c_ulong as acpi_physical_address;
    }
    0
}

/* Return RSDP address on success, otherwise 0. */
#[no_mangle]
pub unsafe fn get_rsdp_addr() -> acpi_physical_address {
    let mut pa = (*boot_params_ptr).acpi_rsdp_addr;
    if pa == 0 { pa = efi_get_rsdp_addr(); }
    if pa == 0 { pa = bios_get_rsdp_addr(); }
    pa
}

// CONFIG_RANDOMIZE_BASE && CONFIG_MEMORY_HOTREMOVE
#[cfg(any())]
const MAX_ADDR_LEN: usize = 19;

#[cfg(any())]
unsafe fn get_cmdline_acpi_rsdp() -> c_ulong {
    let mut addr: c_ulong = 0;
    // CONFIG_KEXEC_CORE
    let mut val = [0i8; MAX_ADDR_LEN];
    let ret = cmdline_find_option(b"acpi_rsdp\0".as_ptr() as *const c_char, val.as_mut_ptr(), val.len());
    if ret < 0 || ret >= val.len() as i32 { return 0; }
    if boot_kstrtoul(val.as_ptr(), 16, &mut addr) != 0 { return 0; }
    addr
}

#[cfg(any())]
unsafe fn get_acpi_srat_table() -> c_ulong {
    let mut root_table: c_ulong;
    let mut acpi_table: c_ulong;
    let mut header: *mut acpi_table_header;
    let rsdp_addr = get_cmdline_acpi_rsdp();
    let rsdp = if rsdp_addr != 0 { rsdp_addr as *mut acpi_table_rsdp } else { (*boot_params_ptr).acpi_rsdp_addr as *mut acpi_table_rsdp };
    if rsdp.is_null() { return 0; }
    let mut arg = [0i8; 10];
    if !(cmdline_find_option(b"acpi\0".as_ptr() as *const c_char, arg.as_mut_ptr(), arg.len()) == 4 && strncmp(arg.as_ptr(), b"rsdt\0".as_ptr() as *const c_char, 4) == 0)
        && (*rsdp).xsdt_physical_address != 0 && (*rsdp).revision > 1 {
        root_table = (*rsdp).xsdt_physical_address as c_ulong;
        let size = ACPI_XSDT_ENTRY_SIZE;
        header = root_table as *mut acpi_table_header;
        let len = (*header).length;
        if len < core::mem::size_of::<acpi_table_header>() as u32 + size { return 0; }
        let mut num_entries = (len - core::mem::size_of::<acpi_table_header>() as u32) / size;
        let mut entry = (root_table + core::mem::size_of::<acpi_table_header>() as c_ulong) as *mut u8;
        while num_entries != 0 { acpi_table = *(entry as *const u64) as c_ulong; if acpi_table != 0 { header = acpi_table as *mut acpi_table_header; if ACPI_COMPARE_NAMESEG((*header).signature, ACPI_SIG_SRAT) { return acpi_table; } } entry = entry.add(size as usize); num_entries -= 1; }
    } else {
        root_table = (*rsdp).rsdt_physical_address as c_ulong;
        let size = ACPI_RSDT_ENTRY_SIZE;
        if root_table == 0 { return 0; }
        header = root_table as *mut acpi_table_header;
        let len = (*header).length;
        if len < core::mem::size_of::<acpi_table_header>() as u32 + size { return 0; }
        let mut num_entries = (len - core::mem::size_of::<acpi_table_header>() as u32) / size;
        let mut entry = (root_table + core::mem::size_of::<acpi_table_header>() as c_ulong) as *mut u8;
        while num_entries != 0 { acpi_table = *(entry as *const u32) as c_ulong; if acpi_table != 0 { header = acpi_table as *mut acpi_table_header; if ACPI_COMPARE_NAMESEG((*header).signature, ACPI_SIG_SRAT) { return acpi_table; } } entry = entry.add(size as usize); num_entries -= 1; }
    }
    0
}

#[cfg(any())]
#[no_mangle]
pub unsafe fn count_immovable_mem_regions() -> i32 {
    let mut arg = [0i8; MAX_ACPI_ARG_LENGTH];
    if cmdline_find_option(b"acpi\0".as_ptr() as *const c_char, arg.as_mut_ptr(), arg.len()) == 3
        && strncmp(arg.as_ptr(), b"off\0".as_ptr() as *const c_char, 3) == 0
    { return 0; }

    let table_addr = get_acpi_srat_table();
    if table_addr == 0 { return 0; }
    let table_header = table_addr as *mut acpi_table_header;
    let table_end = table_addr + (*table_header).length as c_ulong;
    let mut table = table_addr + core::mem::size_of::<acpi_table_srat>() as c_ulong;
    let mut num: i32 = 0;
    while table + core::mem::size_of::<acpi_subtable_header>() as c_ulong < table_end {
        let sub_table = table as *mut acpi_subtable_header;
        if (*sub_table).length == 0 {
            debug_putstr(b"Invalid zero length SRAT subtable.\0".as_ptr() as *const c_char);
            return 0;
        }
        if (*sub_table).type_ == ACPI_SRAT_TYPE_MEMORY_AFFINITY {
            let ma = sub_table as *mut acpi_srat_mem_affinity;
            if (*ma).flags & ACPI_SRAT_MEM_HOT_PLUGGABLE == 0 && (*ma).length != 0 {
                immovable_mem[num as usize].start = (*ma).base_address;
                immovable_mem[num as usize].size = (*ma).length;
                num += 1;
            }
            if num >= (MAX_NUMNODES * 2) as i32 {
                debug_putstr(b"Too many immovable memory regions, aborting.\0".as_ptr() as *const c_char);
                return 0;
            }
        }
        table += (*sub_table).length as c_ulong;
    }
    num
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
