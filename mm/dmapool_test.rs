// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel headers are intentionally external.

const NR_TESTS: usize = 100;

#[repr(C)]
struct dma_pool_pair {
    dma: dma_addr_t,
    v: *mut core::ffi::c_void,
}

#[repr(C)]
struct dmapool_parms {
    size: usize,
    align: usize,
    boundary: usize,
}

static pool_parms: [dmapool_parms; 6] = [
    dmapool_parms { size: 16, align: 16, boundary: 0 },
    dmapool_parms { size: 64, align: 64, boundary: 0 },
    dmapool_parms { size: 256, align: 256, boundary: 0 },
    dmapool_parms { size: 1024, align: 1024, boundary: 0 },
    dmapool_parms { size: 4096, align: 4096, boundary: 0 },
    dmapool_parms { size: 68, align: 32, boundary: 4096 },
];

static mut pool: *mut dma_pool = core::ptr::null_mut();
static mut test_dev: device = unsafe { core::mem::zeroed() };
static mut dma_mask: u64 = 0;

#[inline]
fn nr_blocks(size: i32) -> i32 {
    let value = ((PAGE_SIZE / size) * 512) as i32;
    value.clamp(1024, 8192)
}

unsafe fn dmapool_test_alloc(p: *mut dma_pool_pair, blocks: i32) -> i32 {
    let mut i = 0;
    while i < blocks {
        (*p.add(i as usize)).v = dma_pool_alloc(pool, GFP_KERNEL, &mut (*p.add(i as usize)).dma);
        if (*p.add(i as usize)).v.is_null() {
            break;
        }
        i += 1;
    }

    if i == blocks {
        let mut j = 0;
        while j < blocks {
            dma_pool_free(pool, (*p.add(j as usize)).v, (*p.add(j as usize)).dma);
            j += 1;
        }
        return 0;
    }

    i -= 1;
    while i >= 0 {
        dma_pool_free(pool, (*p.add(i as usize)).v, (*p.add(i as usize)).dma);
        i -= 1;
    }
    -ENOMEM
}

unsafe fn dmapool_test_block(parms: *const dmapool_parms) -> i32 {
    let blocks = nr_blocks((*parms).size as i32);
    let mut start_time: ktime_t;
    let mut end_time: ktime_t;
    let p: *mut dma_pool_pair = kzalloc_objs::<dma_pool_pair>(blocks);
    let mut ret: i32;

    if p.is_null() {
        return -ENOMEM;
    }

    pool = dma_pool_create(c"test pool".as_ptr() as *const i8, &mut test_dev,
                           (*parms).size, (*parms).align, (*parms).boundary);
    if pool.is_null() {
        ret = -ENOMEM;
        kfree(p as *mut core::ffi::c_void);
        return ret;
    }

    start_time = ktime_get();
    let mut i = 0;
    while i < NR_TESTS {
        ret = dmapool_test_alloc(p, blocks);
        if ret != 0 {
            dma_pool_destroy(pool);
            kfree(p as *mut core::ffi::c_void);
            return ret;
        }
        if need_resched() != 0 {
            cond_resched();
        }
        i += 1;
    }
    end_time = ktime_get();

    printk(c"dmapool test: size:%-4zu align:%-4zu blocks:%-4d time:%llu\n".as_ptr() as *const i8,
           (*parms).size, (*parms).align, blocks,
           ktime_us_delta(end_time, start_time));

    dma_pool_destroy(pool);
    kfree(p as *mut core::ffi::c_void);
    0
}

unsafe extern "C" fn dmapool_test_release(_dev: *mut device) {}

unsafe fn dmapool_checks() -> i32 {
    let mut ret = dev_set_name(&mut test_dev, c"dmapool-test".as_ptr() as *const i8);
    if ret != 0 {
        return ret;
    }

    ret = device_register(&mut test_dev);
    if ret != 0 {
        printk(c"%s: register failed:%d\n".as_ptr() as *const i8, c"dmapool_checks".as_ptr(), ret);
        put_device(&mut test_dev);
        return ret;
    }

    test_dev.release = Some(dmapool_test_release);
    set_dma_ops(&mut test_dev, core::ptr::null());
    test_dev.dma_mask = &mut dma_mask;
    ret = dma_set_mask_and_coherent(&mut test_dev, DMA_BIT_MASK(64));
    if ret != 0 {
        printk(c"%s: mask failed:%d\n".as_ptr() as *const i8, c"dmapool_checks".as_ptr(), ret);
        device_del(&mut test_dev);
        put_device(&mut test_dev);
        return ret;
    }

    let mut i = 0;
    while i < pool_parms.len() {
        ret = dmapool_test_block(&pool_parms[i]);
        if ret != 0 {
            break;
        }
        i += 1;
    }

    device_del(&mut test_dev);
    put_device(&mut test_dev);
    ret
}

unsafe extern "C" fn dmapool_exit() {}

// module_init(dmapool_checks);
// module_exit(dmapool_exit);
// MODULE_DESCRIPTION("dma_pool timing test");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
