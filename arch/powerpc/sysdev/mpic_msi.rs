// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2006-2007, Michael Ellerman, IBM Corporation.
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct MsiBitmap {
    pub bitmap: *mut c_void,
}

#[repr(C)]
pub struct IrqDomainOps {
    pub xlate: Option<unsafe extern "C" fn(
        *mut IrqHost,
        *mut FwnodeHandle,
        *const u32,
        u32,
        *mut IrqHwNumber,
        *mut c_int,
    ) -> c_int>,
}

#[repr(C)]
pub struct IrqHost {
    pub ops: *const IrqDomainOps,
}

#[repr(C)]
pub struct Mpic {
    pub msi_bitmap: MsiBitmap,
    pub irqhost: *mut IrqHost,
    pub num_sources: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FwnodeHandle {
    _private: [u8; 0],
}

pub type IrqHwNumber = u64;

#[repr(C)]
pub struct OfPhandleArgs {
    pub args: *mut u32,
    pub args_count: u32,
}

pub const MPIC_U3_HT_IRQS: u32 = 1 << 0;

extern "C" {
    fn msi_bitmap_reserve_hwirq(bitmap: *mut MsiBitmap, hwirq: IrqHwNumber);
    fn msi_bitmap_alloc(
        bitmap: *mut MsiBitmap,
        num_sources: u32,
        node: *mut DeviceNode,
    ) -> c_int;
    fn msi_bitmap_reserve_dt_hwirqs(bitmap: *mut MsiBitmap) -> c_int;
    fn msi_bitmap_free(bitmap: *mut MsiBitmap);
    fn irq_domain_get_of_node(host: *mut IrqHost) -> *mut DeviceNode;
    fn of_find_all_nodes(prev: *mut DeviceNode) -> *mut DeviceNode;
    fn of_irq_parse_one(
        node: *mut DeviceNode,
        index: c_int,
        args: *mut OfPhandleArgs,
    ) -> c_int;
}

pub unsafe extern "C" fn mpic_msi_reserve_hwirq(mpic: *mut Mpic, hwirq: IrqHwNumber) {
    // The mpic calls this even when there is no allocator setup
    if (*mpic).msi_bitmap.bitmap.is_null() {
        return;
    }

    msi_bitmap_reserve_hwirq(&mut (*mpic).msi_bitmap, hwirq);
}

#[cfg(feature = "CONFIG_MPIC_U3_HT_IRQS")]
unsafe extern "C" fn mpic_msi_reserve_u3_hwirqs(mpic: *mut Mpic) -> c_int {
    let mut hwirq: IrqHwNumber = 0;
    let ops = (*(*mpic).irqhost).ops;
    let mut np: *mut DeviceNode;
    let mut flags: c_int;
    let mut index: c_int;
    let mut i: c_int;
    let mut oirq = OfPhandleArgs {
        args: core::ptr::null_mut(),
        args_count: 0,
    };

    // pr_debug("mpic: found U3, guessing msi allocator setup\n");

    // Reserve source numbers we know are reserved in the HW.
    // This is a bit of a mix of U3 and U4 reserves but that's going
    // to work fine, we have plenty enough numbers left so let's just
    // mark anything we don't like reserved.
    i = 0;
    while i < 8 {
        msi_bitmap_reserve_hwirq(&mut (*mpic).msi_bitmap, i as IrqHwNumber);
        i += 1;
    }
    i = 42;
    while i < 46 {
        msi_bitmap_reserve_hwirq(&mut (*mpic).msi_bitmap, i as IrqHwNumber);
        i += 1;
    }
    i = 100;
    while i < 105 {
        msi_bitmap_reserve_hwirq(&mut (*mpic).msi_bitmap, i as IrqHwNumber);
        i += 1;
    }
    i = 124;
    while (i as u32) < (*mpic).num_sources {
        msi_bitmap_reserve_hwirq(&mut (*mpic).msi_bitmap, i as IrqHwNumber);
        i += 1;
    }

    np = core::ptr::null_mut();
    loop {
        np = of_find_all_nodes(np);
        if np.is_null() {
            break;
        }

        // pr_debug("mpic: mapping hwirqs for %pOF\n", np);
        index = 0;
        while of_irq_parse_one(np, index, &mut oirq) == 0 {
            index += 1;
            if let Some(xlate) = (*ops).xlate {
                xlate(
                    (*mpic).irqhost,
                    core::ptr::null_mut(),
                    oirq.args,
                    oirq.args_count,
                    &mut hwirq,
                    &mut flags,
                );
            }
            msi_bitmap_reserve_hwirq(&mut (*mpic).msi_bitmap, hwirq);
        }
    }

    0
}

#[cfg(not(feature = "CONFIG_MPIC_U3_HT_IRQS"))]
unsafe extern "C" fn mpic_msi_reserve_u3_hwirqs(_mpic: *mut Mpic) -> c_int {
    -1
}

pub unsafe extern "C" fn mpic_msi_init_allocator(mpic: *mut Mpic) -> c_int {
    let mut rc = msi_bitmap_alloc(
        &mut (*mpic).msi_bitmap,
        (*mpic).num_sources,
        irq_domain_get_of_node((*mpic).irqhost),
    );
    if rc != 0 {
        return rc;
    }

    rc = msi_bitmap_reserve_dt_hwirqs(&mut (*mpic).msi_bitmap);
    if rc > 0 {
        if (*mpic).flags & MPIC_U3_HT_IRQS != 0 {
            rc = mpic_msi_reserve_u3_hwirqs(mpic);
        }

        if rc != 0 {
            msi_bitmap_free(&mut (*mpic).msi_bitmap);
            return rc;
        }
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
