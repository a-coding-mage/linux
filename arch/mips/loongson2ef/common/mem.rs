// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

// Declarations supplied by the Linux and Loongson dependencies included by the
// original source are represented here as external interfaces.
extern "C" {
    fn memblock_add(base: u64, size: u64);
    fn fls(value: i32) -> i32;
    fn ffs(value: i32) -> i32;
    fn LOONGSON_ADDRWIN_CPUTODDR(
        window: i32,
        cpu_addr: u64,
        ddr_addr: u64,
        size: u32,
    );
    fn mmiowb();
}

// Dependency-provided constant from the original headers.
const ADDRWIN_WIN3: i32 = 3;

pub static mut memsize: u32 = 0;
pub static mut highmemsize: u32 = 0;

pub unsafe fn prom_init_memory() {
    memblock_add(0x0, (memsize.wrapping_shl(20)) as u64);

    #[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
    {
        let mut bit: i32;

        bit = fls(memsize.wrapping_add(highmemsize) as i32);
        if bit != ffs(memsize.wrapping_add(highmemsize) as i32) {
            bit += 20;
        } else {
            bit = bit + 20 - 1;
        }

        /* set cpu window3 to map CPU to DDR: 2G -> 2G */
        LOONGSON_ADDRWIN_CPUTODDR(
            ADDRWIN_WIN3,
            0x80000000u64,
            0x80000000u64,
            1u32.wrapping_shl(bit as u32),
        );
        mmiowb();
    }

    #[cfg(target_pointer_width = "64")]
    {
        if highmemsize > 0 {
            memblock_add(
                0x8000000000000000u64,
                highmemsize.wrapping_shl(20) as u64,
            );
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
