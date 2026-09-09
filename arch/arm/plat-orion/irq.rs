/*
 * arch/arm/plat-orion/irq.c
 *
 * Marvell Orion SoC IRQ handling.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

/* Dependencies supplied by the Linux IRQ, I/O, device-tree, and platform
 * headers are intentionally left as external Rust items. */

#[repr(C)]
pub struct irq_chip {
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
}

#[repr(C)]
pub struct irq_chip_type {
    pub chip: irq_chip,
}

#[repr(C)]
pub struct irq_chip_generic {
    pub chip_types: *mut irq_chip_type,
}

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn writel(value: u32, address: *mut core::ffi::c_void);
    fn irq_alloc_generic_chip(
        name: *const core::ffi::c_char,
        num_ct: u32,
        irq_base: u32,
        reg_base: *mut core::ffi::c_void,
        handler: unsafe extern "C" fn(),
    ) -> *mut irq_chip_generic;
    fn irq_gc_mask_clr_bit(data: *mut irq_data);
    fn irq_gc_mask_set_bit(data: *mut irq_data);
    fn irq_setup_generic_chip(
        gc: *mut irq_chip_generic,
        msk: u32,
        flags: u32,
        clr: u32,
        irq_flags: u32,
    );
    fn handle_level_irq();
}

/* Build-time kernel constants/macros from the included headers. */
const IRQ_GC_INIT_MASK_CACHE: u32 = 1;
const IRQ_NOREQUEST: u32 = 1;
const IRQ_LEVEL: u32 = 1;
const IRQ_NOPROBE: u32 = 1;

#[inline]
const fn irq_msk(bits: u32) -> u32 {
    if bits >= 32 { u32::MAX } else { (1u32 << bits) - 1 }
}

pub unsafe extern "C" fn orion_irq_init(
    irq_start: u32,
    maskaddr: *mut core::ffi::c_void,
) {
    let gc: *mut irq_chip_generic;
    let ct: *mut irq_chip_type;

    /*
     * Mask all interrupts initially.
     */
    unsafe { writel(0, maskaddr); }

    let name = b"orion_irq\0";
    gc = unsafe {
        irq_alloc_generic_chip(
            name.as_ptr() as *const core::ffi::c_char,
            1,
            irq_start,
            maskaddr,
            handle_level_irq,
        )
    };
    ct = unsafe { (*gc).chip_types };
    unsafe {
        (*ct).chip.irq_mask = Some(irq_gc_mask_clr_bit);
        (*ct).chip.irq_unmask = Some(irq_gc_mask_set_bit);
        irq_setup_generic_chip(
            gc,
            irq_msk(32),
            IRQ_GC_INIT_MASK_CACHE,
            IRQ_NOREQUEST,
            IRQ_LEVEL | IRQ_NOPROBE,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
