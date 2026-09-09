/*
** asm-m68k/pcmcia.c -- Amiga Linux PCMCIA support
**                      most information was found by disassembling card.resource
**                      I'm still looking for an official doc !
**
** Copyright 1997 by Alain Malek
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
**
** Created: 12/10/97 by Alain Malek
*/

/* Dependencies supplied by the surrounding kernel/Amiga code. */

#[repr(C)]
pub struct Gayle {
    pub config: u8,
    pub cardstatus: u8,
}

unsafe extern "C" {
    static mut jiffies: usize;
    static mut gayle_reset: u8;
    static mut gayle_attribute: *mut u8;
    static mut gayle: Gayle;
    static HZ: usize;
}

/* Constants supplied by asm/amigayle.h and asm/amipcmcia.h. */

static mut cfg_byte: u8 = GAYLE_CFG_0V | GAYLE_CFG_150NS;

pub unsafe fn pcmcia_reset() {
    let reset_start_time: usize = jiffies;

    gayle_reset = 0x00;
    while jiffies < reset_start_time + (1 * HZ) / 100 {}
    core::ptr::read_volatile(&gayle_reset);
}
/* EXPORT_SYMBOL(pcmcia_reset); */

/* copy a tuple, including tuple header. return nb bytes copied */
/* be careful as this may trigger a GAYLE_IRQ_WR interrupt ! */

pub unsafe fn pcmcia_copy_tuple(tuple_id: u8, tuple: *mut core::ffi::c_void, max_len: i32) -> i32 {
    let mut id: u8;
    let mut dest: *mut u8 = tuple as *mut u8;
    let mut cnt: i32;
    let mut pos: usize;
    let mut len: i32;

    pos = 0;

    id = *gayle_attribute.add(pos);

    while id != CISTPL_END && pos < 0x10000 {
        len = (*gayle_attribute.add(pos + 2) as i32) + 2;
        if id == tuple_id {
            len = if len > max_len { max_len } else { len };
            cnt = 0;
            while cnt < len {
                *dest = *gayle_attribute.add(pos + ((cnt << 1) as usize));
                dest = dest.add(1);
                cnt += 1;
            }

            return len;
        }
        pos += (len << 1) as usize;
        id = *gayle_attribute.add(pos);
    }

    0
}
/* EXPORT_SYMBOL(pcmcia_copy_tuple); */

pub unsafe fn pcmcia_program_voltage(voltage: i32) {
    let v: u8;

    v = match voltage {
        PCMCIA_0V => GAYLE_CFG_0V,
        PCMCIA_5V => GAYLE_CFG_5V,
        PCMCIA_12V => GAYLE_CFG_12V,
        _ => GAYLE_CFG_0V,
    };

    cfg_byte = (cfg_byte & 0xfc) | v;
    gayle.config = cfg_byte;
}
/* EXPORT_SYMBOL(pcmcia_program_voltage); */

pub unsafe fn pcmcia_access_speed(speed: i32) {
    let s: u8;

    if speed <= PCMCIA_SPEED_100NS {
        s = GAYLE_CFG_100NS;
    } else if speed <= PCMCIA_SPEED_150NS {
        s = GAYLE_CFG_150NS;
    } else if speed <= PCMCIA_SPEED_250NS {
        s = GAYLE_CFG_250NS;
    } else {
        s = GAYLE_CFG_720NS;
    }

    cfg_byte = (cfg_byte & 0xf3) | s;
    gayle.config = cfg_byte;
}
/* EXPORT_SYMBOL(pcmcia_access_speed); */

pub unsafe fn pcmcia_write_enable() {
    gayle.cardstatus = GAYLE_CS_WR | GAYLE_CS_DA;
}
/* EXPORT_SYMBOL(pcmcia_write_enable); */

pub unsafe fn pcmcia_write_disable() {
    gayle.cardstatus = 0;
}
/* EXPORT_SYMBOL(pcmcia_write_disable); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
