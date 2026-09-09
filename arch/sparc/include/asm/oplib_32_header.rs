/* SPDX-License-Identifier: GPL-2.0 */
/*
 * oplib.h:  Describes the interface and available routines in the
 *           Linux Prom library.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependencies supplied by the surrounding translation unit:
// asm/openprom.h, linux/spinlock.h, and linux/compiler.h.

/* The master romvec pointer... */
unsafe extern "C" {
    pub static mut romvec: *mut linux_romvec;
}

/* Enumeration to describe the prom major version we have detected. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum prom_major_version {
    PROM_V0,    /* Original sun4c V0 prom */
    PROM_V2,    /* sun4c and early sun4m V2 prom */
    PROM_V3,    /* sun4m and later, up to sun4d/sun4e machines V3 */
    PROM_P1275, /* IEEE compliant ISA based Sun PROM, only sun4u */
}

unsafe extern "C" {
    pub static mut prom_vers: prom_major_version;
    /* Revision, and firmware revision. */
    pub static mut prom_rev: ::core::ffi::c_uint;
    pub static mut prom_prev: ::core::ffi::c_uint;
    /* Root node of the prom device tree, this stays constant after
     * initialization is complete.
     */
    pub static mut prom_root_node: phandle;
    /* Pointer to prom structure containing the device tree traversal
     * and usage utility functions.  Only prom-lib should use these,
     * users use the interface defined by the library only!
     */
    pub static mut prom_nodeops: *mut linux_nodeops;
}

/* The functions... */
unsafe extern "C" {
    /* You must call prom_init() before using any of the library services,
     * preferably as early as possible.  Pass it the romvec pointer.
     */
    pub fn prom_init(rom_ptr: *mut linux_romvec);
    /* Boot argument acquisition, returns the boot command line string. */
    pub fn prom_getbootargs() -> *mut ::core::ffi::c_char;
    /* Miscellaneous routines, don't really fit in any category per se. */
    /* Reboot the machine with the command line passed. */
    pub fn prom_reboot(boot_command: *mut ::core::ffi::c_char);
    /* Evaluate the forth string passed. */
    pub fn prom_feval(forth_string: *mut ::core::ffi::c_char);
    /* Enter the prom, with possibility of continuation with the 'go'
     * command in newer proms.
     */
    pub fn prom_cmdline();
    /* Enter the prom, with no chance of continuation for the stand-alone
     * which calls this.
     */
    pub fn prom_halt() -> !;
}

/* Set the PROM 'sync' callback function to the passed function pointer.
 * When the user gives the 'sync' command at the prom prompt while the
 * kernel is still active, the prom will call this routine.
 *
 * XXX The arguments are different on V0 vs. V2->higher proms, grrr! XXX
 */
pub type sync_func_t = Option<unsafe extern "C" fn()>;

unsafe extern "C" {
    pub fn prom_setsync(func_ptr: sync_func_t);
    /* Acquire the IDPROM of the root node in the prom device tree.  This
     * gets passed a buffer where you would like it stuffed.  The return value
     * is the format type of this idprom or 0xff on error.
     */
    pub fn prom_get_idprom(idp_buffer: *mut ::core::ffi::c_char, idpbuf_size: ::core::ffi::c_int) -> ::core::ffi::c_uchar;
    pub fn prom_version() -> ::core::ffi::c_int;
    pub fn prom_getrev() -> ::core::ffi::c_int;
    pub fn prom_getprev() -> ::core::ffi::c_int;
    pub fn prom_console_write_buf(buf: *const ::core::ffi::c_char, len: ::core::ffi::c_int);
    /* Prom's internal routines, don't use in kernel/boot code. */
    pub fn prom_printf(fmt: *const ::core::ffi::c_char, ...) -> ();
    pub fn prom_write(buf: *const ::core::ffi::c_char, len: ::core::ffi::c_uint);
    pub fn prom_startcpu(cpunode: ::core::ffi::c_int, context_table: *mut linux_prom_registers,
                         context: ::core::ffi::c_int, program_counter: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn prom_meminit();
    pub fn prom_getchild(parent_node: phandle) -> phandle;
    pub fn prom_getsibling(node: phandle) -> phandle;
    pub fn prom_getproplen(thisnode: phandle, property: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn prom_getproperty(thisnode: phandle, property: *const ::core::ffi::c_char,
                            prop_buffer: *mut ::core::ffi::c_char, propbuf_size: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn prom_getint(node: phandle, property: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn prom_getintdefault(node: phandle, property: *mut ::core::ffi::c_char, defval: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn prom_getbool(node: phandle, prop: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn prom_getstring(node: phandle, prop: *mut ::core::ffi::c_char, buf: *mut ::core::ffi::c_char, bufsize: ::core::ffi::c_int);
    pub fn prom_searchsiblings(node_start: phandle, name: *mut ::core::ffi::c_char) -> phandle;
    pub fn prom_nextprop(node: phandle, prev_property: *mut ::core::ffi::c_char, buffer: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn prom_finddevice(name: *mut ::core::ffi::c_char) -> phandle;
    pub fn prom_setprop(node: phandle, prop_name: *const ::core::ffi::c_char,
                        prop_value: *mut ::core::ffi::c_char, value_size: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn prom_inst2pkg(arg: ::core::ffi::c_int) -> phandle;
    pub fn prom_apply_obio_ranges(obioregs: *mut linux_prom_registers, nregs: ::core::ffi::c_int);
    pub fn prom_apply_generic_ranges(node: phandle, parent: phandle,
                                     sbusregs: *mut linux_prom_registers, nregs: ::core::ffi::c_int);
    pub fn prom_ranges_init();
    pub fn cpu_find_by_instance(instance: ::core::ffi::c_int, prom_node: *mut phandle, mid: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn cpu_find_by_mid(mid: ::core::ffi::c_int, prom_node: *mut phandle) -> ::core::ffi::c_int;
    pub fn cpu_get_hwmid(prom_node: phandle) -> ::core::ffi::c_int;
    pub static mut prom_lock: spinlock_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
