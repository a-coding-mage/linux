// SPDX-License-Identifier: GPL-2.0

// Translated from C. Kernel headers and externally supplied symbols are
// expected to provide the referenced types, constants, macros, and functions.

const VIRTIO_BUS_NB: usize = 128;

unsafe fn virt_virtio_init(id: u32) -> *mut platform_device {
    let res: [resource; 2] = [
        DEFINE_RES_MEM(virt_bi_data.virtio.mmio + (id as u64) * 0x200, 0x200),
        DEFINE_RES_IRQ(virt_bi_data.virtio.irq + id),
    ];

    platform_device_register_simple(
        "virtio-mmio",
        id as i32,
        res.as_ptr(),
        res.len(),
    )
}

unsafe fn virt_platform_init() -> i32 {
    let goldfish_tty_res: [resource; 2] = [
        DEFINE_RES_MEM(virt_bi_data.tty.mmio, 1),
        DEFINE_RES_IRQ(virt_bi_data.tty.irq),
    ];
    // this is the second gf-rtc, the first one is used by the scheduler
    let goldfish_rtc_res: [resource; 2] = [
        DEFINE_RES_MEM(virt_bi_data.rtc.mmio + 0x1000, 0x1000),
        DEFINE_RES_IRQ(virt_bi_data.rtc.irq + 1),
    ];
    let virt_ctrl_res: [resource; 1] = [
        DEFINE_RES_MEM(virt_bi_data.ctrl.mmio, 0x100),
    ];
    let mut pdev1: *mut platform_device;
    let mut pdev2: *mut platform_device;
    let mut pdev3: *mut platform_device;
    let mut pdevs: [*mut platform_device; VIRTIO_BUS_NB] =
        [core::ptr::null_mut(); VIRTIO_BUS_NB];
    let mut i: u32;
    let mut ret: i32 = 0;

    if !MACH_IS_VIRT {
        return -ENODEV;
    }

    // We need this to have DMA'able memory provided to goldfish-tty
    min_low_pfn = 0;

    pdev1 = platform_device_register_simple(
        "goldfish_tty",
        PLATFORM_DEVID_NONE,
        goldfish_tty_res.as_ptr(),
        goldfish_tty_res.len(),
    );
    if IS_ERR(pdev1) {
        return PTR_ERR(pdev1);
    }

    pdev2 = platform_device_register_simple(
        "goldfish_rtc",
        PLATFORM_DEVID_NONE,
        goldfish_rtc_res.as_ptr(),
        goldfish_rtc_res.len(),
    );
    if IS_ERR(pdev2) {
        ret = PTR_ERR(pdev2);
        goto err_unregister_tty;
    }

    pdev3 = platform_device_register_simple(
        "qemu-virt-ctrl",
        PLATFORM_DEVID_NONE,
        virt_ctrl_res.as_ptr(),
        virt_ctrl_res.len(),
    );
    if IS_ERR(pdev3) {
        ret = PTR_ERR(pdev3);
        goto err_unregister_rtc;
    }

    i = 0;
    while i < VIRTIO_BUS_NB as u32 {
        pdevs[i as usize] = virt_virtio_init(i);
        if IS_ERR(pdevs[i as usize]) {
            ret = PTR_ERR(pdevs[i as usize]);
            goto err_unregister_virtio;
        }
        i += 1;
    }

    return 0;

err_unregister_virtio:
    while i > 0 {
        i -= 1;
        platform_device_unregister(pdevs[i as usize]);
    }
    platform_device_unregister(pdev3);
err_unregister_rtc:
    platform_device_unregister(pdev2);
err_unregister_tty:
    platform_device_unregister(pdev1);

    ret
}

arch_initcall!(virt_platform_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
