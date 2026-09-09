// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MIPS idle loop and WAIT instruction support.
 *
 * Copyright (C) xxxx  the Anonymous
 * Copyright (C) 1994 - 2006 Ralf Baechle
 * Copyright (C) 2003, 2004  Maciej W. Rozycki
 * Copyright (C) 2001, 2004, 2011, 2012  MIPS Technologies, Inc.
 */

// Kernel and architecture dependencies are supplied by other translation units.

extern "C" {
    pub static mut cpu_wait: Option<unsafe extern "C" fn()>;
    fn read_c0_conf() -> usize;
    fn write_c0_conf(value: usize);
    fn need_resched() -> bool;
    fn read_c0_status() -> usize;
    fn raw_local_irq_disable();
    fn printk(message: *const core::ffi::c_char) -> core::ffi::c_int;
    fn current_cpu_type() -> i32;
    fn read_c0_config7() -> usize;
    static mut current_cpu_data: CpuinfoMips;
    static cpu_has_mips_r6: bool;
    fn r4k_wait();
}

#[repr(C)]
pub struct CpuinfoMips {
    pub processor_id: u32,
}

static mut nowait: i32 = 0;

unsafe extern "C" fn r3081_wait() {
    let cfg = read_c0_conf();
    write_c0_conf(cfg | R30XX_CONF_HALT as usize);
}

pub unsafe extern "C" fn r4k_wait_irqoff() {
    if !need_resched() {
        core::arch::asm!("wait", options(nostack, preserves_flags));
    }
}

unsafe extern "C" fn rm7k_wait_irqoff() {
    if !need_resched() {
        core::arch::asm!(
            "mfc0 $1, $12",
            "sync",
            "mtc0 $1, $12",
            "wait",
            "mtc0 $1, $12",
            options(nostack, preserves_flags)
        );
    }
}

unsafe extern "C" fn au1k_wait() {
    let c0status = read_c0_status() | 1;
    core::arch::asm!(
        "cache 0x14, 0({wait_fn})",
        "cache 0x14, 32({wait_fn})",
        "sync",
        "mtc0 {status}, $12",
        "wait",
        "nop", "nop", "nop", "nop",
        wait_fn = in(reg) au1k_wait as unsafe extern "C" fn(),
        status = in(reg) c0status,
        options(nostack, preserves_flags)
    );
    raw_local_irq_disable();
}

pub unsafe extern "C" fn wait_disable(_s: *mut core::ffi::c_char) -> i32 {
    nowait = 1;
    1
}

pub unsafe extern "C" fn check_wait() {
    let c = &current_cpu_data;
    if nowait != 0 {
        printk(b"Wait instruction disabled.\0".as_ptr() as *const _);
        return;
    }
    if cpu_has_mips_r6 {
        cpu_wait = Some(r4k_wait_irqoff);
        return;
    }
    match current_cpu_type() {
        CPU_R3081 | CPU_R3081E => cpu_wait = Some(r3081_wait),
        CPU_R4200 | CPU_R4600 | CPU_R4640 | CPU_R4650 | CPU_R4700 |
        CPU_R5000 | CPU_R5500 | CPU_NEVADA | CPU_4KC | CPU_4KEC | CPU_4KSC |
        CPU_5KC | CPU_5KE | CPU_25KF | CPU_PR4450 | CPU_BMIPS3300 |
        CPU_BMIPS4350 | CPU_BMIPS4380 | CPU_CAVIUM_OCTEON |
        CPU_CAVIUM_OCTEON_PLUS | CPU_CAVIUM_OCTEON2 | CPU_CAVIUM_OCTEON3 |
        CPU_XBURST | CPU_LOONGSON32 => cpu_wait = Some(r4k_wait),
        CPU_LOONGSON64 => {
            if (c.processor_id & (PRID_IMP_MASK | PRID_REV_MASK)) >=
                (PRID_IMP_LOONGSON_64C | PRID_REV_LOONGSON3A_R2_0) ||
                (c.processor_id & PRID_IMP_MASK) == PRID_IMP_LOONGSON_64R {
                cpu_wait = Some(r4k_wait);
            }
        }
        CPU_BMIPS5000 => cpu_wait = Some(r4k_wait_irqoff),
        CPU_RM7000 => cpu_wait = Some(rm7k_wait_irqoff),
        CPU_PROAPTIV | CPU_P5600 => {
            // CONFIG_MIPS_EJTAG_FDC_TTY: preserve the C fallthrough condition.
            if IS_ENABLED_CONFIG_MIPS_EJTAG_FDC_TTY { return; }
            cpu_wait = Some(r4k_wait);
            if read_c0_config7() & MIPS_CONF7_WII != 0 { cpu_wait = Some(r4k_wait_irqoff); }
        }
        CPU_M14KC | CPU_M14KEC | CPU_24K | CPU_34K | CPU_1004K |
        CPU_1074K | CPU_INTERAPTIV | CPU_M5150 | CPU_QEMU_GENERIC => {
            cpu_wait = Some(r4k_wait);
            if read_c0_config7() & MIPS_CONF7_WII != 0 { cpu_wait = Some(r4k_wait_irqoff); }
        }
        CPU_74K => {
            cpu_wait = Some(r4k_wait);
            if (c.processor_id & 0xff) >= PRID_REV_ENCODE_332(2, 1, 0) { cpu_wait = Some(r4k_wait_irqoff); }
        }
        CPU_TX49XX => cpu_wait = Some(r4k_wait_irqoff),
        CPU_ALCHEMY => cpu_wait = Some(au1k_wait),
        CPU_20KC => { if (c.processor_id & 0xff) <= 0x64 { return; } }
        _ => {}
    }
}

pub unsafe extern "C" fn arch_cpu_idle() {
    if let Some(wait) = cpu_wait { wait(); }
}

// #ifdef CONFIG_CPU_IDLE
pub unsafe extern "C" fn mips_cpuidle_wait_enter(_dev: *mut CpuidleDevice, _drv: *mut CpuidleDriver, index: i32) -> i32 {
    arch_cpu_idle();
    index
}
// #endif

#[repr(C)] pub struct CpuidleDevice;
#[repr(C)] pub struct CpuidleDriver;

extern "C" {
    static IS_ENABLED_CONFIG_MIPS_EJTAG_FDC_TTY: bool;
}

// Constants supplied by the MIPS architecture headers.
extern "C" {
    static R30XX_CONF_HALT: u32;
    static PRID_IMP_MASK: u32; static PRID_REV_MASK: u32;
    static PRID_IMP_LOONGSON_64C: u32; static PRID_REV_LOONGSON3A_R2_0: u32;
    static PRID_IMP_LOONGSON_64R: u32; static MIPS_CONF7_WII: usize;
}

// CPU_* constants and PRID_REV_ENCODE_332 are supplied by asm/cpu-type.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
