// SPDX-License-Identifier: GPL-2.0+

// Dependency declarations supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn inb(addr: c_uint) -> u8;
    fn outb(value: u8, addr: c_uint);
    fn inw(addr: c_uint) -> u32;
    fn outw(value: c_uint, addr: c_uint);
    fn inl(addr: c_uint) -> u32;
    fn outl(value: c_uint, addr: c_uint);
    fn release_region(addr: c_uint, size: c_uint);
    fn request_region(addr: c_uint, size: c_uint, name: *const c_char) -> *mut c_void;
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
}

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const SI_DEVICE_NAME: &[u8] = b"ipmi_si\0";

#[repr(C)]
pub struct si_sm_io {
    pub addr_data: c_uint,
    pub regspacing: c_uint,
    pub regshift: c_uint,
    pub regsize: c_uint,
    pub io_size: c_int,
    pub dev: *mut c_void,
    pub inputb: Option<unsafe extern "C" fn(*const si_sm_io, c_uint) -> u8>,
    pub outputb: Option<unsafe extern "C" fn(*const si_sm_io, c_uint, u8)>,
    pub io_cleanup: Option<unsafe extern "C" fn(*mut si_sm_io)>,
}

unsafe extern "C" fn port_inb(io: *const si_sm_io, offset: c_uint) -> u8 {
    let addr = (*io).addr_data;
    inb(addr.wrapping_add(offset.wrapping_mul((*io).regspacing)))
}

unsafe extern "C" fn port_outb(io: *const si_sm_io, offset: c_uint, b: u8) {
    let addr = (*io).addr_data;
    outb(b, addr.wrapping_add(offset.wrapping_mul((*io).regspacing)));
}

unsafe extern "C" fn port_inw(io: *const si_sm_io, offset: c_uint) -> u8 {
    let addr = (*io).addr_data;
    (inw(addr.wrapping_add(offset.wrapping_mul((*io).regspacing))) >> (*io).regshift) as u8
}

unsafe extern "C" fn port_outw(io: *const si_sm_io, offset: c_uint, b: u8) {
    let addr = (*io).addr_data;
    outw((b as c_uint) << (*io).regshift,
         addr.wrapping_add(offset.wrapping_mul((*io).regspacing)));
}

unsafe extern "C" fn port_inl(io: *const si_sm_io, offset: c_uint) -> u8 {
    let addr = (*io).addr_data;
    (inl(addr.wrapping_add(offset.wrapping_mul((*io).regspacing))) >> (*io).regshift) as u8
}

unsafe extern "C" fn port_outl(io: *const si_sm_io, offset: c_uint, b: u8) {
    let addr = (*io).addr_data;
    outl((b as c_uint) << (*io).regshift,
         addr.wrapping_add(offset.wrapping_mul((*io).regspacing)));
}

unsafe extern "C" fn port_cleanup(io: *mut si_sm_io) {
    let addr = (*io).addr_data;
    let mut idx: c_int;

    if addr != 0 {
        idx = 0;
        while idx < (*io).io_size {
            release_region(addr.wrapping_add((idx as c_uint).wrapping_mul((*io).regspacing)),
                           (*io).regsize);
            idx += 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ipmi_si_port_setup(io: *mut si_sm_io) -> c_int {
    let addr = (*io).addr_data;
    let mut idx: c_int;

    if addr == 0 {
        return -ENODEV;
    }

    /*
     * Figure out the actual inb/inw/inl/etc routine to use based
     * upon the register size.
     */
    match (*io).regsize {
        1 => {
            (*io).inputb = Some(port_inb);
            (*io).outputb = Some(port_outb);
        }
        2 => {
            (*io).inputb = Some(port_inw);
            (*io).outputb = Some(port_outw);
        }
        4 => {
            (*io).inputb = Some(port_inl);
            (*io).outputb = Some(port_outl);
        }
        _ => {
            dev_warn((*io).dev, b"Invalid register size: %d\n\0".as_ptr() as *const c_char,
                     (*io).regsize as c_int);
            return -EINVAL;
        }
    }

    /*
     * Some BIOSes reserve disjoint I/O regions in their ACPI
     * tables.  This causes problems when trying to register the
     * entire I/O region.  Therefore we must register each I/O
     * port separately.
     */
    idx = 0;
    while idx < (*io).io_size {
        if request_region(addr.wrapping_add((idx as c_uint).wrapping_mul((*io).regspacing)),
                          (*io).regsize, SI_DEVICE_NAME.as_ptr() as *const c_char).is_null() {
            /* Undo allocations */
            while idx != 0 {
                idx -= 1;
                release_region(addr.wrapping_add((idx as c_uint).wrapping_mul((*io).regspacing)),
                               (*io).regsize);
            }
            return -EIO;
        }
        idx += 1;
    }

    (*io).io_cleanup = Some(port_cleanup);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
