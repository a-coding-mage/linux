/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Sparc (general) CPU types
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparc_cpu {
    sun4m = 0x00,
    sun4d = 0x01,
    sun4e = 0x02,
    sun4u = 0x03, /* V8 ploos ploos */
    sun_unknown = 0x04,
    ap1000 = 0x05, /* almost a sun4m */
    sparc_leon = 0x06, /* Leon SoC */
}

#[cfg(CONFIG_SPARC32)]
extern "C" {
    pub static mut sparc_cpu_model: sparc_cpu;
}

/* Architectural limit of sun4m. */
#[cfg(CONFIG_SPARC32)]
pub const SUN4M_NCPUS: usize = 4;

#[cfg(not(CONFIG_SPARC32))]
pub const sparc_cpu_model: sparc_cpu = sparc_cpu::sun4u;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
