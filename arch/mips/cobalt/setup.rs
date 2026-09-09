/*
 * Setup pointers to hardware dependent routines.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 1997, 2004, 05 by Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2001, 2002, 2003 by Liam Davies (ldavies@agile.tv)
 *
 */

/* Linux and architecture-specific declarations supplied by other files. */

extern "C" {
    static mut cobalt_board_id: ::core::ffi::c_int;
    static mut fw_arg0: ::core::ffi::c_ulong;
    static mut fw_arg1: ::core::ffi::c_ulong;
    static mut arcs_cmdline: ::core::ffi::c_char;
    static mut ioport_resource: resource;
    static mut _machine_restart: ::core::option::Option<unsafe extern "C" fn()>;
    static mut _machine_halt: ::core::option::Option<unsafe extern "C" fn()>;
    static mut pm_power_off: ::core::option::Option<unsafe extern "C" fn()>;

    fn cobalt_machine_restart();
    fn cobalt_machine_halt();
    fn set_io_port_base(base: usize);
    fn request_resource(parent: *mut resource, new: *mut resource) -> ::core::ffi::c_int;
    fn strlcat(dst: *mut ::core::ffi::c_char,
               src: *const ::core::ffi::c_char,
               size: usize) -> usize;
    fn memblock_add(base: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn setup_8250_early_printk_port(base: usize, regshift: ::core::ffi::c_int,
                                    iotype: ::core::ffi::c_int);
}

#[repr(C)]
pub struct resource {
    pub start: ::core::ffi::c_ulong,
    pub end: ::core::ffi::c_ulong,
    pub name: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_ulong,
}

extern "C" {
    static COBALT_BRD_ID_QUBE1: ::core::ffi::c_int;
    static COBALT_BRD_ID_RAQ1: ::core::ffi::c_int;
    static COBALT_BRD_ID_QUBE2: ::core::ffi::c_int;
    static COBALT_BRD_ID_RAQ2: ::core::ffi::c_int;
    static IORESOURCE_BUSY: ::core::ffi::c_ulong;
    static IORESOURCE_IO: ::core::ffi::c_ulong;
}

const COMMAND_LINE_SIZE: usize = 4096;

pub unsafe extern "C" fn get_system_type() -> *const ::core::ffi::c_char {
    match cobalt_board_id {
        x if x == COBALT_BRD_ID_QUBE1 => b"Cobalt Qube\0".as_ptr() as *const _,
        x if x == COBALT_BRD_ID_RAQ1 => b"Cobalt RaQ\0".as_ptr() as *const _,
        x if x == COBALT_BRD_ID_QUBE2 => b"Cobalt Qube2\0".as_ptr() as *const _,
        x if x == COBALT_BRD_ID_RAQ2 => b"Cobalt RaQ2\0".as_ptr() as *const _,
        _ => b"MIPS Cobalt\0".as_ptr() as *const _,
    }
}

/*
 * Cobalt doesn't have PS/2 keyboard/mouse interfaces,
 * keyboard controller is never used.
 * Also PCI-ISA bridge DMA controller is never used.
 */
static mut cobalt_reserved_resources: [resource; 4] = [
    resource { start: 0x00, end: 0x1f, name: b"reserved\0".as_ptr() as *const _, flags: 0 },
    resource { start: 0x60, end: 0x6f, name: b"reserved\0".as_ptr() as *const _, flags: 0 },
    resource { start: 0x80, end: 0x8f, name: b"reserved\0".as_ptr() as *const _, flags: 0 },
    resource { start: 0xc0, end: 0xdf, name: b"reserved\0".as_ptr() as *const _, flags: 0 },
];

pub unsafe extern "C" fn plat_mem_setup() {
    _machine_restart = Some(cobalt_machine_restart);
    _machine_halt = Some(cobalt_machine_halt);
    pm_power_off = Some(cobalt_machine_halt);

    set_io_port_base((0x8000_0000usize | 0x1000_0000usize) + 0x1000_0000usize);

    /* I/O port resource */
    ioport_resource.end = 0x01ffffff;

    /* These resources have been reserved by VIA SuperI/O chip. */
    let mut i = 0;
    while i < cobalt_reserved_resources.len() {
        cobalt_reserved_resources[i].flags = IORESOURCE_BUSY | IORESOURCE_IO;
        request_resource(&mut ioport_resource, &mut cobalt_reserved_resources[i]);
        i += 1;
    }
}

/*
 * Prom init. We read our one and only communication with the firmware.
 * Grab the amount of installed memory.
 * Better boot loaders (CoLo) pass a command line too :-)
 */
pub unsafe extern "C" fn prom_init() {
    let memsz = fw_arg0 & 0x7fff0000;
    let argc = fw_arg0 & 0x0000ffff;
    let argv = fw_arg1 as *mut *mut ::core::ffi::c_char;

    let mut i = 1;
    while i < argc {
        strlcat(&mut arcs_cmdline, *argv.add(i as usize), COMMAND_LINE_SIZE);
        if i < argc - 1 {
            strlcat(&mut arcs_cmdline, b" \0".as_ptr() as *const _, COMMAND_LINE_SIZE);
        }
        i += 1;
    }

    memblock_add(0, memsz);

    setup_8250_early_printk_port(0x8010_0000, 0, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
