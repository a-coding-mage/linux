// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for HiSilicon Hydra Home Agent (HHA).
 *
 * Copyright (c) 2025 HiSilicon Technologies Co., Ltd.
 * Author: Yicong Yang <yangyicong@hisilicon.com>
 *         Yushan Wang <wangyushan12@huawei.com>
 *
 * A system typically contains multiple HHAs. Each is responsible for a subset
 * of the physical addresses in the system, but interleave can make the mapping
 * from a particular cache line to a responsible HHA complex. As such no
 * filtering is done in the driver, with the hardware being responsible for
 * responding with success for even if it was not responsible for any addresses
 * in the range on which the operation was requested.
 */

// Linux kernel dependencies supplied by other translation units.

const HISI_HHA_CTRL: usize = 0x5004;
const HISI_HHA_CTRL_EN: u32 = 1 << 0;
const HISI_HHA_CTRL_RANGE: u32 = 1 << 1;
const HISI_HHA_CTRL_TYPE: u32 = 0b11 << 2;
const HISI_HHA_START_L: usize = 0x5008;
const HISI_HHA_START_H: usize = 0x500c;
const HISI_HHA_LEN_L: usize = 0x5010;
const HISI_HHA_LEN_H: usize = 0x5014;

/* The maintain operation performs in a 128 Byte granularity */
const HISI_HHA_MAINT_ALIGN: usize = 128;
const HISI_HHA_POLL_GAP_US: u32 = 10;
const HISI_HHA_POLL_TIMEOUT_US: u32 = 50000;

#[repr(C)]
struct hisi_soc_hha {
    /* Must be first element */
    cci: cache_coherency_ops_inst,
    /* Locks HHA instance to forbid overlapping access. */
    lock: mutex,
    base: *mut core::ffi::c_void,
}

#[repr(C)]
struct cache_coherency_ops_inst;
#[repr(C)]
struct cc_inval_params {
    addr: usize,
    size: usize,
}
#[repr(C)]
struct cache_coherency_ops;
#[repr(C)]
struct mutex;
#[repr(C)]
struct platform_device;
#[repr(C)]
struct resource {
    start: usize,
}
#[repr(C)]
struct acpi_device_id;

extern "C" {
    fn readl_poll_timeout_atomic(addr: *mut core::ffi::c_void, val: *mut u32,
                                  condition: bool, gap_us: u32,
                                  timeout_us: u32) -> i32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn ioremap(start: usize, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn resource_size(res: *const resource) -> usize;
    fn platform_get_resource(pdev: *mut platform_device, ty: u32, num: u32)
        -> *mut resource;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut hisi_soc_hha);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn cache_coherency_ops_instance_alloc(ops: *const cache_coherency_ops)
        -> *mut hisi_soc_hha;
    fn cache_coherency_ops_instance_put(cci: *mut cache_coherency_ops_inst);
    fn cache_coherency_ops_instance_register(cci: *mut cache_coherency_ops_inst) -> i32;
    fn cache_coherency_ops_instance_unregister(cci: *mut cache_coherency_ops_inst);
    fn dev_err_probe(pdev: *mut core::ffi::c_void, err: i32,
                     msg: *const core::ffi::c_char) -> i32;
}

unsafe fn hisi_hha_cache_maintain_wait_finished(soc_hha: *mut hisi_soc_hha) -> bool {
    let mut val = 0u32;
    unsafe {
        !readl_poll_timeout_atomic(
            (*soc_hha).base.add(HISI_HHA_CTRL), &mut val,
            (val & HISI_HHA_CTRL_EN) == 0,
            HISI_HHA_POLL_GAP_US, HISI_HHA_POLL_TIMEOUT_US,
        ) != 0
    }
}

unsafe fn hisi_soc_hha_wbinv(
    cci: *mut cache_coherency_ops_inst,
    invp: *mut cc_inval_params,
) -> i32 {
    let soc_hha = cci as *mut hisi_soc_hha;
    let mut addr = (*invp).addr;
    let mut size = (*invp).size;
    if size == 0 { return -22; }

    addr &= !(HISI_HHA_MAINT_ALIGN - 1);
    let top = (addr + size + HISI_HHA_MAINT_ALIGN - 1)
        & !(HISI_HHA_MAINT_ALIGN - 1);
    size = top - addr;

    mutex_lock(&mut (*soc_hha).lock);
    if !hisi_hha_cache_maintain_wait_finished(soc_hha) {
        mutex_unlock(&mut (*soc_hha).lock);
        return -16;
    }

    /* Hardware searches [addr, addr + size - 1] in 128 byte granules. */
    size -= 1;
    writel(addr as u32, (*soc_hha).base.add(HISI_HHA_START_L));
    writel((addr >> 32) as u32, (*soc_hha).base.add(HISI_HHA_START_H));
    writel(size as u32, (*soc_hha).base.add(HISI_HHA_LEN_L));
    writel((size >> 32) as u32, (*soc_hha).base.add(HISI_HHA_LEN_H));
    let mut reg = (1u32 << 2) & HISI_HHA_CTRL_TYPE;
    reg |= HISI_HHA_CTRL_RANGE | HISI_HHA_CTRL_EN;
    writel(reg, (*soc_hha).base.add(HISI_HHA_CTRL));
    mutex_unlock(&mut (*soc_hha).lock);
    0
}

unsafe fn hisi_soc_hha_done(cci: *mut cache_coherency_ops_inst) -> i32 {
    let soc_hha = cci as *mut hisi_soc_hha;
    mutex_lock(&mut (*soc_hha).lock);
    let done = hisi_hha_cache_maintain_wait_finished(soc_hha);
    mutex_unlock(&mut (*soc_hha).lock);
    if !done { return -110; }
    0
}

#[repr(C)]
struct cache_coherency_ops {
    wbinv: unsafe fn(*mut cache_coherency_ops_inst, *mut cc_inval_params) -> i32,
    done: unsafe fn(*mut cache_coherency_ops_inst) -> i32,
}

static hha_ops: cache_coherency_ops = cache_coherency_ops {
    wbinv: hisi_soc_hha_wbinv,
    done: hisi_soc_hha_done,
};

unsafe fn hisi_soc_hha_probe(pdev: *mut platform_device) -> i32 {
    let soc_hha = cache_coherency_ops_instance_alloc(&hha_ops);
    if soc_hha.is_null() { return -12; }
    platform_set_drvdata(pdev, soc_hha);
    mutex_init(&mut (*soc_hha).lock);
    let mem = platform_get_resource(pdev, 0, 0);
    if mem.is_null() {
        cache_coherency_ops_instance_put(&mut (*soc_hha).cci);
        return -12;
    }
    (*soc_hha).base = ioremap((*mem).start, resource_size(mem));
    if (*soc_hha).base.is_null() {
        cache_coherency_ops_instance_put(&mut (*soc_hha).cci);
        return -12;
    }
    let ret = cache_coherency_ops_instance_register(&mut (*soc_hha).cci);
    if ret != 0 {
        iounmap((*soc_hha).base);
        cache_coherency_ops_instance_put(&mut (*soc_hha).cci);
        return ret;
    }
    0
}

unsafe fn hisi_soc_hha_remove(pdev: *mut platform_device) {
    let soc_hha = platform_get_drvdata(pdev) as *mut hisi_soc_hha;
    cache_coherency_ops_instance_unregister(&mut (*soc_hha).cci);
    iounmap((*soc_hha).base);
    cache_coherency_ops_instance_put(&mut (*soc_hha).cci);
}

static hisi_soc_hha_ids: [acpi_device_id; 2] = unsafe { core::mem::zeroed() };

// MODULE_DEVICE_TABLE(acpi, hisi_soc_hha_ids);
// module_platform_driver(hisi_soc_hha_driver);
// MODULE_IMPORT_NS("CACHE_COHERENCY");
// MODULE_DESCRIPTION("HiSilicon Hydra Home Agent driver supporting cache maintenance");
// MODULE_AUTHOR("Yicong Yang <yangyicong@hisilicon.com>");
// MODULE_AUTHOR("Yushan Wang <wangyushan12@huawei.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
