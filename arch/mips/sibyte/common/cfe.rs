// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000, 2001, 2002, 2003 Broadcom Corporation
 */

/* Linux and architecture headers from the original source are external dependencies. */

/* Max ram addressable in 32-bit segments */
#[cfg(feature = "64bit")]
const MAX_RAM_SIZE: u64 = !0u64;
#[cfg(all(not(feature = "64bit"), feature = "highmem", feature = "phys_addr_t_64bit"))]
const MAX_RAM_SIZE: u64 = !0u64;
#[cfg(all(not(feature = "64bit"), feature = "highmem", not(feature = "phys_addr_t_64bit")))]
const MAX_RAM_SIZE: u64 = 0xffff_ffffu64;
#[cfg(all(not(feature = "64bit"), not(feature = "highmem")))]
const MAX_RAM_SIZE: u64 = 0x1fff_ffffu64;

pub static mut cfe_cons_handle: i32 = 0;

#[cfg(feature = "blk_dev_initrd")]
extern "C" {
    static mut initrd_start: libc::c_ulong;
    static mut initrd_end: libc::c_ulong;
}

extern "C" {
    fn smp_processor_id() -> i32;
    fn smp_call_function(func: unsafe extern "C" fn(*mut libc::c_void), info: *mut libc::c_void, wait: i32);
    fn printk(fmt: *const libc::c_char, ...);
    fn cfe_exit(warm: i32, status: i32) -> !;
    fn cfe_enummem(idx: u32, flags: i32, addr: *mut u64, size: *mut u64, mem_type: *mut u64) -> i32;
    fn memblock_add(base: u64, size: u64);
    fn memblock_reserve(base: u64, size: u64);
    fn panic(fmt: *const libc::c_char) -> !;
    fn cfe_init(handle: u64, entry: u64);
    fn cfe_getstdhandle(handle: i32) -> i32;
    fn cfe_getenv(name: *const libc::c_char, value: *mut libc::c_char, length: i32) -> i32;
    fn cfe_write(handle: i32, buffer: *const libc::c_void, length: i32) -> i32;
    fn simple_strtoul(s: *const libc::c_char, endp: *mut *mut libc::c_char, base: u32) -> libc::c_ulong;
    fn strncmp(a: *const libc::c_char, b: *const libc::c_char, n: usize) -> i32;
}

unsafe extern "C" fn cfe_linux_exit(arg: *mut libc::c_void) -> ! {
    let warm = *(arg as *mut i32);

    if smp_processor_id() != 0 {
        static mut reboot_smp: i32 = 0;
        if reboot_smp == 0 {
            reboot_smp = 1;
            smp_call_function(cfe_linux_exit, arg, 0);
        }
    } else {
        printk(b"Passing control back to CFE...\0".as_ptr() as *const libc::c_char);
        cfe_exit(warm, 0);
        printk(b"cfe_exit returned??\n\0".as_ptr() as *const libc::c_char);
    }
    loop {}
}

unsafe extern "C" fn cfe_linux_restart(_command: *mut libc::c_char) -> ! {
    static zero: i32 = 0;
    cfe_linux_exit((&zero as *const i32) as *mut libc::c_void)
}

unsafe extern "C" fn cfe_linux_halt() -> ! {
    static one: i32 = 1;
    cfe_linux_exit((&one as *const i32) as *mut libc::c_void)
}

unsafe fn prom_meminit() {
    let mut addr: u64;
    let mut size: u64;
    let mut mem_type: u64;
    let mem_flags: i32 = 0;
    let mut idx: u32 = 0;
    let mut rd_flag: i32;

    #[cfg(feature = "blk_dev_initrd")]
    let (initrd_pstart, initrd_pend) = {
        let start = CPHYSADDR(initrd_start);
        let end = CPHYSADDR(initrd_end);
        if initrd_start != 0 && (start > MAX_RAM_SIZE || end > MAX_RAM_SIZE) {
            panic(b"initrd out of addressable memory\0".as_ptr() as *const libc::c_char);
        }
        (start, end)
    };

    while cfe_enummem(idx, mem_flags, &mut addr, &mut size, &mut mem_type) != CFE_ERR_NOMORE {
        rd_flag = 0;
        if mem_type == CFE_MI_AVAILABLE {
            #[cfg(feature = "blk_dev_initrd")]
            if initrd_start != 0 {
                if initrd_pstart > addr && initrd_pstart < addr.wrapping_add(size) {
                    memblock_add(addr, initrd_pstart.wrapping_sub(addr));
                    rd_flag = 1;
                }
                if initrd_pend > addr && initrd_pend < addr.wrapping_add(size) {
                    memblock_add(initrd_pend, addr.wrapping_add(size).wrapping_sub(initrd_pend));
                    rd_flag = 1;
                }
            }
            if rd_flag == 0 {
                if addr > MAX_RAM_SIZE { idx = idx.wrapping_add(1); continue; }
                if addr.wrapping_add(size) > MAX_RAM_SIZE {
                    size = MAX_RAM_SIZE.wrapping_sub(addr.wrapping_add(size)).wrapping_add(1);
                }
                if size > 512 { size = size.wrapping_sub(512); }
                memblock_add(addr, size);
            }
        }
        idx = idx.wrapping_add(1);
    }
    #[cfg(feature = "blk_dev_initrd")]
    if initrd_start != 0 {
        memblock_add(initrd_pstart, initrd_pend.wrapping_sub(initrd_pstart));
        memblock_reserve(initrd_pstart, initrd_pend.wrapping_sub(initrd_pstart));
    }
}

#[cfg(feature = "blk_dev_initrd")]
unsafe fn initrd_setup(mut str_: *mut libc::c_char) -> i32 {
    let mut rdarg = [0 as libc::c_char; 64];
    let mut idx = 0usize;
    let mut endptr: *mut libc::c_char = core::ptr::null_mut();
    let mut initrd_size: libc::c_ulong;
    while idx < rdarg.len() - 1 {
        let ch = *str_.add(idx);
        if ch == 0 || ch == b' ' as libc::c_char { break; }
        rdarg[idx] = ch;
        idx += 1;
    }
    rdarg[idx] = 0;
    str_ = rdarg.as_mut_ptr();
    let mut tmp = str_;
    while *tmp != b'@' as libc::c_char {
        if *tmp == 0 { goto_fail!(); }
        tmp = tmp.add(1);
    }
    *tmp = 0; tmp = tmp.add(1);
    if *tmp == 0 { goto_fail!(); }
    initrd_size = simple_strtoul(str_, &mut endptr, 16);
    if !(*endptr == 0) { *tmp.sub(1) = b'@' as libc::c_char; goto_fail!(); }
    *tmp.sub(1) = b'@' as libc::c_char;
    initrd_start = simple_strtoul(tmp, &mut endptr, 16);
    if !(*endptr == 0) { goto_fail!(); }
    initrd_end = initrd_start.wrapping_add(initrd_size);
    printk(b"Found initrd of %lx@%lx\n\0".as_ptr() as *const libc::c_char, initrd_size, initrd_start);
    return 1;
    macro_rules! goto_fail { () => {{ printk(b"Bad initrd argument.  Disabling initrd\n\0".as_ptr() as *const libc::c_char); initrd_start = 0; initrd_end = 0; return 1; }} }
}

pub unsafe fn prom_putchar(c: libc::c_char) {
    while cfe_write(cfe_cons_handle, &c as *const libc::c_char as *const libc::c_void, 1) == 0 {}
}

extern "C" {
    static mut fw_arg0: i32;
    static mut fw_arg2: *mut libc::c_void;
    static mut fw_arg3: *mut libc::c_void;
    static mut arcs_cmdline: [libc::c_char; COMMAND_LINE_SIZE as usize];
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut libc::c_char) -> !>;
    static mut _machine_halt: Option<unsafe extern "C" fn() -> !>;
    static mut pm_power_off: Option<unsafe extern "C" fn() -> !>;
    fn register_smp_ops(ops: *const plat_smp_ops);
}

#[repr(C)]
pub struct plat_smp_ops { _private: [u8; 0] }
extern "C" {
    static sb_smp_ops: plat_smp_ops;
    static bcm1480_smp_ops: plat_smp_ops;
}

pub unsafe extern "C" fn prom_init() {
    let mut cfe_ept: u64;
    let mut cfe_handle: u64;
    let cfe_eptseal: u32;
    let argc = fw_arg0;
    let envp = fw_arg2 as *mut *mut libc::c_char;
    let prom_vec = fw_arg3 as *mut i32;

    _machine_restart = Some(cfe_linux_restart);
    _machine_halt = Some(cfe_linux_halt);
    pm_power_off = Some(cfe_linux_halt);

    if argc < 0 {
        cfe_handle = argc as i64 as u64;
        cfe_ept = envp as u64;
        cfe_eptseal = prom_vec as u32;
    } else if *prom_vec < 0 {
        cfe_handle = *prom_vec as i64 as u64;
        cfe_ept = 0x9fc0_0500u64;
        cfe_eptseal = CFE_EPTSEAL;
    } else {
        cfe_handle = *prom_vec as i64 as u64;
        cfe_ept = *prom_vec.add(2) as i64 as u64;
        cfe_eptseal = *(prom_vec as *mut u32).add(3);
    }
    if cfe_eptseal != CFE_EPTSEAL {
        printk(b"CFE's entrypoint seal doesn't match. Spinning.\0".as_ptr() as *const libc::c_char);
        loop {}
    }
    cfe_init(cfe_handle, cfe_ept);
    cfe_cons_handle = cfe_getstdhandle(CFE_STDHANDLE_CONSOLE);
    if cfe_getenv(b"LINUX_CMDLINE\0".as_ptr() as *const libc::c_char, arcs_cmdline.as_mut_ptr(), COMMAND_LINE_SIZE) < 0 && argc >= 0 {
        printk(b"LINUX_CMDLINE not defined in cfe.\0".as_ptr() as *const libc::c_char);
        loop {}
    }
    #[cfg(feature = "blk_dev_initrd")]
    {
        let mut ptr = arcs_cmdline.as_mut_ptr();
        while *ptr != 0 {
            while *ptr == b' ' as libc::c_char { ptr = ptr.add(1); }
            if strncmp(ptr, b"initrd=\0".as_ptr() as *const libc::c_char, 7) == 0 {
                initrd_setup(ptr.add(7));
                break;
            }
            while *ptr != 0 && *ptr != b' ' as libc::c_char { ptr = ptr.add(1); }
        }
    }
    arcs_cmdline[COMMAND_LINE_SIZE as usize - 1] = 0;
    prom_meminit();
    #[cfg(any(feature = "sibyte_bcm112x", feature = "sibyte_sb1250"))]
    register_smp_ops(&sb_smp_ops);
    #[cfg(feature = "sibyte_bcm1x80")]
    register_smp_ops(&bcm1480_smp_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
