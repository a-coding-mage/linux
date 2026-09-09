// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Silex Insight

// External Linux kernel dependencies supplied by other files.

const BA431_RESET_DELAY: u32 = 1; // usec
const BA431_RESET_READ_STATUS_TIMEOUT: u32 = 1000; // usec
const BA431_RESET_READ_STATUS_INTERVAL: u32 = 10; // usec
const BA431_READ_RETRY_INTERVAL: u32 = 1; // usec

const BA431_REG_CTRL: u32 = 0x00;
const BA431_REG_FIFO_LEVEL: u32 = 0x04;
const BA431_REG_STATUS: u32 = 0x30;
const BA431_REG_FIFODATA: u32 = 0x80;

const BA431_CTRL_ENABLE: u32 = 1 << 0;
const BA431_CTRL_SOFTRESET: u32 = 1 << 8;

const BA431_STATUS_STATE_MASK: u32 = (1 << 1) | (1 << 2) | (1 << 3);
const BA431_STATUS_STATE_OFFSET: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Ba431State {
    Reset,
    Startup,
    Fifofullon,
    Fifofulloff,
    Running,
    Error,
}

#[repr(C)]
struct Ba431Trng {
    dev: *mut Device,
    base: *mut core::ffi::c_void,
    rng: Hwrng,
    reset_pending: Atomic,
    reset_work: WorkStruct,
}

unsafe fn ba431_trng_read_reg(ba431: *mut Ba431Trng, reg: u32) -> u32 {
    ioread32((*ba431).base.cast::<u8>().add(reg as usize).cast())
}

unsafe fn ba431_trng_write_reg(ba431: *mut Ba431Trng, reg: u32, val: u32) {
    iowrite32(val, (*ba431).base.cast::<u8>().add(reg as usize).cast());
}

unsafe fn ba431_trng_get_state(ba431: *mut Ba431Trng) -> Ba431State {
    let status = ba431_trng_read_reg(ba431, BA431_REG_STATUS);
    match (status & BA431_STATUS_STATE_MASK) >> BA431_STATUS_STATE_OFFSET {
        0 => Ba431State::Reset,
        1 => Ba431State::Startup,
        2 => Ba431State::Fifofullon,
        3 => Ba431State::Fifofulloff,
        4 => Ba431State::Running,
        _ => Ba431State::Error,
    }
}

unsafe fn ba431_trng_is_in_error(ba431: *mut Ba431Trng) -> i32 {
    let state = ba431_trng_get_state(ba431);
    if state == Ba431State::Reset || state == Ba431State::Error { 1 } else { 0 }
}

unsafe fn ba431_trng_reset(ba431: *mut Ba431Trng) -> i32 {
    ba431_trng_write_reg(ba431, BA431_REG_CTRL, BA431_CTRL_SOFTRESET);
    udelay(BA431_RESET_DELAY);
    ba431_trng_write_reg(ba431, BA431_REG_CTRL, BA431_CTRL_ENABLE);

    let mut ret = 0;
    if readx_poll_timeout(
        ba431_trng_is_in_error,
        ba431,
        &mut ret,
        ret == 0,
        BA431_RESET_READ_STATUS_INTERVAL,
        BA431_RESET_READ_STATUS_TIMEOUT,
    ) != 0 {
        dev_err((*ba431).dev, "reset failed (state: %d)\n", ba431_trng_get_state(ba431));
        return -ETIMEDOUT;
    }
    dev_info((*ba431).dev, "reset done\n");
    0
}

unsafe extern "C" fn ba431_trng_reset_work(work: *mut WorkStruct) {
    let ba431 = container_of!(work, Ba431Trng, reset_work);
    ba431_trng_reset(ba431);
    atomic_set(&mut (*ba431).reset_pending, 0);
}

unsafe fn ba431_trng_schedule_reset(ba431: *mut Ba431Trng) {
    if atomic_cmpxchg(&mut (*ba431).reset_pending, 0, 1) != 0 { return; }
    schedule_work(&mut (*ba431).reset_work);
}

unsafe extern "C" fn ba431_trng_read(
    rng: *mut Hwrng, buf: *mut core::ffi::c_void, mut max: usize, wait: bool,
) -> isize {
    let ba431 = container_of!(rng, Ba431Trng, rng);
    let data = buf.cast::<u32>();
    let mut n: usize = 0;
    while max > 0 {
        let level = ba431_trng_read_reg(ba431, BA431_REG_FIFO_LEVEL);
        if level == 0 {
            if ba431_trng_is_in_error(ba431) != 0 { ba431_trng_schedule_reset(ba431); break; }
            if !wait { break; }
            udelay(BA431_READ_RETRY_INTERVAL);
            continue;
        }
        let mut i = level;
        loop {
            *data.add(n) = ba431_trng_read_reg(ba431, BA431_REG_FIFODATA);
            n += 1;
            max -= core::mem::size_of::<u32>();
            i -= 1;
            if i == 0 || max == 0 { break; }
        }
        if ba431_trng_is_in_error(ba431) != 0 {
            n -= (level - i) as usize;
            ba431_trng_schedule_reset(ba431);
            break;
        }
    }
    let n = n * core::mem::size_of::<u32>();
    if n != 0 || !wait { n as isize } else { -EIO as isize }
}

unsafe extern "C" fn ba431_trng_cleanup(rng: *mut Hwrng) {
    let ba431 = container_of!(rng, Ba431Trng, rng);
    ba431_trng_write_reg(ba431, BA431_REG_CTRL, 0);
    cancel_work_sync(&mut (*ba431).reset_work);
}

unsafe extern "C" fn ba431_trng_init(rng: *mut Hwrng) -> i32 {
    ba431_trng_reset(container_of!(rng, Ba431Trng, rng))
}

// Platform-driver registration and device-tree/module metadata are supplied by the kernel bindings.
static BA431_TRNG_DT_IDS: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "silex-insight,ba431-rng\0" },
    OfDeviceId::sentinel(),
];

static mut BA431_TRNG_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver { name: "ba431-rng\0", of_match_table: BA431_TRNG_DT_IDS.as_ptr() },
    probe: Some(ba431_trng_probe),
};

unsafe fn ba431_trng_probe(pdev: *mut PlatformDevice) -> i32 {
    let ba431 = devm_kzalloc((*pdev).dev, core::mem::size_of::<Ba431Trng>(), GFP_KERNEL)
        as *mut Ba431Trng;
    if ba431.is_null() { return -ENOMEM; }
    (*ba431).dev = &mut (*pdev).dev;
    (*ba431).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*ba431).base) { return ptr_err((*ba431).base); }
    atomic_set(&mut (*ba431).reset_pending, 0);
    init_work(&mut (*ba431).reset_work, ba431_trng_reset_work);
    (*ba431).rng.name = (*pdev).name;
    (*ba431).rng.init = Some(ba431_trng_init);
    (*ba431).rng.cleanup = Some(ba431_trng_cleanup);
    (*ba431).rng.read = Some(ba431_trng_read);
    let ret = devm_hwrng_register(&mut (*pdev).dev, &mut (*ba431).rng);
    if ret != 0 { return dev_err_probe(&mut (*pdev).dev, ret, "BA431 registration failed\n"); }
    dev_info(&mut (*pdev).dev, "BA431 TRNG registered\n");
    0
}

module_platform_driver!(BA431_TRNG_DRIVER);
// MODULE_AUTHOR("Olivier Sobrie <olivier@sobrie.be>");
// MODULE_DESCRIPTION("TRNG driver for Silex Insight BA431");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
