// SPDX-License-Identifier: GPL-2.0
/* auxio.c: Probing for the Sparc AUXIO register at boot time.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 */

// Linux headers and symbols referenced below are supplied by other translated
// units and are intentionally not implemented here.

extern "C" {
    static mut sparc_cpu_model: sparc_cpu_model_t;
    static mut prom_root_node: phandle;

    fn prom_getchild(node: phandle) -> phandle;
    fn prom_searchsiblings(node: phandle, name: *const core::ffi::c_char) -> phandle;
    fn prom_printf(fmt: *const core::ffi::c_char, ...);
    fn prom_halt() -> !;
    fn prom_getproperty(
        node: phandle,
        name: *const core::ffi::c_char,
        value: *mut core::ffi::c_char,
        size: usize,
    ) -> i32;
    fn prom_apply_obio_ranges(regs: *mut linux_prom_registers, count: i32);
    fn of_ioremap(
        resource: *const resource,
        offset: usize,
        size: usize,
        name: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_void;
    fn sbus_readb(address: *mut core::ffi::c_void) -> u8;
    fn sbus_writeb(value: u8, address: *mut core::ffi::c_void);
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn panic(fmt: *const core::ffi::c_char) -> !;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
}

type phandle = u32;
type sparc_cpu_model_t = i32;

#[repr(C)]
#[derive(Copy, Clone)]
struct linux_prom_registers {
    which_io: u32,
    phys_addr: usize,
    reg_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct resource {
    start: usize,
    end: usize,
    flags: usize,
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

const sparc_leon: sparc_cpu_model_t = 0;
const sun4d: sparc_cpu_model_t = 1;
const sun4m: sparc_cpu_model_t = 2;
const AUXIO_LED: u8 = 0;
const AUXIO_ORMEIN4M: u8 = 0;
const KERN_INFO: &[u8] = b"\x01";

#[no_mangle]
pub static mut auxio_register: *mut core::ffi::c_void = core::ptr::null_mut();
static mut auxio_lock: spinlock_t = spinlock_t { _private: [] };

pub unsafe extern "C" fn auxio_probe() {
    let mut node: phandle;
    let mut auxio_nd: phandle;
    let mut auxregs = [linux_prom_registers { which_io: 0, phys_addr: 0, reg_size: 0 }; 1];
    let mut r = resource { start: 0, end: 0, flags: 0 };

    match sparc_cpu_model {
        sparc_leon | sun4d => return,
        _ => {}
    }
    node = prom_getchild(prom_root_node);
    auxio_nd = prom_searchsiblings(node, b"auxiliary-io\0".as_ptr() as *const _);
    if auxio_nd == 0 {
        node = prom_searchsiblings(node, b"obio\0".as_ptr() as *const _);
        node = prom_getchild(node);
        auxio_nd = prom_searchsiblings(node, b"auxio\0".as_ptr() as *const _);
        if auxio_nd == 0 {
            // CONFIG_PCI: auxio may be present on Ebus; otherwise continue with
            // the original non-PCI VME-chassis checks.
            if prom_searchsiblings(node, b"leds\0".as_ptr() as *const _) != 0 {
                return;
            }
            prom_printf(b"Cannot find auxio node, cannot continue...\n\0".as_ptr() as *const _);
            prom_halt();
        }
    }
    if prom_getproperty(auxio_nd, b"reg\0".as_ptr() as *const _, auxregs.as_mut_ptr() as *mut _, core::mem::size_of_val(&auxregs)) <= 0 {
        return;
    }
    prom_apply_obio_ranges(auxregs.as_mut_ptr(), 1);
    r.flags = (auxregs[0].which_io & 0xF) as usize;
    r.start = auxregs[0].phys_addr;
    r.end = auxregs[0].phys_addr + auxregs[0].reg_size - 1;
    auxio_register = of_ioremap(&r, 0, auxregs[0].reg_size, b"auxio\0".as_ptr() as *const _);
    if (auxregs[0].phys_addr as u64 & 3) == 3 {
        auxio_register = auxio_register.add(3 - (auxio_register as usize & 3));
    }
    set_auxio(AUXIO_LED, 0);
}

#[no_mangle]
pub unsafe extern "C" fn get_auxio() -> u8 {
    if !auxio_register.is_null() { sbus_readb(auxio_register) } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn set_auxio(bits_on: u8, bits_off: u8) {
    let mut regval: u8;
    let mut flags: usize = 0;
    spin_lock_irqsave(&raw mut auxio_lock, &mut flags);
    match sparc_cpu_model {
        sun4m => {
            if auxio_register.is_null() { spin_unlock_irqrestore(&raw mut auxio_lock, flags); return; }
            regval = sbus_readb(auxio_register);
            sbus_writeb((regval | bits_on) & !bits_off | AUXIO_ORMEIN4M, auxio_register);
        }
        sun4d => {}
        _ => panic(b"Can't set AUXIO register on this machine.\0".as_ptr() as *const _),
    }
    spin_unlock_irqrestore(&raw mut auxio_lock, flags);
}

// sun4m power control register (AUXIO2)
pub static mut auxio_power_register: *mut u8 = core::ptr::null_mut();

pub unsafe extern "C" fn auxio_power_probe() {
    let mut regs = linux_prom_registers { which_io: 0, phys_addr: 0, reg_size: 0 };
    let mut node = prom_getchild(prom_root_node);
    let mut r = resource { start: 0, end: 0, flags: 0 };
    node = prom_searchsiblings(node, b"obio\0".as_ptr() as *const _);
    node = prom_getchild(node);
    node = prom_searchsiblings(node, b"power\0".as_ptr() as *const _);
    if node == 0 || node as i32 == -1 { return; }
    if prom_getproperty(node, b"reg\0".as_ptr() as *const _, &mut regs as *mut _ as *mut _, core::mem::size_of_val(&regs)) <= 0 { return; }
    prom_apply_obio_ranges(&mut regs, 1);
    r.flags = (regs.which_io & 0xF) as usize;
    r.start = regs.phys_addr;
    r.end = regs.phys_addr + regs.reg_size - 1;
    auxio_power_register = of_ioremap(&r, 0, regs.reg_size, b"auxpower\0".as_ptr() as *const _) as *mut u8;
    if !auxio_power_register.is_null() { printk(b"\x01Power off control detected.\n\0".as_ptr() as *const _); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
