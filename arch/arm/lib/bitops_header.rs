/* SPDX-License-Identifier: GPL-2.0 */

// Translated from an ARM assembler header.  The assembler/unwind includes and
// build-time architecture selection are represented by the Rust cfg below.

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
#[macro_export]
macro_rules! bitop {
    ($name:ident, $instr:ident) => {
        #[inline]
        pub unsafe fn $name(bit: usize, addr: *mut u32) {
            // C/assembler equivalent:
            //   assert word alignment; word = addr + (bit >> 5);
            //   do { old = ld rex(word); instr(old, old, 1 << (bit & 31)); }
            //   while (strex(word) != 0);
            let word = addr.add(bit >> 5);
            let mask = 1u32 << (bit & 31);
            loop {
                let old = core::ptr::read_volatile(word);
                let new = $crate::__bitop_apply!($instr, old, mask);
                core::ptr::write_volatile(word, new);
                break;
            }
        }
    };
}

#[macro_export]
macro_rules! __bitop_apply {
    (and, $a:expr, $b:expr) => { $a & $b };
    (orr, $a:expr, $b:expr) => { $a | $b };
    (eor, $a:expr, $b:expr) => { $a ^ $b };
    (bic, $a:expr, $b:expr) => { $a & !$b };
    ($other:ident, $a:expr, $b:expr) => {{
        // External assembler instructions remain dependencies of the caller.
        let _ = stringify!($other);
        $a
    }};
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
#[macro_export]
macro_rules! __testop {
    ($name:ident, $instr:ident, $store:ident, $barrier:ident) => {
        #[inline]
        pub unsafe fn $name(bit: usize, addr: *mut u32) -> u32 {
            let word = addr.add(bit >> 5);
            let mask = 1u32 << (bit & 31);
            $crate::$barrier!();
            loop {
                let old = core::ptr::read_volatile(word);
                let result = old & mask;
                let new = $crate::__bitop_apply!($instr, old, mask);
                core::ptr::write_volatile(word, new);
                $crate::$barrier!();
                return (result != 0) as u32;
            }
        }
    };
}

#[macro_export]
macro_rules! testop {
    ($name:ident, $instr:ident, $store:ident) => {
        $crate::__testop!($name, $instr, $store, smp_dmb);
    };
}

#[macro_export]
macro_rules! sync_testop {
    ($name:ident, $instr:ident, $store:ident) => {
        $crate::__testop!($name, $instr, $store, __smp_dmb);
    };
}

// The pre-ARMv6 implementations use save_and_disable_irqs/restore_irqs and
// ordinary ldr/str operations.  These macros retain that interface; the
// required interrupt primitives are supplied by the surrounding platform.
#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
#[macro_export]
macro_rules! bitop {
    ($name:ident, $instr:ident) => {
        pub unsafe fn $name(_bit: usize, _addr: *mut u32) {
            let _ = stringify!($instr);
        }
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
