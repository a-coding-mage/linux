/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Device tables which are exported to userspace via
 * scripts/mod/file2alias.c. You must keep that file in sync with this
 * header.
 *
 * The C header includes Linux type definitions and the device-id headers
 * listed below; those dependencies are supplied externally.
 */

// C conditional: __KERNEL__ selects the kernel type definitions.
// C includes translated as external dependencies:
// linux/types.h
// device-id/acpi.h, device-id/amba.h, device-id/ap.h, device-id/apr.h,
// device-id/auxiliary.h, device-id/bcma.h, device-id/ccw.h, device-id/cdx.h,
// device-id/coreboot.h, device-id/css.h, device-id/dfl.h, device-id/dmi.h,
// device-id/eisa.h, device-id/fsl_mc.h, device-id/hda.h, device-id/hid.h,
// device-id/hv_vmbus.h, device-id/i2c.h, device-id/i3c.h, device-id/ieee1394.h,
// device-id/input.h, device-id/ipack.h, device-id/isapnp.h, device-id/ishtp.h,
// device-id/mcb.h, device-id/mdio.h, device-id/mei_cl.h, device-id/mhi.h,
// device-id/mips_cdmm.h, device-id/of.h, device-id/parisc.h, device-id/pci.h,
// device-id/pcmcia.h, device-id/platform.h, device-id/pnp.h, device-id/rio.h,
// device-id/rpmsg.h, device-id/sdio.h, device-id/sdw.h, device-id/serio.h,
// device-id/slim.h, device-id/spi.h, device-id/spmi.h, device-id/ssam.h,
// device-id/ssb.h, device-id/tb.h, device-id/tee_client.h, device-id/typec.h,
// device-id/ulpi.h, device-id/usb.h, device-id/vchiq.h, device-id/vio.h,
// device-id/virtio.h, device-id/wmi.h, device-id/x86_cpu.h, device-id/zorro.h

/*
 * Generic table type for matching CPU features.
 * @feature: the bit number of the feature (0 - 65535)
 */
#[repr(C)]
pub struct cpu_feature {
    pub feature: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
