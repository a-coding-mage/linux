/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <uapi/asm/pdc.h> declarations.

#[allow(improper_ctypes)]
extern "C" {
    pub static mut parisc_narrow_firmware: ::kernel::ffi::c_int;

    pub static mut pdc_type: ::kernel::ffi::c_int;
    pub static mut parisc_cell_num: ::kernel::ffi::c_ulong; // cell number the CPU runs on (PAT)
    pub static mut parisc_cell_loc: ::kernel::ffi::c_ulong; // cell location of CPU (PAT)
    pub static mut parisc_pat_pdc_cap: ::kernel::ffi::c_ulong; // PDC capabilities (PAT)

    pub fn setup_pdc(); // in inventory.c

    pub fn pdc_add_valid(address: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_instr(instr: *mut ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int;
    pub fn pdc_chassis_info(
        chassis_info: *mut pdc_chassis_info,
        led_info: *mut ::kernel::ffi::c_void,
        len: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_int;
    pub fn pdc_chassis_disp(disp: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_chassis_warn(warn: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_coproc_cfg(pdc_coproc_info: *mut pdc_coproc_cfg) -> ::kernel::ffi::c_int;
    pub fn pdc_coproc_cfg_unlocked(pdc_coproc_info: *mut pdc_coproc_cfg) -> ::kernel::ffi::c_int;
    pub fn pdc_iodc_read(
        actcnt: *mut ::kernel::ffi::c_ulong,
        hpa: ::kernel::ffi::c_ulong,
        index: ::kernel::ffi::c_uint,
        iodc_data: *mut ::kernel::ffi::c_void,
        iodc_data_size: ::kernel::ffi::c_uint,
    ) -> ::kernel::ffi::c_int;
    pub fn pdc_system_map_find_mods(
        pdc_mod_info: *mut pdc_system_map_mod_info,
        mod_path: *mut pdc_module_path,
        mod_index: ::kernel::ffi::c_long,
    ) -> ::kernel::ffi::c_int;
    pub fn pdc_system_map_find_addrs(
        pdc_addr_info: *mut pdc_system_map_addr_info,
        mod_index: ::kernel::ffi::c_long,
        addr_index: ::kernel::ffi::c_long,
    ) -> ::kernel::ffi::c_int;
    pub fn pdc_model_info(model: *mut pdc_model) -> ::kernel::ffi::c_int;
    pub fn pdc_model_sysmodel(os_id: ::kernel::ffi::c_uint, name: *mut ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn pdc_model_cpuid(cpu_id: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_model_versions(versions: *mut ::kernel::ffi::c_ulong, id: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn pdc_model_capabilities(capabilities: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_model_platform_info(orig_prod_num: *mut ::kernel::ffi::c_char, current_prod_num: *mut ::kernel::ffi::c_char, serial_no: *mut ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn pdc_cache_info(cache: *mut pdc_cache_info) -> ::kernel::ffi::c_int;
    pub fn pdc_spaceid_bits(space_bits: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_btlb_info(btlb: *mut pdc_btlb_info) -> ::kernel::ffi::c_int;
    pub fn pdc_btlb_insert(vpage: ::kernel::ffi::c_ulonglong, physpage: ::kernel::ffi::c_ulong, len: ::kernel::ffi::c_ulong, entry_info: ::kernel::ffi::c_ulong, slot: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_btlb_purge_all() -> ::kernel::ffi::c_int;
    pub fn pdc_mem_map_hpa(r_addr: *mut pdc_memory_map, mod_path: *mut pdc_module_path) -> ::kernel::ffi::c_int;
    pub fn pdc_pim_toc11(ret: *mut pdc_toc_pim_11) -> ::kernel::ffi::c_int;
    pub fn pdc_pim_toc20(ret: *mut pdc_toc_pim_20) -> ::kernel::ffi::c_int;
    pub fn pdc_lan_station_id(lan_addr: *mut ::kernel::ffi::c_char, net_hpa: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_stable_read(staddr: ::kernel::ffi::c_ulong, memaddr: *mut ::kernel::ffi::c_void, count: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_stable_write(staddr: ::kernel::ffi::c_ulong, memaddr: *mut ::kernel::ffi::c_void, count: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_stable_get_size(size: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_stable_verify_contents() -> ::kernel::ffi::c_int;
    pub fn pdc_stable_initialize() -> ::kernel::ffi::c_int;
    pub fn pdc_pci_irt_size(num_entries: *mut ::kernel::ffi::c_ulong, hpa: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_pci_irt(num_entries: ::kernel::ffi::c_ulong, hpa: ::kernel::ffi::c_ulong, tbl: *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int;
    pub fn pdc_get_initiator(hwpath: *mut hardware_path, initiator: *mut pdc_initiator) -> ::kernel::ffi::c_int;
    pub fn pdc_tod_read(tod: *mut pdc_tod) -> ::kernel::ffi::c_int;
    pub fn pdc_tod_set(sec: ::kernel::ffi::c_ulong, usec: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_pdt_init();
    pub fn pdc_mem_pdt_info(rinfo: *mut pdc_mem_retinfo) -> ::kernel::ffi::c_int;
    pub fn pdc_mem_pdt_read_entries(rpdt_read: *mut pdc_mem_read_pdt, pdt_entries_ptr: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    // CONFIG_64BIT conditional declaration.
    #[cfg(CONFIG_64BIT)]
    pub fn pdc_mem_mem_table(r_addr: *mut pdc_memory_table_raddr, tbl: *mut pdc_memory_table, entries: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn set_firmware_width();
    pub fn set_firmware_width_unlocked();
    pub fn pdc_do_firm_test_reset(ftc_bitmap: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_do_reset() -> ::kernel::ffi::c_int;
    pub fn pdc_soft_power_info(power_reg: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn pdc_soft_power_button(sw_control: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn pdc_soft_power_button_panic(sw_control: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn pdc_io_reset();
    pub fn pdc_io_reset_devices();
    pub fn pdc_iodc_getc() -> ::kernel::ffi::c_int;
    pub fn pdc_iodc_print(str_: *const ::kernel::ffi::c_uchar, count: ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int;
    pub fn pdc_emergency_unlock();
    pub fn pdc_sti_call(func: ::kernel::ffi::c_ulong, flags: ::kernel::ffi::c_ulong, inptr: ::kernel::ffi::c_ulong, outputr: ::kernel::ffi::c_ulong, glob_cfg: ::kernel::ffi::c_ulong, do_call64: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn __pdc_cpu_rendezvous() -> ::kernel::ffi::c_int;
    pub fn pdc_cpu_rendezvous_lock();
    pub fn pdc_cpu_rendezvous_unlock();
}

pub const PDC_TYPE_ILLEGAL: ::kernel::ffi::c_int = -1;
pub const PDC_TYPE_PAT: ::kernel::ffi::c_int = 0;
pub const PDC_TYPE_SYSTEM_MAP: ::kernel::ffi::c_int = 1;
pub const PDC_TYPE_SNAKE: ::kernel::ffi::c_int = 2;

pub unsafe fn os_id_to_string(os_id: u16) -> *mut ::kernel::ffi::c_char {
    match os_id {
        OS_ID_NONE => b"No OS\0" as *const [u8; 6] as *mut ::kernel::ffi::c_char,
        OS_ID_HPUX => b"HP-UX\0" as *const [u8; 6] as *mut ::kernel::ffi::c_char,
        OS_ID_MPEXL => b"MPE-iX\0" as *const [u8; 7] as *mut ::kernel::ffi::c_char,
        OS_ID_OSF => b"OSF\0" as *const [u8; 4] as *mut ::kernel::ffi::c_char,
        OS_ID_HPRT => b"HP-RT\0" as *const [u8; 6] as *mut ::kernel::ffi::c_char,
        OS_ID_NOVEL => b"Novell Netware\0" as *const [u8; 15] as *mut ::kernel::ffi::c_char,
        OS_ID_LINUX => b"Linux\0" as *const [u8; 6] as *mut ::kernel::ffi::c_char,
        _ => b"Unknown\0" as *const [u8; 8] as *mut ::kernel::ffi::c_char,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
