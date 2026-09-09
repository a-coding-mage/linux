// SPDX-License-Identifier: GPL-2.0+

// pr_fmt(fmt) = "ipmi_hardcode: " fmt
// Kernel module parameter declarations and descriptions are preserved here as
// comments; their registration is supplied by the surrounding kernel module.

use core::ffi::{c_char, c_int, c_ulong};

const SI_MAX_PARMS: usize = 4;
const MAX_SI_TYPE_STR: usize = 30;

static mut SI_TYPE_STR: [c_char; MAX_SI_TYPE_STR] = [0; MAX_SI_TYPE_STR];
static mut ADDRS: [c_ulong; SI_MAX_PARMS] = [0; SI_MAX_PARMS];
static mut NUM_ADDRS: u32 = 0;
static mut PORTS: [u32; SI_MAX_PARMS] = [0; SI_MAX_PARMS];
static mut NUM_PORTS: u32 = 0;
static mut IRQS: [c_int; SI_MAX_PARMS] = [0; SI_MAX_PARMS];
static mut NUM_IRQS: u32 = 0;
static mut REGSPACINGS: [c_int; SI_MAX_PARMS] = [0; SI_MAX_PARMS];
static mut NUM_REGSPACINGS: u32 = 0;
static mut REGSIZES: [c_int; SI_MAX_PARMS] = [0; SI_MAX_PARMS];
static mut NUM_REGSIZES: u32 = 0;
static mut REGSHIFTS: [c_int; SI_MAX_PARMS] = [0; SI_MAX_PARMS];
static mut NUM_REGSHIFTS: u32 = 0;
static mut SLAVE_ADDRS: [c_int; SI_MAX_PARMS] = [0; SI_MAX_PARMS];
static mut NUM_SLAVE_ADDRS: u32 = 0;

// module_param_string(type, si_type_str, MAX_SI_TYPE_STR, 0);
// module_param_hw_array(addrs, ulong, iomem, &num_addrs, 0);
// module_param_hw_array(ports, uint, ioport, &num_ports, 0);
// module_param_hw_array(irqs, int, irq, &num_irqs, 0);
// module_param_hw_array(regspacings, int, other, &num_regspacings, 0);
// module_param_hw_array(regsizes, int, other, &num_regsizes, 0);
// module_param_hw_array(regshifts, int, other, &num_regshifts, 0);
// module_param_hw_array(slave_addrs, int, other, &num_slave_addrs, 0);

unsafe fn ipmi_hardcode_init_one(
    si_type_str: *const c_char,
    i: u32,
    addr: c_ulong,
    addr_space: crate::ipmi_addr_space,
) {
    let mut p: crate::ipmi_plat_data = core::mem::zeroed();
    let mut t: c_int;

    p.iftype = crate::IPMI_PLAT_IF_SI;
    if si_type_str.is_null() || *si_type_str == 0 {
        p.type_ = crate::SI_KCS;
    } else {
        t = crate::match_string(crate::si_to_str, -1, si_type_str);
        if t < 0 {
            crate::pr_warn!(
                "Interface type specified for interface {}, was invalid: {}\n",
                i,
                si_type_str
            );
            return;
        }
        p.type_ = t;
    }

    p.regsize = REGSIZES[i as usize];
    p.regspacing = REGSPACINGS[i as usize];
    p.slave_addr = SLAVE_ADDRS[i as usize];
    p.addr_source = crate::SI_HARDCODED;
    p.regshift = REGSHIFTS[i as usize];
    p.addr = addr;
    p.space = addr_space;

    crate::ipmi_platform_add(b"hardcode-ipmi-si\0".as_ptr() as *const c_char, i, &p);
}

pub unsafe fn ipmi_hardcode_init() {
    let mut i: u32;
    let mut si_type: [*mut c_char; SI_MAX_PARMS] = [core::ptr::null_mut(); SI_MAX_PARMS];

    let mut str_ptr = SI_TYPE_STR.as_mut_ptr();
    if *str_ptr != 0 {
        i = 0;
        while i < SI_MAX_PARMS as u32 && *str_ptr != 0 {
            si_type[i as usize] = str_ptr;
            str_ptr = crate::strchr(str_ptr, b',' as c_int);
            if !str_ptr.is_null() {
                *str_ptr = 0;
                str_ptr = str_ptr.add(1);
            } else {
                break;
            }
            i += 1;
        }
    }

    i = 0;
    while i < SI_MAX_PARMS as u32 {
        if i < NUM_PORTS && PORTS[i as usize] != 0 {
            ipmi_hardcode_init_one(
                si_type[i as usize],
                i,
                PORTS[i as usize] as c_ulong,
                crate::IPMI_IO_ADDR_SPACE,
            );
        }
        if i < NUM_ADDRS && ADDRS[i as usize] != 0 {
            ipmi_hardcode_init_one(
                si_type[i as usize],
                i,
                ADDRS[i as usize],
                crate::IPMI_MEM_ADDR_SPACE,
            );
        }
        i += 1;
    }
}

pub unsafe fn ipmi_si_hardcode_exit() {
    crate::ipmi_remove_platform_device_by_name(b"hardcode-ipmi-si\0".as_ptr() as *const c_char);
}

// Returns true if the given address exists as a hardcoded address, false if not.
pub unsafe fn ipmi_si_hardcode_match(addr_space: c_int, addr: c_ulong) -> c_int {
    let mut i = 0;
    if addr_space == crate::IPMI_IO_ADDR_SPACE {
        while i < NUM_PORTS {
            if PORTS[i as usize] as c_ulong == addr {
                return 1;
            }
            i += 1;
        }
    } else {
        while i < NUM_ADDRS {
            if ADDRS[i as usize] == addr {
                return 1;
            }
            i += 1;
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
