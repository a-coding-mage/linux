// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Synopsys, Inc. (www.synopsys.com)
 */

// Linux kernel and ARC architecture dependencies are supplied externally.

const NR_EXCEPTIONS: u32 = 16;

#[repr(C)]
pub struct BcrIrqArcv2 {
    pub value: u32,
}

impl BcrIrqArcv2 {
    #[inline]
    pub fn prio(&self) -> u32 { (self.value >> 24) & 0x0f }
    #[inline]
    pub fn firq(&self) -> u32 { (self.value >> 28) & 0x01 }
    #[inline]
    pub fn irqs(&self) -> u32 { (self.value >> 8) & 0xff }
}

#[repr(C)]
pub struct AuxIrqCtrl {
    pub value: u32,
}

impl AuxIrqCtrl {
    #[inline]
    pub fn set_save_nr_gpr_pairs(&mut self, value: u32) {
        self.value = (self.value & !0x1f) | (value & 0x1f);
    }
    #[inline]
    pub fn set_save_blink(&mut self, value: u32) {
        self.value = (self.value & !(1 << 9)) | ((value & 1) << 9);
    }
    #[inline]
    pub fn set_save_lp_regs(&mut self, value: u32) {
        self.value = (self.value & !(1 << 10)) | ((value & 1) << 10);
    }
    #[inline]
    pub fn set_save_u_to_u(&mut self, value: u32) {
        self.value = (self.value & !(1 << 11)) | ((value & 1) << 11);
    }
    #[inline]
    pub fn set_save_idx_regs(&mut self, value: u32) {
        self.value = (self.value & !(1 << 13)) | ((value & 1) << 13);
    }
}

extern "C" {
    static ARCV2_IRQ_DEF_PRIO: u32;
    static AUX_IRQ_CTRL: u32;
    static ARC_REG_IRQ_BCR: u32;
    static AUX_IRQ_SELECT: u32;
    static AUX_IRQ_PRIORITY: u32;
    static AUX_IRQ_ENABLE: u32;
    static ARC_REG_STATUS32: u32;
    static STATUS_IE_MASK: u32;
    static FIRST_EXT_IRQ: u32;
    static IPI_IRQ: u32;
    static SOFTIRQ_IRQ: u32;

    fn write_aux_reg(reg: u32, value: u32);
    fn read_aux_reg(reg: u32) -> u32;
    fn write_aux(reg: u32, value: AuxIrqCtrl);
    fn read_bcr(reg: u32, value: *mut BcrIrqArcv2);
    fn pr_info(fmt: *const u8, ...);
    fn panic(fmt: *const u8, ... ) -> !;
}

#[repr(C)]
pub struct IrqData { pub hwirq: u32 }

#[repr(C)]
pub struct IrqChip {
    pub name: *const u8,
    pub irq_mask: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_enable: Option<unsafe extern "C" fn(*mut IrqData)>,
}

extern "C" {
    fn irq_set_percpu_devid(irq: u32);
    fn irq_set_chip_and_handler(irq: u32, chip: *mut IrqChip, handler: *const ());
    fn irq_domain_xlate_onecell();
    fn handle_percpu_irq();
    fn handle_level_irq();
}

/*
 * Early Hardware specific Interrupt setup
 * -Called very early (start_kernel -> setup_arch -> setup_processor)
 * -Platform Independent (must for any ARC Core)
 * -Needed for each CPU (hence not foldable into init_IRQ)
 */
#[no_mangle]
pub unsafe extern "C" fn arc_init_IRQ() {
    let mut tmp: u32;
    let mut irq_prio: u32;
    let mut irq_bcr = BcrIrqArcv2 { value: 0 };
    let mut ictrl = AuxIrqCtrl { value: 0 };

    // CONFIG_ARC_IRQ_NO_AUTOSAVE controls this block at build time.
    ictrl.set_save_nr_gpr_pairs(6); // r0 to r11 (r12 saved manually)
    ictrl.set_save_blink(1);
    ictrl.set_save_lp_regs(1); // LP_COUNT, LP_START, LP_END
    ictrl.set_save_u_to_u(0); // user ctxt saved on kernel stack
    ictrl.set_save_idx_regs(1); // JLI, LDI, EI

    write_aux(*(&AUX_IRQ_CTRL), ictrl);

    /*
     * ARCv2 core intc provides multiple interrupt priorities (up to 16).
     * Typical builds though have only two levels (0-high, 1-low)
     * Linux by default uses lower prio 1 for most irqs, reserving 0 for
     * NMI style interrupts in future (say perf)
     */
    read_bcr(*(&ARC_REG_IRQ_BCR), &mut irq_bcr);
    irq_prio = irq_bcr.prio(); // Encoded as N-1 for N levels
    let _ = irq_prio;

    /* Set defaults and disable private-per-core IRQ lines. */
    let mut i = NR_EXCEPTIONS;
    while i < irq_bcr.irqs() + NR_EXCEPTIONS {
        write_aux_reg(*(&AUX_IRQ_SELECT), i);
        write_aux_reg(*(&AUX_IRQ_PRIORITY), *(&ARCV2_IRQ_DEF_PRIO));
        if i < *(&FIRST_EXT_IRQ) {
            write_aux_reg(*(&AUX_IRQ_ENABLE), 0);
        }
        i = i.wrapping_add(1);
    }

    /* setup status32, don't enable intr yet as kernel doesn't want */
    tmp = read_aux_reg(*(&ARC_REG_STATUS32));
    tmp |= *(&ARCV2_IRQ_DEF_PRIO) << 1;
    tmp &= !*(&STATUS_IE_MASK);
    core::arch::asm!("kflag {0}", in(reg) tmp);
}

unsafe extern "C" fn arcv2_irq_mask(data: *mut IrqData) {
    write_aux_reg(*(&AUX_IRQ_SELECT), (*data).hwirq);
    write_aux_reg(*(&AUX_IRQ_ENABLE), 0);
}

unsafe extern "C" fn arcv2_irq_unmask(data: *mut IrqData) {
    write_aux_reg(*(&AUX_IRQ_SELECT), (*data).hwirq);
    write_aux_reg(*(&AUX_IRQ_ENABLE), 1);
}

unsafe extern "C" fn arcv2_irq_enable(data: *mut IrqData) {
    /* set default priority */
    write_aux_reg(*(&AUX_IRQ_SELECT), (*data).hwirq);
    write_aux_reg(*(&AUX_IRQ_PRIORITY), *(&ARCV2_IRQ_DEF_PRIO));
    /* hw auto enables (linux unmask) all by default */
    write_aux_reg(*(&AUX_IRQ_ENABLE), 1);
}

static mut ARCV2_IRQ_CHIP: IrqChip = IrqChip {
    name: b"ARCv2 core Intc\0".as_ptr(),
    irq_mask: Some(arcv2_irq_mask),
    irq_unmask: Some(arcv2_irq_unmask),
    irq_enable: Some(arcv2_irq_enable),
};

unsafe extern "C" fn arcv2_irq_map(_d: *mut (), irq: u32, hw: u32) -> i32 {
    if hw < *(&FIRST_EXT_IRQ) {
        irq_set_percpu_devid(irq);
        irq_set_chip_and_handler(irq, &raw mut ARCV2_IRQ_CHIP, handle_percpu_irq as *const ());
    } else {
        irq_set_chip_and_handler(irq, &raw mut ARCV2_IRQ_CHIP, handle_level_irq as *const ());
    }
    0
}

// IRQ domain operations: .xlate = irq_domain_xlate_onecell, .map = arcv2_irq_map.
unsafe extern "C" fn init_onchip_IRQ(_intc: *mut (), parent: *mut ()) -> i32 {
    let mut irq_bcr = BcrIrqArcv2 { value: 0 };
    read_bcr(*(&ARC_REG_IRQ_BCR), &mut irq_bcr);
    let nr_cpu_irqs = irq_bcr.irqs() + NR_EXCEPTIONS;
    if !parent.is_null() {
        panic(b"DeviceTree incore intc not a root irq controller\n\0".as_ptr());
    }
    let root_domain = irq_domain_create_linear(_intc, nr_cpu_irqs);
    if root_domain.is_null() {
        panic(b"root irq domain not avail\n\0".as_ptr());
    }
    irq_set_default_domain(root_domain);
    // CONFIG_SMP: irq_create_mapping(root_domain, IPI_IRQ);
    irq_create_mapping(root_domain, *(&SOFTIRQ_IRQ));
    0
}

extern "C" {
    fn irq_domain_create_linear(fwnode: *mut (), size: u32) -> *mut ();
    fn irq_set_default_domain(domain: *mut ());
    fn irq_create_mapping(domain: *mut (), hwirq: u32) -> u32;
}

// IRQCHIP_DECLARE(arc_intc, "snps,archs-intc", init_onchip_IRQ);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
