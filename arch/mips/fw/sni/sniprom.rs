/*
 * Big Endian PROM code for SNI RM machines
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005-2006 Florian Lohoff (flo@rfc822.org)
 * Copyright (C) 2005-2006 Thomas Bogendoerfer (tsbogend@alpha.franken.de)
 */

/* Dependencies are supplied by the surrounding kernel translation unit. */

/* special SNI prom calls */
/*
 * This does not exist in all proms - SINIX compares
 * the prom env variable "version" against "2.0008"
 * or greater. If lesser it tries to probe interesting
 * registers
 */
const PROM_GET_MEMCONF: usize = 58;
const PROM_GET_HWCONF: usize = 61;

const PROM_VEC: *mut u64 = CKSEG1ADDR(0x1fc00000) as *mut u64;

#[inline]
unsafe fn prom_entry(x: usize) -> *mut u64 {
    PROM_VEC.add(x)
}

const ___PROM_PUTCHAR: usize = PROM_PUTCHAR;
const ___PROM_GETENV: usize = PROM_GETENV;

#[cfg(target_pointer_width = "64")]
static mut O32_STK: [u64; 4096] = [0; 4096];

#[cfg(target_pointer_width = "64")]
#[inline]
unsafe fn o32_stk() -> *mut u64 {
    O32_STK.as_mut_ptr().add(O32_STK.len())
}

unsafe fn prom_putchar_call(f: unsafe extern "C" fn(i32) -> *mut i32, x: i32) -> *mut i32 {
    #[cfg(target_pointer_width = "64")]
    {
        return __prom_putchar(f, o32_stk(), x);
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        f(x)
    }
}

unsafe fn prom_getenv_call(f: unsafe extern "C" fn(*mut i8) -> *mut i8, x: *mut i8) -> *mut i8 {
    #[cfg(target_pointer_width = "64")]
    {
        return __prom_getenv(f, o32_stk(), x);
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        f(x)
    }
}

unsafe fn prom_get_memconf_call(f: unsafe extern "C" fn(*mut core::ffi::c_void), x: *mut core::ffi::c_void) {
    #[cfg(target_pointer_width = "64")]
    {
        __prom_get_memconf(f, o32_stk(), x);
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        f(x);
    }
}

unsafe fn prom_get_hwconf_call(f: unsafe extern "C" fn() -> u32) -> u32 {
    #[cfg(target_pointer_width = "64")]
    {
        return __prom_get_hwconf(f, o32_stk());
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        f()
    }
}

pub unsafe fn prom_putchar(c: u8) {
    let f = core::mem::transmute(prom_entry(PROM_PUTCHAR));
    prom_putchar_call(f, c as i32);
}

pub unsafe fn prom_getenv(s: *mut i8) -> *mut i8 {
    let f = core::mem::transmute(prom_entry(PROM_GETENV));
    prom_getenv_call(f, s)
}

pub unsafe fn prom_get_hwconf() -> *mut core::ffi::c_void {
    let f = core::mem::transmute(prom_entry(PROM_GET_HWCONF));
    let hwconf = prom_get_hwconf_call(f);

    if hwconf == 0xffff_ffff {
        core::ptr::null_mut()
    } else {
        CKSEG1ADDR(hwconf) as *mut core::ffi::c_void
    }
}

/*
 * /proc/cpuinfo system type
 *
 */
pub static mut system_type: *mut i8 = b"Unknown\0".as_ptr() as *mut i8;

pub unsafe fn get_system_type() -> *const i8 {
    system_type
}

unsafe fn sni_mem_init() {
    let mut memsize: i32;
    #[repr(C)]
    struct Membank {
        size: u32,
        base: u32,
        size2: u32,
        pad1: u32,
        pad2: u32,
    }
    let mut memconf: [Membank; 8] = core::mem::zeroed();
    let brd_type = *(SNI_IDPROM_BRDTYPE as *const u8);

    /* MemSIZE from prom in 16MByte chunks */
    memsize = (*(SNI_IDPROM_MEMSIZE as *const u8) as i32) * 16;

    pr_debug!("IDProm memsize: %u MByte\n", memsize);

    /* get memory bank layout from prom */
    let f = core::mem::transmute(prom_entry(PROM_GET_MEMCONF));
    prom_get_memconf_call(f, memconf.as_mut_ptr() as *mut core::ffi::c_void);

    pr_debug!("prom_get_mem_conf memory configuration:\n");
    let mut i = 0usize;
    while i < 8 && memconf[i].size != 0 {
        if brd_type == SNI_BRD_PCI_TOWER || brd_type == SNI_BRD_PCI_TOWER_CPLUS {
            if memconf[i].base >= 0x2000_0000 && memconf[i].base < 0x3000_0000 {
                memconf[i].base -= 0x2000_0000;
            }
        }
        pr_debug!("Bank%d: %08x @ %08x\n", i, memconf[i].size, memconf[i].base);
        memblock_add(memconf[i].base as u64, memconf[i].size as u64);
        i += 1;
    }
}

pub unsafe fn prom_init() {
    let argc = fw_arg0;
    let argv = CKSEG0ADDR(fw_arg1) as *mut u32;

    sni_mem_init();

    /* copy prom cmdline parameters to kernel cmdline */
    let mut i = 1;
    while i < argc {
        strcat(arcs_cmdline, CKSEG0ADDR(*argv.add(i)) as *const i8);
        if i < argc - 1 {
            strcat(arcs_cmdline, b" \0".as_ptr() as *const i8);
        }
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
