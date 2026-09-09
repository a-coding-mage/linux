/*
 *  This program is free software; you can redistribute  it and/or modify it
 *  under the terms of  the GNU General  Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 *
 *  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 *  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN
 *  NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 *  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *  NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
 *  USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
 *  ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 *  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 *  THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  You should have received a copy of the GNU General Public License along
 *  with this program; if not, write to the Free Software Foundation, Inc.,
 *  675 Mass Ave, Cambridge, MA 02139, USA.
 *
 * Copyright 2002 MontaVista Software Inc.
 * Author: MontaVista Software, Inc.
 *              stevel@mvista.com or source@mvista.com
 */

#[repr(C)]
struct IntrGroup {
    mask: u32,
    base_addr: *mut u32,
}

const RC32434_NR_IRQS: usize = GROUP4_IRQ_BASE + 32;

static INTR_GROUP: [IntrGroup; NUM_INTR_GROUPS] = [
    IntrGroup { mask: 0x0000efff, base_addr: (KSEG1ADDR(IC_GROUP0_PEND + 0 * IC_GROUP_OFFSET)) as *mut u32 },
    IntrGroup { mask: 0x00001fff, base_addr: (KSEG1ADDR(IC_GROUP0_PEND + 1 * IC_GROUP_OFFSET)) as *mut u32 },
    IntrGroup { mask: 0x00000007, base_addr: (KSEG1ADDR(IC_GROUP0_PEND + 2 * IC_GROUP_OFFSET)) as *mut u32 },
    IntrGroup { mask: 0x0003ffff, base_addr: (KSEG1ADDR(IC_GROUP0_PEND + 3 * IC_GROUP_OFFSET)) as *mut u32 },
    IntrGroup { mask: 0xffffffff, base_addr: (KSEG1ADDR(IC_GROUP0_PEND + 4 * IC_GROUP_OFFSET)) as *mut u32 },
];

#[inline]
unsafe fn read_pend(base: *mut u32) -> u32 { core::ptr::read_volatile(base) }
#[inline]
unsafe fn read_mask(base: *mut u32) -> u32 { core::ptr::read_volatile(base.add(2)) }
#[inline]
unsafe fn write_mask(base: *mut u32, val: u32) { core::ptr::write_volatile(base.add(2), val); }

#[inline]
fn irq_to_group(irq_nr: u32) -> i32 { ((irq_nr - GROUP0_IRQ_BASE) >> 5) as i32 }

#[inline]
fn group_to_ip(group: u32) -> u32 { group + 2 }

#[inline]
unsafe fn enable_local_irq(ip: u32) { set_c0_status(0x100i32 << ip); }

#[inline]
unsafe fn disable_local_irq(ip: u32) { clear_c0_status(0x100i32 << ip); }

#[inline]
unsafe fn ack_local_irq(ip: u32) { clear_c0_cause(0x100i32 << ip); }

unsafe fn rb532_enable_irq(d: *mut irq_data) {
    let mut group: u32;
    let mut intr_bit: u32;
    let irq_nr = (*d).irq;
    let mut ip = irq_nr as i32 - GROUP0_IRQ_BASE as i32;
    if ip < 0 {
        enable_local_irq(irq_nr);
    } else {
        group = (ip as u32) >> 5;
        ip &= (1 << 5) - 1;
        intr_bit = 1 << ip;
        enable_local_irq(group_to_ip(group));
        let addr = INTR_GROUP[group as usize].base_addr;
        write_mask(addr, read_mask(addr) & !intr_bit);
    }
}

unsafe fn rb532_disable_irq(d: *mut irq_data) {
    let irq_nr = (*d).irq;
    let mut ip = irq_nr as i32 - GROUP0_IRQ_BASE as i32;
    if ip < 0 {
        disable_local_irq(irq_nr);
    } else {
        let group = (ip as u32) >> 5;
        ip &= (1 << 5) - 1;
        let intr_bit = 1 << ip;
        let addr = INTR_GROUP[group as usize].base_addr;
        let mut mask = read_mask(addr);
        mask |= intr_bit;
        write_mask(addr, mask);
        /* There is a maximum of 14 GPIO interrupts */
        if group == GPIO_MAPPED_IRQ_GROUP && irq_nr <= GROUP4_IRQ_BASE + 13 {
            rb532_gpio_set_istat(0, irq_nr - GPIO_MAPPED_IRQ_BASE);
        }
        /* if there are no more interrupts enabled in this group, disable corresponding IP */
        if mask == INTR_GROUP[group as usize].mask {
            disable_local_irq(group_to_ip(group));
        }
    }
}

unsafe fn rb532_mask_and_ack_irq(d: *mut irq_data) {
    rb532_disable_irq(d);
    ack_local_irq(group_to_ip(irq_to_group((*d).irq) as u32));
}

unsafe fn rb532_set_type(d: *mut irq_data, type_: u32) -> i32 {
    let gpio = (*d).irq - GPIO_MAPPED_IRQ_BASE;
    let group = irq_to_group((*d).irq);
    if group != GPIO_MAPPED_IRQ_GROUP as i32 || (*d).irq > GROUP4_IRQ_BASE + 13 {
        return if type_ == IRQ_TYPE_LEVEL_HIGH { 0 } else { -EINVAL };
    }
    match type_ {
        IRQ_TYPE_LEVEL_HIGH => rb532_gpio_set_ilevel(1, gpio),
        IRQ_TYPE_LEVEL_LOW => rb532_gpio_set_ilevel(0, gpio),
        _ => return -EINVAL,
    }
    0
}

#[repr(C)]
struct irq_chip {
    name: *const u8,
    irq_ack: Option<unsafe fn(*mut irq_data)>,
    irq_mask: Option<unsafe fn(*mut irq_data)>,
    irq_mask_ack: Option<unsafe fn(*mut irq_data)>,
    irq_unmask: Option<unsafe fn(*mut irq_data)>,
    irq_set_type: Option<unsafe fn(*mut irq_data, u32) -> i32>,
}

static mut RC32434_IRQ_TYPE: irq_chip = irq_chip {
    name: b"RB532\0".as_ptr(),
    irq_ack: Some(rb532_disable_irq), irq_mask: Some(rb532_disable_irq),
    irq_mask_ack: Some(rb532_mask_and_ack_irq), irq_unmask: Some(rb532_enable_irq),
    irq_set_type: Some(rb532_set_type),
};

unsafe fn arch_init_irq() {
    pr_info("Initializing IRQ's: %d out of %d\n", RC32434_NR_IRQS, NR_IRQS);
    for i in 0..RC32434_NR_IRQS { irq_set_chip_and_handler(i, &mut RC32434_IRQ_TYPE, handle_level_irq); }
}

/* Main Interrupt dispatcher */
unsafe fn plat_irq_dispatch() {
    let cp0_cause = read_c0_cause() & read_c0_status();
    if cp0_cause & CAUSEF_IP7 != 0 { do_IRQ(7); }
    else {
        let ip = cp0_cause & 0x7c00;
        if ip != 0 {
            let group = (21 + (fls(ip) - 32)) as usize;
            let addr = INTR_GROUP[group].base_addr;
            let mut pend = read_pend(addr);
            pend &= !read_mask(addr);
            pend = 39 + (fls(pend) - 32);
            do_IRQ(((group as u32) << 5) + pend);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
