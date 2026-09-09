// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  GT641xx IRQ routines.
 *
 *  Copyright (C) 2007	Yoichi Yuasa <yuasa@linux-mips.org>
 */

// Dependencies supplied by the surrounding kernel translation.

const GT641XX_IRQ_TO_BIT: unsafe fn(i32) -> u32 = |irq| 1u32 << (irq - GT641XX_IRQ_BASE);

static mut gt641xx_irq_lock: raw_spinlock_t = raw_spinlock_t { _private: [] };

unsafe fn ack_gt641xx_irq(d: *mut irq_data) {
	let mut flags: c_ulong = 0;
	let mut cause: u32;

	raw_spin_lock_irqsave(&raw mut gt641xx_irq_lock, &mut flags);
	cause = GT_READ(GT_INTRCAUSE_OFS);
	cause &= !(GT641XX_IRQ_TO_BIT)((*d).irq);
	GT_WRITE(GT_INTRCAUSE_OFS, cause);
	raw_spin_unlock_irqrestore(&raw mut gt641xx_irq_lock, flags);
}

unsafe fn mask_gt641xx_irq(d: *mut irq_data) {
	let mut flags: c_ulong = 0;
	let mut mask: u32;

	raw_spin_lock_irqsave(&raw mut gt641xx_irq_lock, &mut flags);
	mask = GT_READ(GT_INTRMASK_OFS);
	mask &= !(GT641XX_IRQ_TO_BIT)((*d).irq);
	GT_WRITE(GT_INTRMASK_OFS, mask);
	raw_spin_unlock_irqrestore(&raw mut gt641xx_irq_lock, flags);
}

unsafe fn mask_ack_gt641xx_irq(d: *mut irq_data) {
	let mut flags: c_ulong = 0;
	let mut cause: u32;
	let mut mask: u32;

	raw_spin_lock_irqsave(&raw mut gt641xx_irq_lock, &mut flags);
	mask = GT_READ(GT_INTRMASK_OFS);
	mask &= !(GT641XX_IRQ_TO_BIT)((*d).irq);
	GT_WRITE(GT_INTRMASK_OFS, mask);

	cause = GT_READ(GT_INTRCAUSE_OFS);
	cause &= !(GT641XX_IRQ_TO_BIT)((*d).irq);
	GT_WRITE(GT_INTRCAUSE_OFS, cause);
	raw_spin_unlock_irqrestore(&raw mut gt641xx_irq_lock, flags);
}

unsafe fn unmask_gt641xx_irq(d: *mut irq_data) {
	let mut flags: c_ulong = 0;
	let mut mask: u32;

	raw_spin_lock_irqsave(&raw mut gt641xx_irq_lock, &mut flags);
	mask = GT_READ(GT_INTRMASK_OFS);
	mask |= (GT641XX_IRQ_TO_BIT)((*d).irq);
	GT_WRITE(GT_INTRMASK_OFS, mask);
	raw_spin_unlock_irqrestore(&raw mut gt641xx_irq_lock, flags);
}

static mut gt641xx_irq_chip: irq_chip = irq_chip {
	name: "GT641xx",
	irq_ack: Some(ack_gt641xx_irq),
	irq_mask: Some(mask_gt641xx_irq),
	irq_mask_ack: Some(mask_ack_gt641xx_irq),
	irq_unmask: Some(unmask_gt641xx_irq),
};

pub unsafe fn gt641xx_irq_dispatch() {
	let mut cause: u32;
	let mut mask: u32;
	let mut i: i32;

	cause = GT_READ(GT_INTRCAUSE_OFS);
	mask = GT_READ(GT_INTRMASK_OFS);
	cause &= mask;

	/*
	 * bit0 : logical or of all the interrupt bits.
	 * bit30: logical or of bits[29:26,20:1].
	 * bit31: logical or of bits[25:1].
	 */
	i = 1;
	while i < 30 {
		if cause & (1u32 << i) != 0 {
			do_IRQ(GT641XX_IRQ_BASE + i);
			return;
		}
		i += 1;
	}

	atomic_inc(&raw mut irq_err_count);
}

pub unsafe fn gt641xx_irq_init() {
	let mut i: i32;

	GT_WRITE(GT_INTRMASK_OFS, 0);
	GT_WRITE(GT_INTRCAUSE_OFS, 0);

	/*
	 * bit0 : logical or of all the interrupt bits.
	 * bit30: logical or of bits[29:26,20:1].
	 * bit31: logical or of bits[25:1].
	 */
	i = 1;
	while i < 30 {
		irq_set_chip_and_handler(
			GT641XX_IRQ_BASE + i,
			&raw mut gt641xx_irq_chip,
			handle_level_irq,
		);
		i += 1;
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
