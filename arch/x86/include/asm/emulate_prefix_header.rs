/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Virt escape sequences to trigger instruction emulation;
 * ideally these would decode to 'whole' instruction and not destroy
 * the instruction stream; sadly this is not true for the 'kvm' one :/
 */

/* ud2 ; .ascii "xen" */
pub const __XEN_EMULATE_PREFIX: [u8; 5] = [0x0f, 0x0b, 0x78, 0x65, 0x6e];

/* ud2 ; .ascii "kvm" */
pub const __KVM_EMULATE_PREFIX: [u8; 5] = [0x0f, 0x0b, 0x6b, 0x76, 0x6d];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
