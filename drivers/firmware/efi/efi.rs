// SPDX-License-Identifier: GPL-2.0-only
/*
 * efi.c - EFI subsystem
 *
 * Copyright (C) 2001,2003,2004 Dell <Matt_Domsch@dell.com>
 * Copyright (C) 2004 Intel Corporation <matthew.e.tolentino@intel.com>
 * Copyright (C) 2013 Tom Gundersen <teg@jklm.no>
 *
 * This code registers /sys/firmware/efi{,/efivars} when EFI is supported,
 * allowing the efivarfs to be mounted or the efivars module to be loaded.
 * The existance of /sys/firmware/efi may also be used by userspace to
 * determine that the system supports EFI.
 */

// Linux kernel headers and configuration-dependent definitions are supplied
// by the surrounding translation unit.

pub static mut efi: struct_efi = struct_efi {
    runtime_supported_mask: EFI_RT_SUPPORTED_ALL,
    acpi: EFI_INVALID_TABLE_ADDR,
    acpi20: EFI_INVALID_TABLE_ADDR,
    smbios: EFI_INVALID_TABLE_ADDR,
    smbios3: EFI_INVALID_TABLE_ADDR,
    esrt: EFI_INVALID_TABLE_ADDR,
    tpm_log: EFI_INVALID_TABLE_ADDR,
    tpm_final_log: EFI_INVALID_TABLE_ADDR,
    ovmf_debug_log: EFI_INVALID_TABLE_ADDR,
};

pub static mut efi_rng_seed: c_ulong = EFI_INVALID_TABLE_ADDR;
static mut mem_reserve: c_ulong = EFI_INVALID_TABLE_ADDR;
static mut rt_prop: c_ulong = EFI_INVALID_TABLE_ADDR;
static mut initrd: c_ulong = EFI_INVALID_TABLE_ADDR;

extern "C" {
    static mut primary_display_table: c_ulong;
}

pub static mut efi_mm: mm_struct = mm_struct {
    mm_mt: MTREE_INIT_EXT!(mm_mt, MM_MT_FLAGS, efi_mm.mmap_lock),
    mm_users: ATOMIC_INIT!(2),
    mm_count: ATOMIC_INIT!(1),
    write_protect_seq: SEQCNT_ZERO!(efi_mm.write_protect_seq),
    mmap_lock: MMAP_LOCK_INITIALIZER!(efi_mm),
    page_table_lock: __SPIN_LOCK_UNLOCKED!(efi_mm.page_table_lock),
    mmlist: LIST_HEAD_INIT!(efi_mm.mmlist),
    flexible_array: MM_STRUCT_FLEXIBLE_ARRAY_INIT,
};

pub static mut efi_rts_wq: *mut workqueue_struct = core::ptr::null_mut();
static mut disable_runtime: bool = IS_ENABLED!(CONFIG_EFI_DISABLE_RUNTIME);

unsafe extern "C" fn setup_noefi(_arg: *mut c_char) -> c_int {
    disable_runtime = true;
    0
}
early_param!("noefi", setup_noefi);

pub unsafe fn efi_runtime_disabled() -> bool { disable_runtime }

pub unsafe fn __efi_soft_reserve_enabled() -> bool {
    !efi_enabled(EFI_MEM_NO_SOFT_RESERVE)
}

unsafe extern "C" fn parse_efi_cmdline(str_: *mut c_char) -> c_int {
    if str_.is_null() { pr_warn!("need at least one option\n"); return -EINVAL; }
    if parse_option_str(str_, c_str!("debug")) { set_bit(EFI_DBG, &mut efi.flags); }
    if parse_option_str(str_, c_str!("noruntime")) { disable_runtime = true; }
    if parse_option_str(str_, c_str!("runtime")) { disable_runtime = false; }
    if parse_option_str(str_, c_str!("nosoftreserve")) { set_bit(EFI_MEM_NO_SOFT_RESERVE, &mut efi.flags); }
    0
}
early_param!("efi", parse_efi_cmdline);

pub static mut efi_kobj: *mut kobject = core::ptr::null_mut();

static unsafe fn systab_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    if kobj.is_null() || buf.is_null() { return -EINVAL as ssize_t; }
    let mut str_ = buf;
    if efi.acpi20 != EFI_INVALID_TABLE_ADDR { str_ = str_.add(sprintf!(str_, "ACPI20=0x%lx\n", efi.acpi20)); }
    if efi.acpi != EFI_INVALID_TABLE_ADDR { str_ = str_.add(sprintf!(str_, "ACPI=0x%lx\n", efi.acpi)); }
    if efi.smbios3 != EFI_INVALID_TABLE_ADDR { str_ = str_.add(sprintf!(str_, "SMBIOS3=0x%lx\n", efi.smbios3)); }
    if efi.smbios != EFI_INVALID_TABLE_ADDR { str_ = str_.add(sprintf!(str_, "SMBIOS=0x%lx\n", efi.smbios)); }
    str_.offset_from(buf) as ssize_t
}

static unsafe fn fw_platform_size_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    sprintf!(buf, "%d\n", if efi_enabled(EFI_64BIT) { 64 } else { 32 })
}

extern "C" {
    static mut efi_attr_fw_vendor: kobj_attribute;
    static mut efi_attr_runtime: kobj_attribute;
    static mut efi_attr_config_table: kobj_attribute;
}

pub static mut efivar_ops_nh: blocking_notifier_head = BLOCKING_NOTIFIER_HEAD_INIT!();
static mut generic_efivars: efivars = efivars::default();
static mut generic_ops: efivar_operations = efivar_operations::default();

unsafe fn generic_ops_supported() -> bool {
    let mut name_size = core::mem::size_of::<efi_char16_t>() as c_ulong;
    let mut name = core::mem::MaybeUninit::<efi_char16_t>::uninit();
    let mut guid = core::mem::MaybeUninit::<efi_guid_t>::uninit();
    if efi.get_next_variable.is_none() { return false; }
    let status = efi.get_next_variable.unwrap()(&mut name_size, name.as_mut_ptr(), guid.as_mut_ptr());
    status != EFI_UNSUPPORTED
}

unsafe fn generic_ops_register() -> c_int {
    if !generic_ops_supported() { return 0; }
    generic_ops.get_variable = efi.get_variable;
    generic_ops.get_next_variable = efi.get_next_variable;
    generic_ops.query_variable_store = Some(efi_query_variable_store);
    generic_ops.query_variable_info = efi.query_variable_info;
    if efi_rt_services_supported(EFI_RT_SUPPORTED_SET_VARIABLE) {
        generic_ops.set_variable = efi.set_variable;
        generic_ops.set_variable_nonblocking = efi.set_variable_nonblocking;
    }
    efivars_register(&mut generic_efivars, &mut generic_ops)
}

unsafe fn generic_ops_unregister() {
    if generic_ops.get_variable.is_none() { return; }
    efivars_unregister(&mut generic_efivars);
}

#[no_mangle] pub unsafe extern "C" fn efivars_generic_ops_register() { generic_ops_register(); }
#[no_mangle] pub unsafe extern "C" fn efivars_generic_ops_unregister() { generic_ops_unregister(); }

#[cfg(CONFIG_DEBUG_FS)]
const EFI_DEBUGFS_MAX_BLOBS: usize = 32;

#[cfg(CONFIG_DEBUG_FS)]
static mut debugfs_blob: [debugfs_blob_wrapper; EFI_DEBUGFS_MAX_BLOBS] = [debugfs_blob_wrapper::default(); EFI_DEBUGFS_MAX_BLOBS];

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn efi_debugfs_init() {
    let efi_debugfs = debugfs_create_dir(c_str!("efi"), core::ptr::null_mut());
    if IS_ERR(efi_debugfs) { return; }
    let mut type_count = [0; EFI_BOOT_SERVICES_DATA as usize + 1];
    let mut i = 0;
    for_each_efi_memory_desc!(md) {
        let name = match md.type_ {
            EFI_BOOT_SERVICES_CODE => { let n = format!("boot_services_code{}", type_count[md.type_ as usize]); type_count[md.type_ as usize] += 1; n },
            EFI_BOOT_SERVICES_DATA => { let n = format!("boot_services_data{}", type_count[md.type_ as usize]); type_count[md.type_ as usize] += 1; n },
            _ => continue,
        };
        if i >= EFI_DEBUGFS_MAX_BLOBS { pr_warn!("More then %d EFI boot service segments, only showing first %d in debugfs\n", EFI_DEBUGFS_MAX_BLOBS, EFI_DEBUGFS_MAX_BLOBS); break; }
        debugfs_blob[i].size = md.num_pages << EFI_PAGE_SHIFT;
        debugfs_blob[i].data = memremap(md.phys_addr, debugfs_blob[i].size, MEMREMAP_WB);
        if debugfs_blob[i].data.is_null() { continue; }
        debugfs_create_blob(name.as_ptr(), 0o400, efi_debugfs, &mut debugfs_blob[i]);
        i += 1;
    }
}
#[cfg(not(CONFIG_DEBUG_FS))]
unsafe fn efi_debugfs_init() {}

unsafe fn efipostcore_init() -> c_int {
    if !efi_enabled(EFI_RUNTIME_SERVICES) { efi.runtime_supported_mask = 0; }
    if efi.runtime_supported_mask != 0 {
        efi_rts_wq = alloc_ordered_workqueue(c_str!("efi_runtime"), WQ_SYSFS);
        if efi_rts_wq.is_null() { pr_err!("Creating efi_rts_wq failed, EFI runtime services disabled.\n"); clear_bit(EFI_RUNTIME_SERVICES, &mut efi.flags); efi.runtime_supported_mask = 0; }
    }
    0
}
postcore_initcall!(efipostcore_init);

unsafe fn efisubsys_init() -> c_int {
    let mut error: c_int;
    if !efi_enabled(EFI_BOOT) { return 0; }
    if efi_rt_services_supported(EFI_RT_SUPPORTED_TIME_SERVICES) { platform_device_register_simple(c_str!("rtc-efi"), 0, core::ptr::null_mut(), 0); }
    efi_kobj = kobject_create_and_add(c_str!("efi"), firmware_kobj);
    if efi_kobj.is_null() { pr_err!("efi: Firmware registration failed.\n"); error = -ENOMEM; goto_err_destroy_wq!(); }
    if efi_rt_services_supported(EFI_RT_SUPPORTED_GET_VARIABLE | EFI_RT_SUPPORTED_GET_NEXT_VARIABLE_NAME) {
        error = generic_ops_register(); if error != 0 { goto_err_put!(); }
        error = efivar_ssdt_load(); if error != 0 { pr_err!("efi: failed to load SSDT, error %d.\n", error); }
        platform_device_register_simple(c_str!("efivars"), 0, core::ptr::null_mut(), 0);
    }
    BLOCKING_INIT_NOTIFIER_HEAD!(&mut efivar_ops_nh);
    error = sysfs_create_group(efi_kobj, &efi_subsys_attr_group); if error != 0 { pr_err!("efi: Sysfs attribute export failed with error %d.\n", error); goto_err_unregister!(); }
    error = sysfs_create_mount_point(efi_kobj, c_str!("efivars")); if error != 0 { pr_err!("efivars: Subsystem registration failed.\n"); goto_err_remove_group!(); }
    if efi_enabled(EFI_DBG) && efi_enabled(EFI_PRESERVE_BS_REGIONS) { efi_debugfs_init(); }
    if IS_ENABLED!(CONFIG_OVMF_DEBUG_LOG) && efi.ovmf_debug_log != EFI_INVALID_TABLE_ADDR { ovmf_log_probe(efi.ovmf_debug_log); }
    0
}
subsys_initcall!(efisubsys_init);

pub unsafe fn efi_find_mirror() {
    if !efi_enabled(EFI_MEMMAP) { return; }
    let mut mirror_size: u64 = 0; let mut total_size: u64 = 0;
    for_each_efi_memory_desc!(md) { let start = md.phys_addr; let size = md.num_pages << EFI_PAGE_SHIFT; total_size += size; if md.attribute & EFI_MEMORY_MORE_RELIABLE != 0 { memblock_mark_mirror(start, size); mirror_size += size; } }
    if mirror_size != 0 { pr_info!("Memory: %lldM/%lldM mirrored memory\n", mirror_size >> 20, total_size >> 20); }
}

pub unsafe fn __efi_mem_desc_lookup(phys_addr: u64, out_md: *mut efi_memory_desc_t) -> c_int {
    if !efi_enabled(EFI_MEMMAP) { pr_err_once!("EFI_MEMMAP is not enabled.\n"); return -EINVAL; }
    if out_md.is_null() { pr_err_once!("out_md is null.\n"); return -EINVAL; }
    for_each_efi_memory_desc!(md) {
        if (md.phys_addr & (EFI_PAGE_SIZE - 1)) != 0 || md.num_pages <= 0 || md.num_pages > (U64_MAX - md.phys_addr) >> EFI_PAGE_SHIFT { continue; }
        let size = md.num_pages << EFI_PAGE_SHIFT; let end = md.phys_addr + size;
        if phys_addr >= md.phys_addr && phys_addr < end { core::ptr::copy_nonoverlapping(md, out_md, 1); return 0; }
    }
    -ENOENT
}

pub unsafe fn efi_mem_desc_end(md: *mut efi_memory_desc_t) -> u64 { (*md).phys_addr + ((*md).num_pages << EFI_PAGE_SHIFT) }
pub unsafe fn efi_arch_mem_reserve(_addr: phys_addr_t, _size: u64) {}

pub unsafe fn efi_mem_reserve(addr: phys_addr_t, size: u64) {
    if WARN_ON_ONCE(efi_enabled(EFI_PARAVIRT)) { return; }
    if !memblock_is_region_reserved(addr, size) { memblock_reserve_kern(addr, size); } else { memblock_reserved_mark_kern(addr, size); }
    efi_arch_mem_reserve(addr, size);
}

pub unsafe fn efi_systab_check_header(systab_hdr: *const efi_table_hdr_t) -> c_int {
    if (*systab_hdr).signature != EFI_SYSTEM_TABLE_SIGNATURE { pr_err!("System table signature incorrect!\n"); return -EINVAL; }
    0
}

pub unsafe fn efi_mem_attributes(phys_addr: c_ulong) -> u64 { let mut md = core::mem::MaybeUninit::uninit(); if efi_mem_desc_lookup(phys_addr, md.as_mut_ptr()) != 0 { 0 } else { md.assume_init().attribute } }
pub unsafe fn efi_mem_type(phys_addr: c_ulong) -> c_int { if !efi_enabled(EFI_MEMMAP) && !efi_enabled(EFI_PARAVIRT) { return -ENOTSUPP; } let mut md = core::mem::MaybeUninit::uninit(); if efi_mem_desc_lookup(phys_addr, md.as_mut_ptr()) != 0 { -EINVAL } else { md.assume_init().type_ as c_int } }

pub fn efi_status_to_err(status: efi_status_t) -> c_int {
    match status { EFI_SUCCESS => 0, EFI_INVALID_PARAMETER => -EINVAL, EFI_OUT_OF_RESOURCES => -ENOSPC, EFI_DEVICE_ERROR => -EIO, EFI_WRITE_PROTECTED => -EROFS, EFI_SECURITY_VIOLATION => -EACCES, EFI_NOT_FOUND => -ENOENT, EFI_ABORTED => -EINTR, _ => -EINVAL }
}

static mut efi_memreserve_root: *mut linux_efi_memreserve = core::ptr::null_mut();

unsafe fn efi_memreserve_map_root() -> c_int {
    if mem_reserve == EFI_INVALID_TABLE_ADDR { return -ENODEV; }
    efi_memreserve_root = memremap(mem_reserve, core::mem::size_of::<linux_efi_memreserve>(), MEMREMAP_WB) as *mut _;
    if WARN_ON_ONCE(efi_memreserve_root.is_null()) { return -ENOMEM; } 0
}

unsafe fn efi_mem_reserve_iomem(addr: phys_addr_t, size: u64) -> c_int {
    let res = kzalloc_obj!(resource, GFP_ATOMIC); if res.is_null() { return -ENOMEM; }
    (*res).name = c_str!("reserved"); (*res).flags = IORESOURCE_MEM; (*res).start = addr; (*res).end = addr + size - 1;
    let parent = request_resource_conflict(&mut iomem_resource, res); let ret = if !parent.is_null() { request_resource(parent, res) } else { 0 };
    if IS_ENABLED!(CONFIG_ARCH_KEEP_MEMBLOCK) && ret == 0 { memblock_reserve(addr, size); } ret
}

pub unsafe fn efi_systab_report_header(systab_hdr: *const efi_table_hdr_t, fw_vendor: c_ulong) {
    let mut vendor = [0i8; 100]; vendor[0] = b'u' as i8; vendor[1] = b'n' as i8; vendor[2] = b'k' as i8; vendor[3] = b'n' as i8; vendor[4] = b'o' as i8; vendor[5] = b'w' as i8; vendor[6] = b'n' as i8;
    let c16 = early_memremap_ro(fw_vendor, vendor.len() * core::mem::size_of::<efi_char16_t>()) as *const efi_char16_t;
    if !c16.is_null() { let mut i = 0; while i < vendor.len() - 1 && *c16.add(i) != 0 { vendor[i] = *c16.add(i) as i8; i += 1; } vendor[i] = 0; early_memunmap(c16 as *mut _, vendor.len() * core::mem::size_of::<efi_char16_t>()); }
    let rev = (*systab_hdr).revision as u16; pr_info!("EFI v%u.%u", (*systab_hdr).revision >> 16, rev / 10); let rev = rev % 10; if rev != 0 { pr_cont!(".%u", rev); } pr_cont!(" by %s\n", vendor.as_ptr());
}

// The remaining configuration-table parsing, EFI memory formatting, persistent
// reservation, and reboot-notifier routines retain the same kernel API calls
// and are represented below with their C control-flow equivalents.

pub unsafe fn efi_mem_desc_end_checked(md: *mut efi_memory_desc_t) -> u64 { (*md).phys_addr + ((*md).num_pages << EFI_PAGE_SHIFT) }

pub unsafe fn efi_config_parse_tables(config_tables: *const efi_config_table_t, count: c_int, arch_tables: *const efi_config_table_type_t) -> c_int {
    let tbl64 = config_tables as *const efi_config_table_64_t;
    let tbl32 = config_tables as *const efi_config_table_32_t;
    pr_info!("");
    for i in 0..count as isize {
        let (guid, table) = if !IS_ENABLED!(CONFIG_X86) { (&(*config_tables.offset(i)).guid, (*config_tables.offset(i)).table as c_ulong) }
        else if efi_enabled(EFI_64BIT) { (&(*tbl64.offset(i)).guid, (*tbl64.offset(i)).table as c_ulong) }
        else { (&(*tbl32.offset(i)).guid, (*tbl32.offset(i)).table as c_ulong) };
        if !match_config_table(guid, table, common_tables) && !arch_tables.is_null() { match_config_table(guid, table, arch_tables); }
    }
    pr_cont!("\n"); set_bit(EFI_CONFIG_TABLES, &mut efi.flags);
    if efi_rng_seed != EFI_INVALID_TABLE_ADDR {
        let seed = early_memremap(efi_rng_seed, core::mem::size_of::<linux_efi_random_seed>()) as *mut linux_efi_random_seed;
        if !seed.is_null() { let size = core::cmp::min((*seed).size, SZ_1K as u32); early_memunmap(seed as *mut _, core::mem::size_of::<linux_efi_random_seed>()); if size > 0 { let seed = early_memremap(efi_rng_seed, core::mem::size_of::<linux_efi_random_seed>() + size as usize) as *mut linux_efi_random_seed; if !seed.is_null() { add_bootloader_randomness((*seed).bits.as_ptr() as *const _, size); memzero_explicit((*seed).bits.as_mut_ptr() as *mut _, size as usize); early_memunmap(seed as *mut _, core::mem::size_of::<linux_efi_random_seed>() + size as usize); } } }
        else { pr_err!("Could not map UEFI random seed!\n"); }
    }
    if !IS_ENABLED!(CONFIG_X86_32) && efi_enabled(EFI_MEMMAP) { efi_memattr_init(); }
    efi_tpm_eventlog_init();
    0
}

unsafe fn match_config_table(guid: *const efi_guid_t, table: c_ulong, table_types: *const efi_config_table_type_t) -> bool {
    let mut i = 0;
    while !efi_guidcmp((*table_types.add(i)).guid, NULL_GUID) {
        if !efi_guidcmp(*guid, (*table_types.add(i)).guid) {
            if !efi_config_table_is_usable(guid, table) { return true; }
            *(*table_types.add(i)).ptr = table; return true;
        }
        i += 1;
    }
    false
}

pub unsafe fn efi_mem_reserve_persistent(addr: phys_addr_t, size: u64) -> c_int {
    if efi_memreserve_root == ULONG_MAX as *mut _ { return -ENODEV; }
    if efi_memreserve_root.is_null() { let rc = efi_memreserve_map_root(); if rc != 0 { return rc; } }
    let mut prsv = (*efi_memreserve_root).next;
    while prsv != 0 {
        let rsv = memremap(prsv, core::mem::size_of::<linux_efi_memreserve>(), MEMREMAP_WB) as *mut linux_efi_memreserve;
        if rsv.is_null() { return -ENOMEM; }
        let index = atomic_fetch_add_unless(&mut (*rsv).count, 1, (*rsv).size);
        if index < (*rsv).size { (*rsv).entry[index as usize].base = addr; (*rsv).entry[index as usize].size = size; memunmap(rsv as *mut _); return efi_mem_reserve_iomem(addr, size); }
        prsv = (*rsv).next; memunmap(rsv as *mut _);
    }
    let rsv = __get_free_page(GFP_ATOMIC) as *mut linux_efi_memreserve; if rsv.is_null() { return -ENOMEM; }
    let rc = efi_mem_reserve_iomem(__pa(rsv), SZ_4K); if rc != 0 { free_page(rsv as c_ulong); return rc; }
    (*rsv).size = EFI_MEMRESERVE_COUNT(SZ_4K); atomic_set(&mut (*rsv).count, 1); (*rsv).entry[0].base = addr; (*rsv).entry[0].size = size;
    spin_lock(&mut efi_mem_reserve_persistent_lock); (*rsv).next = (*efi_memreserve_root).next; (*efi_memreserve_root).next = __pa(rsv); spin_unlock(&mut efi_mem_reserve_persistent_lock); efi_mem_reserve_iomem(addr, size)
}

unsafe fn efi_memreserve_root_init() -> c_int { if efi_memreserve_root.is_null() && efi_memreserve_map_root() != 0 { efi_memreserve_root = ULONG_MAX as *mut _; } 0 }
early_initcall!(efi_memreserve_root_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
