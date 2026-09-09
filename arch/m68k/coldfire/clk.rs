// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	clk.rs -- general ColdFire CPU kernel clk handling
 *
 *	Copyright (C) 2009, Greg Ungerer (gerg@snapgear.com)
 */

/***************************************************************************/

// C dependencies supplied by the surrounding kernel translation.

static mut CLK_LOCK: SpinLock = DEFINE_SPINLOCK!();

/* MCFPM_PPMCR0 conditional: advanced ColdFire clock enable/disable support. */
pub unsafe fn __clk_init_enabled(clk: *mut clk) {
	(*clk).enabled = 1;
	((*clk).clk_ops).enable.unwrap()(clk);
}

pub unsafe fn __clk_init_disabled(clk: *mut clk) {
	(*clk).enabled = 0;
	((*clk).clk_ops).disable.unwrap()(clk);
}

unsafe fn __clk_enable0(clk: *mut clk) {
	mcf_write8((*clk).slot, MCFPM_PPMCR0);
}

unsafe fn __clk_disable0(clk: *mut clk) {
	mcf_write8((*clk).slot, MCFPM_PPMSR0);
}

pub static mut clk_ops0: clk_ops = clk_ops {
	enable: Some(__clk_enable0),
	disable: Some(__clk_disable0),
};

/* MCFPM_PPMCR1 conditional: second clock control register support. */
unsafe fn __clk_enable1(clk: *mut clk) {
	mcf_write8((*clk).slot, MCFPM_PPMCR1);
}

unsafe fn __clk_disable1(clk: *mut clk) {
	mcf_write8((*clk).slot, MCFPM_PPMSR1);
}

pub static mut clk_ops1: clk_ops = clk_ops {
	enable: Some(__clk_enable1),
	disable: Some(__clk_disable1),
};

pub unsafe fn clk_enable(clk: *mut clk) -> i32 {
	let mut flags: c_ulong = 0;

	if clk.is_null() {
		return 0;
	}

	spin_lock_irqsave(&mut CLK_LOCK, &mut flags);
	let old_enabled = (*clk).enabled;
	(*clk).enabled = (*clk).enabled.wrapping_add(1);
	if old_enabled == 0 {
		if let Some(enable) = (*clk).clk_ops.enable {
			enable(clk);
		}
	}
	spin_unlock_irqrestore(&mut CLK_LOCK, flags);

	0
}

pub unsafe fn clk_disable(clk: *mut clk) {
	let mut flags: c_ulong = 0;

	if clk.is_null() {
		return;
	}

	spin_lock_irqsave(&mut CLK_LOCK, &mut flags);
	(*clk).enabled = (*clk).enabled.wrapping_sub(1);
	if (*clk).enabled == 0 {
		if let Some(disable) = (*clk).clk_ops.disable {
			disable(clk);
		}
	}
	spin_unlock_irqrestore(&mut CLK_LOCK, flags);
}

pub unsafe fn clk_get_rate(clk: *mut clk) -> c_ulong {
	if clk.is_null() {
		return 0;
	}

	(*clk).rate
}

/* dummy functions, should not be called */
pub unsafe fn clk_round_rate(clk: *mut clk, _rate: c_ulong) -> c_long {
	WARN_ON(!clk.is_null());
	0
}

pub unsafe fn clk_set_rate(clk: *mut clk, _rate: c_ulong) -> i32 {
	WARN_ON(!clk.is_null());
	0
}

pub unsafe fn clk_set_parent(clk: *mut clk, _parent: *mut clk) -> i32 {
	WARN_ON(!clk.is_null());
	0
}

pub unsafe fn clk_get_parent(clk: *mut clk) -> *mut clk {
	WARN_ON(!clk.is_null());
	core::ptr::null_mut()
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
