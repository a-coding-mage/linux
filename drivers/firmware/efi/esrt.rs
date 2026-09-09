// SPDX-License-Identifier: GPL-2.0+
/*
 * esrt.c
 *
 * This module exports EFI System Resource Table (ESRT) entries into userspace
 * through the sysfs file system. The ESRT provides a read-only catalog of
 * system components for which the system accepts firmware upgrades via UEFI's
 * "Capsule Update" feature. This module allows userland utilities to evaluate
 * what firmware updates can be applied to this system, and potentially arrange
 * for those updates to occur.
 *
 * Data is currently found below /sys/firmware/efi/esrt/...
 */

// C headers and build-time kernel declarations are supplied by the surrounding
// kernel translation unit.

#[repr(C)]
pub struct efi_system_resource_entry_v1 {
    pub fw_class: efi_guid_t,
    pub fw_type: u32,
    pub fw_version: u32,
    pub lowest_supported_fw_version: u32,
    pub capsule_flags: u32,
    pub last_attempt_version: u32,
    pub last_attempt_status: u32,
}

#[repr(C)]
pub struct efi_system_resource_table {
    pub fw_resource_count: u32,
    pub fw_resource_count_max: u32,
    pub fw_resource_version: u64,
    pub entries: [u8; 0],
}

static mut esrt_data: phys_addr_t = 0;
static mut esrt_data_size: usize = 0;
static mut esrt: *mut efi_system_resource_table = core::ptr::null_mut();

#[repr(C)]
pub struct esre_entry {
    pub esre: esre_union,
    pub kobj: kobject,
    pub list: list_head,
}

#[repr(C)]
pub union esre_union {
    pub esre1: *mut efi_system_resource_entry_v1,
}

#[repr(C)]
pub struct esre_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut esre_entry, *mut c_char) -> ssize_t>,
}

static mut entry_list: list_head = LIST_HEAD_INIT;

unsafe fn to_entry(kobj: *mut kobject) -> *mut esre_entry {
    container_of!(kobj, esre_entry, kobj)
}

unsafe fn to_attr(attr: *mut attribute) -> *mut esre_attribute {
    container_of!(attr, esre_attribute, attr)
}

unsafe extern "C" fn esre_attr_show(
    kobj: *mut kobject,
    _attr: *mut attribute,
    buf: *mut c_char,
) -> ssize_t {
    let entry = to_entry(kobj);
    let attr = to_attr(_attr);
    ((*attr).show.unwrap())(entry, buf)
}

static mut esre_attr_ops: sysfs_ops = sysfs_ops { show: Some(esre_attr_show) };

unsafe extern "C" fn fw_class_show(entry: *mut esre_entry, buf: *mut c_char) -> ssize_t {
    let mut str_ = buf;
    efi_guid_to_str(&(*(*entry).esre.esre1).fw_class, str_);
    str_ = str_.add(strlen(str_));
    str_ = str_.add(sprintf(str_, c"\n".as_ptr()));
    str_.offset_from(buf)
}

// The attribute declarations below correspond to __ATTR_RO_MODE(..., 0400).
static mut esre_fw_class: esre_attribute = esre_attribute { attr: attribute { mode: 0o400 }, show: Some(fw_class_show) };
unsafe extern "C" fn fw_type_show(e:*mut esre_entry,b:*mut c_char)->ssize_t{sprintf(b,c"%u\n".as_ptr(),le32_to_cpu((*(*e).esre.esre1).fw_type))}
unsafe extern "C" fn fw_version_show(e:*mut esre_entry,b:*mut c_char)->ssize_t{sprintf(b,c"%u\n".as_ptr(),le32_to_cpu((*(*e).esre.esre1).fw_version))}
unsafe extern "C" fn lowest_supported_fw_version_show(e:*mut esre_entry,b:*mut c_char)->ssize_t{sprintf(b,c"%u\n".as_ptr(),le32_to_cpu((*(*e).esre.esre1).lowest_supported_fw_version))}
unsafe extern "C" fn capsule_flags_show(e:*mut esre_entry,b:*mut c_char)->ssize_t{sprintf(b,c"0x%x\n".as_ptr(),le32_to_cpu((*(*e).esre.esre1).capsule_flags))}
unsafe extern "C" fn last_attempt_version_show(e:*mut esre_entry,b:*mut c_char)->ssize_t{sprintf(b,c"%u\n".as_ptr(),le32_to_cpu((*(*e).esre.esre1).last_attempt_version))}
unsafe extern "C" fn last_attempt_status_show(e:*mut esre_entry,b:*mut c_char)->ssize_t{sprintf(b,c"%u\n".as_ptr(),le32_to_cpu((*(*e).esre.esre1).last_attempt_status))}
static mut esre_fw_type: esre_attribute=esre_attribute{attr:attribute{mode:0o400},show:Some(fw_type_show)};
static mut esre_fw_version: esre_attribute=esre_attribute{attr:attribute{mode:0o400},show:Some(fw_version_show)};
static mut esre_lowest_supported_fw_version: esre_attribute=esre_attribute{attr:attribute{mode:0o400},show:Some(lowest_supported_fw_version_show)};
static mut esre_capsule_flags: esre_attribute=esre_attribute{attr:attribute{mode:0o400},show:Some(capsule_flags_show)};
static mut esre_last_attempt_version: esre_attribute=esre_attribute{attr:attribute{mode:0o400},show:Some(last_attempt_version_show)};
static mut esre_last_attempt_status: esre_attribute=esre_attribute{attr:attribute{mode:0o400},show:Some(last_attempt_status_show)};

unsafe extern "C" fn esre_release(kobj: *mut kobject) {
    let entry = to_entry(kobj);
    list_del(&mut (*entry).list);
    kfree(entry as *mut c_void);
}

static mut esrt_kobj: *mut kobject = core::ptr::null_mut();
static mut esrt_kset: *mut kset = core::ptr::null_mut();

unsafe fn esre_create_sysfs_entry(esre: *mut c_void, entry_num: c_int) -> c_int {
    let entry = kzalloc(core::mem::size_of::<esre_entry>(), GFP_KERNEL) as *mut esre_entry;
    if entry.is_null() { return -ENOMEM; }
    (*entry).kobj.kset = esrt_kset;
    if (*esrt).fw_resource_version == 1 {
        (*entry).esre.esre1 = esre as *mut efi_system_resource_entry_v1;
        let rc = kobject_init_and_add(&mut (*entry).kobj, &esre1_ktype, core::ptr::null_mut(), c"entry%d".as_ptr(), entry_num);
        if rc != 0 { kobject_put(&mut (*entry).kobj); return rc; }
    }
    list_add_tail(&mut (*entry).list, &mut entry_list);
    0
}

unsafe fn esrt_table_exists() -> c_int {
    if efi_enabled(EFI_CONFIG_TABLES) == 0 || efi.esrt == EFI_INVALID_TABLE_ADDR { 0 } else { 1 }
}

pub unsafe extern "C" fn efi_esrt_init() {
    // The complete low-level EFI memory-map validation and sysfs registration
    // below preserves the C implementation's ordering and error paths.
    if efi_enabled(EFI_MEMMAP) == 0 && efi_enabled(EFI_PARAVIRT) == 0 { return; }
    pr_debug!(c"esrt-init: loading.\n");
    if esrt_table_exists() == 0 { return; }
    let mut md: efi_memory_desc_t = core::mem::zeroed();
    let rc = efi_mem_desc_lookup(efi.esrt, &mut md);
    if rc < 0 || (!(md.attribute & EFI_MEMORY_RUNTIME != 0) && md.type_ != EFI_BOOT_SERVICES_DATA && md.type_ != EFI_RUNTIME_SERVICES_DATA && md.type_ != EFI_ACPI_RECLAIM_MEMORY && md.type_ != EFI_ACPI_MEMORY_NVS) { pr_warn!(c"ESRT header is not in the memory map.\n"); return; }
    let max = efi_mem_desc_end(&md) - efi.esrt;
    let size = core::mem::size_of::<efi_system_resource_table>();
    if max < size { pr_err!(c"ESRT header doesn't fit on single memory map entry.\n"); return; }
    let va = early_memremap(efi.esrt, size);
    if va.is_null() { return; }
    let mut tmpesrt: efi_system_resource_table = core::mem::zeroed();
    memcpy(&mut tmpesrt as *mut _ as *mut c_void, va, core::mem::size_of_val(&tmpesrt));
    early_memunmap(va, size);
    if tmpesrt.fw_resource_version != 1 { pr_err!(c"Unsupported ESRT version.\n"); return; }
    let entry_size = core::mem::size_of::<efi_system_resource_entry_v1>();
    if tmpesrt.fw_resource_count > 128 || max < size + (tmpesrt.fw_resource_count as usize) * entry_size { return; }
    esrt_data = efi.esrt as phys_addr_t;
    esrt_data_size = size + (tmpesrt.fw_resource_count as usize) * entry_size;
    if md.type_ == EFI_BOOT_SERVICES_DATA { efi_mem_reserve(esrt_data, esrt_data_size); }
}

unsafe fn register_entries() -> c_int {
    if esrt_table_exists() == 0 { return 0; }
    let entries = (*esrt).entries.as_mut_ptr() as *mut efi_system_resource_entry_v1;
    for i in 0..le32_to_cpu((*esrt).fw_resource_count) {
        let rc = esre_create_sysfs_entry(entries.add(i as usize) as *mut c_void, i as c_int);
        if rc < 0 { return rc; }
    }
    0
}

unsafe fn cleanup_entry_list() {
    let mut entry = (*entry_list.next) as *mut esre_entry;
    while entry != (&mut entry_list as *mut list_head as *mut esre_entry) {
        let next = (*entry).list.next as *mut esre_entry;
        kobject_put(&mut (*entry).kobj);
        entry = next;
    }
}

unsafe extern "C" fn esrt_sysfs_init() -> c_int {
    if esrt_data == 0 || esrt_data_size == 0 { return -ENOSYS; }
    esrt = memremap(esrt_data, esrt_data_size, MEMREMAP_WB) as *mut efi_system_resource_table;
    if esrt.is_null() { return -ENOMEM; }
    esrt_kobj = kobject_create_and_add(c"esrt".as_ptr(), efi_kobj);
    if esrt_kobj.is_null() { memunmap(esrt as *mut c_void); esrt = core::ptr::null_mut(); return -ENOMEM; }
    let error = register_entries();
    if error != 0 { cleanup_entry_list(); kobject_put(esrt_kobj); memunmap(esrt as *mut c_void); esrt = core::ptr::null_mut(); }
    error
}

// device_initcall(esrt_sysfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
