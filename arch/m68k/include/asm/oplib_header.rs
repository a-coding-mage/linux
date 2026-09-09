/* SPDX-License-Identifier: GPL-2.0 */
/*
 * oplib.h: Describes the interface and available routines in the Linux PROM
 * library.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_uchar};

// Types declared by asm/openprom.h and supplied by other translated files.
use crate::{linux_mem_v0, linux_nodeops, linux_prom_ranges, linux_prom_registers, linux_romvec};

/* The master romvec pointer... */
extern "C" {
    pub static mut romvec: *mut linux_romvec;
}

/* Enumeration to describe the prom major version we have detected. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum prom_major_version {
    PROM_V0,
    PROM_V2,
    PROM_V3,
    PROM_P1275,
}

extern "C" {
    pub static mut prom_vers: prom_major_version;
    /* Revision, and firmware revision. */
    pub static mut prom_rev: c_uint;
    pub static mut prom_prev: c_uint;

    /* Root node of the prom device tree, this stays constant after
     * initialization is complete.
     */
    pub static mut prom_root_node: c_int;

    /* Pointer to prom structure containing the device tree traversal
     * and usage utility functions.  Only prom-lib should use these,
     * users use the interface defined by the library only!
     */
    pub static mut prom_nodeops: *mut linux_nodeops;

    /* You must call prom_init() before using any of the library services,
     * preferably as early as possible.  Pass it the romvec pointer.
     */
    pub fn prom_init(rom_ptr: *mut linux_romvec);
    /* Boot argument acquisition, returns the boot command line string. */
    pub fn prom_getbootargs() -> *mut c_char;

    pub fn prom_mapio(virt_hint: *mut c_char, io_space: c_int, phys_addr: c_uint, num_bytes: c_uint) -> *mut c_char;
    pub fn prom_unmapio(virt_addr: *mut c_char, num_bytes: c_uint);
    pub fn prom_devopen(device_string: *mut c_char) -> c_int;
    pub fn prom_devclose(device_handle: c_int) -> c_int;
    pub fn prom_seek(device_handle: c_int, seek_hival: c_uint, seek_lowval: c_uint);

    pub fn prom_meminfo() -> *mut linux_mem_v0;
    pub fn prom_reboot(boot_command: *mut c_char);
    pub fn prom_feval(forth_string: *mut c_char);
    pub fn prom_cmdline();
    pub fn prom_halt();
}

pub type sync_func_t = unsafe extern "C" fn();

extern "C" {
    pub fn prom_setsync(func_ptr: sync_func_t);
    pub fn prom_get_idprom(idp_buffer: *mut c_char, idpbuf_size: c_int) -> c_uchar;
    pub fn prom_version() -> c_int;
    pub fn prom_getrev() -> c_int;
    pub fn prom_getprev() -> c_int;

    pub fn prom_nbgetchar() -> c_int;
    pub fn prom_nbputchar(character: c_int) -> c_int;
    pub fn prom_getchar() -> c_char;
    pub fn prom_putchar(character: c_int);

    // __printf(1, 2) annotation from linux/compiler.h.
    pub fn prom_printf(fmt: *mut c_char, ...);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum prom_input_device {
    PROMDEV_IKBD,
    PROMDEV_ITTYA,
    PROMDEV_ITTYB,
    PROMDEV_I_UNK,
}

extern "C" {
    pub fn prom_query_input_device() -> prom_input_device;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum prom_output_device {
    PROMDEV_OSCREEN,
    PROMDEV_OTTYA,
    PROMDEV_OTTYB,
    PROMDEV_O_UNK,
}

extern "C" {
    pub fn prom_query_output_device() -> prom_output_device;
    pub fn prom_startcpu(cpunode: c_int, context_table: *mut linux_prom_registers, context: c_int, program_counter: *mut c_char) -> c_int;
    pub fn prom_stopcpu(cpunode: c_int) -> c_int;
    pub fn prom_idlecpu(cpunode: c_int) -> c_int;
    pub fn prom_restartcpu(cpunode: c_int) -> c_int;
    pub fn prom_alloc(virt_hint: *mut c_char, size: c_uint) -> *mut c_char;
    pub fn prom_free(virt_addr: *mut c_char, size: c_uint);
    pub fn prom_putsegment(context: c_int, virt_addr: c_ulong, physical_segment: c_int);

    pub fn prom_getchild(parent_node: c_int) -> c_int;
    pub fn prom_getsibling(node: c_int) -> c_int;
    pub fn prom_getproplen(thisnode: c_int, property: *mut c_char) -> c_int;
    pub fn prom_getproperty(thisnode: c_int, property: *mut c_char, prop_buffer: *mut c_char, propbuf_size: c_int) -> c_int;
    pub fn prom_getint(node: c_int, property: *mut c_char) -> c_int;
    pub fn prom_getintdefault(node: c_int, property: *mut c_char, defval: c_int) -> c_int;
    pub fn prom_getbool(node: c_int, prop: *mut c_char) -> c_int;
    pub fn prom_getstring(node: c_int, prop: *mut c_char, buf: *mut c_char, bufsize: c_int);
    pub fn prom_nodematch(thisnode: c_int, name: *mut c_char) -> c_int;
    pub fn prom_searchsiblings(node_start: c_int, name: *mut c_char) -> c_int;
    pub fn prom_firstprop(node: c_int) -> *mut c_char;
    pub fn prom_nextprop(node: c_int, prev_property: *mut c_char) -> *mut c_char;
    pub fn prom_node_has_property(node: c_int, property: *mut c_char) -> c_int;
    pub fn prom_setprop(node: c_int, prop_name: *mut c_char, prop_value: *mut c_char, value_size: c_int) -> c_int;
    pub fn prom_pathtoinode(path: *mut c_char) -> c_int;
    pub fn prom_inst2pkg(_: c_int) -> c_int;

    pub fn prom_adjust_regs(regp: *mut linux_prom_registers, nregs: c_int, rangep: *mut linux_prom_ranges, nranges: c_int);
    pub fn prom_adjust_ranges(cranges: *mut linux_prom_ranges, ncranges: c_int, pranges: *mut linux_prom_ranges, npranges: c_int);
    pub fn prom_apply_obio_ranges(obioregs: *mut linux_prom_registers, nregs: c_int);
    pub fn prom_apply_generic_ranges(node: c_int, parent: c_int, sbusregs: *mut linux_prom_registers, nregs: c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
