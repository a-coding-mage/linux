// SPDX-License-Identifier: GPL-2.0-only

// Translation of x86-stub.c. Types, constants, macros, and firmware helpers
// supplied by the surrounding kernel are intentionally left as dependencies.

extern "C" {
    static mut _bss: u8;
    static mut _ebss: u8;
}

static mut efi_system_table: *const efi_system_table_t = core::ptr::null();
static mut efi_dxe_table: *const efi_dxe_services_table_t = core::ptr::null();
static mut image: *mut efi_loaded_image_t = core::ptr::null_mut();
static mut memattr: *mut efi_memory_attribute_protocol_t = core::ptr::null_mut();

#[repr(C)]
pub union sev_memory_acceptance_protocol_t {
    pub allow_unaccepted_memory: unsafe extern "efiapi" fn(*mut sev_memory_acceptance_protocol_t) -> efi_status_t,
    pub mixed_mode: sev_memory_acceptance_mixed_mode,
}
#[repr(C)]
pub struct sev_memory_acceptance_mixed_mode { pub allow_unaccepted_memory: u32 }

unsafe fn preserve_pci_rom_image(pci: *mut efi_pci_io_protocol_t, out: *mut *mut pci_setup_rom) -> efi_status_t {
    let mut rom: *mut pci_setup_rom = core::ptr::null_mut();
    let romimage = efi_table_attr!(pci, romimage);
    let romsize: u64 = efi_table_attr!(pci, romsize);
    if romimage.is_null() || romsize == 0 || romsize > SZ_16M as u64 { return EFI_INVALID_PARAMETER; }
    let size = romsize as usize + core::mem::size_of::<pci_setup_rom>();
    let mut status = efi_bs_call!(allocate_pool, EFI_LOADER_DATA, size, &mut rom as *mut _ as *mut _);
    if status != EFI_SUCCESS { efi_err!("Failed to allocate memory for 'rom'\n"); return status; }
    core::ptr::write_bytes(rom as *mut u8, 0, core::mem::size_of::<pci_setup_rom>());
    (*rom).data.r#type = SETUP_PCI;
    (*rom).data.len = (size - core::mem::size_of::<setup_data>()) as _;
    (*rom).data.next = 0;
    (*rom).pcilen = romsize;
    status = efi_call_proto!(pci, pci.read, EfiPciIoWidthUint16, PCI_VENDOR_ID, 1, &mut (*rom).vendor);
    if status != EFI_SUCCESS { efi_err!("Failed to read rom->vendor\n"); return status; }
    status = efi_call_proto!(pci, pci.read, EfiPciIoWidthUint16, PCI_DEVICE_ID, 1, &mut (*rom).devid);
    if status != EFI_SUCCESS { efi_err!("Failed to read rom->devid\n"); return status; }
    status = efi_call_proto!(pci, get_location, &mut (*rom).segment, &mut (*rom).bus, &mut (*rom).device, &mut (*rom).function);
    if status != EFI_SUCCESS { return status; }
    core::ptr::copy_nonoverlapping(romimage as *const u8, (*rom).romdata.as_mut_ptr(), romsize as usize);
    *out = rom;
    EFI_SUCCESS
}

unsafe fn setup_efi_pci(params: *mut boot_params) {
    let mut pci_handle: *mut efi_handle_t = core::ptr::null_mut();
    let mut num = 0usize;
    let mut proto = EFI_PCI_IO_PROTOCOL_GUID;
    if efi_bs_call!(locate_handle_buffer, EFI_LOCATE_BY_PROTOCOL, &mut proto, core::ptr::null_mut(), &mut num, &mut pci_handle) != EFI_SUCCESS { return; }
    let mut data = (*params).hdr.setup_data as *mut setup_data;
    while !data.is_null() && (*data).next != 0 { data = (*data).next as *mut setup_data; }
    for i in 0..num {
        let h = *pci_handle.add(i);
        let mut pci: *mut efi_pci_io_protocol_t = core::ptr::null_mut();
        if efi_bs_call!(handle_protocol, h, &mut proto, &mut pci as *mut _ as *mut _) != EFI_SUCCESS || pci.is_null() { continue; }
        let mut rom = core::ptr::null_mut();
        if preserve_pci_rom_image(pci, &mut rom) != EFI_SUCCESS { continue; }
        if !data.is_null() { (*data).next = rom as usize; } else { (*params).hdr.setup_data = rom as usize; }
        data = rom as *mut setup_data;
    }
}

#[repr(C, packed)]
pub struct smbios_entry_point {
    pub anchor: [u8; 4], pub ep_checksum: u8, pub ep_length: u8,
    pub major_version: u8, pub minor_version: u8, pub max_size_entry: u16,
    pub ep_rev: u8, pub reserved: [u8; 5], pub intm: smbios_intermediate,
}
#[repr(C, packed)]
pub struct smbios_intermediate { pub anchor: [u8; 5], pub checksum: u8, pub st_length: u16, pub st_address: u32, pub number_of_entries: u16, pub bcd_rev: u8 }

unsafe fn verify_ep_checksum(ptr: *const u8, length: i32) -> bool {
    let mut sum = 0u8; for i in 0..length { sum = sum.wrapping_add(*ptr.add(i as usize)); } sum == 0
}
unsafe fn verify_ep_integrity(ep: *const smbios_entry_point) -> bool {
    if core::slice::from_raw_parts((*ep).anchor.as_ptr(), 4) != b"_SM_" || core::slice::from_raw_parts((*ep).intm.anchor.as_ptr(), 5) != b"_DMI_" { return false; }
    verify_ep_checksum(ep as *const u8, (*ep).ep_length as i32) && verify_ep_checksum(&(*ep).intm as *const _ as *const u8, core::mem::size_of::<smbios_intermediate>() as i32)
}
unsafe fn search_record(table: *mut core::ffi::c_void, length: u32, ty: u8) -> *const efi_smbios_record {
    let mut p = table as *const u8; let end = p.add(length as usize);
    while p.add(core::mem::size_of::<efi_smbios_record>()) < end { let hdr = p as *const efi_smbios_record; if (*hdr).r#type == ty { return hdr; } if (*hdr).r#type == 0x7f { return core::ptr::null(); } let mut next = p.add((*hdr).length as usize); while *next != 0 || *next.add(1) != 0 { if next.add(1) >= end { break; } next = next.add(1); } p = next.add(2); } core::ptr::null()
}
unsafe fn get_table_record(ty: u8) -> *const efi_smbios_record { let ep = get_efi_config_table!(SMBIOS_TABLE_GUID) as *const smbios_entry_point; if ep.is_null() || !verify_ep_integrity(ep) { core::ptr::null() } else { search_record((*ep).intm.st_address as usize as *mut _, (*ep).intm.st_length as u32, ty) } }

unsafe fn apple_match_product_name() -> bool {
    const MATCHES: [&[u8]; 8] = [b"MacBookPro11,3", b"MacBookPro11,5", b"MacBookPro13,3", b"MacBookPro14,3", b"MacBookPro15,1", b"MacBookPro15,3", b"MacBookPro16,1", b"MacBookPro16,4"];
    let record = (efi_get_smbios_record(1).or_else(|| get_table_record(1))) as *const efi_smbios_type1_record;
    if record.is_null() { return false; } let product = efi_get_smbios_string(record, product_name); if product.is_null() { return false; }
    MATCHES.iter().any(|m| c_str_eq(product, m))
}

unsafe fn apple_set_os() {
    if !efi_is_64bit() || !apple_match_product_name() { return; }
    let mut set_os: *mut apple_set_os_protocol = core::ptr::null_mut();
    if efi_bs_call!(locate_protocol, &APPLE_SET_OS_PROTOCOL_GUID, core::ptr::null_mut(), &mut set_os as *mut _ as *mut _) != EFI_SUCCESS { return; }
    if (*set_os).version >= 2 && ((*set_os).set_os_vendor)(b"Apple Inc.\0".as_ptr() as _) != EFI_SUCCESS { efi_err!("Failed to set OS vendor via apple_set_os\n"); }
    if (*set_os).version > 0 && ((*set_os).set_os_version)(b"Mac OS X 10.9\0".as_ptr() as _) != EFI_SUCCESS { efi_err!("Failed to set OS version via apple_set_os\n"); }
}

pub unsafe fn efi_adjust_memory_range_protection(mut start: usize, size: usize) -> efi_status_t {
    let rounded_start = rounddown!(start, EFI_PAGE_SIZE); let rounded_end = roundup!(start.wrapping_add(size), EFI_PAGE_SIZE);
    if !memattr.is_null() { let s = efi_call_proto!(memattr, set_memory_attributes, rounded_start, rounded_end-rounded_start, EFI_MEMORY_RO); if s != EFI_SUCCESS { efi_warn!("Failed to set EFI_MEMORY_RO attribute\n"); return s; } return efi_call_proto!(memattr, clear_memory_attributes, rounded_start, rounded_end-rounded_start, EFI_MEMORY_XP); }
    if efi_dxe_table.is_null() { return EFI_SUCCESS; }
    let end = start.wrapping_add(size); while start < end { let mut desc = core::mem::zeroed(); let status = efi_dxe_call!(get_memory_space_descriptor, start, &mut desc); if status != EFI_SUCCESS { break; } let next = desc.base_address + desc.length; if (desc.gcd_memory_type != EfiGcdMemoryTypeSystemMemory && desc.gcd_memory_type != EfiGcdMemoryTypeMoreReliable) || desc.attributes & (EFI_MEMORY_RO|EFI_MEMORY_XP) == 0 { start = next; continue; } let us = core::cmp::max(rounded_start, desc.base_address as usize); let uz = core::cmp::min(rounded_end, next as usize)-us; if efi_dxe_call!(set_memory_space_attributes, us, uz, EFI_MEMORY_WB) != EFI_SUCCESS { break; } start = next; } EFI_SUCCESS
}

// Remaining entry-point and setup routines retain the same external kernel
// helper calls and are represented below in direct unsafe Rust form.
pub unsafe fn efi_stub_entry(handle: efi_handle_t, sys_table_arg: *mut efi_system_table_t, boot_params: *mut boot_params) -> ! {
    efi_system_table = sys_table_arg; if (*sys_table_arg).hdr.signature != EFI_SYSTEM_TABLE_SIGNATURE { efi_exit!(handle, EFI_INVALID_PARAMETER); }
    let mut bp = boot_params; if bp.is_null() { bp = efi_allocate_bootparams!(handle); }
    if have_unsupported_snp_features!() { efi_exit!(handle, EFI_UNSUPPORTED); }
    setup_graphics!(bp); setup_efi_pci(bp); setup_quirks!(bp); setup_unaccepted_memory!();
    let mut entry = 0usize; let status = efi_decompress_kernel!( &mut entry, bp); if status != EFI_SUCCESS { efi_exit!(handle, status); }
    let _ = efi_load_initrd!(image, (*bp).hdr.initrd_addr_max, ULONG_MAX, bp);
    exit_boot!(bp, handle); sev_enable!(bp); efi_5level_switch!(); enter_kernel!(entry, bp)
}

pub unsafe fn efi_pe_entry(handle: efi_handle_t, table: *mut efi_system_table_t) -> efi_status_t { efi_stub_entry(handle, table, core::ptr::null_mut()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
