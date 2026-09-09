// SPDX-License-Identifier: GPL-2.0

/*
 * The associativity domain numbers are returned from the hypervisor as a
 * stream of mixed 16-bit and 32-bit fields. The stream is terminated by the
 * special value of "all ones" (aka. 0xffff) and its size may not exceed 48
 * bytes.
 *
 *    --- 16-bit fields -->
 *  _________________________
 *  |  0  |  1  |  2  |  3  |   be_packed[0]
 *  ------+-----+-----+------
 *  _________________________
 *  |  4  |  5  |  6  |  7  |   be_packed[1]
 *  -------------------------
 *            ...
 *  _________________________
 *  | 20  | 21  | 22  | 23  |   be_packed[5]
 *  -------------------------
 *
 * Convert to the sequence they would appear in the ibm,associativity property.
 */
unsafe fn vphn_unpack_associativity(packed: *const libc::c_long, unpacked: *mut __be32) -> libc::c_int {
    let mut be_packed: [u64; VPHN_REGISTER_COUNT] = [0; VPHN_REGISTER_COUNT];
    let mut nr_assoc_doms: libc::c_int = 0;
    let mut field = be_packed.as_ptr() as *const __be16;
    let mut last: u16 = 0;
    let mut is_32bit = false;

    const VPHN_FIELD_UNUSED: u16 = 0xffff;
    const VPHN_FIELD_MSB: u16 = 0x8000;
    const VPHN_FIELD_MASK: u16 = !VPHN_FIELD_MSB;

    /* Let's fix the values returned by plpar_hcall9() */
    for i in 0..VPHN_REGISTER_COUNT {
        be_packed[i] = (packed.add(i).read() as u64).to_be();
    }

    for _i in 1..VPHN_ASSOC_BUFSIZE {
        let new = u16::from_be(field.read_unaligned());
        field = field.add(1);

        if is_32bit {
            /*
             * Let's concatenate the 16 bits of this field to the
             * 15 lower bits of the previous field
             */
            nr_assoc_doms += 1;
            unpacked.add(nr_assoc_doms as usize).write(((((last as u32) << 16) | new as u32)).to_be());
            is_32bit = false;
        } else if new == VPHN_FIELD_UNUSED {
            /* This is the list terminator */
            break;
        } else if new & VPHN_FIELD_MSB != 0 {
            /* Data is in the lower 15 bits of this field */
            nr_assoc_doms += 1;
            unpacked.add(nr_assoc_doms as usize).write(((new & VPHN_FIELD_MASK) as u32).to_be());
        } else {
            /*
             * Data is in the lower 15 bits of this field
             * concatenated with the next 16 bit field
             */
            last = new;
            is_32bit = true;
        }
    }

    /* The first cell contains the length of the property */
    unpacked.write((nr_assoc_doms as u32).to_be());

    nr_assoc_doms
}

/* NOTE: This file is included by a selftest and built in userspace. */
#[cfg(kernel)]
unsafe fn hcall_vphn(cpu: c_ulong, flags: u64, associativity: *mut __be32) -> c_long {
    let mut retbuf: [c_long; PLPAR_HCALL9_BUFSIZE] = [0; PLPAR_HCALL9_BUFSIZE];

    let rc = plpar_hcall9(H_HOME_NODE_ASSOCIATIVITY, retbuf.as_mut_ptr(), flags, cpu);
    if rc == H_SUCCESS {
        vphn_unpack_associativity(retbuf.as_ptr(), associativity);
    }

    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
