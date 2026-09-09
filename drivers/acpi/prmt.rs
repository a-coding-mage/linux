// SPDX-License-Identifier: GPL-2.0-only
/*
 * Author: Erik Kaneda <erik.kaneda@intel.com>
 * Copyright 2020 Intel Corporation
 *
 * prmt.c
 *
 * Each PRM service is an executable that is run in a restricted environment
 * that is invoked by writing to the PlatformRtMechanism OperationRegion from
 * AML bytecode.
 *
 * init_prmt initializes the Platform Runtime Mechanism (PRM) services by
 * processing data in the PRMT as well as registering an ACPI OperationRegion
 * handler for the PlatformRtMechanism subtype.
 */

// External kernel/ACPI/EFI declarations and macros are supplied by dependent files.

#[repr(C, packed)]
struct PrmMmioAddrRange {
    phys_addr: u64,
    virt_addr: u64,
    length: u32,
}

#[repr(C, packed)]
struct PrmMmioInfo {
    mmio_count: u64,
    addr_ranges: [PrmMmioAddrRange; 0],
}

#[repr(C, packed)]
struct PrmBuffer {
    prm_status: u8,
    efi_status: u64,
    prm_cmd: u8,
    handler_guid: Guid,
}

#[repr(C, packed)]
struct PrmContextBuffer {
    signature: [libc::c_char; ACPI_NAMESEG_SIZE],
    revision: u16,
    reserved: u16,
    identifier: Guid,
    static_data_buffer: u64,
    mmio_ranges: *mut PrmMmioInfo,
}

static mut PRM_MODULE_LIST: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

#[repr(C)]
struct PrmHandlerInfo {
    guid: EfiGuid,
    handler_addr: Option<unsafe extern "efiapi" fn(u64, *mut core::ffi::c_void) -> EfiStatus>,
    static_data_buffer_addr: u64,
    acpi_param_buffer_addr: u64,
    handler_list: ListHead,
}

#[repr(C)]
struct PrmModuleInfo {
    guid: Guid,
    major_rev: u16,
    minor_rev: u16,
    handler_count: u16,
    mmio_info: *mut PrmMmioInfo,
    updatable: bool,
    module_list: ListHead,
    handlers: [PrmHandlerInfo; 0],
}

unsafe fn efi_pa_va_lookup(_guid: *mut EfiGuid, pa: u64) -> u64 {
    let pa_offset = pa & !PAGE_MASK;
    let page = pa & PAGE_MASK;
    let mut md: *mut EfiMemoryDesc;

    for_each_efi_memory_desc!(md) {
        if ((*md).attribute & EFI_MEMORY_RUNTIME) != 0
            && (*md).phys_addr < pa
            && pa < (*md).phys_addr + PAGE_SIZE * (*md).num_pages
        {
            return pa_offset + (*md).virt_addr + page - (*md).phys_addr;
        }
    }
    0
}

unsafe fn acpi_parse_prmt(header: *mut AcpiSubtableHeaders, _end: usize) -> i32 {
    let module_info = header as *mut AcpiPrmtModuleInfo;
    let module_info_size = core::mem::size_of::<PrmModuleInfo>()
        + (*module_info).handler_info_count as usize * core::mem::size_of::<PrmHandlerInfo>();
    let tm = kmalloc(module_info_size, GFP_KERNEL) as *mut PrmModuleInfo;
    if tm.is_null() { return -ENOMEM; }

    guid_copy(&mut (*tm).guid, (*module_info).module_guid as *mut Guid);
    (*tm).major_rev = (*module_info).major_rev;
    (*tm).minor_rev = (*module_info).minor_rev;
    (*tm).handler_count = (*module_info).handler_info_count;
    (*tm).updatable = true;

    let mmio_count: *mut u64;
    if (*module_info).mmio_list_pointer != 0 {
        mmio_count = memremap((*module_info).mmio_list_pointer, 8, MEMREMAP_WB) as *mut u64;
        if mmio_count.is_null() { kfree(tm as *mut core::ffi::c_void); return -ENOMEM; }
        let mmio_range_size = core::mem::size_of::<PrmMmioInfo>()
            + (*mmio_count as usize) * core::mem::size_of::<PrmMmioAddrRange>();
        (*tm).mmio_info = kmalloc(mmio_range_size, GFP_KERNEL) as *mut PrmMmioInfo;
        if (*tm).mmio_info.is_null() { memunmap(mmio_count as *mut core::ffi::c_void); kfree(tm as *mut core::ffi::c_void); return -ENOMEM; }
        let temp_mmio = memremap((*module_info).mmio_list_pointer, mmio_range_size, MEMREMAP_WB);
        if temp_mmio.is_null() { kfree((*tm).mmio_info as *mut core::ffi::c_void); memunmap(mmio_count as *mut core::ffi::c_void); kfree(tm as *mut core::ffi::c_void); return -ENOMEM; }
        memmove((*tm).mmio_info as *mut core::ffi::c_void, temp_mmio, mmio_range_size);
    } else {
        (*tm).mmio_info = kmalloc(core::mem::size_of::<PrmMmioInfo>(), GFP_KERNEL) as *mut PrmMmioInfo;
        if (*tm).mmio_info.is_null() { kfree(tm as *mut core::ffi::c_void); return -ENOMEM; }
        (*(*tm).mmio_info).mmio_count = 0;
    }

    INIT_LIST_HEAD!(&mut (*tm).module_list);
    list_add!(&mut (*tm).module_list, &mut PRM_MODULE_LIST);
    let mut handler_info = get_first_handler!(module_info);
    let mut cur_handler: u64 = 0;
    loop {
        let th = &mut (*tm).handlers.as_mut_ptr().add(cur_handler as usize);
        guid_copy(&mut (*th).guid, (*handler_info).handler_guid as *mut Guid);
        if (*handler_info).handler_address == 0 { continue; }
        (*th).handler_addr = core::mem::transmute(efi_pa_va_lookup(&mut (*th).guid, (*handler_info).handler_address));
        if (*th).handler_addr.is_none() { continue; }
        (*th).static_data_buffer_addr = efi_pa_va_lookup(&mut (*th).guid, (*handler_info).static_data_buffer_address);
        (*th).acpi_param_buffer_addr = efi_pa_va_lookup(&mut (*th).guid, (*handler_info).acpi_param_buffer_address);
        cur_handler += 1;
        if cur_handler >= (*tm).handler_count as u64 { break; }
        handler_info = get_next_handler!(handler_info);
    }
    0
}

const GET_MODULE: u8 = 0;
const GET_HANDLER: u8 = 1;

unsafe fn find_guid_info(guid: *const Guid, mode: u8) -> *mut core::ffi::c_void {
    let mut cur_module: *mut PrmModuleInfo;
    list_for_each_entry!(cur_module, PRM_MODULE_LIST, module_list) {
        for i in 0..(*cur_module).handler_count as usize {
            let cur_handler = (*cur_module).handlers.as_mut_ptr().add(i);
            if guid_equal(guid, &(*cur_handler).guid) {
                return if mode == GET_MODULE { cur_module as *mut _ } else { cur_handler as *mut _ };
            }
        }
    }
    core::ptr::null_mut()
}

unsafe fn find_prm_module(guid: *const Guid) -> *mut PrmModuleInfo { find_guid_info(guid, GET_MODULE) as *mut PrmModuleInfo }
unsafe fn find_prm_handler(guid: *const Guid) -> *mut PrmHandlerInfo { find_guid_info(guid, GET_HANDLER) as *mut PrmHandlerInfo }

pub unsafe fn acpi_prm_handler_available(guid: *const Guid) -> bool {
    !find_prm_handler(guid).is_null() && !find_prm_module(guid).is_null()
}

const PRM_CMD_RUN_SERVICE: u8 = 0;
const PRM_CMD_START_TRANSACTION: u8 = 1;
const PRM_CMD_END_TRANSACTION: u8 = 2;
const PRM_HANDLER_SUCCESS: u8 = 0;
const PRM_HANDLER_ERROR: u8 = 1;
const INVALID_PRM_COMMAND: u8 = 2;
const PRM_HANDLER_GUID_NOT_FOUND: u8 = 3;
const UPDATE_LOCK_ALREADY_HELD: u8 = 4;
const UPDATE_UNLOCK_WITHOUT_LOCK: u8 = 5;

pub unsafe fn acpi_call_prm_handler(handler_guid: Guid, param_buffer: *mut core::ffi::c_void) -> i32 {
    let handler = find_prm_handler(&handler_guid);
    let module = find_prm_module(&handler_guid);
    if handler.is_null() || module.is_null() { return -ENODEV; }
    let mut context: PrmContextBuffer = core::mem::zeroed();
    acpi_copy_nameseg!(context.signature, "PRMC");
    context.identifier = (*handler).guid;
    context.static_data_buffer = (*handler).static_data_buffer_addr;
    context.mmio_ranges = (*module).mmio_info;
    efi_status_to_err(efi_call_acpi_prm_handler!((*handler).handler_addr, param_buffer as u64, &mut context))
}

unsafe fn acpi_platformrt_space_handler(_function: u32, _addr: AcpiPhysicalAddress, _bits: u32, value: *mut AcpiInteger, _handler_context: *mut core::ffi::c_void, _region_context: *mut core::ffi::c_void) -> AcpiStatus {
    let buffer = value as *mut PrmBuffer;
    if !efi_enabled!(EFI_RUNTIME_SERVICES) { return AE_NO_HANDLER; }
    match (*buffer).prm_cmd {
        PRM_CMD_RUN_SERVICE => {
            let handler = find_prm_handler(&(*buffer).handler_guid);
            let module = find_prm_module(&(*buffer).handler_guid);
            if handler.is_null() || module.is_null() { (*buffer).prm_status = PRM_HANDLER_GUID_NOT_FOUND; return AE_OK; }
            if (*handler).handler_addr.is_none() { (*buffer).prm_status = PRM_HANDLER_ERROR; return AE_OK; }
            let mut context: PrmContextBuffer = core::mem::zeroed();
            acpi_copy_nameseg!(context.signature, "PRMC");
            context.identifier = (*handler).guid;
            context.static_data_buffer = (*handler).static_data_buffer_addr;
            context.mmio_ranges = (*module).mmio_info;
            let status = efi_call_acpi_prm_handler!((*handler).handler_addr, (*handler).acpi_param_buffer_addr, &mut context);
            if status == EFI_SUCCESS { (*buffer).prm_status = PRM_HANDLER_SUCCESS; } else { (*buffer).prm_status = PRM_HANDLER_ERROR; (*buffer).efi_status = status; }
        }
        PRM_CMD_START_TRANSACTION => {
            let module = find_prm_module(&(*buffer).handler_guid);
            if module.is_null() { (*buffer).prm_status = PRM_HANDLER_GUID_NOT_FOUND; return AE_OK; }
            if (*module).updatable { (*module).updatable = false; } else { (*buffer).prm_status = UPDATE_LOCK_ALREADY_HELD; }
        }
        PRM_CMD_END_TRANSACTION => {
            let module = find_prm_module(&(*buffer).handler_guid);
            if module.is_null() { (*buffer).prm_status = PRM_HANDLER_GUID_NOT_FOUND; return AE_OK; }
            if (*module).updatable { (*buffer).prm_status = UPDATE_UNLOCK_WITHOUT_LOCK; } else { (*module).updatable = true; }
        }
        _ => (*buffer).prm_status = INVALID_PRM_COMMAND,
    }
    AE_OK
}

pub unsafe fn init_prmt() {
    let mut tbl: *mut AcpiTableHeader = core::ptr::null_mut();
    let status = acpi_get_table!(ACPI_SIG_PRMT, 0, &mut tbl);
    if ACPI_FAILURE!(status) { return; }
    let mc = acpi_table_parse_entries!(ACPI_SIG_PRMT, core::mem::size_of::<AcpiTablePrmt>() + core::mem::size_of::<AcpiTablePrmtHeader>(), 0, acpi_parse_prmt, 0);
    acpi_put_table!(tbl);
    if mc <= 0 { return; }
    if !efi_enabled!(EFI_RUNTIME_SERVICES) { return; }
    let status = acpi_install_address_space_handler!(ACPI_ROOT_OBJECT, ACPI_ADR_SPACE_PLATFORM_RT, acpi_platformrt_space_handler, core::ptr::null_mut(), core::ptr::null_mut());
    if ACPI_FAILURE!(status) { }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
