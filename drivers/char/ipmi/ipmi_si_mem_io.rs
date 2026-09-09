// SPDX-License-Identifier: GPL-2.0+

// Dependencies supplied by the surrounding translation unit/kernel bindings.

use core::ffi::c_void;

#[repr(C)]
pub struct si_sm_io {
    pub addr: *mut c_void,
    pub addr_data: usize,
    pub regspacing: usize,
    pub regshift: u32,
    pub regsize: i32,
    pub io_size: i32,
    pub dev: *mut c_void,
    pub inputb: Option<unsafe extern "C" fn(*const si_sm_io, u32) -> u8>,
    pub outputb: Option<unsafe extern "C" fn(*const si_sm_io, u32, u8)>,
    pub io_cleanup: Option<unsafe extern "C" fn(*mut si_sm_io)>,
}

extern "C" {
    fn readb(addr: *mut c_void) -> u8;
    fn readw(addr: *mut c_void) -> u16;
    fn readl(addr: *mut c_void) -> u32;
    fn writeb(value: u8, addr: *mut c_void);
    fn writel(value: u32, addr: *mut c_void);
    fn iounmap(addr: *mut c_void);
    fn ioremap(addr: usize, size: i32) -> *mut c_void;
    fn release_mem_region(addr: usize, size: i32);
    fn request_mem_region(addr: usize, size: i32, name: *const u8) -> *mut c_void;
    fn dev_warn(dev: *mut c_void, format: *const u8, ...);
}

const ENODEV: i32 = 19;
const EINVAL: i32 = 22;
const EIO: i32 = 5;
const SI_DEVICE_NAME: &[u8] = b"ipmi_si\0";

unsafe extern "C" fn intf_mem_inb(io: *const si_sm_io, offset: u32) -> u8 {
    let io = &*io;
    readb((io.addr as usize + offset as usize * io.regspacing) as *mut c_void)
}

unsafe extern "C" fn intf_mem_outb(io: *const si_sm_io, offset: u32, b: u8) {
    let io = &*io;
    writeb(b, (io.addr as usize + offset as usize * io.regspacing) as *mut c_void);
}

unsafe extern "C" fn intf_mem_inw(io: *const si_sm_io, offset: u32) -> u8 {
    let io = &*io;
    ((readw((io.addr as usize + offset as usize * io.regspacing) as *mut c_void)
        >> io.regshift) & 0xff) as u8
}

unsafe extern "C" fn intf_mem_outw(io: *const si_sm_io, offset: u32, b: u8) {
    let io = &*io;
    writeb(b.wrapping_shl(io.regshift),
        (io.addr as usize + offset as usize * io.regspacing) as *mut c_void);
}

unsafe extern "C" fn intf_mem_inl(io: *const si_sm_io, offset: u32) -> u8 {
    let io = &*io;
    ((readl((io.addr as usize + offset as usize * io.regspacing) as *mut c_void)
        >> io.regshift) & 0xff) as u8
}

unsafe extern "C" fn intf_mem_outl(io: *const si_sm_io, offset: u32, b: u8) {
    let io = &*io;
    writel((b as u32).wrapping_shl(io.regshift),
        (io.addr as usize + offset as usize * io.regspacing) as *mut c_void);
}

unsafe extern "C" fn mem_region_cleanup(io: *mut si_sm_io, num: i32) {
    let io = &*io;
    let addr = io.addr_data;
    let mut idx = 0;
    while idx < num {
        release_mem_region(addr + idx as usize * io.regspacing, io.regsize);
        idx += 1;
    }
}

unsafe extern "C" fn mem_cleanup(io: *mut si_sm_io) {
    if !(*io).addr.is_null() {
        iounmap((*io).addr);
        mem_region_cleanup(io, (*io).io_size);
    }
}

pub unsafe extern "C" fn ipmi_si_mem_setup(io: *mut si_sm_io) -> i32 {
    let addr = (*io).addr_data;
    let mut mapsize: i32;
    let mut idx: i32;

    if addr == 0 {
        return -ENODEV;
    }

    match (*io).regsize {
        1 => {
            (*io).inputb = Some(intf_mem_inb);
            (*io).outputb = Some(intf_mem_outb);
        }
        2 => {
            (*io).inputb = Some(intf_mem_inw);
            (*io).outputb = Some(intf_mem_outw);
        }
        4 => {
            (*io).inputb = Some(intf_mem_inl);
            (*io).outputb = Some(intf_mem_outl);
        }
        // #ifdef readq: 64-bit MMIO access is enabled by the target bindings.
        _ => {
            dev_warn((*io).dev, b"Invalid register size: %d\n\0".as_ptr(), (*io).regsize);
            return -EINVAL;
        }
    }

    idx = 0;
    while idx < (*io).io_size {
        if request_mem_region(addr + idx as usize * (*io).regspacing,
                              (*io).regsize, SI_DEVICE_NAME.as_ptr()).is_null() {
            mem_region_cleanup(io, idx);
            return -EIO;
        }
        idx += 1;
    }

    mapsize = (*io).io_size * (*io).regspacing as i32
        - ((*io).regspacing - (*io).regsize as usize) as i32;
    (*io).addr = ioremap(addr, mapsize);
    if (*io).addr.is_null() {
        mem_region_cleanup(io, (*io).io_size);
        return -EIO;
    }

    (*io).io_cleanup = Some(mem_cleanup);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
