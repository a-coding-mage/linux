/* SPDX-License-Identifier: GPL-2.0 */

// The declarations supplied by <linux/cpu.h> are external dependencies.

/// Extract a register field. The C macro forms the mask and shift names from
/// `_field`; Rust callers provide those values explicitly.
#[macro_export]
macro_rules! ATH25_REG_MS {
    ($val:expr, $field_m:expr, $field_s:expr) => {
        (($val & $field_m) >> $field_s)
    };
}

pub const ATH25_IRQ_CPU_CLOCK: i32 = MIPS_CPU_IRQ_BASE + 7; // C0_CAUSE: 0x8000

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ath25_soc_type {
    // handled by ar5312.c
    ATH25_SOC_AR2312,
    ATH25_SOC_AR2313,
    ATH25_SOC_AR5312,

    // handled by ar2315.c
    ATH25_SOC_AR2315,
    ATH25_SOC_AR2316,
    ATH25_SOC_AR2317,
    ATH25_SOC_AR2318,

    ATH25_SOC_UNKNOWN,
}

extern "C" {
    pub static mut ath25_soc: ath25_soc_type;
    pub static mut ath25_board: ar231x_board_config;
    pub static mut ath25_irq_dispatch: Option<unsafe extern "C" fn()>;

    pub fn ath25_find_config(offset: phys_addr_t, size: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn ath25_serial_setup(
        mapbase: u32,
        irq: ::core::ffi::c_int,
        uartclk: ::core::ffi::c_uint,
    );
    pub fn ath25_add_wmac(
        nr: ::core::ffi::c_int,
        base: u32,
        irq: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// Type and constant supplied by <linux/cpu.h>.
extern "C" {
    pub static current_cpu_data: CpuData;
    pub static CPU_4KEC: ::core::ffi::c_int;
}

#[inline]
pub unsafe fn is_ar2315() -> bool {
    current_cpu_data.cputype == CPU_4KEC
}

#[inline]
pub unsafe fn is_ar5312() -> bool {
    !is_ar2315()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
