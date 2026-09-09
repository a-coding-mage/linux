// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/range.c
 * comedi routines for voltage ranges
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-8 David A. Schleef <ds@schleef.org>
 */

// Dependencies supplied by the surrounding comedi and Linux interfaces.

pub static range_bipolar10: comedi_lrange = comedi_lrange {
    length: 1,
    range: [BIP_RANGE!(10)],
};
pub static range_bipolar5: comedi_lrange = comedi_lrange {
    length: 1,
    range: [BIP_RANGE!(5)],
};
pub static range_bipolar2_5: comedi_lrange = comedi_lrange {
    length: 1,
    range: [BIP_RANGE!(2.5)],
};
pub static range_unipolar10: comedi_lrange = comedi_lrange {
    length: 1,
    range: [UNI_RANGE!(10)],
};
pub static range_unipolar5: comedi_lrange = comedi_lrange {
    length: 1,
    range: [UNI_RANGE!(5)],
};
pub static range_unipolar2_5: comedi_lrange = comedi_lrange {
    length: 1,
    range: [UNI_RANGE!(2.5)],
};
pub static range_0_20mA: comedi_lrange = comedi_lrange {
    length: 1,
    range: [RANGE_mA!(0, 20)],
};
pub static range_4_20mA: comedi_lrange = comedi_lrange {
    length: 1,
    range: [RANGE_mA!(4, 20)],
};
pub static range_0_32mA: comedi_lrange = comedi_lrange {
    length: 1,
    range: [RANGE_mA!(0, 32)],
};
pub static range_unknown: comedi_lrange = comedi_lrange {
    length: 1,
    range: [comedi_krange { min: 0, max: 1_000_000, flags: UNIT_none }],
};

/*
 * COMEDI_RANGEINFO ioctl
 * range information
 */
pub unsafe fn do_rangeinfo_ioctl(
    dev: *mut comedi_device,
    it: *mut comedi_rangeinfo,
) -> i32 {
    let subd: i32 = ((*it).range_type >> 24 & 0xff) as i32;
    let chan: i32 = ((*it).range_type >> 16 & 0xff) as i32;
    let lr: *const comedi_lrange;
    let s: *mut comedi_subdevice;

    if !(*dev).attached {
        return -EINVAL;
    }
    if subd >= (*dev).n_subdevices {
        return -EINVAL;
    }
    s = (*dev).subdevices.add(subd as usize);
    if !(*s).range_table.is_null() {
        lr = (*s).range_table;
    } else if !(*s).range_table_list.is_null() {
        if chan >= (*s).n_chan {
            return -EINVAL;
        }
        lr = *(*s).range_table_list.add(chan as usize);
    } else {
        return -EINVAL;
    }

    if RANGE_LENGTH!((*it).range_type) != (*lr).length {
        dev_dbg!((*dev).class_dev,
                 "wrong length %d should be %d (0x%08x)\n",
                 RANGE_LENGTH!((*it).range_type), (*lr).length,
                 (*it).range_type);
        return -EINVAL;
    }

    if copy_to_user((*it).range_ptr as *mut _, (*lr).range.as_ptr() as *const _,
                    core::mem::size_of::<comedi_krange>() * (*lr).length as usize) != 0 {
        return -EFAULT;
    }

    0
}

pub unsafe fn comedi_check_chanlist(
    s: *mut comedi_subdevice,
    n: i32,
    chanlist: *mut u32,
) -> i32 {
    let dev = (*s).device;
    let mut chanspec: u32;
    let mut chan: i32;
    let mut range_len: i32;

    for i in 0..n {
        chanspec = *chanlist.add(i as usize);
        chan = CR_CHAN!(chanspec);
        if !(*s).range_table.is_null() {
            range_len = (*(*s).range_table).length;
        } else if !(*s).range_table_list.is_null() && chan < (*s).n_chan {
            range_len = (*(*(*s).range_table_list.add(chan as usize))).length;
        } else {
            range_len = 0;
        }
        if chan >= (*s).n_chan || CR_RANGE!(chanspec) >= range_len {
            dev_warn!((*dev).class_dev,
                      "bad chanlist[%d]=0x%08x chan=%d range length=%d\n",
                      i, chanspec, chan, range_len);
            return -EINVAL;
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
