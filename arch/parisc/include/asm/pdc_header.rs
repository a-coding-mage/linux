/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <uapi/asm/pdc.h> declarations.

#[allow(improper_ctypes)]
extern "C" {
    pub static mut parisc_narrow_firmware: ::core::ffi::c_int;

    pub static mut pdc_type: ::core::ffi::c_int;
    pub static mut parisc_cell_num: ::core::ffi::c_ulong; // cell number the CPU runs on (PAT)
    pub static mut parisc_cell_loc: ::core::ffi::c_ulong; // cell location of CPU (PAT)
    pub static mut parisc_pat_pdc_cap: ::core::ffi::c_ulong; // PDC capabilities (PAT)

    pub fn setup_pdc(); // in inventory.c

    pub fn pdc_add_valid(address: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_instr(instr: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn pdc_chassis_info(
        chassis_info: *mut pdc_chassis_info,
        led_info: *mut ::core::ffi::c_void,
        len: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn pdc_chassis_disp(disp: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_chassis_warn(warn: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_coproc_cfg(pdc_coproc_info: *mut pdc_coproc_cfg) -> ::core::ffi::c_int;
    pub fn pdc_coproc_cfg_unlocked(pdc_coproc_info: *mut pdc_coproc_cfg) -> ::core::ffi::c_int;
    pub fn pdc_iodc_read(
        actcnt: *mut ::core::ffi::c_ulong,
        hpa: ::core::ffi::c_ulong,
        index: ::core::ffi::c_uint,
        iodc_data: *mut ::core::ffi::c_void,
        iodc_data_size: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn pdc_system_map_find_mods(
        pdc_mod_info: *mut pdc_system_map_mod_info,
        mod_path: *mut pdc_module_path,
        mod_index: ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    pub fn pdc_system_map_find_addrs(
        pdc_addr_info: *mut pdc_system_map_addr_info,
        mod_index: ::core::ffi::c_long,
        addr_index: ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    pub fn pdc_model_info(model: *mut pdc_model) -> ::core::ffi::c_int;
    pub fn pdc_model_sysmodel(os_id: ::core::ffi::c_uint, name: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn pdc_model_cpuid(cpu_id: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_model_versions(versions: *mut ::core::ffi::c_ulong, id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn pdc_model_capabilities(capabilities: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_model_platform_info(orig_prod_num: *mut ::core::ffi::c_char, current_prod_num: *mut ::core::ffi::c_char, serial_no: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn pdc_cache_info(cache: *mut pdc_cache_info) -> ::core::ffi::c_int;
    pub fn pdc_spaceid_bits(space_bits: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_btlb_info(btlb: *mut pdc_btlb_info) -> ::core::ffi::c_int;
    pub fn pdc_btlb_insert(vpage: ::core::ffi::c_ulonglong, physpage: ::core::ffi::c_ulong, len: ::core::ffi::c_ulong, entry_info: ::core::ffi::c_ulong, slot: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_btlb_purge_all() -> ::core::ffi::c_int;
    pub fn pdc_mem_map_hpa(r_addr: *mut pdc_memory_map, mod_path: *mut pdc_module_path) -> ::core::ffi::c_int;
    pub fn pdc_pim_toc11(ret: *mut pdc_toc_pim_11) -> ::core::ffi::c_int;
    pub fn pdc_pim_toc20(ret: *mut pdc_toc_pim_20) -> ::core::ffi::c_int;
    pub fn pdc_lan_station_id(lan_addr: *mut ::core::ffi::c_char, net_hpa: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_stable_read(staddr: ::core::ffi::c_ulong, memaddr: *mut ::core::ffi::c_void, count: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_stable_write(staddr: ::core::ffi::c_ulong, memaddr: *mut ::core::ffi::c_void, count: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_stable_get_size(size: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_stable_verify_contents() -> ::core::ffi::c_int;
    pub fn pdc_stable_initialize() -> ::core::ffi::c_int;
    pub fn pdc_pci_irt_size(num_entries: *mut ::core::ffi::c_ulong, hpa: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_pci_irt(num_entries: ::core::ffi::c_ulong, hpa: ::core::ffi::c_ulong, tbl: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn pdc_get_initiator(hwpath: *mut hardware_path, initiator: *mut pdc_initiator) -> ::core::ffi::c_int;
    pub fn pdc_tod_read(tod: *mut pdc_tod) -> ::core::ffi::c_int;
    pub fn pdc_tod_set(sec: ::core::ffi::c_ulong, usec: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_pdt_init();
    pub fn pdc_mem_pdt_info(rinfo: *mut pdc_mem_retinfo) -> ::core::ffi::c_int;
    pub fn pdc_mem_pdt_read_entries(rpdt_read: *mut pdc_mem_read_pdt, pdt_entries_ptr: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    // CONFIG_64BIT conditional declaration.
    #[cfg(CONFIG_64BIT)]
    pub fn pdc_mem_mem_table(r_addr: *mut pdc_memory_table_raddr, tbl: *mut pdc_memory_table, entries: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn set_firmware_width();
    pub fn set_firmware_width_unlocked();
    pub fn pdc_do_firm_test_reset(ftc_bitmap: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_do_reset() -> ::core::ffi::c_int;
    pub fn pdc_soft_power_info(power_reg: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pdc_soft_power_button(sw_control: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn pdc_soft_power_button_panic(sw_control: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn pdc_io_reset();
    pub fn pdc_io_reset_devices();
    pub fn pdc_iodc_getc() -> ::core::ffi::c_int;
    pub fn pdc_iodc_print(str_: *const ::core::ffi::c_uchar, count: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn pdc_emergency_unlock();
    pub fn pdc_sti_call(func: ::core::ffi::c_ulong, flags: ::core::ffi::c_ulong, inptr: ::core::ffi::c_ulong, outputr: ::core::ffi::c_ulong, glob_cfg: ::core::ffi::c_ulong, do_call64: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn __pdc_cpu_rendezvous() -> ::core::ffi::c_int;
    pub fn pdc_cpu_rendezvous_lock();
    pub fn pdc_cpu_rendezvous_unlock();
}

pub const PDC_TYPE_ILLEGAL: ::core::ffi::c_int = -1;
pub const PDC_TYPE_PAT: ::core::ffi::c_int = 0;
pub const PDC_TYPE_SYSTEM_MAP: ::core::ffi::c_int = 1;
pub const PDC_TYPE_SNAKE: ::core::ffi::c_int = 2;

pub unsafe fn os_id_to_string(os_id: u16) -> *mut ::core::ffi::c_char {
    match os_id {
        OS_ID_NONE => b"No OS\0" as *const [u8; 6] as *mut ::core::ffi::c_char,
        OS_ID_HPUX => b"HP-UX\0" as *const [u8; 6] as *mut ::core::ffi::c_char,
        OS_ID_MPEXL => b"MPE-iX\0" as *const [u8; 7] as *mut ::core::ffi::c_char,
        OS_ID_OSF => b"OSF\0" as *const [u8; 4] as *mut ::core::ffi::c_char,
        OS_ID_HPRT => b"HP-RT\0" as *const [u8; 6] as *mut ::core::ffi::c_char,
        OS_ID_NOVEL => b"Novell Netware\0" as *const [u8; 15] as *mut ::core::ffi::c_char,
        OS_ID_LINUX => b"Linux\0" as *const [u8; 6] as *mut ::core::ffi::c_char,
        _ => b"Unknown\0" as *const [u8; 8] as *mut ::core::ffi::c_char,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
