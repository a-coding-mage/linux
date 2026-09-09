// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel and architecture sources.

const GFPIC_REG_IRQ_PENDING: usize = 0x04;
const GFPIC_REG_IRQ_DISABLE_ALL: usize = 0x08;
const GFPIC_REG_IRQ_DISABLE: usize = 0x0c;
const GFPIC_REG_IRQ_ENABLE: usize = 0x10;

static mut picres: [resource; 6] = [resource::default(); 6];
static picname: [&'static [u8]; 6] = [
    b"goldfish_pic.0\0",
    b"goldfish_pic.1\0",
    b"goldfish_pic.2\0",
    b"goldfish_pic.3\0",
    b"goldfish_pic.4\0",
    b"goldfish_pic.5\0",
];

/*
 * 6 goldfish-pic for CPU IRQ #1 to IRQ #6
 * CPU IRQ #1 -> PIC #1
 *               IRQ #1 to IRQ #31 -> unused
 *               IRQ #32 -> goldfish-tty
 * CPU IRQ #2 -> PIC #2
 *               IRQ #1 to IRQ #32 -> virtio-mmio from 1 to 32
 * CPU IRQ #3 -> PIC #3
 *               IRQ #1 to IRQ #32 -> virtio-mmio from 33 to 64
 * CPU IRQ #4 -> PIC #4
 *               IRQ #1 to IRQ #32 -> virtio-mmio from 65 to 96
 * CPU IRQ #5 -> PIC #5
 *               IRQ #1 to IRQ #32 -> virtio-mmio from 97 to 128
 * CPU IRQ #6 -> PIC #6
 *               IRQ #1 -> goldfish-timer
 *               IRQ #2 -> goldfish-rtc
 *               IRQ #3 to IRQ #32 -> unused
 * CPU IRQ #7 -> NMI
 */

unsafe fn gfpic_read(pic: i32, reg: usize) -> u32 {
    let base = (virt_bi_data.pic.mmio + (pic as usize) * 0x1000) as *mut core::ffi::c_void;
    ioread32be((base as *mut u8).add(reg))
}

unsafe fn gfpic_write(value: u32, pic: i32, reg: usize) {
    let base = (virt_bi_data.pic.mmio + (pic as usize) * 0x1000) as *mut core::ffi::c_void;
    iowrite32be(value, (base as *mut u8).add(reg));
}

#[inline]
const fn gf_pic(irq: u32) -> i32 { ((irq - IRQ_USER) / 32) as i32 }
#[inline]
const fn gf_irq(irq: u32) -> u32 { (irq - IRQ_USER) % 32 }

unsafe fn virt_irq_enable(data: *mut irq_data) {
    gfpic_write(1u32.wrapping_shl(gf_irq((*data).irq)), gf_pic((*data).irq), GFPIC_REG_IRQ_ENABLE);
}

unsafe fn virt_irq_disable(data: *mut irq_data) {
    gfpic_write(1u32.wrapping_shl(gf_irq((*data).irq)), gf_pic((*data).irq), GFPIC_REG_IRQ_DISABLE);
}

unsafe fn virt_irq_startup(data: *mut irq_data) -> u32 {
    virt_irq_enable(data);
    0
}

unsafe fn virt_nmi_handler(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    static mut in_nmi: i32 = 0;
    if core::ptr::read_volatile(&in_nmi) != 0 { return IRQ_HANDLED; }
    core::ptr::write_volatile(&mut in_nmi, 1);
    pr_warn(b"Non-Maskable Interrupt\n\0".as_ptr());
    show_registers(get_irq_regs());
    core::ptr::write_volatile(&mut in_nmi, 0);
    IRQ_HANDLED
}

static mut virt_irq_chip: irq_chip = irq_chip {
    name: b"virt\0".as_ptr(), irq_enable: Some(virt_irq_enable),
    irq_disable: Some(virt_irq_disable), irq_startup: Some(virt_irq_startup),
    irq_shutdown: Some(virt_irq_disable),
};

unsafe fn goldfish_pic_irq(desc: *mut irq_desc) {
    let mut irq_pending = gfpic_read((*desc).irq_data.irq as i32 - 1, GFPIC_REG_IRQ_PENDING);
    let mut irq_num = IRQ_USER + (((*desc).irq_data.irq - 1) * 32);
    loop {
        if irq_pending & 1 != 0 { generic_handle_irq(irq_num); }
        irq_num += 1;
        irq_pending >>= 1;
        if irq_pending == 0 { break; }
    }
}

unsafe fn virt_init_IRQ() {
    m68k_setup_irq_controller(&mut virt_irq_chip, handle_simple_irq, IRQ_USER,
                              NUM_VIRT_SOURCES - IRQ_USER);
    for i in 0..6 {
        picres[i] = define_res_mem_named(virt_bi_data.pic.mmio + i * 0x1000,
                                         0x1000, picname[i].as_ptr());
        if request_resource(&mut iomem_resource, &mut picres[i]) != 0 {
            pr_err(b"Cannot allocate %s resource\n\0".as_ptr(), picname[i].as_ptr());
            return;
        }
        irq_set_chained_handler(virt_bi_data.pic.irq + i, goldfish_pic_irq);
    }
    if request_irq(IRQ_AUTO_7, virt_nmi_handler, 0, b"NMI\0".as_ptr(),
                   virt_nmi_handler as *mut core::ffi::c_void) != 0 {
        pr_err(b"Couldn't register NMI\n\0".as_ptr());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
