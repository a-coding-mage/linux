// SPDX-License-Identifier: GPL-2.0
/* Test context switching to see if the DSCR SPR is correctly preserved
 * when within a transaction.
 *
 * Note: We assume that the DSCR has been left at the default value (0)
 * for all CPUs.
 *
 * Method:
 *
 * Set a value into the DSCR.
 *
 * Start a transaction, and suspend it (*).
 *
 * Hard loop checking to see if the transaction has become doomed.
 *
 * Now that we *may* have been preempted, record the DSCR and TEXASR SPRS.
 *
 * If the abort was because of a context switch, check the DSCR value.
 * Otherwise, try again.
 *
 * (*) If the transaction is not suspended we can't see the problem because
 * the transaction abort handler will restore the DSCR to it's checkpointed
 * value before we regain control.
 */

use core::arch::asm;
use std::ffi::c_char;

/* Dependencies supplied by the original C includes:
 * <asm/tm.h>, "utils.h", "tm.h", and "../pmu/lib.h".
 */
unsafe extern "C" {
    fn have_htm() -> bool;
    fn htm_is_synthetic() -> bool;
    fn SKIP_IF(cond: bool);
    fn eat_cpu(test: extern "C" fn() -> i32) -> i32;
    fn test_harness(test: extern "C" fn() -> i32, name: *const c_char) -> i32;
}

const SPRN_DSCR: i32 = 0x03;

unsafe extern "C" {
    static SPRN_TEXASR: i32;
    static TM_CAUSE_RESCHED: u64;
}

#[no_mangle]
pub extern "C" fn test_body() -> i32 {
    let mut rv: u64;
    let dscr1: u64 = 1;
    let mut dscr2: u64 = 0;
    let mut texasr: u64 = 0;

    unsafe {
        SKIP_IF(!have_htm());
        SKIP_IF(htm_is_synthetic());
    }

    print!("Check DSCR TM context switch: ");
    use std::io::Write;
    std::io::stdout().flush().unwrap();

    loop {
        unsafe {
            asm!(
                /* set a known value into the DSCR */
                "ld      3, 0({dscr1_ptr})",
                "mtspr   {sprn_dscr}, 3",

                "li      {rv}, 1",
                /* start and suspend a transaction */
                "tbegin.",
                "beq     1f",
                "tsuspend.",

                /* hard loop until the transaction becomes doomed */
                "2:",
                "tcheck 0",
                "bc      4, 0, 2b",

                /* record DSCR and TEXASR */
                "mfspr   3, {sprn_dscr}",
                "std     3, 0({dscr2_ptr})",
                "mfspr   3, {sprn_texasr}",
                "std     3, 0({texasr_ptr})",

                "tresume.",
                "tend.",
                "li      {rv}, 0",
                "1:",
                rv = lateout(reg) rv,
                dscr1_ptr = in(reg) &dscr1,
                dscr2_ptr = in(reg) &mut dscr2,
                texasr_ptr = in(reg) &mut texasr,
                sprn_dscr = const SPRN_DSCR,
                sprn_texasr = const SPRN_TEXASR,
                out("r3") _,
                options(nostack),
            );
        }
        assert!(rv != 0); /* make sure the transaction aborted */
        unsafe {
            if (texasr >> 56) != TM_CAUSE_RESCHED {
                continue;
            }
        }
        if dscr2 != dscr1 {
            println!(" FAIL");
            return 1;
        } else {
            println!(" OK");
            return 0;
        }
    }
}

extern "C" fn tm_resched_dscr() -> i32 {
    unsafe { eat_cpu(test_body) }
}

fn main() {
    let _argc = std::env::args().count() as i32;
    let _argv: Vec<String> = std::env::args().collect();

    unsafe {
        std::process::exit(test_harness(
            tm_resched_dscr,
            c"tm_resched_dscr".as_ptr(),
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
