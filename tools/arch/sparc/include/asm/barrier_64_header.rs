/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard __TOOLS_LINUX_SPARC64_BARRIER_H omitted in Rust. */

/* Copied from the kernel sources to tools/:
 *
 * These are here in an effort to more fully work around Spitfire Errata
 * #51.  Essentially, if a memory barrier occurs soon after a mispredicted
 * branch, the chip can stop executing instructions until a trap occurs.
 * Therefore, if interrupts are disabled, the chip can hang forever.
 *
 * It used to be believed that the memory barrier had to be right in the
 * delay slot, but a case has been traced recently wherein the memory barrier
 * was one instruction after the branch delay slot and the chip still hung.
 * The offending sequence was the following in sym_wakeup_done() of the
 * sym53c8xx_2 driver:
 *
 *	call	sym_ccb_from_dsa, 0
 *	 movge	%icc, 0, %l0
 *	brz,pn	%o0, .LL1303
 *	 mov	%o0, %l2
 *	membar	#LoadLoad
 *
 * The branch has to be mispredicted for the bug to occur.  Therefore, we put
 * the memory barrier explicitly into a "branch always, predicted taken"
 * delay slot to avoid the problem case.
 */
#[macro_export]
macro_rules! membar_safe {
    ($type:literal) => {{
        unsafe {
            core::arch::asm!(
                concat!("ba,pt\t%xcc, 1f\n\t", " membar\t", $type, "\n", "1:\n"),
                options(nostack, preserves_flags)
            );
        }
    }};
}

/* The kernel always executes in TSO memory model these days,
 * and furthermore most sparc64 chips implement more stringent
 * memory ordering than required by the specifications.
 */
#[macro_export]
macro_rules! mb {
    () => {
        $crate::membar_safe!("#StoreLoad")
    };
}

#[macro_export]
macro_rules! rmb {
    () => {{
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }};
}

#[macro_export]
macro_rules! wmb {
    () => {{
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }};
}

#[macro_export]
macro_rules! smp_store_release {
    ($p:expr, $v:expr) => {{
        barrier!();
        WRITE_ONCE!(unsafe { *$p }, $v);
    }};
}

#[macro_export]
macro_rules! smp_load_acquire {
    ($p:expr) => {{
        let ___p1 = READ_ONCE!(unsafe { *$p });
        barrier!();
        ___p1
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
