/*
 * CPU-version specific code
 *
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2006-2009 PetaLogix
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/string.h, linux/seq_file.h, linux/cpu.h,
// linux/initrd.h, linux/bug.h, asm/cpuinfo.h, linux/delay.h, linux/io.h,
// asm/page.h, linux/param.h, asm/pvr.h, asm/sections.h, asm/setup.h

unsafe fn show_cpuinfo(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let mut fpga_family: *const core::ffi::c_char = b"Unknown\0".as_ptr() as *const _;
    let mut cpu_ver: *const core::ffi::c_char = b"Unknown\0".as_ptr() as *const _;
    let mut i: i32;

    /* Denormalised to get the fpga family string */
    i = 0;
    while (*family_string_lookup.add(i as usize)).s != core::ptr::null() {
        if cpuinfo.fpga_family_code == (*family_string_lookup.add(i as usize)).k {
            fpga_family = (*family_string_lookup.add(i as usize)).s;
            break;
        }
        i += 1;
    }

    /* Denormalised to get the hw version string */
    i = 0;
    while (*cpu_ver_lookup.add(i as usize)).s != core::ptr::null() {
        if cpuinfo.ver_code == (*cpu_ver_lookup.add(i as usize)).k {
            cpu_ver = (*cpu_ver_lookup.add(i as usize)).s;
            break;
        }
        i += 1;
    }

    seq_printf(m,
        b"CPU-Family:\tMicroBlaze\nFPGA-Arch:\t%s\nCPU-Ver:\t%s, %s endian\nCPU-MHz:\t%d.%02d\nBogoMips:\t%lu.%02lu\n\0".as_ptr() as *const _,
        fpga_family, cpu_ver, if cpuinfo.endian { b"little\0".as_ptr() } else { b"big\0".as_ptr() },
        cpuinfo.cpu_clock_freq / 1000000, cpuinfo.cpu_clock_freq % 1000000,
        loops_per_jiffy / (500000 / HZ), (loops_per_jiffy / (5000 / HZ)) % 100);

    seq_printf(m, b"HW:\n Shift:\t\t%s\n MSR:\t\t%s\n PCMP:\t\t%s\n DIV:\t\t%s\n\0".as_ptr() as *const _,
        str_yes_no(cpuinfo.use_instr & PVR0_USE_BARREL_MASK),
        str_yes_no(cpuinfo.use_instr & PVR2_USE_MSR_INSTR),
        str_yes_no(cpuinfo.use_instr & PVR2_USE_PCMP_INSTR),
        str_yes_no(cpuinfo.use_instr & PVR0_USE_DIV_MASK));

    seq_printf(m, b" MMU:\t\t%x\n\0".as_ptr() as *const _, cpuinfo.mmu);
    seq_printf(m, b" MUL:\t\t%s\n FPU:\t\t%s\n\0".as_ptr() as *const _,
        if cpuinfo.use_mult & PVR2_USE_MUL64_MASK != 0 { b"v2\0".as_ptr() } else if cpuinfo.use_mult & PVR0_USE_HW_MUL_MASK != 0 { b"v1\0".as_ptr() } else { b"no\0".as_ptr() },
        if cpuinfo.use_fpu & PVR2_USE_FPU2_MASK != 0 { b"v2\0".as_ptr() } else if cpuinfo.use_fpu & PVR0_USE_FPU_MASK != 0 { b"v1\0".as_ptr() } else { b"no\0".as_ptr() });

    seq_printf(m, b" Exc:\t\t%s%s%s%s%s%s%s%s\n\0".as_ptr() as *const _,
        if cpuinfo.use_exc & PVR2_OPCODE_0x0_ILL_MASK != 0 { b"op0x0 \0".as_ptr() } else { b"\0".as_ptr() },
        if cpuinfo.use_exc & PVR2_UNALIGNED_EXC_MASK != 0 { b"unal \0".as_ptr() } else { b"\0".as_ptr() },
        if cpuinfo.use_exc & PVR2_ILL_OPCODE_EXC_MASK != 0 { b"ill \0".as_ptr() } else { b"\0".as_ptr() },
        if cpuinfo.use_exc & PVR2_IOPB_BUS_EXC_MASK != 0 { b"iopb \0".as_ptr() } else { b"\0".as_ptr() },
        if cpuinfo.use_exc & PVR2_DOPB_BUS_EXC_MASK != 0 { b"dopb \0".as_ptr() } else { b"\0".as_ptr() },
        if cpuinfo.use_exc & PVR2_DIV_ZERO_EXC_MASK != 0 { b"zero \0".as_ptr() } else { b"\0".as_ptr() },
        if cpuinfo.use_exc & PVR2_FPU_EXC_MASK != 0 { b"fpu \0".as_ptr() } else { b"\0".as_ptr() },
        if cpuinfo.use_exc & PVR2_USE_FSL_EXC != 0 { b"fsl \0".as_ptr() } else { b"\0".as_ptr() });

    seq_printf(m, b"Stream-insns:\t%sprivileged\n\0".as_ptr() as *const _, if cpuinfo.mmu_privins { b"un\0".as_ptr() } else { b"\0".as_ptr() });
    if cpuinfo.use_icache { seq_printf(m, b"Icache:\t\t%ukB\tline length:\t%dB\n\0".as_ptr() as *const _, cpuinfo.icache_size >> 10, cpuinfo.icache_line_length); } else { seq_puts(m, b"Icache:\t\tno\n\0".as_ptr() as *const _); }
    if cpuinfo.use_dcache {
        seq_printf(m, b"Dcache:\t\t%ukB\tline length:\t%dB\n\0".as_ptr() as *const _, cpuinfo.dcache_size >> 10, cpuinfo.dcache_line_length);
        seq_puts(m, b"Dcache-Policy:\t\0".as_ptr() as *const _);
        if cpuinfo.dcache_wb { seq_puts(m, b"write-back\n\0".as_ptr() as *const _); } else { seq_puts(m, b"write-through\n\0".as_ptr() as *const _); }
    } else { seq_puts(m, b"Dcache:\t\tno\n\0".as_ptr() as *const _); }
    seq_printf(m, b"HW-Debug:\t%s\n\0".as_ptr() as *const _, str_yes_no(cpuinfo.hw_debug));
    seq_printf(m, b"PVR-USR1:\t%02x\nPVR-USR2:\t%08x\n\0".as_ptr() as *const _, cpuinfo.pvr_user1, cpuinfo.pvr_user2);
    seq_printf(m, b"Page size:\t%lu\n\0".as_ptr() as *const _, PAGE_SIZE);
    0
}

unsafe fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let i = *pos;
    if i < NR_CPUS { (i + 1) as usize as *mut core::ffi::c_void } else { core::ptr::null_mut() }
}

unsafe fn c_next(m: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos += 1;
    c_start(m, pos)
}

unsafe fn c_stop(_m: *mut seq_file, _v: *mut core::ffi::c_void) {}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe fn(*mut seq_file, *mut loff_t) -> *mut core::ffi::c_void>,
    pub next: Option<unsafe fn(*mut seq_file, *mut core::ffi::c_void, *mut loff_t) -> *mut core::ffi::c_void>,
    pub stop: Option<unsafe fn(*mut seq_file, *mut core::ffi::c_void)>,
    pub show: Option<unsafe fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
}

pub static cpuinfo_op: seq_operations = seq_operations {
    start: Some(c_start),
    next: Some(c_next),
    stop: Some(c_stop),
    show: Some(show_cpuinfo),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
