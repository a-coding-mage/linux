// SPDX-License-Identifier: GPL-2.0
/*
 *	Precise Delay Loops for parisc
 *
 *	based on code by:
 *	Copyright (C) 1993 Linus Torvalds
 *	Copyright (C) 1997 Martin Mares <mj@atrey.karlin.mff.cuni.cz>
 *	Copyright (C) 2008 Jiri Hladky <hladky _dot_ jiri _at_ gmail _dot_ com>
 *
 *	parisc implementation:
 *	Copyright (C) 2013 Helge Deller <deller@gmx.de>
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn preempt_disable();
    fn preempt_enable();
    fn smp_processor_id() -> i32;
    fn mfctl(reg: i32) -> u32;
    static boot_cpu_data: BootCpuData;
}

#[repr(C)]
pub struct BootCpuData {
    pub cpu_hz: u64,
}

/* CR16 based delay: */
unsafe fn __cr16_delay(__loops: u64) {
    /*
     * Note: Due to unsigned math, cr16 rollovers shouldn't be
     * a problem here. However, on 32 bit, we need to make sure
     * we don't pass in too big a value. The current default
     * value of MAX_UDELAY_MS should help prevent this.
     */
    let mut bclock: u32;
    let mut now: u32;
    let mut loops: u32 = __loops as u32;
    let mut cpu: i32;

    preempt_disable();
    cpu = smp_processor_id();
    bclock = mfctl(16);
    loop {
        now = mfctl(16);
        if now.wrapping_sub(bclock) >= loops {
            break;
        }

        /* Allow RT tasks to run */
        preempt_enable();
        core::arch::asm!("nop");
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        preempt_disable();

        /*
         * It is possible that we moved to another CPU, and
         * since CR16's are per-cpu we need to calculate
         * that. The delay must guarantee that we wait "at
         * least" the amount of time. Being moved to another
         * CPU could make the wait longer but we just need to
         * make sure we waited long enough. Rebalance the
         * counter for this CPU.
         */
        if cpu != smp_processor_id() {
            loops = loops.wrapping_sub(now.wrapping_sub(bclock));
            cpu = smp_processor_id();
            bclock = mfctl(16);
        }
    }
    preempt_enable();
}

pub unsafe fn __udelay(usecs: u64) {
    __cr16_delay(usecs.wrapping_mul(boot_cpu_data.cpu_hz / 1_000_000u64));
}

// EXPORT_SYMBOL(__udelay);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
