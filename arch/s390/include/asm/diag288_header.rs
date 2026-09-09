/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the corresponding architecture headers:
// `register_pair` and `EINVAL`.

pub const MIN_INTERVAL: u32 = 15; // Minimal time supported by diag288
pub const MAX_INTERVAL: u32 = 3600; // One hour should be enough - pure estimation

pub const WDT_DEFAULT_TIMEOUT: u32 = 30;

// Function codes - init, change, cancel
pub const WDT_FUNC_INIT: u32 = 0;
pub const WDT_FUNC_CHANGE: u32 = 1;
pub const WDT_FUNC_CANCEL: u32 = 2;
pub const WDT_FUNC_CONCEAL: u32 = 0x8000_0000;

// Action codes for LPAR watchdog
pub const LPARWDT_RESTART: u32 = 0;

#[inline]
pub unsafe fn __diag288(
    func: u32,
    timeout: u32,
    action: u64,
    len: u32,
) -> i32 {
    let r1 = register_pair {
        even: func,
        odd: timeout,
    };
    let r3 = register_pair {
        even: action,
        odd: len,
    };
    let mut rc: i32 = -EINVAL;

    // The C inline assembly uses s390's DIAG 0x288 instruction and an
    // exception-table entry for recovery from the instruction fault.
    #[cfg(target_arch = "s390x")]
    core::arch::asm!(
        "diag {r1}, {r3}, 0x288",
        "0: lhi {rc}, 0",
        "1:",
        r1 = inlateout("r1") r1.pair => _,
        r3 = inlateout("r3") r3.pair => _,
        rc = inout(reg) rc,
        options(nostack)
    );

    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
