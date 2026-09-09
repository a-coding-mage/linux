/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_VIRT_H

pub const NUM_VIRT_SOURCES: u32 = 200;

#[repr(C)]
pub struct virt_booter_device_data {
    pub mmio: u32,
    pub irq: u32,
}

#[repr(C)]
pub struct virt_booter_data {
    pub qemu_version: u32,
    pub pic: virt_booter_device_data,
    pub rtc: virt_booter_device_data,
    pub tty: virt_booter_device_data,
    pub ctrl: virt_booter_device_data,
    pub virtio: virt_booter_device_data,
}

unsafe extern "C" {
    pub static mut virt_bi_data: virt_booter_data;

    // C declaration includes the __init attribute.
    pub fn virt_init_IRQ();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
