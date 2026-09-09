// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) Maxime Coquelin 2015
 * Copyright (C) STMicroelectronics 2017
 * Author:  Maxime Coquelin <mcoquelin.stm32@gmail.com>
 */

// Dependencies supplied by the kernel headers:
// <linux/kernel.h>
// <asm/mach/arch.h>
// <asm/v7m.h>

use core::ffi::c_char;

static STM32_COMPAT_STM32F429: &[u8] = b"st,stm32f429\0";
static STM32_COMPAT_STM32F469: &[u8] = b"st,stm32f469\0";
static STM32_COMPAT_STM32F746: &[u8] = b"st,stm32f746\0";
static STM32_COMPAT_STM32F769: &[u8] = b"st,stm32f769\0";
static STM32_COMPAT_STM32H743: &[u8] = b"st,stm32h743\0";
static STM32_COMPAT_STM32H747: &[u8] = b"st,stm32h747\0";
static STM32_COMPAT_STM32H750: &[u8] = b"st,stm32h750\0";
static STM32_COMPAT_STM32MP131: &[u8] = b"st,stm32mp131\0";
static STM32_COMPAT_STM32MP133: &[u8] = b"st,stm32mp133\0";
static STM32_COMPAT_STM32MP135: &[u8] = b"st,stm32mp135\0";
static STM32_COMPAT_STM32MP151: &[u8] = b"st,stm32mp151\0";
static STM32_COMPAT_STM32MP157: &[u8] = b"st,stm32mp157\0";

#[link_name = "stm32_compat"]
#[used]
static STM32_COMPAT: [*const c_char; 13] = [
    STM32_COMPAT_STM32F429.as_ptr() as *const c_char,
    STM32_COMPAT_STM32F469.as_ptr() as *const c_char,
    STM32_COMPAT_STM32F746.as_ptr() as *const c_char,
    STM32_COMPAT_STM32F769.as_ptr() as *const c_char,
    STM32_COMPAT_STM32H743.as_ptr() as *const c_char,
    STM32_COMPAT_STM32H747.as_ptr() as *const c_char,
    STM32_COMPAT_STM32H750.as_ptr() as *const c_char,
    STM32_COMPAT_STM32MP131.as_ptr() as *const c_char,
    STM32_COMPAT_STM32MP133.as_ptr() as *const c_char,
    STM32_COMPAT_STM32MP135.as_ptr() as *const c_char,
    STM32_COMPAT_STM32MP151.as_ptr() as *const c_char,
    STM32_COMPAT_STM32MP157.as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(STM32DT, "STM32 (Device Tree Support)")
// MACHINE_END
// These macros define and register the architecture machine descriptor.
#[repr(C)]
struct MachineDesc {
    dt_compat: *const *const c_char,
    #[cfg(CONFIG_ARM_SINGLE_ARMV7M)]
    restart: unsafe extern "C" fn(),
}

#[cfg(CONFIG_ARM_SINGLE_ARMV7M)]
extern "C" {
    fn armv7m_restart();
}

#[used]
static STM32DT: MachineDesc = MachineDesc {
    dt_compat: STM32_COMPAT.as_ptr(),
    #[cfg(CONFIG_ARM_SINGLE_ARMV7M)]
    restart: armv7m_restart,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
