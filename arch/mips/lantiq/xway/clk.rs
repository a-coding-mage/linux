// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2010 John Crispin <john@phrozen.org>
 *  Copyright (C) 2013-2015 Lantiq Beteiligungs-GmbH & Co.KG
 */

// C dependencies: linux/io.h, linux/export.h, linux/clk.h, asm/time.h,
// asm/irq.h, asm/div64.h, lantiq_soc.h, and ../clk.h.

extern "C" {
    fn ltq_cgu_r32(reg: u32) -> u32;
}

static mut RAM_CLOCKS: [u64; 4] = [CLOCK_167M, CLOCK_133M, CLOCK_111M, CLOCK_83M];

/* legacy xway clock */
const CGU_SYS: u32 = 0x10;

/* vr9, ar10/grx390 clock */
const CGU_SYS_XRX: u32 = 0x0c;
const CGU_IF_CLK_AR10: u32 = 0x24;

// DDR_HZ ram_clocks[ltq_cgu_r32(CGU_SYS) & 0x3]

pub unsafe fn ltq_danube_fpi_hz() -> u64 {
    let ddr_clock = RAM_CLOCKS[(ltq_cgu_r32(CGU_SYS) & 0x3) as usize];

    if ltq_cgu_r32(CGU_SYS) & 0x40 != 0 {
        return ddr_clock >> 1;
    }
    ddr_clock
}

pub unsafe fn ltq_danube_cpu_hz() -> u64 {
    let ddr_hz = RAM_CLOCKS[(ltq_cgu_r32(CGU_SYS) & 0x3) as usize];
    match ltq_cgu_r32(CGU_SYS) & 0xc {
        0 => CLOCK_333M,
        4 => ddr_hz,
        8 => ddr_hz << 1,
        _ => ddr_hz >> 1,
    }
}

pub unsafe fn ltq_danube_pp32_hz() -> u64 {
    let clksys = (ltq_cgu_r32(CGU_SYS) >> 7) & 3;
    match clksys {
        1 => CLOCK_240M,
        2 => CLOCK_222M,
        3 => CLOCK_133M,
        _ => CLOCK_266M,
    }
}

unsafe fn ltq_ar9_sys_hz() -> u64 {
    if ((ltq_cgu_r32(CGU_SYS) >> 3) & 0x3) == 0x2 { CLOCK_393M } else { CLOCK_333M }
}

pub unsafe fn ltq_ar9_fpi_hz() -> u64 {
    let sys = ltq_ar9_sys_hz();
    if ltq_cgu_r32(CGU_SYS) & (1 << 0) != 0 { sys / 3 } else { sys / 2 }
}

pub unsafe fn ltq_ar9_cpu_hz() -> u64 {
    if ltq_cgu_r32(CGU_SYS) & (1 << 2) != 0 { ltq_ar9_fpi_hz() } else { ltq_ar9_sys_hz() }
}

pub unsafe fn ltq_vr9_cpu_hz() -> u64 {
    match (ltq_cgu_r32(CGU_SYS_XRX) >> 4) & 0xf {
        0 => CLOCK_600M,
        1 => CLOCK_500M,
        2 => CLOCK_393M,
        3 => CLOCK_333M,
        5 | 6 => CLOCK_196_608M,
        7 => CLOCK_167M,
        4 | 8 | 9 => CLOCK_125M,
        _ => 0,
    }
}

pub unsafe fn ltq_vr9_fpi_hz() -> u64 {
    let cpu_clk = ltq_vr9_cpu_hz();
    match ltq_cgu_r32(CGU_SYS_XRX) & 0x3 {
        0 => cpu_clk, // OCP ratio 1
        2 => cpu_clk / 2, // OCP ratio 2
        3 => (cpu_clk * 2) / 5, // OCP ratio 2.5
        4 => cpu_clk / 3, // OCP ratio 3
        _ => 0,
    }
}

pub unsafe fn ltq_vr9_pp32_hz() -> u64 {
    match (ltq_cgu_r32(CGU_SYS) >> 16) & 0x7 {
        0 => CLOCK_500M,
        1 => CLOCK_432M,
        2 => CLOCK_288M,
        _ => CLOCK_500M,
    }
}

pub unsafe fn ltq_ar10_cpu_hz() -> u64 {
    let clksys = match (ltq_cgu_r32(CGU_SYS_XRX) >> 8) & 0x1 {
        0 => CLOCK_500M,
        1 => CLOCK_600M,
        _ => CLOCK_500M,
    };
    match (ltq_cgu_r32(CGU_SYS_XRX) >> 4) & 0x7 {
        0 => clksys,
        1 => clksys >> 1,
        2 => clksys >> 2,
        _ => clksys,
    }
}

pub unsafe fn ltq_ar10_fpi_hz() -> u64 {
    match (ltq_cgu_r32(CGU_IF_CLK_AR10) >> 25) & 0xf {
        1 => CLOCK_300M,
        5 => CLOCK_250M,
        2 => CLOCK_150M,
        6 => CLOCK_125M,
        _ => CLOCK_125M,
    }
}

pub unsafe fn ltq_ar10_pp32_hz() -> u64 {
    match (ltq_cgu_r32(CGU_SYS) >> 16) & 0x7 {
        1 => CLOCK_250M,
        4 => CLOCK_400M,
        _ => CLOCK_250M,
    }
}

pub unsafe fn ltq_grx390_cpu_hz() -> u64 {
    let clksys = match (ltq_cgu_r32(CGU_SYS_XRX) >> 9) & 0x3 {
        0 => CLOCK_600M,
        1 => CLOCK_666M,
        2 => CLOCK_720M,
        _ => CLOCK_600M,
    };
    match (ltq_cgu_r32(CGU_SYS_XRX) >> 4) & 0x7 {
        0 => clksys,
        1 => clksys >> 1,
        2 => clksys >> 2,
        _ => clksys,
    }
}

pub unsafe fn ltq_grx390_fpi_hz() -> u64 {
    /* fpi clock is derived from ddr_clk */
    let clksys = match (ltq_cgu_r32(CGU_SYS_XRX) >> 9) & 0x3 {
        0 => CLOCK_600M,
        1 => CLOCK_666M,
        2 => CLOCK_720M,
        _ => CLOCK_600M,
    };
    match ltq_cgu_r32(CGU_SYS_XRX) & 0x7 {
        1 => clksys >> 1,
        2 => clksys >> 2,
        _ => clksys >> 1,
    }
}

pub unsafe fn ltq_grx390_pp32_hz() -> u64 {
    match (ltq_cgu_r32(CGU_SYS) >> 16) & 0x7 {
        1 => CLOCK_250M,
        2 => CLOCK_432M,
        4 => CLOCK_400M,
        _ => CLOCK_250M,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
