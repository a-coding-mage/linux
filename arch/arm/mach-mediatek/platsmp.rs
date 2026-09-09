// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-mediatek/platsmp.c
 *
 * Copyright (c) 2014 Mediatek Inc.
 * Author: Shunli Wang <shunli.wang@mediatek.com>
 *         Yingjoe Chen <yingjoe.chen@mediatek.com>
 */

const MTK_MAX_CPU: usize = 8;
const MTK_SMP_REG_SIZE: usize = 0x1000;

#[repr(C)]
struct MtkSmpBootInfo {
    smp_base: libc::c_ulong,
    jump_reg: libc::c_uint,
    core_keys: [libc::c_uint; MTK_MAX_CPU - 1],
    core_regs: [libc::c_uint; MTK_MAX_CPU - 1],
}

static MTK_MT8135_TZ_BOOT: MtkSmpBootInfo = MtkSmpBootInfo {
    smp_base: 0x80002000,
    jump_reg: 0x3fc,
    core_keys: [0x534c4131, 0x4c415332, 0x41534c33, 0, 0, 0, 0],
    core_regs: [0x3f8, 0x3f8, 0x3f8, 0, 0, 0, 0],
};

static MTK_MT6572_BOOT: MtkSmpBootInfo = MtkSmpBootInfo {
    smp_base: 0x10001400,
    jump_reg: 0x08,
    core_keys: [0x534c4131, 0, 0, 0, 0, 0, 0],
    core_regs: [0x0c, 0, 0, 0, 0, 0, 0],
};

static MTK_MT6589_BOOT: MtkSmpBootInfo = MtkSmpBootInfo {
    smp_base: 0x10002000,
    jump_reg: 0x34,
    core_keys: [0x534c4131, 0x4c415332, 0x41534c33, 0, 0, 0, 0],
    core_regs: [0x38, 0x3c, 0x40, 0, 0, 0, 0],
};

static MTK_MT7623_BOOT: MtkSmpBootInfo = MtkSmpBootInfo {
    smp_base: 0x10202000,
    jump_reg: 0x34,
    core_keys: [0x534c4131, 0x4c415332, 0x41534c33, 0, 0, 0, 0],
    core_regs: [0x38, 0x3c, 0x40, 0, 0, 0, 0],
};

#[repr(C)]
struct OfDeviceId {
    compatible: *const libc::c_char,
    data: *const MtkSmpBootInfo,
}

static MTK_TZ_SMP_BOOT_INFOS: [OfDeviceId; 4] = [
    OfDeviceId { compatible: b"mediatek,mt8135\0".as_ptr() as *const _, data: &MTK_MT8135_TZ_BOOT },
    OfDeviceId { compatible: b"mediatek,mt8127\0".as_ptr() as *const _, data: &MTK_MT8135_TZ_BOOT },
    OfDeviceId { compatible: b"mediatek,mt2701\0".as_ptr() as *const _, data: &MTK_MT8135_TZ_BOOT },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

static MTK_SMP_BOOT_INFOS: [OfDeviceId; 6] = [
    OfDeviceId { compatible: b"mediatek,mt6572\0".as_ptr() as *const _, data: &MTK_MT6572_BOOT },
    OfDeviceId { compatible: b"mediatek,mt6582\0".as_ptr() as *const _, data: &MTK_MT7623_BOOT },
    OfDeviceId { compatible: b"mediatek,mt6589\0".as_ptr() as *const _, data: &MTK_MT6589_BOOT },
    OfDeviceId { compatible: b"mediatek,mt7623\0".as_ptr() as *const _, data: &MTK_MT7623_BOOT },
    OfDeviceId { compatible: b"mediatek,mt7629\0".as_ptr() as *const _, data: &MTK_MT7623_BOOT },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

// The device-tree tables are supplied by the kernel's OF interfaces.
static mut mtk_smp_base: *mut libc::c_void = core::ptr::null_mut();
static mut mtk_smp_info: *const MtkSmpBootInfo = core::ptr::null();

unsafe extern "C" {
    fn writel_relaxed(value: libc::c_uint, addr: *mut libc::c_void);
    fn arch_send_wakeup_ipi_mask(mask: *const libc::c_void);
    fn cpumask_of(cpu: libc::c_uint) -> *const libc::c_void;
    fn of_machine_is_compatible(compatible: *const libc::c_char) -> bool;
    fn phys_to_virt(addr: libc::c_ulong) -> *mut libc::c_void;
    fn ioremap(addr: libc::c_ulong, size: usize) -> *mut libc::c_void;
    fn __pa_symbol(symbol: unsafe extern "C" fn());
    fn secondary_startup_arm();
}

unsafe fn mtk_boot_secondary(cpu: libc::c_uint, _idle: *mut libc::c_void) -> libc::c_int {
    if mtk_smp_base.is_null() {
        return -libc::EINVAL;
    }
    if (*mtk_smp_info).core_keys[(cpu - 1) as usize] == 0 {
        return -libc::EINVAL;
    }
    writel_relaxed(
        (*mtk_smp_info).core_keys[(cpu - 1) as usize],
        mtk_smp_base.add((*mtk_smp_info).core_regs[(cpu - 1) as usize] as usize),
    );
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));
    0
}

unsafe fn __mtk_smp_prepare_cpus(_max_cpus: libc::c_uint, trustzone: libc::c_int) {
    let (infos, num) = if trustzone != 0 {
        (&MTK_TZ_SMP_BOOT_INFOS, MTK_TZ_SMP_BOOT_INFOS.len())
    } else {
        (&MTK_SMP_BOOT_INFOS, MTK_SMP_BOOT_INFOS.len())
    };
    for i in 0..num {
        if !infos[i].compatible.is_null() && of_machine_is_compatible(infos[i].compatible) {
            mtk_smp_info = infos[i].data;
            break;
        }
    }
    if mtk_smp_info.is_null() {
        // pr_err("%s: Device is not supported\\n", __func__);
        return;
    }
    if trustzone != 0 {
        mtk_smp_base = phys_to_virt((*mtk_smp_info).smp_base);
    } else {
        mtk_smp_base = ioremap((*mtk_smp_info).smp_base, MTK_SMP_REG_SIZE);
        if mtk_smp_base.is_null() {
            return;
        }
    }
    writel_relaxed(
        __pa_symbol(secondary_startup_arm),
        mtk_smp_base.add((*mtk_smp_info).jump_reg as usize),
    );
}

unsafe fn mtk_tz_smp_prepare_cpus(max_cpus: libc::c_uint) {
    __mtk_smp_prepare_cpus(max_cpus, 1);
}

unsafe fn mtk_smp_prepare_cpus(max_cpus: libc::c_uint) {
    __mtk_smp_prepare_cpus(max_cpus, 0);
}

// smp_operations and CPU_METHOD_OF_DECLARE registrations are kernel declarations:
// mt81xx_tz_smp uses mtk_tz_smp_prepare_cpus and mtk_boot_secondary for
// "mediatek,mt81xx-tz-smp"; mt6589_smp uses mtk_smp_prepare_cpus and
// mtk_boot_secondary for "mediatek,mt6589-smp".

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
