// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// Base S3C64XX UART resource and device definitions

// C dependencies:
// linux/kernel.h, linux/types.h, linux/interrupt.h, linux/list.h,
// linux/platform_device.h, asm/mach/arch.h, asm/mach/irq.h, map.h,
// irqs.h, and devs.h supply the referenced types, constants, and macros.

/* Serial port registrations */

/* 64xx uarts are closer together */

static mut s3c64xx_uart0_resource: [resource; 2] = [
    DEFINE_RES_MEM!(S3C_PA_UART0, SZ_256),
    DEFINE_RES_IRQ!(IRQ_UART0),
];

static mut s3c64xx_uart1_resource: [resource; 2] = [
    DEFINE_RES_MEM!(S3C_PA_UART1, SZ_256),
    DEFINE_RES_IRQ!(IRQ_UART1),
];

static mut s3c6xx_uart2_resource: [resource; 2] = [
    DEFINE_RES_MEM!(S3C_PA_UART2, SZ_256),
    DEFINE_RES_IRQ!(IRQ_UART2),
];

static mut s3c64xx_uart3_resource: [resource; 2] = [
    DEFINE_RES_MEM!(S3C_PA_UART3, SZ_256),
    DEFINE_RES_IRQ!(IRQ_UART3),
];

#[link_section = ".init.data"]
pub static mut s3c64xx_uart_resources: [s3c24xx_uart_resources; 4] = [
    s3c24xx_uart_resources {
        resources: unsafe { s3c64xx_uart0_resource.as_mut_ptr() },
        nr_resources: unsafe { s3c64xx_uart0_resource.len() },
    },
    s3c24xx_uart_resources {
        resources: unsafe { s3c64xx_uart1_resource.as_mut_ptr() },
        nr_resources: unsafe { s3c64xx_uart1_resource.len() },
    },
    s3c24xx_uart_resources {
        resources: unsafe { s3c6xx_uart2_resource.as_mut_ptr() },
        nr_resources: unsafe { s3c6xx_uart2_resource.len() },
    },
    s3c24xx_uart_resources {
        resources: unsafe { s3c64xx_uart3_resource.as_mut_ptr() },
        nr_resources: unsafe { s3c64xx_uart3_resource.len() },
    },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
