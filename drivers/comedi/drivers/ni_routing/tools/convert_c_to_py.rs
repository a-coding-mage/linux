// SPDX-License-Identifier: GPL-2.0+

use std::ffi::CStr;
use std::fs::File;
use std::io::{self, Write};

pub type u8 = u8;
pub type u16 = u16;
pub type s8 = i8;

pub const NI_ROUTE_VALUE_EXTERNAL_CONVERSION: i32 = 1;

// Dependencies supplied by the included C sources:
// ../ni_route_values.c, ../ni_device_routes.c, all_cfiles.c

pub type register_type = u32;

#[repr(C)]
pub struct family_route_values {
    pub family: *const std::os::raw::c_char,
    pub register_values: *const *const register_type,
}

#[repr(C)]
pub struct ni_route {
    pub dest: u32,
    pub src: *const u32,
}

#[repr(C)]
pub struct ni_device_routes {
    pub device: *const std::os::raw::c_char,
    pub routes: *const ni_route,
}

extern "C" {
    pub static ni_all_route_values: *const *const family_route_values;
    pub static ni_device_routes_list: *const *const ni_device_routes;
}

// These constants and marking helpers are supplied by the included sources.
extern "C" {
    static NI_NAMES_BASE: u32;
    static NI_NUM_NAMES: u32;
}

extern "C" {
    fn B(value: u32) -> u32;
    fn MARKED_V(value: register_type) -> bool;
    fn MARKED_I(value: register_type) -> bool;
    fn MARKED_U(value: register_type) -> bool;
    fn UNMARK(value: register_type) -> register_type;
}

unsafe fn rvij(rv: *const family_route_values, src: u32, dest: u32) -> register_type {
    *(*(*rv).register_values.add(dest as usize)).add(src as usize)
}

unsafe fn c_str(value: *const std::os::raw::c_char) -> String {
    CStr::from_ptr(value).to_string_lossy().into_owned()
}

pub unsafe fn family_write(rv: *const family_route_values, fp: &mut File) -> io::Result<()> {
    writeln!(fp, "  \"{}\" : {{", c_str((*rv).family))?;
    writeln!(fp, "    # dest -> {{src0:val0, src1:val1, ...}}")?;

    let base = NI_NAMES_BASE;
    let limit = base + NI_NUM_NAMES;
    let mut dest = base;
    while dest < limit {
        let mut src = base;
        while src < limit && rvij(rv, B(src), B(dest)) == 0 {
            src += 1;
        }
        if src >= limit {
            dest += 1;
            continue;
        }

        writeln!(fp, "    {} : {{", dest)?;
        src = base;
        while src < limit {
            let r = rvij(rv, B(src), B(dest));
            if r != 0 {
                let m = if MARKED_V(r) {
                    "V"
                } else if MARKED_I(r) {
                    "I"
                } else if MARKED_U(r) {
                    "U"
                } else {
                    eprintln!(
                        "Invalid register marking {}[{}][{}] = {}",
                        c_str((*rv).family), dest, src, r
                    );
                    std::process::exit(1);
                };
                writeln!(fp, "      {} : \"{}({})\",", src, m, UNMARK(r))?;
            }
            src += 1;
        }
        writeln!(fp, "    }},")?;
        dest += 1;
    }
    writeln!(fp, "  }},")?;
    writeln!(fp)?;
    Ok(())
}

pub fn is_valid_ni_sig(sig: u32) -> bool {
    unsafe { sig >= NI_NAMES_BASE && sig < NI_NAMES_BASE + NI_NUM_NAMES }
}

pub unsafe fn device_write(d_r: *const ni_device_routes, fp: &mut File) -> io::Result<()> {
    writeln!(fp, "  \"{}\" : {{", c_str((*d_r).device))?;
    writeln!(fp, "    # dest -> [src0, src1, ...]")?;

    let mut i = 0usize;
    while (*(*d_r).routes.add(i)).dest != 0 {
        let route = (*d_r).routes.add(i);
        if !is_valid_ni_sig((*route).dest) {
            eprintln!("Invalid NI signal value [{}] for destination {}.[{}]", (*route).dest, c_str((*d_r).device), i);
            std::process::exit(1);
        }
        write!(fp, "    {} : [", (*route).dest)?;
        let mut j = 0usize;
        while *(*route).src.add(j) != 0 {
            let source = *(*route).src.add(j);
            if !is_valid_ni_sig(source) {
                eprintln!("Invalid NI signal value [{}] for source {}.[{}].[{}]", source, c_str((*d_r).device), i, j);
                std::process::exit(1);
            }
            write!(fp, "{},", source)?;
            j += 1;
        }
        writeln!(fp, "],")?;
        i += 1;
    }
    writeln!(fp, "  }},")?;
    writeln!(fp)?;
    Ok(())
}

pub unsafe fn main() -> i32 {
    let mut fp = match File::create("ni_values.py") {
        Ok(file) => file,
        Err(_) => {
            eprint!("Could not open file!");
            return -1;
        }
    };

    writeln!(fp, "ni_route_values = {{").unwrap();
    let mut i = 0usize;
    while !(*ni_all_route_values.add(i)).is_null() {
        family_write(*ni_all_route_values.add(i), &mut fp).unwrap();
        i += 1;
    }
    writeln!(fp, "}}\n").unwrap();

    writeln!(fp, "ni_device_routes = {{").unwrap();
    i = 0;
    while !(*ni_device_routes_list.add(i)).is_null() {
        device_write(*ni_device_routes_list.add(i), &mut fp).unwrap();
        i += 1;
    }
    writeln!(fp, "}}").unwrap();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
