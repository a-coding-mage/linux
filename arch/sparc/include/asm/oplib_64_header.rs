/* SPDX-License-Identifier: GPL-2.0 */
/* oplib.h: Describes the interface and available routines in the
 * Linux Prom library.
 *
 * Copyright (C) 1995, 2007 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1996 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

/* Dependency supplied by asm/openprom.h. */

/* OBP version string. */
extern "C" {
    pub static mut prom_version: [::kernel::ffi::c_char; 0];
    pub static mut prom_root_node: phandle;
    pub static mut prom_stdout: ::kernel::ffi::c_int;
    pub static mut prom_chosen_node: phandle;

    pub static prom_peer_name: [::kernel::ffi::c_char; 0];
    pub static prom_compatible_name: [::kernel::ffi::c_char; 0];
    pub static prom_root_compatible: [::kernel::ffi::c_char; 0];
    pub static prom_cpu_compatible: [::kernel::ffi::c_char; 0];
    pub static prom_finddev_name: [::kernel::ffi::c_char; 0];
    pub static prom_chosen_path: [::kernel::ffi::c_char; 0];
    pub static prom_cpu_path: [::kernel::ffi::c_char; 0];
    pub static prom_getprop_name: [::kernel::ffi::c_char; 0];
    pub static prom_mmu_name: [::kernel::ffi::c_char; 0];
    pub static prom_callmethod_name: [::kernel::ffi::c_char; 0];
    pub static prom_translate_name: [::kernel::ffi::c_char; 0];
    pub static prom_map_name: [::kernel::ffi::c_char; 0];
    pub static prom_unmap_name: [::kernel::ffi::c_char; 0];
    pub static mut prom_mmu_ihandle_cache: ::kernel::ffi::c_int;
    pub static mut prom_boot_mapped_pc: ::kernel::ffi::c_uint;
    pub static mut prom_boot_mapping_mode: ::kernel::ffi::c_uint;
    pub static mut prom_boot_mapping_phys_high: ::kernel::ffi::c_ulong;
    pub static mut prom_boot_mapping_phys_low: ::kernel::ffi::c_ulong;
}

#[repr(C)]
pub struct linux_mlist_p1275 {
    pub theres_more: *mut linux_mlist_p1275,
    pub start_adr: ::kernel::ffi::c_ulong,
    pub num_bytes: ::kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct linux_mem_p1275 {
    pub p1275_totphys: *mut *mut linux_mlist_p1275,
    pub p1275_prommap: *mut *mut linux_mlist_p1275,
    pub p1275_available: *mut *mut linux_mlist_p1275, /* What we can use */
}

extern "C" {
    pub fn prom_init(cif_handler: *mut ::kernel::ffi::c_void);
    pub fn prom_init_report();
    pub fn prom_getbootargs() -> *mut ::kernel::ffi::c_char;
    pub fn prom_reboot(boot_command: *const ::kernel::ffi::c_char);
    pub fn prom_feval(forth_string: *const ::kernel::ffi::c_char);
    pub fn prom_cmdline();
    pub fn prom_halt() -> !;
    pub fn prom_halt_power_off() -> !;
    pub fn prom_get_idprom(idp_buffer: *mut ::kernel::ffi::c_char, idpbuf_size: ::kernel::ffi::c_int) -> ::kernel::ffi::c_uchar;
    pub fn prom_console_write_buf(buf: *const ::kernel::ffi::c_char, len: ::kernel::ffi::c_int);
    pub fn prom_printf(fmt: *const ::kernel::ffi::c_char, ...);
    pub fn prom_write(buf: *const ::kernel::ffi::c_char, len: ::kernel::ffi::c_uint);

    /* Power management interfaces. */
    pub fn prom_sleepself();
    pub fn prom_sleepsystem() -> ::kernel::ffi::c_int;
    pub fn prom_wakeupsystem() -> ::kernel::ffi::c_int;

    pub fn prom_getunumber(syndrome_code: ::kernel::ffi::c_int, phys_addr: ::kernel::ffi::c_ulong, buf: *mut ::kernel::ffi::c_char, buflen: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn prom_retain(name: *const ::kernel::ffi::c_char, size: ::kernel::ffi::c_ulong, align: ::kernel::ffi::c_ulong, paddr: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn prom_itlb_load(index: ::kernel::ffi::c_ulong, tte_data: ::kernel::ffi::c_ulong, vaddr: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_long;
    pub fn prom_dtlb_load(index: ::kernel::ffi::c_ulong, tte_data: ::kernel::ffi::c_ulong, vaddr: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_long;
    pub fn prom_map(mode: ::kernel::ffi::c_int, size: ::kernel::ffi::c_ulong, vaddr: ::kernel::ffi::c_ulong, paddr: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn prom_unmap(size: ::kernel::ffi::c_ulong, vaddr: ::kernel::ffi::c_ulong);

    pub fn prom_getchild(parent_node: phandle) -> phandle;
    pub fn prom_getsibling(node: phandle) -> phandle;
    pub fn prom_getproplen(thisnode: phandle, property: *const ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn prom_getproperty(thisnode: phandle, property: *const ::kernel::ffi::c_char, prop_buffer: *mut ::kernel::ffi::c_char, propbuf_size: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn prom_getint(node: phandle, property: *const ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn prom_getintdefault(node: phandle, property: *const ::kernel::ffi::c_char, defval: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn prom_getbool(node: phandle, prop: *const ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn prom_getstring(node: phandle, prop: *const ::kernel::ffi::c_char, buf: *mut ::kernel::ffi::c_char, bufsize: ::kernel::ffi::c_int);
    pub fn prom_nodematch(thisnode: phandle, name: *const ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn prom_searchsiblings(node_start: phandle, name: *const ::kernel::ffi::c_char) -> phandle;
    pub fn prom_firstprop(node: phandle, buffer: *mut ::kernel::ffi::c_char) -> *mut ::kernel::ffi::c_char;
    pub fn prom_nextprop(node: phandle, prev_property: *const ::kernel::ffi::c_char, buf: *mut ::kernel::ffi::c_char) -> *mut ::kernel::ffi::c_char;
    pub fn prom_node_has_property(node: phandle, property: *const ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn prom_finddevice(name: *const ::kernel::ffi::c_char) -> phandle;
    pub fn prom_setprop(node: phandle, prop_name: *const ::kernel::ffi::c_char, prop_value: *mut ::kernel::ffi::c_char, value_size: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn prom_inst2pkg(arg: ::kernel::ffi::c_int) -> phandle;
    pub fn prom_sun4v_guest_soft_state();
    pub fn prom_ihandle2path(handle: ::kernel::ffi::c_int, buffer: *mut ::kernel::ffi::c_char, bufsize: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn prom_cif_init(cif_handler: *mut ::kernel::ffi::c_void);
    pub fn p1275_cmd_direct(args: *mut ::kernel::ffi::c_ulong);
}

pub const PROM_MAP_WRITE: ::kernel::ffi::c_int = 0x0001;
pub const PROM_MAP_READ: ::kernel::ffi::c_int = 0x0002;
pub const PROM_MAP_EXEC: ::kernel::ffi::c_int = 0x0004;
pub const PROM_MAP_LOCKED: ::kernel::ffi::c_int = 0x0010;
pub const PROM_MAP_CACHED: ::kernel::ffi::c_int = 0x0020;
pub const PROM_MAP_SE: ::kernel::ffi::c_int = 0x0040;
pub const PROM_MAP_GLOB: ::kernel::ffi::c_int = 0x0080;
pub const PROM_MAP_IE: ::kernel::ffi::c_int = 0x0100;
pub const PROM_MAP_DEFAULT: ::kernel::ffi::c_int = PROM_MAP_WRITE | PROM_MAP_READ | PROM_MAP_EXEC | PROM_MAP_CACHED;

/* CONFIG_SMP conditional declarations are preserved here. */
#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn prom_startcpu(cpunode: ::kernel::ffi::c_int, pc: ::kernel::ffi::c_ulong, arg: ::kernel::ffi::c_ulong);
    pub fn prom_startcpu_cpuid(cpuid: ::kernel::ffi::c_int, pc: ::kernel::ffi::c_ulong, arg: ::kernel::ffi::c_ulong);
    pub fn prom_stopcpu_cpuid(cpuid: ::kernel::ffi::c_int);
    pub fn prom_stopself();
    pub fn prom_idleself();
    pub fn prom_resumecpu(cpunode: ::kernel::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
