// SPDX-License-Identifier: GPL-2.0

// External declarations corresponding to the Linux and architecture headers.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MachdepCalls {
    pub dma_set_mask:
        Option<unsafe extern "C" fn(dev: *mut device, dma_mask: u64)>,
}

unsafe extern "C" {
    pub static mut ppc_md: MachdepCalls;
}

pub unsafe extern "C" fn arch_dma_set_mask(dev: *mut device, dma_mask: u64) {
    if let Some(dma_set_mask) = ppc_md.dma_set_mask {
        dma_set_mask(dev, dma_mask);
    }
}

// EXPORT_SYMBOL(arch_dma_set_mask);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
