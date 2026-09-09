/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2009, Wind River Systems Inc
 * Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 */

// Dependencies supplied by the surrounding kernel translation.

core::arch::global_asm!(r#"
    .global raw_copy_from_user
    .type raw_copy_from_user, @function
raw_copy_from_user:
    movi  r2,7
    mov   r3,r4
    bge   r2,r6,1f
    xor   r2,r4,r5
    andi  r2,r2,3
    movi  r7,3
    beq   r2,zero,4f
1:  addi  r6,r6,-1
    movi  r2,-1
    beq   r6,r2,3f
    mov   r7,r2
2:  ldbu  r2,0(r5)
    addi  r6,r6,-1
    addi  r5,r5,1
    stb   r2,0(r3)
    addi  r3,r3,1
    bne   r6,r7,2b
3:
    addi  r2,r6,1
    ret
13: mov   r2,r6
    ret
4:  andi  r2,r4,1
    cmpeq r2,r2,zero
    beq   r2,zero,7f
5:  andi  r2,r3,2
    beq   r2,zero,6f
9:  ldhu  r2,0(r5)
    addi  r6,r6,-2
    addi  r5,r5,2
    sth   r2,0(r3)
    addi  r3,r3,2
6:  bge   r7,r6,1b
10: ldw   r2,0(r5)
    addi  r6,r6,-4
    addi  r5,r5,4
    stw   r2,0(r3)
    addi  r3,r3,4
    br    6b
7:  ldbu  r2,0(r5)
    addi  r6,r6,-1
    addi  r5,r5,1
    addi  r3,r4,1
    stb   r2,0(r4)
    br    5b
    .section __ex_table,"a"
    .word 2b,3b
    .word 9b,13b
    .word 10b,13b
    .word 7b,13b
    .previous
"#);

// EXPORT_SYMBOL(raw_copy_from_user);

core::arch::global_asm!(r#"
    .global raw_copy_to_user
    .type raw_copy_to_user, @function
raw_copy_to_user:
    movi  r2,7
    mov   r3,r4
    bge   r2,r6,1f
    xor   r2,r4,r5
    andi  r2,r2,3
    movi  r7,3
    beq   r2,zero,4f
    /* Bail if we try to copy zero bytes */
1:  addi  r6,r6,-1
    movi  r2,-1
    beq   r6,r2,3f
    /* Copy byte by byte for small copies and if src^dst != 0 */
    mov   r7,r2
2:  ldbu  r2,0(r5)
    addi  r5,r5,1
9:  stb   r2,0(r3)
    addi  r6,r6,-1
    addi  r3,r3,1
    bne   r6,r7,2b
3:  addi  r2,r6,1
    ret
13: mov   r2,r6
    ret
    /* If 'to' is an odd address byte copy */
4:  andi  r2,r4,1
    cmpeq r2,r2,zero
    beq   r2,zero,7f
    /* If 'to' is not divideable by four copy halfwords */
5:  andi  r2,r3,2
    beq   r2,zero,6f
    ldhu  r2,0(r5)
    addi  r5,r5,2
10: sth   r2,0(r3)
    addi  r6,r6,-2
    addi  r3,r3,2
    /* Copy words */
6:  bge   r7,r6,1b
    ldw   r2,0(r5)
    addi  r5,r5,4
11: stw   r2,0(r3)
    addi  r6,r6,-4
    addi  r3,r3,4
    br    6b
    /* Copy remaining bytes */
7:  ldbu  r2,0(r5)
    addi  r5,r5,1
    addi  r3,r4,1
12: stb   r2,0(r4)
    addi  r6,r6,-1
    br    5b
    .section __ex_table,"a"
    .word 9b,3b
    .word 10b,13b
    .word 11b,13b
    .word 12b,13b
    .previous
"#);

// EXPORT_SYMBOL(raw_copy_to_user);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
