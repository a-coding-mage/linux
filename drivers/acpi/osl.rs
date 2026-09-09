// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  acpi_osl.c - OS-dependent functions ($Revision: 83 $)
 *
 *  Copyright (C) 2000       Andrew Henroid
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 *  Copyright (c) 2008 Intel Corporation
 *   Author: Matthew Wilcox <willy@linux.intel.com>
 */

// Linux kernel headers and ACPICA headers are supplied by other translation units.
// The original build-time configuration conditions are retained below.

/* Definitions for ACPI_DEBUG_PRINT() */
const _COMPONENT: u32 = ACPI_OS_SERVICES;
ACPI_MODULE_NAME!("osl");

#[repr(C)]
pub struct acpi_os_dpc {
    pub function: acpi_osd_exec_callback,
    pub context: *mut core::ffi::c_void,
    pub work: work_struct,
}

#[cfg(ENABLE_DEBUGGER)]
pub static mut acpi_in_debugger: i32 = 0;

static mut __acpi_os_prepare_sleep: Option<unsafe extern "C" fn(u8, u32, u32) -> i32> = None;
static mut __acpi_os_prepare_extended_sleep: Option<unsafe extern "C" fn(u8, u32, u32) -> i32> = None;
static mut acpi_irq_handler: Option<acpi_osd_handler> = None;
static mut acpi_irq_context: *mut core::ffi::c_void = core::ptr::null_mut();
static mut kacpid_wq: *mut workqueue_struct = core::ptr::null_mut();
static mut kacpi_notify_wq: *mut workqueue_struct = core::ptr::null_mut();
static mut kacpi_hotplug_wq: *mut workqueue_struct = core::ptr::null_mut();
static mut acpi_os_initialized: bool = false;
pub static mut acpi_sci_irq: u32 = INVALID_ACPI_IRQ;
pub static mut acpi_permanent_mmap: bool = false;

static mut poweroff_on_fatal: bool = true;

#[repr(C)]
pub struct acpi_ioremap {
    pub list: list_head,
    pub virt: *mut core::ffi::c_void,
    pub phys: acpi_physical_address,
    pub size: acpi_size,
    pub track: acpi_ioremap_track,
}

#[repr(C)]
pub union acpi_ioremap_track {
    pub refcount: core::ffi::c_ulong,
    pub rwork: rcu_work,
}

static mut acpi_ioremaps: list_head = LIST_HEAD_INIT!(acpi_ioremaps);
static mut acpi_ioremap_lock: mutex = DEFINE_MUTEX!(acpi_ioremap_lock);

unsafe fn acpi_request_region(gas: *mut acpi_generic_address, length: u32, desc: *mut i8) {
    let mut addr: u64 = 0;
    core::ptr::copy_nonoverlapping(
        core::ptr::addr_of!((*gas).address) as *const u8,
        core::ptr::addr_of_mut!(addr) as *mut u8,
        core::mem::size_of::<u64>(),
    );
    if addr == 0 || length == 0 { return; }
    if (*gas).space_id == ACPI_ADR_SPACE_SYSTEM_IO {
        request_region(addr, length, desc);
    } else if (*gas).space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY {
        request_mem_region(addr, length, desc);
    }
}

unsafe fn acpi_reserve_resources() -> i32 {
    acpi_request_region(core::ptr::addr_of_mut!(acpi_gbl_FADT.xpm1a_event_block), acpi_gbl_FADT.pm1_event_length as u32, b"ACPI PM1a_EVT_BLK\0" as *const _ as *mut i8);
    acpi_request_region(core::ptr::addr_of_mut!(acpi_gbl_FADT.xpm1b_event_block), acpi_gbl_FADT.pm1_event_length as u32, b"ACPI PM1b_EVT_BLK\0" as *const _ as *mut i8);
    acpi_request_region(core::ptr::addr_of_mut!(acpi_gbl_FADT.xpm1a_control_block), acpi_gbl_FADT.pm1_control_length as u32, b"ACPI PM1a_CNT_BLK\0" as *const _ as *mut i8);
    acpi_request_region(core::ptr::addr_of_mut!(acpi_gbl_FADT.xpm1b_control_block), acpi_gbl_FADT.pm1_control_length as u32, b"ACPI PM1b_CNT_BLK\0" as *const _ as *mut i8);
    if acpi_gbl_FADT.pm_timer_length == 4 { acpi_request_region(core::ptr::addr_of_mut!(acpi_gbl_FADT.xpm_timer_block), 4, b"ACPI PM_TMR\0" as *const _ as *mut i8); }
    acpi_request_region(core::ptr::addr_of_mut!(acpi_gbl_FADT.xpm2_control_block), acpi_gbl_FADT.pm2_control_length as u32, b"ACPI PM2_CNT_BLK\0" as *const _ as *mut i8);
    if acpi_gbl_FADT.gpe0_block_length & 1 == 0 { acpi_request_region(core::ptr::addr_of_mut!(acpi_gbl_FADT.xgpe0_block), acpi_gbl_FADT.gpe0_block_length as u32, b"ACPI GPE0_BLK\0" as *const _ as *mut i8); }
    if acpi_gbl_FADT.gpe1_block_length & 1 == 0 { acpi_request_region(core::ptr::addr_of_mut!(acpi_gbl_FADT.xgpe1_block), acpi_gbl_FADT.gpe1_block_length as u32, b"ACPI GPE1_BLK\0" as *const _ as *mut i8); }
    0
}

// The remaining functions retain the C implementation's external kernel calls and ABI.
// They are intentionally expressed as direct unsafe Rust translations.

pub unsafe extern "C" fn acpi_os_printf(fmt: *const i8, mut args: ...) {
    acpi_os_vprintf(fmt, args);
}

pub unsafe extern "C" fn acpi_os_vprintf(fmt: *const i8, args: va_list) {
    static mut buffer: [i8; 512] = [0; 512];
    vsprintf(buffer.as_mut_ptr(), fmt, args);
    if acpi_debugger_write_log(buffer.as_ptr()) < 0 {
        if printk_get_level(buffer.as_ptr()) != 0 { printk(buffer.as_ptr()); }
        else { printk(KERN_CONT.as_ptr(), buffer.as_ptr()); }
    }
}

pub unsafe extern "C" fn acpi_os_get_root_pointer() -> acpi_physical_address {
    let mut pa: acpi_physical_address = 0;
    pa = acpi_arch_get_root_pointer();
    if pa != 0 { return pa; }
    if efi_enabled(EFI_CONFIG_TABLES) {
        if efi.acpi20 != EFI_INVALID_TABLE_ADDR { return efi.acpi20; }
        if efi.acpi != EFI_INVALID_TABLE_ADDR { return efi.acpi; }
        pr_err!("System description tables not found\n");
    } else if IS_ENABLED!(CONFIG_ACPI_LEGACY_TABLES_LOOKUP) { acpi_find_root_pointer(&mut pa); }
    pa
}

pub unsafe extern "C" fn acpi_os_sleep(ms: u64) {
    let usec = ms.wrapping_mul(USEC_PER_MSEC as u64);
    let mut delta_us = 50;
    if ms > 5 { delta_us = (USEC_PER_MSEC / 100) as u64 * ms; }
    usleep_range(usec as u32, usec.wrapping_add(delta_us) as u32);
}

pub unsafe extern "C" fn acpi_os_stall(mut us: u32) {
    while us != 0 { let delay = if us < 1000 { us } else { 1000 }; udelay(delay); touch_nmi_watchdog(); us -= delay; }
}

pub unsafe extern "C" fn acpi_os_get_timer() -> u64 {
    (get_jiffies_64() - INITIAL_JIFFIES) * (ACPI_100NSEC_PER_SEC / HZ)
}

pub unsafe extern "C" fn acpi_os_read_port(port: acpi_io_address, value: *mut u32, width: u32) -> acpi_status {
    let mut dummy = 0u32;
    if !IS_ENABLED!(CONFIG_HAS_IOPORT) { *value = GENMASK!(width, 0); return AE_NOT_IMPLEMENTED; }
    let out = if value.is_null() { &mut dummy } else { &mut *value }; *out = 0;
    if width <= 8 { *out = inb(port) as u32; } else if width <= 16 { *out = inw(port) as u32; } else if width <= 32 { *out = inl(port); } else { pr_debug!("{}: Access width {} not supported\n", "acpi_os_read_port", width); return AE_BAD_PARAMETER; } AE_OK
}

pub unsafe extern "C" fn acpi_os_write_port(port: acpi_io_address, value: u32, width: u32) -> acpi_status {
    if !IS_ENABLED!(CONFIG_HAS_IOPORT) { return AE_NOT_IMPLEMENTED; }
    if width <= 8 { outb(value, port); } else if width <= 16 { outw(value, port); } else if width <= 32 { outl(value, port); } else { pr_debug!("{}: Access width {} not supported\n", "acpi_os_write_port", width); return AE_BAD_PARAMETER; } AE_OK
}

unsafe fn acpi_map_lookup(phys: acpi_physical_address, size: acpi_size) -> *mut acpi_ioremap {
    let mut map: *mut acpi_ioremap = core::ptr::null_mut();
    list_for_each_entry_rcu!(map, &acpi_ioremaps, list, acpi_ioremap_lock_held!());
    if !map.is_null() && (*map).phys <= phys && phys.wrapping_add(size) <= (*map).phys.wrapping_add((*map).size) { map } else { core::ptr::null_mut() }
}

unsafe fn acpi_map_vaddr_lookup(phys: acpi_physical_address, size: u32) -> *mut core::ffi::c_void {
    let map = acpi_map_lookup(phys, size as acpi_size);
    if map.is_null() { core::ptr::null_mut() } else { ((*map).virt as usize).wrapping_add((phys - (*map).phys) as usize) as *mut _ }
}

pub unsafe extern "C" fn acpi_os_get_iomem(phys: acpi_physical_address, size: u32) -> *mut core::ffi::c_void {
    mutex_lock!(&mut acpi_ioremap_lock);
    let map = acpi_map_lookup(phys, size as acpi_size);
    let virt = if map.is_null() { core::ptr::null_mut() } else { (*map).track.refcount = (*map).track.refcount.wrapping_add(1); ((*map).virt as usize).wrapping_add((phys - (*map).phys) as usize) as *mut _ };
    mutex_unlock!(&mut acpi_ioremap_lock); virt
}

unsafe fn acpi_os_drop_map_ref(map: *mut acpi_ioremap) {
    (*map).track.refcount -= 1;
    if (*map).track.refcount != 0 { return; }
    list_del_rcu!(&mut (*map).list);
    INIT_RCU_WORK!(&mut (*map).track.rwork, acpi_os_map_remove);
    queue_rcu_work!(system_percpu_wq, &mut (*map).track.rwork);
}

unsafe extern "C" fn acpi_os_map_remove(work: *mut work_struct) {
    let map = container_of!(to_rcu_work!(work), acpi_ioremap, track.rwork);
    acpi_unmap((*map).phys, (*map).virt); kfree!(map);
}

pub unsafe extern "C" fn acpi_os_map_iomem(phys: acpi_physical_address, size: acpi_size) -> *mut core::ffi::c_void {
    if phys > ULONG_MAX as u64 { pr_err!("Cannot map memory that high: 0x{:x}\n", phys); return core::ptr::null_mut(); }
    if !acpi_permanent_mmap { return __acpi_map_table(phys as usize, size); }
    mutex_lock!(&mut acpi_ioremap_lock);
    let map = acpi_map_lookup(phys, size);
    if !map.is_null() { (*map).track.refcount += 1; mutex_unlock!(&mut acpi_ioremap_lock); return ((*map).virt as usize + (phys - (*map).phys) as usize) as *mut _; }
    let map = kzalloc_obj::<acpi_ioremap>();
    if map.is_null() { mutex_unlock!(&mut acpi_ioremap_lock); return core::ptr::null_mut(); }
    let pg_off = round_down!(phys, PAGE_SIZE); let pg_sz = round_up!(phys + size, PAGE_SIZE) - pg_off;
    let virt = acpi_os_ioremap(phys, size); if virt.is_null() { mutex_unlock!(&mut acpi_ioremap_lock); kfree!(map); return core::ptr::null_mut(); }
    (*map).virt = (virt as usize & PAGE_MASK) as *mut _; (*map).phys = pg_off; (*map).size = pg_sz; (*map).track.refcount = 1;
    list_add_tail_rcu!(&mut (*map).list, &mut acpi_ioremaps); mutex_unlock!(&mut acpi_ioremap_lock);
    ((*map).virt as usize + (phys - (*map).phys) as usize) as *mut _
}

pub unsafe extern "C" fn acpi_os_map_memory(phys: acpi_physical_address, size: acpi_size) -> *mut core::ffi::c_void { acpi_os_map_iomem(phys, size) }
pub unsafe extern "C" fn acpi_os_unmap_iomem(virt: *mut core::ffi::c_void, size: acpi_size) {
    if !acpi_permanent_mmap { __acpi_unmap_table(virt, size); return; }
    mutex_lock!(&mut acpi_ioremap_lock); let map = acpi_map_lookup_virt(virt, size); if map.is_null() { mutex_unlock!(&mut acpi_ioremap_lock); WARN!(true, "ACPI: bad address {:p}\n", virt); return; } acpi_os_drop_map_ref(map); mutex_unlock!(&mut acpi_ioremap_lock);
}
pub unsafe extern "C" fn acpi_os_unmap_memory(virt: *mut core::ffi::c_void, size: acpi_size) { acpi_os_unmap_iomem(virt, size); }

unsafe fn acpi_map_lookup_virt(virt: *mut core::ffi::c_void, size: acpi_size) -> *mut acpi_ioremap {
    let mut map: *mut acpi_ioremap = core::ptr::null_mut(); list_for_each_entry_rcu!(map, &acpi_ioremaps, list, acpi_ioremap_lock_held!()); if !map.is_null() && (*map).virt as usize <= virt as usize && virt as usize + size as usize <= (*map).virt as usize + (*map).size as usize { map } else { core::ptr::null_mut() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
