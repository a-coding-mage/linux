// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains instructions for testing by the test titled:
 *
 *         "Test x86 instruction decoder - new instructions"
 *
 * Note that the 'Expecting' comment lines are consumed by the
 * gen-insn-x86-dat.awk script and have the format:
 *
 *         Expecting: <op> <branch> <rel>
 *
 * If this file is changed, remember to run the gen-insn-x86-dat.sh
 * script and commit the result.
 *
 * Refer to insn-x86.c for more details.
 */

pub unsafe fn main() -> i32
{
	/* Following line is a marker for the awk script - do not change */
	core::arch::asm!("rdtsc", options(att_syntax)); /* Start here */

	/* Test fix for vcvtph2ps in x86-opcode-map.txt */

	core::arch::asm!("vcvtph2ps %xmm3,%ymm5", options(att_syntax));

#[cfg(target_arch = "x86_64")]
{

	/* AVX-512: Instructions with the same op codes as Mask Instructions  */

	core::arch::asm!("cmovno %rax,%rbx", options(att_syntax));
	core::arch::asm!("cmovno 0x12345678(%rax),%rcx", options(att_syntax));
	core::arch::asm!("cmovno 0x12345678(%rax),%cx", options(att_syntax));

	core::arch::asm!("cmove  %rax,%rbx", options(att_syntax));
	core::arch::asm!("cmove 0x12345678(%rax),%rcx", options(att_syntax));
	core::arch::asm!("cmove 0x12345678(%rax),%cx", options(att_syntax));

	core::arch::asm!("seto    0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("setno   0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("setb    0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("setc    0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("setnae  0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("setae   0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("setnb   0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("setnc   0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("sets    0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("setns   0x12345678(%rax)", options(att_syntax));

	/* AVX-512: Mask Instructions */

	core::arch::asm!("kandw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandd  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kandnw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandnq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandnb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandnd  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("knotw  %k7,%k6", options(att_syntax));
	core::arch::asm!("knotq  %k7,%k6", options(att_syntax));
	core::arch::asm!("knotb  %k7,%k6", options(att_syntax));
	core::arch::asm!("knotd  %k7,%k6", options(att_syntax));

	core::arch::asm!("korw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("korq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("korb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kord  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kxnorw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxnorq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxnorb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxnord  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kxorw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxorq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxorb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxord  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kaddw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kaddq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kaddb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kaddd  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kunpckbw %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kunpckwd %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kunpckdq %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kmovw  %k6,%k5", options(att_syntax));
	core::arch::asm!("kmovw  (%rcx),%k5", options(att_syntax));
	core::arch::asm!("kmovw  0x123(%rax,%r14,8),%k5", options(att_syntax));
	core::arch::asm!("kmovw  %k5,(%rcx)", options(att_syntax));
	core::arch::asm!("kmovw  %k5,0x123(%rax,%r14,8)", options(att_syntax));
	core::arch::asm!("kmovw  %eax,%k5", options(att_syntax));
	core::arch::asm!("kmovw  %ebp,%k5", options(att_syntax));
	core::arch::asm!("kmovw  %r13d,%k5", options(att_syntax));
	core::arch::asm!("kmovw  %k5,%eax", options(att_syntax));
	core::arch::asm!("kmovw  %k5,%ebp", options(att_syntax));
	core::arch::asm!("kmovw  %k5,%r13d", options(att_syntax));

	core::arch::asm!("kmovq  %k6,%k5", options(att_syntax));
	core::arch::asm!("kmovq  (%rcx),%k5", options(att_syntax));
	core::arch::asm!("kmovq  0x123(%rax,%r14,8),%k5", options(att_syntax));
	core::arch::asm!("kmovq  %k5,(%rcx)", options(att_syntax));
	core::arch::asm!("kmovq  %k5,0x123(%rax,%r14,8)", options(att_syntax));
	core::arch::asm!("kmovq  %rax,%k5", options(att_syntax));
	core::arch::asm!("kmovq  %rbp,%k5", options(att_syntax));
	core::arch::asm!("kmovq  %r13,%k5", options(att_syntax));
	core::arch::asm!("kmovq  %k5,%rax", options(att_syntax));
	core::arch::asm!("kmovq  %k5,%rbp", options(att_syntax));
	core::arch::asm!("kmovq  %k5,%r13", options(att_syntax));

	core::arch::asm!("kmovb  %k6,%k5", options(att_syntax));
	core::arch::asm!("kmovb  (%rcx),%k5", options(att_syntax));
	core::arch::asm!("kmovb  0x123(%rax,%r14,8),%k5", options(att_syntax));
	core::arch::asm!("kmovb  %k5,(%rcx)", options(att_syntax));
	core::arch::asm!("kmovb  %k5,0x123(%rax,%r14,8)", options(att_syntax));
	core::arch::asm!("kmovb  %eax,%k5", options(att_syntax));
	core::arch::asm!("kmovb  %ebp,%k5", options(att_syntax));
	core::arch::asm!("kmovb  %r13d,%k5", options(att_syntax));
	core::arch::asm!("kmovb  %k5,%eax", options(att_syntax));
	core::arch::asm!("kmovb  %k5,%ebp", options(att_syntax));
	core::arch::asm!("kmovb  %k5,%r13d", options(att_syntax));

	core::arch::asm!("kmovd  %k6,%k5", options(att_syntax));
	core::arch::asm!("kmovd  (%rcx),%k5", options(att_syntax));
	core::arch::asm!("kmovd  0x123(%rax,%r14,8),%k5", options(att_syntax));
	core::arch::asm!("kmovd  %k5,(%rcx)", options(att_syntax));
	core::arch::asm!("kmovd  %k5,0x123(%rax,%r14,8)", options(att_syntax));
	core::arch::asm!("kmovd  %eax,%k5", options(att_syntax));
	core::arch::asm!("kmovd  %ebp,%k5", options(att_syntax));
	core::arch::asm!("kmovd  %r13d,%k5", options(att_syntax));
	core::arch::asm!("kmovd  %k5,%eax", options(att_syntax));
	core::arch::asm!("kmovd  %k5,%ebp", options(att_syntax));
	core::arch::asm!("kmovd %k5,%r13d", options(att_syntax));

	core::arch::asm!("kortestw %k6,%k5", options(att_syntax));
	core::arch::asm!("kortestq %k6,%k5", options(att_syntax));
	core::arch::asm!("kortestb %k6,%k5", options(att_syntax));
	core::arch::asm!("kortestd %k6,%k5", options(att_syntax));

	core::arch::asm!("ktestw %k6,%k5", options(att_syntax));
	core::arch::asm!("ktestq %k6,%k5", options(att_syntax));
	core::arch::asm!("ktestb %k6,%k5", options(att_syntax));
	core::arch::asm!("ktestd %k6,%k5", options(att_syntax));

	core::arch::asm!("kshiftrw $0x12,%k6,%k5", options(att_syntax));
	core::arch::asm!("kshiftrq $0x5b,%k6,%k5", options(att_syntax));
	core::arch::asm!("kshiftlw $0x12,%k6,%k5", options(att_syntax));
	core::arch::asm!("kshiftlq $0x5b,%k6,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 5b */
	core::arch::asm!("vcvtdq2ps %xmm5,%xmm6", options(att_syntax));
	core::arch::asm!("vcvtqq2ps %zmm29,%ymm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtps2dq %xmm5,%xmm6", options(att_syntax));
	core::arch::asm!("vcvttps2dq %xmm5,%xmm6", options(att_syntax));

	/* AVX-512: Op code 0f 6f */

	core::arch::asm!("movq   %mm0,%mm4", options(att_syntax));
	core::arch::asm!("vmovdqa %ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vmovdqa32 %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vmovdqa64 %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vmovdqu %ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vmovdqu32 %zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vmovdqu64 %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vmovdqu8 %zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vmovdqu16 %zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 78 */

	core::arch::asm!("vmread %rax,%rbx", options(att_syntax));
	core::arch::asm!("vcvttps2udq %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vcvttpd2udq %zmm29,%ymm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvttsd2usi %xmm6,%rax", options(att_syntax));
	core::arch::asm!("vcvttss2usi %xmm6,%rax", options(att_syntax));
	core::arch::asm!("vcvttps2uqq %ymm5,%zmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvttpd2uqq %zmm29,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 79 */

	core::arch::asm!("vmwrite %rax,%rbx", options(att_syntax));
	core::arch::asm!("vcvtps2udq %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vcvtpd2udq %zmm29,%ymm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtsd2usi %xmm6,%rax", options(att_syntax));
	core::arch::asm!("vcvtss2usi %xmm6,%rax", options(att_syntax));
	core::arch::asm!("vcvtps2uqq %ymm5,%zmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtpd2uqq %zmm29,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 7a */

	core::arch::asm!("vcvtudq2pd %ymm5,%zmm29{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtuqq2pd %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vcvtudq2ps %zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vcvtuqq2ps %zmm25,%ymm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvttps2qq %ymm25,%zmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvttpd2qq %zmm29,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 7b */

	core::arch::asm!("vcvtusi2sd %eax,%xmm5,%xmm6", options(att_syntax));
	core::arch::asm!("vcvtusi2ss %eax,%xmm5,%xmm6", options(att_syntax));
	core::arch::asm!("vcvtps2qq %ymm5,%zmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtpd2qq %zmm29,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 7f */

	core::arch::asm!("movq.s  %mm0,%mm4", options(att_syntax));
	core::arch::asm!("vmovdqa %ymm8,%ymm6", options(att_syntax));
	core::arch::asm!("vmovdqa32.s %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vmovdqa64.s %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vmovdqu %ymm8,%ymm6", options(att_syntax));
	core::arch::asm!("vmovdqu32.s %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vmovdqu64.s %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vmovdqu8.s %zmm30,(%rcx)", options(att_syntax));
	core::arch::asm!("vmovdqu16.s %zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f db */

	core::arch::asm!("pand  %mm1,%mm2", options(att_syntax));
	core::arch::asm!("pand  %xmm1,%xmm2", options(att_syntax));
	core::arch::asm!("vpand  %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpandd %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpandq %zmm24,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f df */

	core::arch::asm!("pandn  %mm1,%mm2", options(att_syntax));
	core::arch::asm!("pandn  %xmm1,%xmm2", options(att_syntax));
	core::arch::asm!("vpandn %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpandnd %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpandnq %zmm24,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f e6 */

	core::arch::asm!("vcvttpd2dq %xmm1,%xmm2", options(att_syntax));
	core::arch::asm!("vcvtdq2pd %xmm5,%xmm6", options(att_syntax));
	core::arch::asm!("vcvtdq2pd %ymm5,%zmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtqq2pd %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vcvtpd2dq %xmm1,%xmm2", options(att_syntax));

	/* AVX-512: Op code 0f eb */

	core::arch::asm!("por   %mm4,%mm6", options(att_syntax));
	core::arch::asm!("vpor   %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpord  %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vporq  %zmm24,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f ef */

	core::arch::asm!("pxor   %mm4,%mm6", options(att_syntax));
	core::arch::asm!("vpxor  %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpxord %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpxorq %zmm24,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 10 */

	core::arch::asm!("pblendvb %xmm1,%xmm0", options(att_syntax));
	core::arch::asm!("vpsrlvw %zmm27,%zmm28,%zmm29", options(att_syntax));
	core::arch::asm!("vpmovuswb %zmm28,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 11 */

	core::arch::asm!("vpmovusdb %zmm28,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vpsravw %zmm27,%zmm28,%zmm29", options(att_syntax));

	/* AVX-512: Op code 0f 38 12 */

	core::arch::asm!("vpmovusqb %zmm27,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vpsllvw %zmm27,%zmm28,%zmm29", options(att_syntax));

	/* AVX-512: Op code 0f 38 13 */

	core::arch::asm!("vcvtph2ps %xmm3,%ymm5", options(att_syntax));
	core::arch::asm!("vcvtph2ps %ymm5,%zmm27{{%k7}}", options(att_syntax));
	core::arch::asm!("vpmovusdw %zmm27,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 14 */

	core::arch::asm!("blendvps %xmm1,%xmm0", options(att_syntax));
	core::arch::asm!("vpmovusqw %zmm27,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vprorvd %zmm27,%zmm28,%zmm29", options(att_syntax));
	core::arch::asm!("vprorvq %zmm27,%zmm28,%zmm29", options(att_syntax));

	/* AVX-512: Op code 0f 38 15 */

	core::arch::asm!("blendvpd %xmm1,%xmm0", options(att_syntax));
	core::arch::asm!("vpmovusqd %zmm27,%ymm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vprolvd %zmm27,%zmm28,%zmm29", options(att_syntax));
	core::arch::asm!("vprolvq %zmm27,%zmm28,%zmm29", options(att_syntax));

	/* AVX-512: Op code 0f 38 16 */

	core::arch::asm!("vpermps %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpermps %ymm24,%ymm26,%ymm22{{%k7}}", options(att_syntax));
	core::arch::asm!("vpermpd %ymm24,%ymm26,%ymm22{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 19 */

	core::arch::asm!("vbroadcastsd %xmm4,%ymm6", options(att_syntax));
	core::arch::asm!("vbroadcastf32x2 %xmm27,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 1a */

	core::arch::asm!("vbroadcastf128 (%rcx),%ymm4", options(att_syntax));
	core::arch::asm!("vbroadcastf32x4 (%rcx),%zmm26", options(att_syntax));
	core::arch::asm!("vbroadcastf64x2 (%rcx),%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 1b */

	core::arch::asm!("vbroadcastf32x8 (%rcx),%zmm27", options(att_syntax));
	core::arch::asm!("vbroadcastf64x4 (%rcx),%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 1f */

	core::arch::asm!("vpabsq %zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 20 */

	core::arch::asm!("vpmovsxbw %xmm4,%xmm5", options(att_syntax));
	core::arch::asm!("vpmovswb %zmm27,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 21 */

	core::arch::asm!("vpmovsxbd %xmm4,%ymm6", options(att_syntax));
	core::arch::asm!("vpmovsdb %zmm27,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 22 */

	core::arch::asm!("vpmovsxbq %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovsqb %zmm27,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 23 */

	core::arch::asm!("vpmovsxwd %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovsdw %zmm27,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 24 */

	core::arch::asm!("vpmovsxwq %xmm4,%ymm6", options(att_syntax));
	core::arch::asm!("vpmovsqw %zmm27,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 25 */

	core::arch::asm!("vpmovsxdq %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovsqd %zmm27,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 26 */

	core::arch::asm!("vptestmb %zmm27,%zmm28,%k5", options(att_syntax));
	core::arch::asm!("vptestmw %zmm27,%zmm28,%k5", options(att_syntax));
	core::arch::asm!("vptestnmb %zmm26,%zmm27,%k5", options(att_syntax));
	core::arch::asm!("vptestnmw %zmm26,%zmm27,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 38 27 */

	core::arch::asm!("vptestmd %zmm27,%zmm28,%k5", options(att_syntax));
	core::arch::asm!("vptestmq %zmm27,%zmm28,%k5", options(att_syntax));
	core::arch::asm!("vptestnmd %zmm26,%zmm27,%k5", options(att_syntax));
	core::arch::asm!("vptestnmq %zmm26,%zmm27,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 38 28 */

	core::arch::asm!("vpmuldq %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmovm2b %k5,%zmm28", options(att_syntax));
	core::arch::asm!("vpmovm2w %k5,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 29 */

	core::arch::asm!("vpcmpeqq %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmovb2m %zmm28,%k5", options(att_syntax));
	core::arch::asm!("vpmovw2m %zmm28,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 38 2a */

	core::arch::asm!("vmovntdqa (%rcx),%ymm4", options(att_syntax));
	core::arch::asm!("vpbroadcastmb2q %k6,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 38 2c */

	core::arch::asm!("vmaskmovps (%rcx),%ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vscalefps %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vscalefpd %zmm24,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 2d */

	core::arch::asm!("vmaskmovpd (%rcx),%ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vscalefss %xmm24,%xmm25,%xmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vscalefsd %xmm24,%xmm25,%xmm26{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 30 */

	core::arch::asm!("vpmovzxbw %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovwb %zmm27,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 31 */

	core::arch::asm!("vpmovzxbd %xmm4,%ymm6", options(att_syntax));
	core::arch::asm!("vpmovdb %zmm27,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 32 */

	core::arch::asm!("vpmovzxbq %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovqb %zmm27,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 33 */

	core::arch::asm!("vpmovzxwd %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovdw %zmm27,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 34 */

	core::arch::asm!("vpmovzxwq %xmm4,%ymm6", options(att_syntax));
	core::arch::asm!("vpmovqw %zmm27,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 35 */

	core::arch::asm!("vpmovzxdq %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovqd %zmm27,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 38 */

	core::arch::asm!("vpermd %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpermd %ymm24,%ymm26,%ymm22{{%k7}}", options(att_syntax));
	core::arch::asm!("vpermq %ymm24,%ymm26,%ymm22{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 38 */

	core::arch::asm!("vpminsb %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmovm2d %k5,%zmm28", options(att_syntax));
	core::arch::asm!("vpmovm2q %k5,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 39 */

	core::arch::asm!("vpminsd %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpminsd %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpminsq %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpmovd2m %zmm28,%k5", options(att_syntax));
	core::arch::asm!("vpmovq2m %zmm28,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 38 3a */

	core::arch::asm!("vpminuw %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpbroadcastmw2d %k6,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 3b */

	core::arch::asm!("vpminud %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpminud %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpminuq %zmm24,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 3d */

	core::arch::asm!("vpmaxsd %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmaxsd %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpmaxsq %zmm24,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 3f */

	core::arch::asm!("vpmaxud %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmaxud %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpmaxuq %zmm24,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 42 */

	core::arch::asm!("vpmulld %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmulld %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpmullq %zmm24,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 42 */

	core::arch::asm!("vgetexpps %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vgetexppd %zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 43 */

	core::arch::asm!("vgetexpss %xmm24,%xmm25,%xmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vgetexpsd %xmm28,%xmm29,%xmm30{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 44 */

	core::arch::asm!("vplzcntd %zmm27,%zmm28", options(att_syntax));
	core::arch::asm!("vplzcntq %zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 46 */

	core::arch::asm!("vpsravd %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpsravd %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpsravq %zmm24,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 4c */

	core::arch::asm!("vrcp14ps %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vrcp14pd %zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 4d */

	core::arch::asm!("vrcp14ss %xmm24,%xmm25,%xmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vrcp14sd %xmm24,%xmm25,%xmm26{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 4e */

	core::arch::asm!("vrsqrt14ps %zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vrsqrt14pd %zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 4f */

	core::arch::asm!("vrsqrt14ss %xmm24,%xmm25,%xmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vrsqrt14sd %xmm24,%xmm25,%xmm26{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 50 */

	core::arch::asm!("vpdpbusd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpdpbusd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpdpbusd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpdpbusd 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpdpbusd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 51 */

	core::arch::asm!("vpdpbusds %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpdpbusds %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpdpbusds %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpdpbusds 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpdpbusds 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 52 */

	core::arch::asm!("vdpbf16ps %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vdpbf16ps %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vdpbf16ps %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vdpbf16ps 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vdpbf16ps 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vpdpwssd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpdpwssd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpdpwssd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpdpwssd 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpdpwssd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vp4dpwssd (%rax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("vp4dpwssd (%eax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("vp4dpwssd 0x12345678(%rax,%rcx,8),%zmm0,%zmm4", options(att_syntax));
	core::arch::asm!("vp4dpwssd 0x12345678(%eax,%ecx,8),%zmm0,%zmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 53 */

	core::arch::asm!("vpdpwssds %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpdpwssds %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpdpwssds %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpdpwssds 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpdpwssds 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vp4dpwssds (%rax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("vp4dpwssds (%eax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("vp4dpwssds 0x12345678(%rax,%rcx,8),%zmm0,%zmm4", options(att_syntax));
	core::arch::asm!("vp4dpwssds 0x12345678(%eax,%ecx,8),%zmm0,%zmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 54 */

	core::arch::asm!("vpopcntb %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpopcntb %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpopcntb %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpopcntb 0x12345678(%rax,%rcx,8),%zmm2", options(att_syntax));
	core::arch::asm!("vpopcntb 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	core::arch::asm!("vpopcntw %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpopcntw %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpopcntw %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpopcntw 0x12345678(%rax,%rcx,8),%zmm2", options(att_syntax));
	core::arch::asm!("vpopcntw 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	/* AVX-512: Op code 0f 38 55 */

	core::arch::asm!("vpopcntd %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpopcntd %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpopcntd %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpopcntd 0x12345678(%rax,%rcx,8),%zmm2", options(att_syntax));
	core::arch::asm!("vpopcntd 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	core::arch::asm!("vpopcntq %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpopcntq %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpopcntq %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpopcntq 0x12345678(%rax,%rcx,8),%zmm2", options(att_syntax));
	core::arch::asm!("vpopcntq 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	/* AVX-512: Op code 0f 38 59 */

	core::arch::asm!("vpbroadcastq %xmm4,%xmm6", options(att_syntax));
	core::arch::asm!("vbroadcasti32x2 %xmm27,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 5a */

	core::arch::asm!("vbroadcasti128 (%rcx),%ymm4", options(att_syntax));
	core::arch::asm!("vbroadcasti32x4 (%rcx),%zmm26", options(att_syntax));
	core::arch::asm!("vbroadcasti64x2 (%rcx),%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 5b */

	core::arch::asm!("vbroadcasti32x8 (%rcx),%zmm28", options(att_syntax));
	core::arch::asm!("vbroadcasti64x4 (%rcx),%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 62 */

	core::arch::asm!("vpexpandb %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpexpandb %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpexpandb %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpexpandb 0x12345678(%rax,%rcx,8),%zmm2", options(att_syntax));
	core::arch::asm!("vpexpandb 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	core::arch::asm!("vpexpandw %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpexpandw %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpexpandw %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpexpandw 0x12345678(%rax,%rcx,8),%zmm2", options(att_syntax));
	core::arch::asm!("vpexpandw 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	/* AVX-512: Op code 0f 38 63 */

	core::arch::asm!("vpcompressb %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpcompressb %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpcompressb %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpcompressb %zmm2,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("vpcompressb %zmm2,0x12345678(%eax,%ecx,8)", options(att_syntax));

	core::arch::asm!("vpcompressw %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpcompressw %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpcompressw %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpcompressw %zmm2,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("vpcompressw %zmm2,0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* AVX-512: Op code 0f 38 64 */

	core::arch::asm!("vpblendmd %zmm26,%zmm27,%zmm28", options(att_syntax));
	core::arch::asm!("vpblendmq %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 65 */

	core::arch::asm!("vblendmps %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vblendmpd %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 66 */

	core::arch::asm!("vpblendmb %zmm26,%zmm27,%zmm28", options(att_syntax));
	core::arch::asm!("vpblendmw %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 68 */

	core::arch::asm!("vp2intersectd %xmm1, %xmm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectd %ymm1, %ymm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectd %zmm1, %zmm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectd 0x12345678(%rax,%rcx,8),%zmm2,%k3", options(att_syntax));
	core::arch::asm!("vp2intersectd 0x12345678(%eax,%ecx,8),%zmm2,%k3", options(att_syntax));

	core::arch::asm!("vp2intersectq %xmm1, %xmm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectq %ymm1, %ymm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectq %zmm1, %zmm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectq 0x12345678(%rax,%rcx,8),%zmm2,%k3", options(att_syntax));
	core::arch::asm!("vp2intersectq 0x12345678(%eax,%ecx,8),%zmm2,%k3", options(att_syntax));

	/* AVX-512: Op code 0f 38 70 */

	core::arch::asm!("vpshldvw %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshldvw %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshldvw %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshldvw 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshldvw 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 71 */

	core::arch::asm!("vpshldvd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshldvd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshldvd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshldvd 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshldvd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vpshldvq %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshldvq %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshldvq %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshldvq 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshldvq 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 72 */

	core::arch::asm!("vcvtne2ps2bf16 %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vcvtne2ps2bf16 %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vcvtne2ps2bf16 %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vcvtne2ps2bf16 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vcvtne2ps2bf16 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vcvtneps2bf16 %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vcvtneps2bf16 %ymm1, %xmm2", options(att_syntax));
	core::arch::asm!("vcvtneps2bf16 %zmm1, %ymm2", options(att_syntax));
	core::arch::asm!("vcvtneps2bf16 0x12345678(%rax,%rcx,8),%ymm2", options(att_syntax));
	core::arch::asm!("vcvtneps2bf16 0x12345678(%eax,%ecx,8),%ymm2", options(att_syntax));

	core::arch::asm!("vpshrdvw %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshrdvw %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshrdvw %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshrdvw 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshrdvw 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 73 */

	core::arch::asm!("vpshrdvd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshrdvd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshrdvd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshrdvd 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshrdvd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vpshrdvq %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshrdvq %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshrdvq %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshrdvq 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshrdvq 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 75 */

	core::arch::asm!("vpermi2b %zmm24,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vpermi2w %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 76 */

	core::arch::asm!("vpermi2d %zmm26,%zmm27,%zmm28", options(att_syntax));
	core::arch::asm!("vpermi2q %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 77 */

	core::arch::asm!("vpermi2ps %zmm26,%zmm27,%zmm28", options(att_syntax));
	core::arch::asm!("vpermi2pd %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 7a */

	core::arch::asm!("vpbroadcastb %eax,%xmm30", options(att_syntax));

	/* AVX-512: Op code 0f 38 7b */

	core::arch::asm!("vpbroadcastw %eax,%xmm30", options(att_syntax));

	/* AVX-512: Op code 0f 38 7c */

	core::arch::asm!("vpbroadcastd %eax,%xmm30", options(att_syntax));
	core::arch::asm!("vpbroadcastq %rax,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 38 7d */

	core::arch::asm!("vpermt2b %zmm26,%zmm27,%zmm28", options(att_syntax));
	core::arch::asm!("vpermt2w %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 7e */

	core::arch::asm!("vpermt2d %zmm26,%zmm27,%zmm28", options(att_syntax));
	core::arch::asm!("vpermt2q %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 7f */

	core::arch::asm!("vpermt2ps %zmm26,%zmm27,%zmm28", options(att_syntax));
	core::arch::asm!("vpermt2pd %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 83 */

	core::arch::asm!("vpmultishiftqb %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 88 */

	core::arch::asm!("vexpandps (%rcx),%zmm26", options(att_syntax));
	core::arch::asm!("vexpandpd (%rcx),%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 89 */

	core::arch::asm!("vpexpandd (%rcx),%zmm28", options(att_syntax));
	core::arch::asm!("vpexpandq (%rcx),%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 38 8a */

	core::arch::asm!("vcompressps %zmm28,(%rcx)", options(att_syntax));
	core::arch::asm!("vcompresspd %zmm28,(%rcx)", options(att_syntax));

	/* AVX-512: Op code 0f 38 8b */

	core::arch::asm!("vpcompressd %zmm28,(%rcx)", options(att_syntax));
	core::arch::asm!("vpcompressq %zmm26,(%rcx)", options(att_syntax));

	/* AVX-512: Op code 0f 38 8d */

	core::arch::asm!("vpermb %zmm26,%zmm27,%zmm28", options(att_syntax));
	core::arch::asm!("vpermw %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 8f */

	core::arch::asm!("vpshufbitqmb %xmm1, %xmm2, %k3", options(att_syntax));
	core::arch::asm!("vpshufbitqmb %ymm1, %ymm2, %k3", options(att_syntax));
	core::arch::asm!("vpshufbitqmb %zmm1, %zmm2, %k3", options(att_syntax));
	core::arch::asm!("vpshufbitqmb 0x12345678(%rax,%rcx,8),%zmm2,%k3", options(att_syntax));
	core::arch::asm!("vpshufbitqmb 0x12345678(%eax,%ecx,8),%zmm2,%k3", options(att_syntax));

	/* AVX-512: Op code 0f 38 90 */

	core::arch::asm!("vpgatherdd %xmm2,0x02(%rbp,%xmm7,2),%xmm1", options(att_syntax));
	core::arch::asm!("vpgatherdq %xmm2,0x04(%rbp,%xmm7,2),%xmm1", options(att_syntax));
	core::arch::asm!("vpgatherdd 0x7b(%rbp,%zmm27,8),%zmm26{{%k1}}", options(att_syntax));
	core::arch::asm!("vpgatherdq 0x7b(%rbp,%ymm27,8),%zmm26{{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 91 */

	core::arch::asm!("vpgatherqd %xmm2,0x02(%rbp,%xmm7,2),%xmm1", options(att_syntax));
	core::arch::asm!("vpgatherqq %xmm2,0x02(%rbp,%xmm7,2),%xmm1", options(att_syntax));
	core::arch::asm!("vpgatherqd 0x7b(%rbp,%zmm27,8),%ymm26{{%k1}}", options(att_syntax));
	core::arch::asm!("vpgatherqq 0x7b(%rbp,%zmm27,8),%zmm26{{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 9a */

	core::arch::asm!("vfmsub132ps %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub132ps %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vfmsub132ps %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vfmsub132ps 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vfmsub132ps 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vfmsub132pd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub132pd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vfmsub132pd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vfmsub132pd 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vfmsub132pd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("v4fmaddps (%rax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("v4fmaddps (%eax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("v4fmaddps 0x12345678(%rax,%rcx,8),%zmm0,%zmm4", options(att_syntax));
	core::arch::asm!("v4fmaddps 0x12345678(%eax,%ecx,8),%zmm0,%zmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 9b */

	core::arch::asm!("vfmsub132ss %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub132ss 0x12345678(%rax,%rcx,8),%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vfmsub132ss 0x12345678(%eax,%ecx,8),%xmm2,%xmm3", options(att_syntax));

	core::arch::asm!("vfmsub132sd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub132sd 0x12345678(%rax,%rcx,8),%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vfmsub132sd 0x12345678(%eax,%ecx,8),%xmm2,%xmm3", options(att_syntax));

	core::arch::asm!("v4fmaddss (%rax), %xmm0, %xmm4", options(att_syntax));
	core::arch::asm!("v4fmaddss (%eax), %xmm0, %xmm4", options(att_syntax));
	core::arch::asm!("v4fmaddss 0x12345678(%rax,%rcx,8),%xmm0,%xmm4", options(att_syntax));
	core::arch::asm!("v4fmaddss 0x12345678(%eax,%ecx,8),%xmm0,%xmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 a0 */

	core::arch::asm!("vpscatterdd %zmm28,0x7b(%rbp,%zmm29,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vpscatterdq %zmm26,0x7b(%rbp,%ymm27,8){{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 a1 */

	core::arch::asm!("vpscatterqd %ymm6,0x7b(%rbp,%zmm29,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vpscatterqq %ymm6,0x7b(%rbp,%ymm27,8){{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 a2 */

	core::arch::asm!("vscatterdps %zmm28,0x7b(%rbp,%zmm29,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterdpd %zmm28,0x7b(%rbp,%ymm27,8){{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 a3 */

	core::arch::asm!("vscatterqps %ymm6,0x7b(%rbp,%zmm29,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterqpd %zmm28,0x7b(%rbp,%zmm29,8){{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 aa */

	core::arch::asm!("vfmsub213ps %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub213ps %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vfmsub213ps %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vfmsub213ps 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vfmsub213ps 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vfmsub213pd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub213pd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vfmsub213pd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vfmsub213pd 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vfmsub213pd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("v4fnmaddps (%rax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("v4fnmaddps (%eax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("v4fnmaddps 0x12345678(%rax,%rcx,8),%zmm0,%zmm4", options(att_syntax));
	core::arch::asm!("v4fnmaddps 0x12345678(%eax,%ecx,8),%zmm0,%zmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 ab */

	core::arch::asm!("vfmsub213ss %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub213ss 0x12345678(%rax,%rcx,8),%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vfmsub213ss 0x12345678(%eax,%ecx,8),%xmm2,%xmm3", options(att_syntax));

	core::arch::asm!("vfmsub213sd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub213sd 0x12345678(%rax,%rcx,8),%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vfmsub213sd 0x12345678(%eax,%ecx,8),%xmm2,%xmm3", options(att_syntax));

	core::arch::asm!("v4fnmaddss (%rax), %xmm0, %xmm4", options(att_syntax));
	core::arch::asm!("v4fnmaddss (%eax), %xmm0, %xmm4", options(att_syntax));
	core::arch::asm!("v4fnmaddss 0x12345678(%rax,%rcx,8),%xmm0,%xmm4", options(att_syntax));
	core::arch::asm!("v4fnmaddss 0x12345678(%eax,%ecx,8),%xmm0,%xmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 b4 */

	core::arch::asm!("vpmadd52luq %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 b5 */

	core::arch::asm!("vpmadd52huq %zmm26,%zmm27,%zmm28", options(att_syntax));

	/* AVX-512: Op code 0f 38 c4 */

	core::arch::asm!("vpconflictd %zmm26,%zmm27", options(att_syntax));
	core::arch::asm!("vpconflictq %zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 38 c8 */

	core::arch::asm!("vexp2ps %zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vexp2pd %zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 38 ca */

	core::arch::asm!("vrcp28ps %zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vrcp28pd %zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 38 cb */

	core::arch::asm!("vrcp28ss %xmm28,%xmm29,%xmm30{{%k7}}", options(att_syntax));
	core::arch::asm!("vrcp28sd %xmm25,%xmm26,%xmm27{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 cc */

	core::arch::asm!("vrsqrt28ps %zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vrsqrt28pd %zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 38 cd */

	core::arch::asm!("vrsqrt28ss %xmm28,%xmm29,%xmm30{{%k7}}", options(att_syntax));
	core::arch::asm!("vrsqrt28sd %xmm25,%xmm26,%xmm27{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 cf */

	core::arch::asm!("gf2p8mulb %xmm1, %xmm3", options(att_syntax));
	core::arch::asm!("gf2p8mulb 0x12345678(%rax,%rcx,8),%xmm3", options(att_syntax));
	core::arch::asm!("gf2p8mulb 0x12345678(%eax,%ecx,8),%xmm3", options(att_syntax));

	core::arch::asm!("vgf2p8mulb %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vgf2p8mulb %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vgf2p8mulb %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vgf2p8mulb 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vgf2p8mulb 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 dc */

	core::arch::asm!("vaesenc %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vaesenc %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vaesenc %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vaesenc 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vaesenc 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 dd */

	core::arch::asm!("vaesenclast %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vaesenclast %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vaesenclast %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vaesenclast 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vaesenclast 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 de */

	core::arch::asm!("vaesdec %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vaesdec %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vaesdec %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vaesdec 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vaesdec 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 df */

	core::arch::asm!("vaesdeclast %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vaesdeclast %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vaesdeclast %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vaesdeclast 0x12345678(%rax,%rcx,8),%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vaesdeclast 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 3a 03 */

	core::arch::asm!("valignd $0x12,%zmm28,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("valignq $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 3a 08 */

	core::arch::asm!("vroundps $0x5,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vrndscaleps $0x12,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 3a 09 */

	core::arch::asm!("vroundpd $0x5,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vrndscalepd $0x12,%zmm25,%zmm26", options(att_syntax));

	/* AVX-512: Op code 0f 3a 1a */

	core::arch::asm!("vroundss $0x5,%xmm4,%xmm6,%xmm2", options(att_syntax));
	core::arch::asm!("vrndscaless $0x12,%xmm24,%xmm25,%xmm26{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 0b */

	core::arch::asm!("vroundsd $0x5,%xmm4,%xmm6,%xmm2", options(att_syntax));
	core::arch::asm!("vrndscalesd $0x12,%xmm24,%xmm25,%xmm26{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 18 */

	core::arch::asm!("vinsertf128 $0x5,%xmm4,%ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vinsertf32x4 $0x12,%xmm24,%zmm25,%zmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vinsertf64x2 $0x12,%xmm24,%zmm25,%zmm26{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 19 */

	core::arch::asm!("vextractf128 $0x5,%ymm4,%xmm4", options(att_syntax));
	core::arch::asm!("vextractf32x4 $0x12,%zmm25,%xmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vextractf64x2 $0x12,%zmm25,%xmm26{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 1a */

	core::arch::asm!("vinsertf32x8 $0x12,%ymm25,%zmm26,%zmm27{{%k7}}", options(att_syntax));
	core::arch::asm!("vinsertf64x4 $0x12,%ymm28,%zmm29,%zmm30{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 1b */

	core::arch::asm!("vextractf32x8 $0x12,%zmm29,%ymm30{{%k7}}", options(att_syntax));
	core::arch::asm!("vextractf64x4 $0x12,%zmm26,%ymm27{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 1e */

	core::arch::asm!("vpcmpud $0x12,%zmm29,%zmm30,%k5", options(att_syntax));
	core::arch::asm!("vpcmpuq $0x12,%zmm26,%zmm27,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 1f */

	core::arch::asm!("vpcmpd $0x12,%zmm29,%zmm30,%k5", options(att_syntax));
	core::arch::asm!("vpcmpq $0x12,%zmm26,%zmm27,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 23 */

	core::arch::asm!("vshuff32x4 $0x12,%zmm28,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vshuff64x2 $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 3a 25 */

	core::arch::asm!("vpternlogd $0x12,%zmm28,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vpternlogq $0x12,%zmm28,%zmm29,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 3a 26 */

	core::arch::asm!("vgetmantps $0x12,%zmm26,%zmm27", options(att_syntax));
	core::arch::asm!("vgetmantpd $0x12,%zmm29,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 3a 27 */

	core::arch::asm!("vgetmantss $0x12,%xmm25,%xmm26,%xmm27{{%k7}}", options(att_syntax));
	core::arch::asm!("vgetmantsd $0x12,%xmm28,%xmm29,%xmm30{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 38 */

	core::arch::asm!("vinserti128 $0x5,%xmm4,%ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vinserti32x4 $0x12,%xmm24,%zmm25,%zmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vinserti64x2 $0x12,%xmm24,%zmm25,%zmm26{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 39 */

	core::arch::asm!("vextracti128 $0x5,%ymm4,%xmm6", options(att_syntax));
	core::arch::asm!("vextracti32x4 $0x12,%zmm25,%xmm26{{%k7}}", options(att_syntax));
	core::arch::asm!("vextracti64x2 $0x12,%zmm25,%xmm26{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 3a */

	core::arch::asm!("vinserti32x8 $0x12,%ymm28,%zmm29,%zmm30{{%k7}}", options(att_syntax));
	core::arch::asm!("vinserti64x4 $0x12,%ymm25,%zmm26,%zmm27{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 3b */

	core::arch::asm!("vextracti32x8 $0x12,%zmm29,%ymm30{{%k7}}", options(att_syntax));
	core::arch::asm!("vextracti64x4 $0x12,%zmm26,%ymm27{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 3e */

	core::arch::asm!("vpcmpub $0x12,%zmm29,%zmm30,%k5", options(att_syntax));
	core::arch::asm!("vpcmpuw $0x12,%zmm26,%zmm27,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 3f */

	core::arch::asm!("vpcmpb $0x12,%zmm29,%zmm30,%k5", options(att_syntax));
	core::arch::asm!("vpcmpw $0x12,%zmm26,%zmm27,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 43 */

	core::arch::asm!("vmpsadbw $0x5,%ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vdbpsadbw $0x12,%zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 3a 43 */

	core::arch::asm!("vshufi32x4 $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));
	core::arch::asm!("vshufi64x2 $0x12,%zmm28,%zmm29,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 3a 44 */

	core::arch::asm!("vpclmulqdq $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpclmulqdq $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpclmulqdq $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpclmulqdq $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 3a 50 */

	core::arch::asm!("vrangeps $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));
	core::arch::asm!("vrangepd $0x12,%zmm28,%zmm29,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 3a 51 */

	core::arch::asm!("vrangess $0x12,%xmm25,%xmm26,%xmm27", options(att_syntax));
	core::arch::asm!("vrangesd $0x12,%xmm28,%xmm29,%xmm30", options(att_syntax));

	/* AVX-512: Op code 0f 3a 54 */

	core::arch::asm!("vfixupimmps $0x12,%zmm28,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vfixupimmpd $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 3a 55 */

	core::arch::asm!("vfixupimmss $0x12,%xmm28,%xmm29,%xmm30{{%k7}}", options(att_syntax));
	core::arch::asm!("vfixupimmsd $0x12,%xmm25,%xmm26,%xmm27{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 56 */

	core::arch::asm!("vreduceps $0x12,%zmm26,%zmm27", options(att_syntax));
	core::arch::asm!("vreducepd $0x12,%zmm29,%zmm30", options(att_syntax));

	/* AVX-512: Op code 0f 3a 57 */

	core::arch::asm!("vreducess $0x12,%xmm25,%xmm26,%xmm27", options(att_syntax));
	core::arch::asm!("vreducesd $0x12,%xmm28,%xmm29,%xmm30", options(att_syntax));

	/* AVX-512: Op code 0f 3a 66 */

	core::arch::asm!("vfpclassps $0x12,%zmm27,%k5", options(att_syntax));
	core::arch::asm!("vfpclasspd $0x12,%zmm30,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 67 */

	core::arch::asm!("vfpclassss $0x12,%xmm27,%k5", options(att_syntax));
	core::arch::asm!("vfpclasssd $0x12,%xmm30,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 70 */

	core::arch::asm!("vpshldw $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshldw $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshldw $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshldw $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 3a 71 */

	core::arch::asm!("vpshldd $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshldd $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshldd $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshldd $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	core::arch::asm!("vpshldq $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshldq $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshldq $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshldq $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 3a 72 */

	core::arch::asm!("vpshrdw $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshrdw $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshrdw $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshrdw $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 3a 73 */

	core::arch::asm!("vpshrdd $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshrdd $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshrdd $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshrdd $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	core::arch::asm!("vpshrdq $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshrdq $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshrdq $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vpshrdq $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 3a ce */

	core::arch::asm!("gf2p8affineqb $0x12,%xmm1,%xmm3", options(att_syntax));

	core::arch::asm!("vgf2p8affineqb $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vgf2p8affineqb $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vgf2p8affineqb $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vgf2p8affineqb $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 3a cf */

	core::arch::asm!("gf2p8affineinvqb $0x12,%xmm1,%xmm3", options(att_syntax));

	core::arch::asm!("vgf2p8affineinvqb $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vgf2p8affineinvqb $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vgf2p8affineinvqb $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));
	core::arch::asm!("vgf2p8affineinvqb $0x12,%zmm25,%zmm26,%zmm27", options(att_syntax));

	/* AVX-512: Op code 0f 72 (Grp13) */

	core::arch::asm!("vprord $0x12,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vprorq $0x12,%zmm25,%zmm26", options(att_syntax));
	core::arch::asm!("vprold $0x12,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vprolq $0x12,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("psrad  $0x2,%mm6", options(att_syntax));
	core::arch::asm!("vpsrad $0x5,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpsrad $0x5,%zmm26,%zmm22", options(att_syntax));
	core::arch::asm!("vpsraq $0x5,%zmm26,%zmm22", options(att_syntax));

	/* AVX-512: Op code 0f 38 c6 (Grp18) */

	core::arch::asm!("vgatherpf0dps 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf0dpd 0x7b(%r14,%ymm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf1dps 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf1dpd 0x7b(%r14,%ymm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf0dps 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf0dpd 0x7b(%r14,%ymm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf1dps 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf1dpd 0x7b(%r14,%ymm31,8){{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 c7 (Grp19) */

	core::arch::asm!("vgatherpf0qps 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf0qpd 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf1qps 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf1qpd 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf0qps 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf0qpd 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf1qps 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf1qpd 0x7b(%r14,%zmm31,8){{%k1}}", options(att_syntax));

	/* AVX-512: Examples */

	core::arch::asm!("vaddpd %zmm28,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vaddpd %zmm28,%zmm29,%zmm30{{%k7}}", options(att_syntax));
	core::arch::asm!("vaddpd %zmm28,%zmm29,%zmm30{{%k7}}{{z}}", options(att_syntax));
	core::arch::asm!("vaddpd {{rn-sae}},%zmm28,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vaddpd {{ru-sae}},%zmm28,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vaddpd {{rd-sae}},%zmm28,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vaddpd {{rz-sae}},%zmm28,%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vaddpd (%rcx),%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vaddpd 0x123(%rax,%r14,8),%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vaddpd (%rcx){{1to8}},%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vaddpd 0x1fc0(%rdx),%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vaddpd 0x3f8(%rdx){{1to8}},%zmm29,%zmm30", options(att_syntax));
	core::arch::asm!("vcmpeq_uqps 0x1fc(%rdx){{1to16}},%zmm30,%k5", options(att_syntax));
	core::arch::asm!("vcmpltsd 0x123(%rax,%r14,8),%xmm29,%k5{{%k7}}", options(att_syntax));
	core::arch::asm!("vcmplesd {{sae}},%xmm28,%xmm29,%k5{{%k7}}", options(att_syntax));
	core::arch::asm!("vgetmantss $0x5b,0x123(%rax,%r14,8),%xmm29,%xmm30{{%k7}}", options(att_syntax));

	/* bndmk m64, bnd */

	core::arch::asm!("bndmk (%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk (%r8), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk (%rax), %bnd3", options(att_syntax));
	core::arch::asm!("bndmk (%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk (%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk (%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%rax,%rcx,8), %bnd0", options(att_syntax));

	/* bndcl r/m64, bnd */

	core::arch::asm!("bndcl (%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl (%r8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl (%rax), %bnd3", options(att_syntax));
	core::arch::asm!("bndcl (%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl (%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl (%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl %rax, %bnd0", options(att_syntax));

	/* bndcu r/m64, bnd */

	core::arch::asm!("bndcu (%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu (%r8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu (%rax), %bnd3", options(att_syntax));
	core::arch::asm!("bndcu (%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu (%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu (%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu %rax, %bnd0", options(att_syntax));

	/* bndcn r/m64, bnd */

	core::arch::asm!("bndcn (%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn (%r8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn (%rax), %bnd3", options(att_syntax));
	core::arch::asm!("bndcn (%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn (%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn (%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn %rax, %bnd0", options(att_syntax));

	/* bndmov m128, bnd */

	core::arch::asm!("bndmov (%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov (%r8), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov (%rax), %bnd3", options(att_syntax));
	core::arch::asm!("bndmov (%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov (%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov (%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%rax,%rcx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%rax,%rcx,8), %bnd0", options(att_syntax));

	/* bndmov bnd, m128 */

	core::arch::asm!("bndmov %bnd0, (%rax)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, (%r8)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, (0x12345678)", options(att_syntax));
	core::arch::asm!("bndmov %bnd3, (%rax)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, (%rcx,%rax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(,%rax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, (%rax,%rcx,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, (%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%rax)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%rbp)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%rcx,%rax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%rbp,%rax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%rax,%rcx,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%rbp)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%rcx,%rax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%rbp,%rax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%rax,%rcx,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%rax,%rcx,8)", options(att_syntax));

	/* bndmov bnd2, bnd1 */

	core::arch::asm!("bndmov %bnd0, %bnd1", options(att_syntax));
	core::arch::asm!("bndmov %bnd1, %bnd0", options(att_syntax));

	/* bndldx mib, bnd */

	core::arch::asm!("bndldx (%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx (%r8), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx (%rax), %bnd3", options(att_syntax));
	core::arch::asm!("bndldx (%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx (%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12(%rax,%rcx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(%rax), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(%rbp), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(%rcx,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(%rbp,%rax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(%rax,%rcx,1), %bnd0", options(att_syntax));

	/* bndstx bnd, mib */

	core::arch::asm!("bndstx %bnd0, (%rax)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, (%r8)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, (0x12345678)", options(att_syntax));
	core::arch::asm!("bndstx %bnd3, (%rax)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, (%rcx,%rax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(,%rax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, (%rax,%rcx,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12(%rax)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12(%rbp)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12(%rcx,%rax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12(%rbp,%rax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12(%rax,%rcx,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(%rax)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(%rbp)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(%rcx,%rax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(%rbp,%rax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(%rax,%rcx,1)", options(att_syntax));

	/* bnd prefix on call, ret, jmp and all jcc */

	core::arch::asm!("bnd call label1", options(att_syntax));  /* Expecting: call unconditional 0 */
	core::arch::asm!("bnd call *(%eax)", options(att_syntax)); /* Expecting: call indirect      0 */
	core::arch::asm!("bnd ret", options(att_syntax));          /* Expecting: ret  indirect      0 */
	core::arch::asm!("bnd jmp label1", options(att_syntax));   /* Expecting: jmp  unconditional 0 */
	core::arch::asm!("bnd jmp label1", options(att_syntax));   /* Expecting: jmp  unconditional 0 */
	core::arch::asm!("bnd jmp *(%ecx)", options(att_syntax));  /* Expecting: jmp  indirect      0 */
	core::arch::asm!("bnd jne label1", options(att_syntax));   /* Expecting: jcc  conditional   0 */

	/* sha1rnds4 imm8, xmm2/m128, xmm1 */

	core::arch::asm!("sha1rnds4 $0x0, %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, %xmm8, %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, %xmm7, %xmm8", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, %xmm15, %xmm8", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%r8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%rax), %xmm3", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%rax,%rcx,8), %xmm15", options(att_syntax));

	/* sha1nexte xmm2/m128, xmm1 */

	core::arch::asm!("sha1nexte %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha1nexte %xmm8, %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte %xmm7, %xmm8", options(att_syntax));
	core::arch::asm!("sha1nexte %xmm15, %xmm8", options(att_syntax));
	core::arch::asm!("sha1nexte (%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte (%r8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte (%rax), %xmm3", options(att_syntax));
	core::arch::asm!("sha1nexte (%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte (%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte (%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%rax,%rcx,8), %xmm15", options(att_syntax));

	/* sha1msg1 xmm2/m128, xmm1 */

	core::arch::asm!("sha1msg1 %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha1msg1 %xmm8, %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 %xmm7, %xmm8", options(att_syntax));
	core::arch::asm!("sha1msg1 %xmm15, %xmm8", options(att_syntax));
	core::arch::asm!("sha1msg1 (%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 (%r8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 (%rax), %xmm3", options(att_syntax));
	core::arch::asm!("sha1msg1 (%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 (%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 (%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%rax,%rcx,8), %xmm15", options(att_syntax));

	/* sha1msg2 xmm2/m128, xmm1 */

	core::arch::asm!("sha1msg2 %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha1msg2 %xmm8, %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 %xmm7, %xmm8", options(att_syntax));
	core::arch::asm!("sha1msg2 %xmm15, %xmm8", options(att_syntax));
	core::arch::asm!("sha1msg2 (%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 (%r8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 (%rax), %xmm3", options(att_syntax));
	core::arch::asm!("sha1msg2 (%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 (%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 (%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%rax,%rcx,8), %xmm15", options(att_syntax));

	/* sha256rnds2 <XMM0>, xmm2/m128, xmm1 */
	/* Note sha256rnds2 has an implicit operand 'xmm0' */

	core::arch::asm!("sha256rnds2 %xmm4, %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha256rnds2 %xmm8, %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 %xmm7, %xmm8", options(att_syntax));
	core::arch::asm!("sha256rnds2 %xmm15, %xmm8", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%rax), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%r8), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 (0x12345678), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%rax), %xmm3", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%rcx,%rax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(,%rax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%rax,%rcx,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%rax), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%rbp), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%rcx,%rax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%rbp,%rax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%rax,%rcx,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%rax), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%rbp), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%rcx,%rax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%rbp,%rax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%rax,%rcx,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%rax,%rcx,8), %xmm15", options(att_syntax));

	/* sha256msg1 xmm2/m128, xmm1 */

	core::arch::asm!("sha256msg1 %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha256msg1 %xmm8, %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 %xmm7, %xmm8", options(att_syntax));
	core::arch::asm!("sha256msg1 %xmm15, %xmm8", options(att_syntax));
	core::arch::asm!("sha256msg1 (%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 (%r8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 (%rax), %xmm3", options(att_syntax));
	core::arch::asm!("sha256msg1 (%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 (%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 (%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%rax,%rcx,8), %xmm15", options(att_syntax));

	/* sha256msg2 xmm2/m128, xmm1 */

	core::arch::asm!("sha256msg2 %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha256msg2 %xmm8, %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 %xmm7, %xmm8", options(att_syntax));
	core::arch::asm!("sha256msg2 %xmm15, %xmm8", options(att_syntax));
	core::arch::asm!("sha256msg2 (%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 (%r8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 (%rax), %xmm3", options(att_syntax));
	core::arch::asm!("sha256msg2 (%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 (%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 (%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%rax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%rbp), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%rcx,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%rbp,%rax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%rax,%rcx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%rax,%rcx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%rax,%rcx,8), %xmm15", options(att_syntax));

	/* clflushopt m8 */

	core::arch::asm!("clflushopt (%rax)", options(att_syntax));
	core::arch::asm!("clflushopt (%r8)", options(att_syntax));
	core::arch::asm!("clflushopt (0x12345678)", options(att_syntax));
	core::arch::asm!("clflushopt 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("clflushopt 0x12345678(%r8,%rcx,8)", options(att_syntax));
	/* Also check instructions in the same group encoding as clflushopt */
	core::arch::asm!("clflush (%rax)", options(att_syntax));
	core::arch::asm!("clflush (%r8)", options(att_syntax));
	core::arch::asm!("sfence", options(att_syntax));

	/* clwb m8 */

	core::arch::asm!("clwb (%rax)", options(att_syntax));
	core::arch::asm!("clwb (%r8)", options(att_syntax));
	core::arch::asm!("clwb (0x12345678)", options(att_syntax));
	core::arch::asm!("clwb 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("clwb 0x12345678(%r8,%rcx,8)", options(att_syntax));
	/* Also check instructions in the same group encoding as clwb */
	core::arch::asm!("xsaveopt (%rax)", options(att_syntax));
	core::arch::asm!("xsaveopt (%r8)", options(att_syntax));
	core::arch::asm!("mfence", options(att_syntax));

	/* cldemote m8 */

	core::arch::asm!("cldemote (%rax)", options(att_syntax));
	core::arch::asm!("cldemote (%r8)", options(att_syntax));
	core::arch::asm!("cldemote (0x12345678)", options(att_syntax));
	core::arch::asm!("cldemote 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("cldemote 0x12345678(%r8,%rcx,8)", options(att_syntax));

	/* xsavec mem */

	core::arch::asm!("xsavec (%rax)", options(att_syntax));
	core::arch::asm!("xsavec (%r8)", options(att_syntax));
	core::arch::asm!("xsavec (0x12345678)", options(att_syntax));
	core::arch::asm!("xsavec 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("xsavec 0x12345678(%r8,%rcx,8)", options(att_syntax));

	/* xsaves mem */

	core::arch::asm!("xsaves (%rax)", options(att_syntax));
	core::arch::asm!("xsaves (%r8)", options(att_syntax));
	core::arch::asm!("xsaves (0x12345678)", options(att_syntax));
	core::arch::asm!("xsaves 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("xsaves 0x12345678(%r8,%rcx,8)", options(att_syntax));

	/* xrstors mem */

	core::arch::asm!("xrstors (%rax)", options(att_syntax));
	core::arch::asm!("xrstors (%r8)", options(att_syntax));
	core::arch::asm!("xrstors (0x12345678)", options(att_syntax));
	core::arch::asm!("xrstors 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("xrstors 0x12345678(%r8,%rcx,8)", options(att_syntax));

	/* ptwrite */

	core::arch::asm!("ptwrite (%rax)", options(att_syntax));
	core::arch::asm!("ptwrite (%r8)", options(att_syntax));
	core::arch::asm!("ptwrite (0x12345678)", options(att_syntax));
	core::arch::asm!("ptwrite 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("ptwrite 0x12345678(%r8,%rcx,8)", options(att_syntax));

	core::arch::asm!("ptwritel (%rax)", options(att_syntax));
	core::arch::asm!("ptwritel (%r8)", options(att_syntax));
	core::arch::asm!("ptwritel (0x12345678)", options(att_syntax));
	core::arch::asm!("ptwritel 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("ptwritel 0x12345678(%r8,%rcx,8)", options(att_syntax));

	core::arch::asm!("ptwriteq (%rax)", options(att_syntax));
	core::arch::asm!("ptwriteq (%r8)", options(att_syntax));
	core::arch::asm!("ptwriteq (0x12345678)", options(att_syntax));
	core::arch::asm!("ptwriteq 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("ptwriteq 0x12345678(%r8,%rcx,8)", options(att_syntax));

	/* tpause */

	core::arch::asm!("tpause %ebx", options(att_syntax));
	core::arch::asm!("tpause %r8d", options(att_syntax));

	/* umonitor */

	core::arch::asm!("umonitor %eax", options(att_syntax));
	core::arch::asm!("umonitor %rax", options(att_syntax));
	core::arch::asm!("umonitor %r8d", options(att_syntax));

	/* umwait */

	core::arch::asm!("umwait %eax", options(att_syntax));
	core::arch::asm!("umwait %r8d", options(att_syntax));

	/* movdiri */

	core::arch::asm!("movdiri %rax,(%rbx)", options(att_syntax));
	core::arch::asm!("movdiri %rcx,0x12345678(%rax)", options(att_syntax));

	/* movdir64b */

	core::arch::asm!("movdir64b (%rax),%rbx", options(att_syntax));
	core::arch::asm!("movdir64b 0x12345678(%rax),%rcx", options(att_syntax));
	core::arch::asm!("movdir64b (%eax),%ebx", options(att_syntax));
	core::arch::asm!("movdir64b 0x12345678(%eax),%ecx", options(att_syntax));

	/* enqcmd */

	core::arch::asm!("enqcmd (%rax),%rbx", options(att_syntax));
	core::arch::asm!("enqcmd 0x12345678(%rax),%rcx", options(att_syntax));
	core::arch::asm!("enqcmd (%eax),%ebx", options(att_syntax));
	core::arch::asm!("enqcmd 0x12345678(%eax),%ecx", options(att_syntax));

	/* enqcmds */

	core::arch::asm!("enqcmds (%rax),%rbx", options(att_syntax));
	core::arch::asm!("enqcmds 0x12345678(%rax),%rcx", options(att_syntax));
	core::arch::asm!("enqcmds (%eax),%ebx", options(att_syntax));
	core::arch::asm!("enqcmds 0x12345678(%eax),%ecx", options(att_syntax));

	/* incsspd/q */

	core::arch::asm!("incsspd %eax", options(att_syntax));
	core::arch::asm!("incsspd %r8d", options(att_syntax));
	core::arch::asm!("incsspq %rax", options(att_syntax));
	core::arch::asm!("incsspq %r8", options(att_syntax));
	/* Also check instructions in the same group encoding as incsspd/q */
	core::arch::asm!("xrstor (%rax)", options(att_syntax));
	core::arch::asm!("xrstor (%r8)", options(att_syntax));
	core::arch::asm!("xrstor (0x12345678)", options(att_syntax));
	core::arch::asm!("xrstor 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("xrstor 0x12345678(%r8,%rcx,8)", options(att_syntax));
	core::arch::asm!("lfence", options(att_syntax));

	/* rdsspd/q */

	core::arch::asm!("rdsspd %eax", options(att_syntax));
	core::arch::asm!("rdsspd %r8d", options(att_syntax));
	core::arch::asm!("rdsspq %rax", options(att_syntax));
	core::arch::asm!("rdsspq %r8", options(att_syntax));

	/* saveprevssp */

	core::arch::asm!("saveprevssp", options(att_syntax));

	/* rstorssp */

	core::arch::asm!("rstorssp (%rax)", options(att_syntax));
	core::arch::asm!("rstorssp (%r8)", options(att_syntax));
	core::arch::asm!("rstorssp (0x12345678)", options(att_syntax));
	core::arch::asm!("rstorssp 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("rstorssp 0x12345678(%r8,%rcx,8)", options(att_syntax));

	/* wrssd/q */

	core::arch::asm!("wrssd %ecx,(%rax)", options(att_syntax));
	core::arch::asm!("wrssd %edx,(%r8)", options(att_syntax));
	core::arch::asm!("wrssd %edx,(0x12345678)", options(att_syntax));
	core::arch::asm!("wrssd %edx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("wrssd %edx,0x12345678(%r8,%rcx,8)", options(att_syntax));
	core::arch::asm!("wrssq %rcx,(%rax)", options(att_syntax));
	core::arch::asm!("wrssq %rdx,(%r8)", options(att_syntax));
	core::arch::asm!("wrssq %rdx,(0x12345678)", options(att_syntax));
	core::arch::asm!("wrssq %rdx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("wrssq %rdx,0x12345678(%r8,%rcx,8)", options(att_syntax));

	/* wrussd/q */

	core::arch::asm!("wrussd %ecx,(%rax)", options(att_syntax));
	core::arch::asm!("wrussd %edx,(%r8)", options(att_syntax));
	core::arch::asm!("wrussd %edx,(0x12345678)", options(att_syntax));
	core::arch::asm!("wrussd %edx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("wrussd %edx,0x12345678(%r8,%rcx,8)", options(att_syntax));
	core::arch::asm!("wrussq %rcx,(%rax)", options(att_syntax));
	core::arch::asm!("wrussq %rdx,(%r8)", options(att_syntax));
	core::arch::asm!("wrussq %rdx,(0x12345678)", options(att_syntax));
	core::arch::asm!("wrussq %rdx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("wrussq %rdx,0x12345678(%r8,%rcx,8)", options(att_syntax));

	/* setssbsy */

	core::arch::asm!("setssbsy", options(att_syntax));
	/* Also check instructions in the same group encoding as setssbsy */
	core::arch::asm!("rdpkru", options(att_syntax));
	core::arch::asm!("wrpkru", options(att_syntax));

	/* clrssbsy */

	core::arch::asm!("clrssbsy (%rax)", options(att_syntax));
	core::arch::asm!("clrssbsy (%r8)", options(att_syntax));
	core::arch::asm!("clrssbsy (0x12345678)", options(att_syntax));
	core::arch::asm!("clrssbsy 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("clrssbsy 0x12345678(%r8,%rcx,8)", options(att_syntax));

	/* endbr32/64 */

	core::arch::asm!("endbr32", options(att_syntax));
	core::arch::asm!("endbr64", options(att_syntax));

	/* call with/without notrack prefix */

	core::arch::asm!("callq *%rax", options(att_syntax));				/* Expecting: call indirect 0 */
	core::arch::asm!("callq *(%rax)", options(att_syntax));				/* Expecting: call indirect 0 */
	core::arch::asm!("callq *(%r8)", options(att_syntax));				/* Expecting: call indirect 0 */
	core::arch::asm!("callq *(0x12345678)", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("callq *0x12345678(%rax,%rcx,8)", options(att_syntax));		/* Expecting: call indirect 0 */
	core::arch::asm!("callq *0x12345678(%r8,%rcx,8)", options(att_syntax));		/* Expecting: call indirect 0 */

	core::arch::asm!("bnd callq *%rax", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("bnd callq *(%rax)", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("bnd callq *(%r8)", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("bnd callq *(0x12345678)", options(att_syntax));		/* Expecting: call indirect 0 */
	core::arch::asm!("bnd callq *0x12345678(%rax,%rcx,8)", options(att_syntax));	/* Expecting: call indirect 0 */
	core::arch::asm!("bnd callq *0x12345678(%r8,%rcx,8)", options(att_syntax));	/* Expecting: call indirect 0 */

	core::arch::asm!("notrack callq *%rax", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("notrack callq *(%rax)", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("notrack callq *(%r8)", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("notrack callq *(0x12345678)", options(att_syntax));		/* Expecting: call indirect 0 */
	core::arch::asm!("notrack callq *0x12345678(%rax,%rcx,8)", options(att_syntax));	/* Expecting: call indirect 0 */
	core::arch::asm!("notrack callq *0x12345678(%r8,%rcx,8)", options(att_syntax));	/* Expecting: call indirect 0 */

	core::arch::asm!("notrack bnd callq *%rax", options(att_syntax));		/* Expecting: call indirect 0 */
	core::arch::asm!("notrack bnd callq *(%rax)", options(att_syntax));		/* Expecting: call indirect 0 */
	core::arch::asm!("notrack bnd callq *(%r8)", options(att_syntax));		/* Expecting: call indirect 0 */
	core::arch::asm!("notrack bnd callq *(0x12345678)", options(att_syntax));	/* Expecting: call indirect 0 */
	core::arch::asm!("notrack bnd callq *0x12345678(%rax,%rcx,8)", options(att_syntax));	/* Expecting: call indirect 0 */
	core::arch::asm!("notrack bnd callq *0x12345678(%r8,%rcx,8)", options(att_syntax));	/* Expecting: call indirect 0 */

	/* jmp with/without notrack prefix */

	core::arch::asm!("jmpq *%rax", options(att_syntax));				/* Expecting: jmp indirect 0 */
	core::arch::asm!("jmpq *(%rax)", options(att_syntax));				/* Expecting: jmp indirect 0 */
	core::arch::asm!("jmpq *(%r8)", options(att_syntax));				/* Expecting: jmp indirect 0 */
	core::arch::asm!("jmpq *(0x12345678)", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("jmpq *0x12345678(%rax,%rcx,8)", options(att_syntax));		/* Expecting: jmp indirect 0 */
	core::arch::asm!("jmpq *0x12345678(%r8,%rcx,8)", options(att_syntax));		/* Expecting: jmp indirect 0 */

	core::arch::asm!("bnd jmpq *%rax", options(att_syntax));				/* Expecting: jmp indirect 0 */
	core::arch::asm!("bnd jmpq *(%rax)", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("bnd jmpq *(%r8)", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("bnd jmpq *(0x12345678)", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("bnd jmpq *0x12345678(%rax,%rcx,8)", options(att_syntax));	/* Expecting: jmp indirect 0 */
	core::arch::asm!("bnd jmpq *0x12345678(%r8,%rcx,8)", options(att_syntax));	/* Expecting: jmp indirect 0 */

	core::arch::asm!("notrack jmpq *%rax", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack jmpq *(%rax)", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack jmpq *(%r8)", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack jmpq *(0x12345678)", options(att_syntax));		/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack jmpq *0x12345678(%rax,%rcx,8)", options(att_syntax));	/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack jmpq *0x12345678(%r8,%rcx,8)", options(att_syntax));	/* Expecting: jmp indirect 0 */

	core::arch::asm!("notrack bnd jmpq *%rax", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack bnd jmpq *(%rax)", options(att_syntax));		/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack bnd jmpq *(%r8)", options(att_syntax));		/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack bnd jmpq *(0x12345678)", options(att_syntax));		/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack bnd jmpq *0x12345678(%rax,%rcx,8)", options(att_syntax));	/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack bnd jmpq *0x12345678(%r8,%rcx,8)", options(att_syntax));	/* Expecting: jmp indirect 0 */

	/* AMX */

	core::arch::asm!("ldtilecfg (%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("ldtilecfg (%r8,%rcx,8)", options(att_syntax));
	core::arch::asm!("sttilecfg (%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("sttilecfg (%r8,%rcx,8)", options(att_syntax));
	core::arch::asm!("tdpbf16ps %tmm0, %tmm1, %tmm2", options(att_syntax));
	core::arch::asm!("tdpbssd %tmm0, %tmm1, %tmm2", options(att_syntax));
	core::arch::asm!("tdpbsud %tmm0, %tmm1, %tmm2", options(att_syntax));
	core::arch::asm!("tdpbusd %tmm0, %tmm1, %tmm2", options(att_syntax));
	core::arch::asm!("tdpbuud %tmm0, %tmm1, %tmm2", options(att_syntax));
	core::arch::asm!("tileloadd (%rax,%rcx,8), %tmm1", options(att_syntax));
	core::arch::asm!("tileloadd (%r8,%rcx,8), %tmm2", options(att_syntax));
	core::arch::asm!("tileloaddt1 (%rax,%rcx,8), %tmm1", options(att_syntax));
	core::arch::asm!("tileloaddt1 (%r8,%rcx,8), %tmm2", options(att_syntax));
	core::arch::asm!("tilerelease", options(att_syntax));
	core::arch::asm!("tilestored %tmm1, (%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("tilestored %tmm2, (%r8,%rcx,8)", options(att_syntax));
	core::arch::asm!("tilezero %tmm0", options(att_syntax));
	core::arch::asm!("tilezero %tmm7", options(att_syntax));

	/* User Interrupt */

	core::arch::asm!("clui", options(att_syntax));
	core::arch::asm!("senduipi %rax", options(att_syntax));
	core::arch::asm!("senduipi %r8", options(att_syntax));
	core::arch::asm!("stui", options(att_syntax));
	core::arch::asm!("testui", options(att_syntax));
	core::arch::asm!("uiret", options(att_syntax));

	/* AVX512-FP16 */

	core::arch::asm!("vaddph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vaddph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vaddph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vaddph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vaddph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vaddph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vaddph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vaddph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vaddph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vaddsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vaddsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vaddsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, %zmm3, %zmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, 0x12345678(%rax,%rcx,8), %zmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, 0x12345678(%eax,%ecx,8), %zmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, %xmm3, %xmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, 0x12345678(%rax,%rcx,8), %xmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, 0x12345678(%eax,%ecx,8), %xmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, %ymm3, %ymm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, 0x12345678(%rax,%rcx,8), %ymm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, 0x12345678(%eax,%ecx,8), %ymm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpsh $0x12, %xmm3, %xmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpsh $0x12, 0x12345678(%rax,%rcx,8), %xmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpsh $0x12, 0x12345678(%eax,%ecx,8), %xmm2, %k5", options(att_syntax));
	core::arch::asm!("vcomish %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcomish 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcomish 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtdq2ph %zmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtdq2ph 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtdq2ph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtdq2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtdq2ph %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtpd2ph %zmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtpd2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtpd2ph %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd %xmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq %xmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq %xmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2w %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2w 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2w 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2w %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2w 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2w 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2w %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2w 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2w 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %zmm1, 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %zmm1, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %zmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %ymm1, 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %ymm1, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %xmm1, 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %xmm1, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %ymm2, 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %ymm2, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %xmm2, 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %xmm2, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2phx %zmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtps2phx 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtps2phx 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtps2phx %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtps2phx %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtqq2ph %zmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtqq2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtqq2ph %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsd2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsh2sd 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsh2si 0x12345678(%eax,%ecx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvtsh2si 0x12345678(%eax,%ecx,8), %rax", options(att_syntax));
	core::arch::asm!("vcvtsh2ss 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsh2usi %xmm1, %eax", options(att_syntax));
	core::arch::asm!("vcvtsh2usi 0x12345678(%rax,%rcx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvtsh2usi 0x12345678(%eax,%ecx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvtsh2usi %xmm1, %rax", options(att_syntax));
	core::arch::asm!("vcvtsh2usi 0x12345678(%rax,%rcx,8), %rax", options(att_syntax));
	core::arch::asm!("vcvtsh2usi 0x12345678(%eax,%ecx,8), %rax", options(att_syntax));
	core::arch::asm!("vcvtsi2sh %eax, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsi2sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsi2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsi2sh %rax, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsi2sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsi2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtss2sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtss2sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtss2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq %xmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq %xmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2w %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2w 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2w 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2w %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2w 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2w 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2w %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2w 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2w 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttsh2si %xmm1, %eax", options(att_syntax));
	core::arch::asm!("vcvttsh2si 0x12345678(%rax,%rcx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvttsh2si 0x12345678(%eax,%ecx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvttsh2si %xmm1, %rax", options(att_syntax));
	core::arch::asm!("vcvttsh2si 0x12345678(%rax,%rcx,8), %rax", options(att_syntax));
	core::arch::asm!("vcvttsh2si 0x12345678(%eax,%ecx,8), %rax", options(att_syntax));
	core::arch::asm!("vcvttsh2usi %xmm1, %eax", options(att_syntax));
	core::arch::asm!("vcvttsh2usi 0x12345678(%rax,%rcx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvttsh2usi 0x12345678(%eax,%ecx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvttsh2usi %xmm1, %rax", options(att_syntax));
	core::arch::asm!("vcvttsh2usi 0x12345678(%rax,%rcx,8), %rax", options(att_syntax));
	core::arch::asm!("vcvttsh2usi 0x12345678(%eax,%ecx,8), %rax", options(att_syntax));
	core::arch::asm!("vcvtudq2ph %zmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtudq2ph 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtudq2ph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtudq2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtudq2ph %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuqq2ph %zmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuqq2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuqq2ph %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtusi2sh %eax, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtusi2sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtusi2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtusi2sh %rax, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtusi2sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtusi2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vdivph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vdivph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vdivph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vdivph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vdivph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vdivph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vdivph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vdivph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vdivph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vdivsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vdivsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vdivsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfcmaddcsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfcmulcph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfcmulcph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfcmulcsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmulcsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmulcsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd132sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd132sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd132sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd213sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd213sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd213sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd231sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd231sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd231sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddcph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddcph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddcsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddcsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddcsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub132sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub132sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub132sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub213sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub213sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub213sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub231sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub231sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub231sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmulcph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmulcph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmulcph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmulcph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmulcph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmulcph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmulcph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmulcph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmulcph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmulcsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmulcsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmulcsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd132sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd213sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd231sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub132sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub213sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub231sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231sh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfpclassph $0x12, %zmm1, %k5", options(att_syntax));
	core::arch::asm!("vfpclassph $0x12, %xmm1, %k5", options(att_syntax));
	core::arch::asm!("vfpclassph $0x12, %ymm1, %k5", options(att_syntax));
	core::arch::asm!("vfpclasssh $0x12, %xmm1, %k5", options(att_syntax));
	core::arch::asm!("vfpclasssh $0x12, 0x12345678(%rax,%rcx,8), %k5", options(att_syntax));
	core::arch::asm!("vfpclasssh $0x12, 0x12345678(%eax,%ecx,8), %k5", options(att_syntax));
	core::arch::asm!("vgetexpph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vgetexpph 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vgetexpph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vgetexpph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetexpph 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vgetexpph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vgetexpph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vgetexpph 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vgetexpph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vgetexpsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetexpsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetexpsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vgetmantsh $0x12, %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetmantsh $0x12, 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetmantsh $0x12, 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmaxph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vmaxph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vmaxph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vmaxph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmaxph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmaxph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmaxph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vmaxph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vmaxph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vmaxsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmaxsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmaxsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vminph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vminph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vminph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vminph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vminph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vminph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vminph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vminph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vminph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vminsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vminsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vminsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmovsh %xmm1, 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("vmovsh %xmm1, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vmovsh 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vmovsh 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vmovsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmovw %xmm1, %eax", options(att_syntax));
	core::arch::asm!("vmovw %xmm1, 0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("vmovw %xmm1, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vmovw %eax, %xmm1", options(att_syntax));
	core::arch::asm!("vmovw 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vmovw 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vmulph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vmulph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vmulph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vmulph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmulph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmulph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmulph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vmulph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vmulph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vmulsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmulsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmulsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrcpph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vrcpph 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vrcpph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vrcpph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrcpph 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vrcpph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vrcpph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vrcpph 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vrcpph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vrcpsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrcpsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrcpsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vreducesh $0x12, %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vreducesh $0x12, 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vreducesh $0x12, 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vrndscalesh $0x12, %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrndscalesh $0x12, 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrndscalesh $0x12, 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vrsqrtph 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vrsqrtph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vrsqrtsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrsqrtsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrsqrtsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vscalefph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vscalefph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vscalefph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vscalefph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vscalefph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vscalefph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vscalefph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vscalefph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vscalefph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vscalefsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vscalefsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vscalefsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsqrtph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vsqrtph 0x12345678(%rax,%rcx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vsqrtph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vsqrtph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsqrtph 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vsqrtph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vsqrtph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vsqrtph 0x12345678(%rax,%rcx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vsqrtph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vsqrtsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsqrtsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsqrtsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsubph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vsubph 0x12345678(%rax,%rcx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vsubph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vsubph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsubph 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsubph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsubph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vsubph 0x12345678(%rax,%rcx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vsubph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vsubsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsubsh 0x12345678(%rax,%rcx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsubsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vucomish %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vucomish 0x12345678(%rax,%rcx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vucomish 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));

	/* Key Locker */

	core::arch::asm!("loadiwkey %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("encodekey128 %eax, %edx", options(att_syntax));
	core::arch::asm!("encodekey256 %eax, %edx", options(att_syntax));
	core::arch::asm!("aesenc128kl 0x77(%rdx), %xmm3", options(att_syntax));
	core::arch::asm!("aesenc256kl 0x77(%rdx), %xmm3", options(att_syntax));
	core::arch::asm!("aesdec128kl 0x77(%rdx), %xmm3", options(att_syntax));
	core::arch::asm!("aesdec256kl 0x77(%rdx), %xmm3", options(att_syntax));
	core::arch::asm!("aesencwide128kl	0x77(%rdx)", options(att_syntax));
	core::arch::asm!("aesencwide256kl	0x77(%rdx)", options(att_syntax));
	core::arch::asm!("aesdecwide128kl	0x77(%rdx)", options(att_syntax));
	core::arch::asm!("aesdecwide256kl	0x77(%rdx)", options(att_syntax));

	/* Remote Atomic Operations */

	core::arch::asm!("aadd %ecx,(%rax)", options(att_syntax));
	core::arch::asm!("aadd %edx,(%r8)", options(att_syntax));
	core::arch::asm!("aadd %edx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("aadd %edx,0x12345678(%r8,%rcx,8)", options(att_syntax));
	core::arch::asm!("aadd %rcx,(%rax)", options(att_syntax));
	core::arch::asm!("aadd %rdx,(%r8)", options(att_syntax));
	core::arch::asm!("aadd %rdx,(0x12345678)", options(att_syntax));
	core::arch::asm!("aadd %rdx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("aadd %rdx,0x12345678(%r8,%rcx,8)", options(att_syntax));

	core::arch::asm!("aand %ecx,(%rax)", options(att_syntax));
	core::arch::asm!("aand %edx,(%r8)", options(att_syntax));
	core::arch::asm!("aand %edx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("aand %edx,0x12345678(%r8,%rcx,8)", options(att_syntax));
	core::arch::asm!("aand %rcx,(%rax)", options(att_syntax));
	core::arch::asm!("aand %rdx,(%r8)", options(att_syntax));
	core::arch::asm!("aand %rdx,(0x12345678)", options(att_syntax));
	core::arch::asm!("aand %rdx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("aand %rdx,0x12345678(%r8,%rcx,8)", options(att_syntax));

	core::arch::asm!("aor %ecx,(%rax)", options(att_syntax));
	core::arch::asm!("aor %edx,(%r8)", options(att_syntax));
	core::arch::asm!("aor %edx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("aor %edx,0x12345678(%r8,%rcx,8)", options(att_syntax));
	core::arch::asm!("aor %rcx,(%rax)", options(att_syntax));
	core::arch::asm!("aor %rdx,(%r8)", options(att_syntax));
	core::arch::asm!("aor %rdx,(0x12345678)", options(att_syntax));
	core::arch::asm!("aor %rdx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("aor %rdx,0x12345678(%r8,%rcx,8)", options(att_syntax));

	core::arch::asm!("axor %ecx,(%rax)", options(att_syntax));
	core::arch::asm!("axor %edx,(%r8)", options(att_syntax));
	core::arch::asm!("axor %edx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("axor %edx,0x12345678(%r8,%rcx,8)", options(att_syntax));
	core::arch::asm!("axor %rcx,(%rax)", options(att_syntax));
	core::arch::asm!("axor %rdx,(%r8)", options(att_syntax));
	core::arch::asm!("axor %rdx,(0x12345678)", options(att_syntax));
	core::arch::asm!("axor %rdx,0x12345678(%rax,%rcx,8)", options(att_syntax));
	core::arch::asm!("axor %rdx,0x12345678(%r8,%rcx,8)", options(att_syntax));

	/* VEX CMPxxXADD */

	core::arch::asm!("cmpbexadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpbxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmplexadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmplxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpnbexadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpnbxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpnlexadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpnlxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpnoxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpnpxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpnsxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpnzxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpoxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmppxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpsxadd %ebx,%ecx,(%r9)", options(att_syntax));
	core::arch::asm!("cmpzxadd %ebx,%ecx,(%r9)", options(att_syntax));

	/* Pre-fetch */

	core::arch::asm!("prefetch (%rax)", options(att_syntax));
	core::arch::asm!("prefetcht0 (%rax)", options(att_syntax));
	core::arch::asm!("prefetcht1 (%rax)", options(att_syntax));
	core::arch::asm!("prefetcht2 (%rax)", options(att_syntax));
	core::arch::asm!("prefetchnta (%rax)", options(att_syntax));
	core::arch::asm!("prefetchit0 0x12345678(%rip)", options(att_syntax));
	core::arch::asm!("prefetchit1 0x12345678(%rip)", options(att_syntax));

	/* MSR List */

	core::arch::asm!("rdmsrlist", options(att_syntax));
	core::arch::asm!("wrmsrlist", options(att_syntax));

	/* User Read/Write MSR */

	core::arch::asm!("urdmsr %rdx,%rax", options(att_syntax));
	core::arch::asm!("urdmsr %rdx,%r22", options(att_syntax));
	core::arch::asm!("urdmsr $0x7f,%r12", options(att_syntax));
	core::arch::asm!("uwrmsr %rax,%rdx", options(att_syntax));
	core::arch::asm!("uwrmsr %r22,%rdx", options(att_syntax));
	core::arch::asm!("uwrmsr %r12,$0x7f", options(att_syntax));

	/* AVX NE Convert */

	core::arch::asm!("vbcstnebf162ps (%rcx),%xmm6", options(att_syntax));
	core::arch::asm!("vbcstnesh2ps (%rcx),%xmm6", options(att_syntax));
	core::arch::asm!("vcvtneebf162ps (%rcx),%xmm6", options(att_syntax));
	core::arch::asm!("vcvtneeph2ps (%rcx),%xmm6", options(att_syntax));
	core::arch::asm!("vcvtneobf162ps (%rcx),%xmm6", options(att_syntax));
	core::arch::asm!("vcvtneoph2ps (%rcx),%xmm6", options(att_syntax));
	core::arch::asm!("vcvtneps2bf16 %xmm1,%xmm6", options(att_syntax));

	/* FRED */

	core::arch::asm!("erets", options(att_syntax));	/* Expecting: erets indirect 0 */
	core::arch::asm!("eretu", options(att_syntax));	/* Expecting: eretu indirect 0 */

	/* AMX Complex */

	core::arch::asm!("tcmmimfp16ps %tmm1,%tmm2,%tmm3", options(att_syntax));
	core::arch::asm!("tcmmrlfp16ps %tmm1,%tmm2,%tmm3", options(att_syntax));

	/* AMX FP16 */

	core::arch::asm!("tdpfp16ps %tmm1,%tmm2,%tmm3", options(att_syntax));

	/* REX2 */

	core::arch::asm!("test $0x5, %r18b", options(att_syntax));
	core::arch::asm!("test $0x5, %r18d", options(att_syntax));
	core::arch::asm!("test $0x5, %r18", options(att_syntax));
	core::arch::asm!("test $0x5, %r18w", options(att_syntax));
	core::arch::asm!("imull %eax, %r14d", options(att_syntax));
	core::arch::asm!("imull %eax, %r17d", options(att_syntax));
	core::arch::asm!("punpckldq (%r18), %mm2", options(att_syntax));
	core::arch::asm!("leal (%rax), %r16d", options(att_syntax));
	core::arch::asm!("leal (%rax), %r31d", options(att_syntax));
	core::arch::asm!("leal (,%r16), %eax", options(att_syntax));
	core::arch::asm!("leal (,%r31), %eax", options(att_syntax));
	core::arch::asm!("leal (%r16), %eax", options(att_syntax));
	core::arch::asm!("leal (%r31), %eax", options(att_syntax));
	core::arch::asm!("leaq (%rax), %r15", options(att_syntax));
	core::arch::asm!("leaq (%rax), %r16", options(att_syntax));
	core::arch::asm!("leaq (%r15), %rax", options(att_syntax));
	core::arch::asm!("leaq (%r16), %rax", options(att_syntax));
	core::arch::asm!("leaq (,%r15), %rax", options(att_syntax));
	core::arch::asm!("leaq (,%r16), %rax", options(att_syntax));
	core::arch::asm!("add (%r16), %r8", options(att_syntax));
	core::arch::asm!("add (%r16), %r15", options(att_syntax));
	core::arch::asm!("mov (,%r9), %r16", options(att_syntax));
	core::arch::asm!("mov (,%r14), %r16", options(att_syntax));
	core::arch::asm!("sub (%r10), %r31", options(att_syntax));
	core::arch::asm!("sub (%r13), %r31", options(att_syntax));
	core::arch::asm!("leal 1(%r16, %r21), %eax", options(att_syntax));
	core::arch::asm!("leal 1(%r16, %r26), %r31d", options(att_syntax));
	core::arch::asm!("leal 129(%r21, %r9), %eax", options(att_syntax));
	core::arch::asm!("leal 129(%r26, %r9), %r31d", options(att_syntax));
	/*
	 * Have to use .byte for jmpabs because gas does not support the
	 * mnemonic for some reason, but then it also gets the source line wrong
	 * with .byte, so the following is a workaround.
	 */
	core::arch::asm!("", options(att_syntax)); /* Expecting: jmp indirect 0 */
	core::arch::asm!(".byte 0xd5, 0x00, 0xa1, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34, 0x12", options(att_syntax));
	core::arch::asm!("pushp %rbx", options(att_syntax));
	core::arch::asm!("pushp %r16", options(att_syntax));
	core::arch::asm!("pushp %r31", options(att_syntax));
	core::arch::asm!("popp %r31", options(att_syntax));
	core::arch::asm!("popp %r16", options(att_syntax));
	core::arch::asm!("popp %rbx", options(att_syntax));

	/* APX */

	core::arch::asm!("bextr %r25d,%edx,%r10d", options(att_syntax));
	core::arch::asm!("bextr %r25d,0x123(%r31,%rax,4),%edx", options(att_syntax));
	core::arch::asm!("bextr %r31,%r15,%r11", options(att_syntax));
	core::arch::asm!("bextr %r31,0x123(%r31,%rax,4),%r15", options(att_syntax));
	core::arch::asm!("blsi %r25d,%edx", options(att_syntax));
	core::arch::asm!("blsi %r31,%r15", options(att_syntax));
	core::arch::asm!("blsi 0x123(%r31,%rax,4),%r25d", options(att_syntax));
	core::arch::asm!("blsi 0x123(%r31,%rax,4),%r31", options(att_syntax));
	core::arch::asm!("blsmsk %r25d,%edx", options(att_syntax));
	core::arch::asm!("blsmsk %r31,%r15", options(att_syntax));
	core::arch::asm!("blsmsk 0x123(%r31,%rax,4),%r25d", options(att_syntax));
	core::arch::asm!("blsmsk 0x123(%r31,%rax,4),%r31", options(att_syntax));
	core::arch::asm!("blsr %r25d,%edx", options(att_syntax));
	core::arch::asm!("blsr %r31,%r15", options(att_syntax));
	core::arch::asm!("blsr 0x123(%r31,%rax,4),%r25d", options(att_syntax));
	core::arch::asm!("blsr 0x123(%r31,%rax,4),%r31", options(att_syntax));
	core::arch::asm!("bzhi %r25d,%edx,%r10d", options(att_syntax));
	core::arch::asm!("bzhi %r25d,0x123(%r31,%rax,4),%edx", options(att_syntax));
	core::arch::asm!("bzhi %r31,%r15,%r11", options(att_syntax));
	core::arch::asm!("bzhi %r31,0x123(%r31,%rax,4),%r15", options(att_syntax));
	core::arch::asm!("cmpbexadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpbexadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpbxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpbxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmplxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmplxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnbexadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnbexadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnbxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnbxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnlexadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnlexadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnlxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnlxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnoxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnoxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnpxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnpxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnsxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnsxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnzxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpnzxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpoxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpoxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmppxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmppxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpsxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpsxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpzxadd %r25d,%edx,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("cmpzxadd %r31,%r15,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("crc32q %r31, %r22", options(att_syntax));
	core::arch::asm!("crc32q (%r31), %r22", options(att_syntax));
	core::arch::asm!("crc32b %r19b, %r17", options(att_syntax));
	core::arch::asm!("crc32b %r19b, %r21d", options(att_syntax));
	core::arch::asm!("crc32b (%r19),%ebx", options(att_syntax));
	core::arch::asm!("crc32l %r31d, %r23d", options(att_syntax));
	core::arch::asm!("crc32l (%r31), %r23d", options(att_syntax));
	core::arch::asm!("crc32w %r31w, %r21d", options(att_syntax));
	core::arch::asm!("crc32w (%r31),%r21d", options(att_syntax));
	core::arch::asm!("crc32 %rax, %r18", options(att_syntax));
	core::arch::asm!("enqcmd 0x123(%r31d,%eax,4),%r25d", options(att_syntax));
	core::arch::asm!("enqcmd 0x123(%r31,%rax,4),%r31", options(att_syntax));
	core::arch::asm!("enqcmds 0x123(%r31d,%eax,4),%r25d", options(att_syntax));
	core::arch::asm!("enqcmds 0x123(%r31,%rax,4),%r31", options(att_syntax));
	core::arch::asm!("invept 0x123(%r31,%rax,4),%r31", options(att_syntax));
	core::arch::asm!("invpcid 0x123(%r31,%rax,4),%r31", options(att_syntax));
	core::arch::asm!("invvpid 0x123(%r31,%rax,4),%r31", options(att_syntax));
	core::arch::asm!("kmovb %k5,%r25d", options(att_syntax));
	core::arch::asm!("kmovb %k5,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("kmovb %r25d,%k5", options(att_syntax));
	core::arch::asm!("kmovb 0x123(%r31,%rax,4),%k5", options(att_syntax));
	core::arch::asm!("kmovd %k5,%r25d", options(att_syntax));
	core::arch::asm!("kmovd %k5,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("kmovd %r25d,%k5", options(att_syntax));
	core::arch::asm!("kmovd 0x123(%r31,%rax,4),%k5", options(att_syntax));
	core::arch::asm!("kmovq %k5,%r31", options(att_syntax));
	core::arch::asm!("kmovq %k5,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("kmovq %r31,%k5", options(att_syntax));
	core::arch::asm!("kmovq 0x123(%r31,%rax,4),%k5", options(att_syntax));
	core::arch::asm!("kmovw %k5,%r25d", options(att_syntax));
	core::arch::asm!("kmovw %k5,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("kmovw %r25d,%k5", options(att_syntax));
	core::arch::asm!("kmovw 0x123(%r31,%rax,4),%k5", options(att_syntax));
	core::arch::asm!("ldtilecfg 0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("movbe %r18w,%ax", options(att_syntax));
	core::arch::asm!("movbe %r15w,%ax", options(att_syntax));
	core::arch::asm!("movbe %r18w,0x123(%r16,%rax,4)", options(att_syntax));
	core::arch::asm!("movbe %r18w,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("movbe %r25d,%edx", options(att_syntax));
	core::arch::asm!("movbe %r15d,%edx", options(att_syntax));
	core::arch::asm!("movbe %r25d,0x123(%r16,%rax,4)", options(att_syntax));
	core::arch::asm!("movbe %r31,%r15", options(att_syntax));
	core::arch::asm!("movbe %r8,%r15", options(att_syntax));
	core::arch::asm!("movbe %r31,0x123(%r16,%rax,4)", options(att_syntax));
	core::arch::asm!("movbe %r31,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("movbe 0x123(%r16,%rax,4),%r31", options(att_syntax));
	core::arch::asm!("movbe 0x123(%r31,%rax,4),%r18w", options(att_syntax));
	core::arch::asm!("movbe 0x123(%r31,%rax,4),%r25d", options(att_syntax));
	core::arch::asm!("movdir64b 0x123(%r31d,%eax,4),%r25d", options(att_syntax));
	core::arch::asm!("movdir64b 0x123(%r31,%rax,4),%r31", options(att_syntax));
	core::arch::asm!("movdiri %r25d,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("movdiri %r31,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("pdep %r25d,%edx,%r10d", options(att_syntax));
	core::arch::asm!("pdep %r31,%r15,%r11", options(att_syntax));
	core::arch::asm!("pdep 0x123(%r31,%rax,4),%r25d,%edx", options(att_syntax));
	core::arch::asm!("pdep 0x123(%r31,%rax,4),%r31,%r15", options(att_syntax));
	core::arch::asm!("pext %r25d,%edx,%r10d", options(att_syntax));
	core::arch::asm!("pext %r31,%r15,%r11", options(att_syntax));
	core::arch::asm!("pext 0x123(%r31,%rax,4),%r25d,%edx", options(att_syntax));
	core::arch::asm!("pext 0x123(%r31,%rax,4),%r31,%r15", options(att_syntax));
	core::arch::asm!("shlx %r25d,%edx,%r10d", options(att_syntax));
	core::arch::asm!("shlx %r25d,0x123(%r31,%rax,4),%edx", options(att_syntax));
	core::arch::asm!("shlx %r31,%r15,%r11", options(att_syntax));
	core::arch::asm!("shlx %r31,0x123(%r31,%rax,4),%r15", options(att_syntax));
	core::arch::asm!("shrx %r25d,%edx,%r10d", options(att_syntax));
	core::arch::asm!("shrx %r25d,0x123(%r31,%rax,4),%edx", options(att_syntax));
	core::arch::asm!("shrx %r31,%r15,%r11", options(att_syntax));
	core::arch::asm!("shrx %r31,0x123(%r31,%rax,4),%r15", options(att_syntax));
	core::arch::asm!("sttilecfg 0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("tileloadd 0x123(%r31,%rax,4),%tmm6", options(att_syntax));
	core::arch::asm!("tileloaddt1 0x123(%r31,%rax,4),%tmm6", options(att_syntax));
	core::arch::asm!("tilestored %tmm6,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("vbroadcastf128 (%r16),%ymm3", options(att_syntax));
	core::arch::asm!("vbroadcasti128 (%r16),%ymm3", options(att_syntax));
	core::arch::asm!("vextractf128 $1,%ymm3,(%r16)", options(att_syntax));
	core::arch::asm!("vextracti128 $1,%ymm3,(%r16)", options(att_syntax));
	core::arch::asm!("vinsertf128 $1,(%r16),%ymm3,%ymm8", options(att_syntax));
	core::arch::asm!("vinserti128 $1,(%r16),%ymm3,%ymm8", options(att_syntax));
	core::arch::asm!("vroundpd $1,(%r24),%xmm6", options(att_syntax));
	core::arch::asm!("vroundps $2,(%r24),%xmm6", options(att_syntax));
	core::arch::asm!("vroundsd $3,(%r24),%xmm6,%xmm3", options(att_syntax));
	core::arch::asm!("vroundss $4,(%r24),%xmm6,%xmm3", options(att_syntax));
	core::arch::asm!("wrssd %r25d,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("wrssq %r31,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("wrussd %r25d,0x123(%r31,%rax,4)", options(att_syntax));
	core::arch::asm!("wrussq %r31,0x123(%r31,%rax,4)", options(att_syntax));

	/* APX new data destination */

	core::arch::asm!("adc $0x1234,%ax,%r30w", options(att_syntax));
	core::arch::asm!("adc %r15b,%r17b,%r18b", options(att_syntax));
	core::arch::asm!("adc %r15d,(%r8),%r18d", options(att_syntax));
	core::arch::asm!("adc (%r15,%rax,1),%r16b,%r8b", options(att_syntax));
	core::arch::asm!("adc (%r15,%rax,1),%r16w,%r8w", options(att_syntax));
	core::arch::asm!("adcl $0x11,(%r19,%rax,4),%r20d", options(att_syntax));
	core::arch::asm!("adcx %r15d,%r8d,%r18d", options(att_syntax));
	core::arch::asm!("adcx (%r15,%r31,1),%r8", options(att_syntax));
	core::arch::asm!("adcx (%r15,%r31,1),%r8d,%r18d", options(att_syntax));
	core::arch::asm!("add $0x1234,%ax,%r30w", options(att_syntax));
	core::arch::asm!("add $0x12344433,%r15,%r16", options(att_syntax));
	core::arch::asm!("add $0x34,%r13b,%r17b", options(att_syntax));
	core::arch::asm!("add $0xfffffffff4332211,%rax,%r8", options(att_syntax));
	core::arch::asm!("add %r31,%r8,%r16", options(att_syntax));
	core::arch::asm!("add %r31,(%r8),%r16", options(att_syntax));
	core::arch::asm!("add %r31,(%r8,%r16,8),%r16", options(att_syntax));
	core::arch::asm!("add %r31b,%r8b,%r16b", options(att_syntax));
	core::arch::asm!("add %r31d,%r8d,%r16d", options(att_syntax));
	core::arch::asm!("add %r31w,%r8w,%r16w", options(att_syntax));
	core::arch::asm!("add (%r31),%r8,%r16", options(att_syntax));
	core::arch::asm!("add 0x9090(%r31,%r16,1),%r8,%r16", options(att_syntax));
	core::arch::asm!("addb %r31b,%r8b,%r16b", options(att_syntax));
	core::arch::asm!("addl %r31d,%r8d,%r16d", options(att_syntax));
	core::arch::asm!("addl $0x11,(%r19,%rax,4),%r20d", options(att_syntax));
	core::arch::asm!("addq %r31,%r8,%r16", options(att_syntax));
	core::arch::asm!("addq $0x12344433,(%r15,%rcx,4),%r16", options(att_syntax));
	core::arch::asm!("addw %r31w,%r8w,%r16w", options(att_syntax));
	core::arch::asm!("adox %r15d,%r8d,%r18d", options(att_syntax));
	core::arch::asm!("{{load}} add %r31,%r8,%r16", options(att_syntax));
	core::arch::asm!("{{store}} add %r31,%r8,%r16", options(att_syntax));
	core::arch::asm!("adox (%r15,%r31,1),%r8", options(att_syntax));
	core::arch::asm!("adox (%r15,%r31,1),%r8d,%r18d", options(att_syntax));
	core::arch::asm!("and $0x1234,%ax,%r30w", options(att_syntax));
	core::arch::asm!("and %r15b,%r17b,%r18b", options(att_syntax));
	core::arch::asm!("and %r15d,(%r8),%r18d", options(att_syntax));
	core::arch::asm!("and (%r15,%rax,1),%r16b,%r8b", options(att_syntax));
	core::arch::asm!("and (%r15,%rax,1),%r16w,%r8w", options(att_syntax));
	core::arch::asm!("andl $0x11,(%r19,%rax,4),%r20d", options(att_syntax));
	core::arch::asm!("cmova 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovae 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovb 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovbe 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmove 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovg 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovge 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovl 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovle 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovne 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovno 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovnp 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovns 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovo 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovp 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("cmovs 0x90909090(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("dec %rax,%r17", options(att_syntax));
	core::arch::asm!("decb (%r31,%r12,1),%r8b", options(att_syntax));
	core::arch::asm!("imul 0x909(%rax,%r31,8),%rdx,%r25", options(att_syntax));
	core::arch::asm!("imul 0x90909(%eax),%edx,%r8d", options(att_syntax));
	core::arch::asm!("inc %r31,%r16", options(att_syntax));
	core::arch::asm!("inc %r31,%r8", options(att_syntax));
	core::arch::asm!("inc %rax,%rbx", options(att_syntax));
	core::arch::asm!("neg %rax,%r17", options(att_syntax));
	core::arch::asm!("negb (%r31,%r12,1),%r8b", options(att_syntax));
	core::arch::asm!("not %rax,%r17", options(att_syntax));
	core::arch::asm!("notb (%r31,%r12,1),%r8b", options(att_syntax));
	core::arch::asm!("or $0x1234,%ax,%r30w", options(att_syntax));
	core::arch::asm!("or %r15b,%r17b,%r18b", options(att_syntax));
	core::arch::asm!("or %r15d,(%r8),%r18d", options(att_syntax));
	core::arch::asm!("or (%r15,%rax,1),%r16b,%r8b", options(att_syntax));
	core::arch::asm!("or (%r15,%rax,1),%r16w,%r8w", options(att_syntax));
	core::arch::asm!("orl $0x11,(%r19,%rax,4),%r20d", options(att_syntax));
	core::arch::asm!("rcl $0x2,%r12b,%r31b", options(att_syntax));
	core::arch::asm!("rcl %cl,%r16b,%r8b", options(att_syntax));
	core::arch::asm!("rclb $0x1,(%rax),%r31b", options(att_syntax));
	core::arch::asm!("rcll $0x2,(%rax),%r31d", options(att_syntax));
	core::arch::asm!("rclw $0x1,(%rax),%r31w", options(att_syntax));
	core::arch::asm!("rclw %cl,(%r19,%rax,4),%r31w", options(att_syntax));
	core::arch::asm!("rcr $0x2,%r12b,%r31b", options(att_syntax));
	core::arch::asm!("rcr %cl,%r16b,%r8b", options(att_syntax));
	core::arch::asm!("rcrb $0x1,(%rax),%r31b", options(att_syntax));
	core::arch::asm!("rcrl $0x2,(%rax),%r31d", options(att_syntax));
	core::arch::asm!("rcrw $0x1,(%rax),%r31w", options(att_syntax));
	core::arch::asm!("rcrw %cl,(%r19,%rax,4),%r31w", options(att_syntax));
	core::arch::asm!("rol $0x2,%r12b,%r31b", options(att_syntax));
	core::arch::asm!("rol %cl,%r16b,%r8b", options(att_syntax));
	core::arch::asm!("rolb $0x1,(%rax),%r31b", options(att_syntax));
	core::arch::asm!("roll $0x2,(%rax),%r31d", options(att_syntax));
	core::arch::asm!("rolw $0x1,(%rax),%r31w", options(att_syntax));
	core::arch::asm!("rolw %cl,(%r19,%rax,4),%r31w", options(att_syntax));
	core::arch::asm!("ror $0x2,%r12b,%r31b", options(att_syntax));
	core::arch::asm!("ror %cl,%r16b,%r8b", options(att_syntax));
	core::arch::asm!("rorb $0x1,(%rax),%r31b", options(att_syntax));
	core::arch::asm!("rorl $0x2,(%rax),%r31d", options(att_syntax));
	core::arch::asm!("rorw $0x1,(%rax),%r31w", options(att_syntax));
	core::arch::asm!("rorw %cl,(%r19,%rax,4),%r31w", options(att_syntax));
	core::arch::asm!("sar $0x2,%r12b,%r31b", options(att_syntax));
	core::arch::asm!("sar %cl,%r16b,%r8b", options(att_syntax));
	core::arch::asm!("sarb $0x1,(%rax),%r31b", options(att_syntax));
	core::arch::asm!("sarl $0x2,(%rax),%r31d", options(att_syntax));
	core::arch::asm!("sarw $0x1,(%rax),%r31w", options(att_syntax));
	core::arch::asm!("sarw %cl,(%r19,%rax,4),%r31w", options(att_syntax));
	core::arch::asm!("sbb $0x1234,%ax,%r30w", options(att_syntax));
	core::arch::asm!("sbb %r15b,%r17b,%r18b", options(att_syntax));
	core::arch::asm!("sbb %r15d,(%r8),%r18d", options(att_syntax));
	core::arch::asm!("sbb (%r15,%rax,1),%r16b,%r8b", options(att_syntax));
	core::arch::asm!("sbb (%r15,%rax,1),%r16w,%r8w", options(att_syntax));
	core::arch::asm!("sbbl $0x11,(%r19,%rax,4),%r20d", options(att_syntax));
	core::arch::asm!("shl $0x2,%r12b,%r31b", options(att_syntax));
	core::arch::asm!("shl $0x2,%r12b,%r31b", options(att_syntax));
	core::arch::asm!("shl %cl,%r16b,%r8b", options(att_syntax));
	core::arch::asm!("shl %cl,%r16b,%r8b", options(att_syntax));
	core::arch::asm!("shlb $0x1,(%rax),%r31b", options(att_syntax));
	core::arch::asm!("shlb $0x1,(%rax),%r31b", options(att_syntax));
	core::arch::asm!("shld $0x1,%r12,(%rax),%r31", options(att_syntax));
	core::arch::asm!("shld $0x2,%r15d,(%rax),%r31d", options(att_syntax));
	core::arch::asm!("shld $0x2,%r8w,%r12w,%r31w", options(att_syntax));
	core::arch::asm!("shld %cl,%r12,%r16,%r8", options(att_syntax));
	core::arch::asm!("shld %cl,%r13w,(%r19,%rax,4),%r31w", options(att_syntax));
	core::arch::asm!("shld %cl,%r9w,(%rax),%r31w", options(att_syntax));
	core::arch::asm!("shll $0x2,(%rax),%r31d", options(att_syntax));
	core::arch::asm!("shll $0x2,(%rax),%r31d", options(att_syntax));
	core::arch::asm!("shlw $0x1,(%rax),%r31w", options(att_syntax));
	core::arch::asm!("shlw $0x1,(%rax),%r31w", options(att_syntax));
	core::arch::asm!("shlw %cl,(%r19,%rax,4),%r31w", options(att_syntax));
	core::arch::asm!("shlw %cl,(%r19,%rax,4),%r31w", options(att_syntax));
	core::arch::asm!("shr $0x2,%r12b,%r31b", options(att_syntax));
	core::arch::asm!("shr %cl,%r16b,%r8b", options(att_syntax));
	core::arch::asm!("shrb $0x1,(%rax),%r31b", options(att_syntax));
	core::arch::asm!("shrd $0x1,%r12,(%rax),%r31", options(att_syntax));
	core::arch::asm!("shrd $0x2,%r15d,(%rax),%r31d", options(att_syntax));
	core::arch::asm!("shrd $0x2,%r8w,%r12w,%r31w", options(att_syntax));
	core::arch::asm!("shrd %cl,%r12,%r16,%r8", options(att_syntax));
	core::arch::asm!("shrd %cl,%r13w,(%r19,%rax,4),%r31w", options(att_syntax));
	core::arch::asm!("shrd %cl,%r9w,(%rax),%r31w", options(att_syntax));
	core::arch::asm!("shrl $0x2,(%rax),%r31d", options(att_syntax));
	core::arch::asm!("shrw $0x1,(%rax),%r31w", options(att_syntax));
	core::arch::asm!("shrw %cl,(%r19,%rax,4),%r31w", options(att_syntax));
	core::arch::asm!("sub $0x1234,%ax,%r30w", options(att_syntax));
	core::arch::asm!("sub %r15b,%r17b,%r18b", options(att_syntax));
	core::arch::asm!("sub %r15d,(%r8),%r18d", options(att_syntax));
	core::arch::asm!("sub (%r15,%rax,1),%r16b,%r8b", options(att_syntax));
	core::arch::asm!("sub (%r15,%rax,1),%r16w,%r8w", options(att_syntax));
	core::arch::asm!("subl $0x11,(%r19,%rax,4),%r20d", options(att_syntax));
	core::arch::asm!("xor $0x1234,%ax,%r30w", options(att_syntax));
	core::arch::asm!("xor %r15b,%r17b,%r18b", options(att_syntax));
	core::arch::asm!("xor %r15d,(%r8),%r18d", options(att_syntax));
	core::arch::asm!("xor (%r15,%rax,1),%r16b,%r8b", options(att_syntax));
	core::arch::asm!("xor (%r15,%rax,1),%r16w,%r8w", options(att_syntax));
	core::arch::asm!("xorl $0x11,(%r19,%rax,4),%r20d", options(att_syntax));

	/* APX suppress status flags */

	core::arch::asm!("{{nf}} add %bl,%dl,%r8b", options(att_syntax));
	core::arch::asm!("{{nf}} add %dx,%ax,%r9w", options(att_syntax));
	core::arch::asm!("{{nf}} add 0x123(%r8,%rax,4),%bl,%dl", options(att_syntax));
	core::arch::asm!("{{nf}} add 0x123(%r8,%rax,4),%dx,%ax", options(att_syntax));
	core::arch::asm!("{{nf}} or %bl,%dl,%r8b", options(att_syntax));
	core::arch::asm!("{{nf}} or %dx,%ax,%r9w", options(att_syntax));
	core::arch::asm!("{{nf}} or 0x123(%r8,%rax,4),%bl,%dl", options(att_syntax));
	core::arch::asm!("{{nf}} or 0x123(%r8,%rax,4),%dx,%ax", options(att_syntax));
	core::arch::asm!("{{nf}} and %bl,%dl,%r8b", options(att_syntax));
	core::arch::asm!("{{nf}} and %dx,%ax,%r9w", options(att_syntax));
	core::arch::asm!("{{nf}} and 0x123(%r8,%rax,4),%bl,%dl", options(att_syntax));
	core::arch::asm!("{{nf}} and 0x123(%r8,%rax,4),%dx,%ax", options(att_syntax));
	core::arch::asm!("{{nf}} shld $0x7b,%dx,%ax,%r9w", options(att_syntax));
	core::arch::asm!("{{nf}} sub %bl,%dl,%r8b", options(att_syntax));
	core::arch::asm!("{{nf}} sub %dx,%ax,%r9w", options(att_syntax));
	core::arch::asm!("{{nf}} sub 0x123(%r8,%rax,4),%bl,%dl", options(att_syntax));
	core::arch::asm!("{{nf}} sub 0x123(%r8,%rax,4),%dx,%ax", options(att_syntax));
	core::arch::asm!("{{nf}} shrd $0x7b,%dx,%ax,%r9w", options(att_syntax));
	core::arch::asm!("{{nf}} xor %bl,%dl,%r8b", options(att_syntax));
	core::arch::asm!("{{nf}} xor %r31,%r31", options(att_syntax));
	core::arch::asm!("{{nf}} xor 0x123(%r8,%rax,4),%bl,%dl", options(att_syntax));
	core::arch::asm!("{{nf}} xor 0x123(%r8,%rax,4),%dx,%ax", options(att_syntax));
	core::arch::asm!("{{nf}} imul $0xff90,%r9,%r15", options(att_syntax));
	core::arch::asm!("{{nf}} imul $0x7b,%r9,%r15", options(att_syntax));
	core::arch::asm!("{{nf}} xor $0x7b,%bl,%dl", options(att_syntax));
	core::arch::asm!("{{nf}} xor $0x7b,%dx,%ax", options(att_syntax));
	core::arch::asm!("{{nf}} popcnt %r9,%r31", options(att_syntax));
	core::arch::asm!("{{nf}} shld %cl,%dx,%ax,%r9w", options(att_syntax));
	core::arch::asm!("{{nf}} shrd %cl,%dx,%ax,%r9w", options(att_syntax));
	core::arch::asm!("{{nf}} imul %r9,%r31,%r11", options(att_syntax));
	core::arch::asm!("{{nf}} sar $0x7b,%bl,%dl", options(att_syntax));
	core::arch::asm!("{{nf}} sar $0x7b,%dx,%ax", options(att_syntax));
	core::arch::asm!("{{nf}} sar $1,%bl,%dl", options(att_syntax));
	core::arch::asm!("{{nf}} sar $1,%dx,%ax", options(att_syntax));
	core::arch::asm!("{{nf}} sar %cl,%bl,%dl", options(att_syntax));
	core::arch::asm!("{{nf}} sar %cl,%dx,%ax", options(att_syntax));
	core::arch::asm!("{{nf}} andn %r9,%r31,%r11", options(att_syntax));
	core::arch::asm!("{{nf}} blsi %r9,%r31", options(att_syntax));
	core::arch::asm!("{{nf}} tzcnt %r9,%r31", options(att_syntax));
	core::arch::asm!("{{nf}} lzcnt %r9,%r31", options(att_syntax));
	core::arch::asm!("{{nf}} idiv %bl", options(att_syntax));
	core::arch::asm!("{{nf}} idiv %dx", options(att_syntax));
	core::arch::asm!("{{nf}} dec %bl,%dl", options(att_syntax));
	core::arch::asm!("{{nf}} dec %dx,%ax", options(att_syntax));

}
#[cfg(not(target_arch = "x86_64"))]
{

	/* bound r32, mem (same op code as EVEX prefix) */

	core::arch::asm!("bound %eax, 0x12345678(%ecx)", options(att_syntax));
	core::arch::asm!("bound %ecx, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %ebx, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %esp, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %ebp, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %esi, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %edi, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %ecx, (%eax)", options(att_syntax));
	core::arch::asm!("bound %eax, (0x12345678)", options(att_syntax));
	core::arch::asm!("bound %edx, (%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12345678(,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %edx, (%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bound %edx, (%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12(%eax)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12(%ebp)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12(%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12(%ebp,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12(%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12345678(%ebp)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12345678(%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12345678(%ebp,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12345678(%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bound %edx, 0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* bound r16, mem (same op code as EVEX prefix) */

	core::arch::asm!("bound %ax, 0x12345678(%ecx)", options(att_syntax));
	core::arch::asm!("bound %cx, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %bx, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %sp, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %bp, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %si, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %di, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %cx, (%eax)", options(att_syntax));
	core::arch::asm!("bound %ax, (0x12345678)", options(att_syntax));
	core::arch::asm!("bound %dx, (%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12345678(,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %dx, (%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bound %dx, (%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12(%eax)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12(%ebp)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12(%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12(%ebp,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12(%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12345678(%ebp)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12345678(%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12345678(%ebp,%eax,1)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12345678(%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bound %dx, 0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* AVX-512: Instructions with the same op codes as Mask Instructions  */

	core::arch::asm!("cmovno %eax,%ebx", options(att_syntax));
	core::arch::asm!("cmovno 0x12345678(%eax),%ecx", options(att_syntax));
	core::arch::asm!("cmovno 0x12345678(%eax),%cx", options(att_syntax));

	core::arch::asm!("cmove  %eax,%ebx", options(att_syntax));
	core::arch::asm!("cmove 0x12345678(%eax),%ecx", options(att_syntax));
	core::arch::asm!("cmove 0x12345678(%eax),%cx", options(att_syntax));

	core::arch::asm!("seto    0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("setno   0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("setb    0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("setc    0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("setnae  0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("setae   0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("setnb   0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("setnc   0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("sets    0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("setns   0x12345678(%eax)", options(att_syntax));

	/* AVX-512: Mask Instructions */

	core::arch::asm!("kandw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandd  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kandnw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandnq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandnb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kandnd  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("knotw  %k7,%k6", options(att_syntax));
	core::arch::asm!("knotq  %k7,%k6", options(att_syntax));
	core::arch::asm!("knotb  %k7,%k6", options(att_syntax));
	core::arch::asm!("knotd  %k7,%k6", options(att_syntax));

	core::arch::asm!("korw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("korq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("korb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kord  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kxnorw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxnorq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxnorb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxnord  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kxorw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxorq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxorb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kxord  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kaddw  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kaddq  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kaddb  %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kaddd  %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kunpckbw %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kunpckwd %k7,%k6,%k5", options(att_syntax));
	core::arch::asm!("kunpckdq %k7,%k6,%k5", options(att_syntax));

	core::arch::asm!("kmovw  %k6,%k5", options(att_syntax));
	core::arch::asm!("kmovw  (%ecx),%k5", options(att_syntax));
	core::arch::asm!("kmovw  0x123(%eax,%ecx,8),%k5", options(att_syntax));
	core::arch::asm!("kmovw  %k5,(%ecx)", options(att_syntax));
	core::arch::asm!("kmovw  %k5,0x123(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("kmovw  %eax,%k5", options(att_syntax));
	core::arch::asm!("kmovw  %ebp,%k5", options(att_syntax));
	core::arch::asm!("kmovw  %k5,%eax", options(att_syntax));
	core::arch::asm!("kmovw  %k5,%ebp", options(att_syntax));

	core::arch::asm!("kmovq  %k6,%k5", options(att_syntax));
	core::arch::asm!("kmovq  (%ecx),%k5", options(att_syntax));
	core::arch::asm!("kmovq  0x123(%eax,%ecx,8),%k5", options(att_syntax));
	core::arch::asm!("kmovq  %k5,(%ecx)", options(att_syntax));
	core::arch::asm!("kmovq  %k5,0x123(%eax,%ecx,8)", options(att_syntax));

	core::arch::asm!("kmovb  %k6,%k5", options(att_syntax));
	core::arch::asm!("kmovb  (%ecx),%k5", options(att_syntax));
	core::arch::asm!("kmovb  0x123(%eax,%ecx,8),%k5", options(att_syntax));
	core::arch::asm!("kmovb  %k5,(%ecx)", options(att_syntax));
	core::arch::asm!("kmovb  %k5,0x123(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("kmovb  %eax,%k5", options(att_syntax));
	core::arch::asm!("kmovb  %ebp,%k5", options(att_syntax));
	core::arch::asm!("kmovb  %k5,%eax", options(att_syntax));
	core::arch::asm!("kmovb  %k5,%ebp", options(att_syntax));

	core::arch::asm!("kmovd  %k6,%k5", options(att_syntax));
	core::arch::asm!("kmovd  (%ecx),%k5", options(att_syntax));
	core::arch::asm!("kmovd  0x123(%eax,%ecx,8),%k5", options(att_syntax));
	core::arch::asm!("kmovd  %k5,(%ecx)", options(att_syntax));
	core::arch::asm!("kmovd  %k5,0x123(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("kmovd  %eax,%k5", options(att_syntax));
	core::arch::asm!("kmovd  %ebp,%k5", options(att_syntax));
	core::arch::asm!("kmovd  %k5,%eax", options(att_syntax));
	core::arch::asm!("kmovd  %k5,%ebp", options(att_syntax));

	core::arch::asm!("kortestw %k6,%k5", options(att_syntax));
	core::arch::asm!("kortestq %k6,%k5", options(att_syntax));
	core::arch::asm!("kortestb %k6,%k5", options(att_syntax));
	core::arch::asm!("kortestd %k6,%k5", options(att_syntax));

	core::arch::asm!("ktestw %k6,%k5", options(att_syntax));
	core::arch::asm!("ktestq %k6,%k5", options(att_syntax));
	core::arch::asm!("ktestb %k6,%k5", options(att_syntax));
	core::arch::asm!("ktestd %k6,%k5", options(att_syntax));

	core::arch::asm!("kshiftrw $0x12,%k6,%k5", options(att_syntax));
	core::arch::asm!("kshiftrq $0x5b,%k6,%k5", options(att_syntax));
	core::arch::asm!("kshiftlw $0x12,%k6,%k5", options(att_syntax));
	core::arch::asm!("kshiftlq $0x5b,%k6,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 5b */
	core::arch::asm!("vcvtdq2ps %xmm5,%xmm6", options(att_syntax));
	core::arch::asm!("vcvtqq2ps %zmm5,%ymm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtps2dq %xmm5,%xmm6", options(att_syntax));
	core::arch::asm!("vcvttps2dq %xmm5,%xmm6", options(att_syntax));

	/* AVX-512: Op code 0f 6f */

	core::arch::asm!("movq   %mm0,%mm4", options(att_syntax));
	core::arch::asm!("vmovdqa %ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vmovdqa32 %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vmovdqa64 %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vmovdqu %ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vmovdqu32 %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vmovdqu64 %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vmovdqu8 %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vmovdqu16 %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 78 */

	core::arch::asm!("vmread %eax,%ebx", options(att_syntax));
	core::arch::asm!("vcvttps2udq %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vcvttpd2udq %zmm5,%ymm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvttsd2usi %xmm6,%eax", options(att_syntax));
	core::arch::asm!("vcvttss2usi %xmm6,%eax", options(att_syntax));
	core::arch::asm!("vcvttps2uqq %ymm5,%zmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvttpd2uqq %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 79 */

	core::arch::asm!("vmwrite %eax,%ebx", options(att_syntax));
	core::arch::asm!("vcvtps2udq %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vcvtpd2udq %zmm5,%ymm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtsd2usi %xmm6,%eax", options(att_syntax));
	core::arch::asm!("vcvtss2usi %xmm6,%eax", options(att_syntax));
	core::arch::asm!("vcvtps2uqq %ymm5,%zmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtpd2uqq %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 7a */

	core::arch::asm!("vcvtudq2pd %ymm5,%zmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtuqq2pd %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vcvtudq2ps %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vcvtuqq2ps %zmm5,%ymm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvttps2qq %ymm5,%zmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvttpd2qq %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 7b */

	core::arch::asm!("vcvtusi2sd %eax,%xmm5,%xmm6", options(att_syntax));
	core::arch::asm!("vcvtusi2ss %eax,%xmm5,%xmm6", options(att_syntax));
	core::arch::asm!("vcvtps2qq %ymm5,%zmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtpd2qq %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 7f */

	core::arch::asm!("movq.s  %mm0,%mm4", options(att_syntax));
	core::arch::asm!("vmovdqa.s %ymm5,%ymm6", options(att_syntax));
	core::arch::asm!("vmovdqa32.s %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vmovdqa64.s %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vmovdqu.s %ymm5,%ymm6", options(att_syntax));
	core::arch::asm!("vmovdqu32.s %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vmovdqu64.s %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vmovdqu8.s %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vmovdqu16.s %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f db */

	core::arch::asm!("pand  %mm1,%mm2", options(att_syntax));
	core::arch::asm!("pand  %xmm1,%xmm2", options(att_syntax));
	core::arch::asm!("vpand  %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpandd %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpandq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f df */

	core::arch::asm!("pandn  %mm1,%mm2", options(att_syntax));
	core::arch::asm!("pandn  %xmm1,%xmm2", options(att_syntax));
	core::arch::asm!("vpandn %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpandnd %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpandnq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f e6 */

	core::arch::asm!("vcvttpd2dq %xmm1,%xmm2", options(att_syntax));
	core::arch::asm!("vcvtdq2pd %xmm5,%xmm6", options(att_syntax));
	core::arch::asm!("vcvtdq2pd %ymm5,%zmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vcvtqq2pd %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vcvtpd2dq %xmm1,%xmm2", options(att_syntax));

	/* AVX-512: Op code 0f eb */

	core::arch::asm!("por   %mm4,%mm6", options(att_syntax));
	core::arch::asm!("vpor   %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpord  %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vporq  %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f ef */

	core::arch::asm!("pxor   %mm4,%mm6", options(att_syntax));
	core::arch::asm!("vpxor  %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpxord %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpxorq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 10 */

	core::arch::asm!("pblendvb %xmm1,%xmm0", options(att_syntax));
	core::arch::asm!("vpsrlvw %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpmovuswb %zmm5,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 11 */

	core::arch::asm!("vpmovusdb %zmm5,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vpsravw %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 12 */

	core::arch::asm!("vpmovusqb %zmm5,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vpsllvw %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 13 */

	core::arch::asm!("vcvtph2ps %xmm3,%ymm5", options(att_syntax));
	core::arch::asm!("vcvtph2ps %ymm5,%zmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vpmovusdw %zmm5,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 14 */

	core::arch::asm!("blendvps %xmm1,%xmm0", options(att_syntax));
	core::arch::asm!("vpmovusqw %zmm5,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vprorvd %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vprorvq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 15 */

	core::arch::asm!("blendvpd %xmm1,%xmm0", options(att_syntax));
	core::arch::asm!("vpmovusqd %zmm5,%ymm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vprolvd %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vprolvq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 16 */

	core::arch::asm!("vpermps %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpermps %ymm4,%ymm6,%ymm2{{%k7}}", options(att_syntax));
	core::arch::asm!("vpermpd %ymm4,%ymm6,%ymm2{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 19 */

	core::arch::asm!("vbroadcastsd %xmm4,%ymm6", options(att_syntax));
	core::arch::asm!("vbroadcastf32x2 %xmm7,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 1a */

	core::arch::asm!("vbroadcastf128 (%ecx),%ymm4", options(att_syntax));
	core::arch::asm!("vbroadcastf32x4 (%ecx),%zmm6", options(att_syntax));
	core::arch::asm!("vbroadcastf64x2 (%ecx),%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 1b */

	core::arch::asm!("vbroadcastf32x8 (%ecx),%zmm6", options(att_syntax));
	core::arch::asm!("vbroadcastf64x4 (%ecx),%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 1f */

	core::arch::asm!("vpabsq %zmm4,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 20 */

	core::arch::asm!("vpmovsxbw %xmm4,%xmm5", options(att_syntax));
	core::arch::asm!("vpmovswb %zmm5,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 21 */

	core::arch::asm!("vpmovsxbd %xmm4,%ymm6", options(att_syntax));
	core::arch::asm!("vpmovsdb %zmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 22 */

	core::arch::asm!("vpmovsxbq %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovsqb %zmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 23 */

	core::arch::asm!("vpmovsxwd %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovsdw %zmm5,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 24 */

	core::arch::asm!("vpmovsxwq %xmm4,%ymm6", options(att_syntax));
	core::arch::asm!("vpmovsqw %zmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 25 */

	core::arch::asm!("vpmovsxdq %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovsqd %zmm5,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 26 */

	core::arch::asm!("vptestmb %zmm5,%zmm6,%k5", options(att_syntax));
	core::arch::asm!("vptestmw %zmm5,%zmm6,%k5", options(att_syntax));
	core::arch::asm!("vptestnmb %zmm4,%zmm5,%k5", options(att_syntax));
	core::arch::asm!("vptestnmw %zmm4,%zmm5,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 38 27 */

	core::arch::asm!("vptestmd %zmm5,%zmm6,%k5", options(att_syntax));
	core::arch::asm!("vptestmq %zmm5,%zmm6,%k5", options(att_syntax));
	core::arch::asm!("vptestnmd %zmm4,%zmm5,%k5", options(att_syntax));
	core::arch::asm!("vptestnmq %zmm4,%zmm5,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 38 28 */

	core::arch::asm!("vpmuldq %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmovm2b %k5,%zmm6", options(att_syntax));
	core::arch::asm!("vpmovm2w %k5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 29 */

	core::arch::asm!("vpcmpeqq %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmovb2m %zmm6,%k5", options(att_syntax));
	core::arch::asm!("vpmovw2m %zmm6,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 38 2a */

	core::arch::asm!("vmovntdqa (%ecx),%ymm4", options(att_syntax));
	core::arch::asm!("vpbroadcastmb2q %k6,%zmm1", options(att_syntax));

	/* AVX-512: Op code 0f 38 2c */

	core::arch::asm!("vmaskmovps (%ecx),%ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vscalefps %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vscalefpd %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 2d */

	core::arch::asm!("vmaskmovpd (%ecx),%ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vscalefss %xmm4,%xmm5,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vscalefsd %xmm4,%xmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 30 */

	core::arch::asm!("vpmovzxbw %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovwb %zmm5,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 31 */

	core::arch::asm!("vpmovzxbd %xmm4,%ymm6", options(att_syntax));
	core::arch::asm!("vpmovdb %zmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 32 */

	core::arch::asm!("vpmovzxbq %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovqb %zmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 33 */

	core::arch::asm!("vpmovzxwd %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovdw %zmm5,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 34 */

	core::arch::asm!("vpmovzxwq %xmm4,%ymm6", options(att_syntax));
	core::arch::asm!("vpmovqw %zmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 35 */

	core::arch::asm!("vpmovzxdq %xmm4,%ymm4", options(att_syntax));
	core::arch::asm!("vpmovqd %zmm5,%ymm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 36 */

	core::arch::asm!("vpermd %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpermd %ymm4,%ymm6,%ymm2{{%k7}}", options(att_syntax));
	core::arch::asm!("vpermq %ymm4,%ymm6,%ymm2{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 38 */

	core::arch::asm!("vpminsb %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmovm2d %k5,%zmm6", options(att_syntax));
	core::arch::asm!("vpmovm2q %k5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 39 */

	core::arch::asm!("vpminsd %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpminsd %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpminsq %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpmovd2m %zmm6,%k5", options(att_syntax));
	core::arch::asm!("vpmovq2m %zmm6,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 38 3a */

	core::arch::asm!("vpminuw %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpbroadcastmw2d %k6,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 3b */

	core::arch::asm!("vpminud %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpminud %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpminuq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 3d */

	core::arch::asm!("vpmaxsd %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmaxsd %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpmaxsq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 3f */

	core::arch::asm!("vpmaxud %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmaxud %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpmaxuq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 40 */

	core::arch::asm!("vpmulld %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpmulld %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpmullq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 42 */

	core::arch::asm!("vgetexpps %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vgetexppd %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 43 */

	core::arch::asm!("vgetexpss %xmm4,%xmm5,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vgetexpsd %xmm2,%xmm3,%xmm4{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 44 */

	core::arch::asm!("vplzcntd %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vplzcntq %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 46 */

	core::arch::asm!("vpsravd %ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpsravd %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpsravq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 4c */

	core::arch::asm!("vrcp14ps %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vrcp14pd %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 4d */

	core::arch::asm!("vrcp14ss %xmm4,%xmm5,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vrcp14sd %xmm4,%xmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 4e */

	core::arch::asm!("vrsqrt14ps %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vrsqrt14pd %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 4f */

	core::arch::asm!("vrsqrt14ss %xmm4,%xmm5,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vrsqrt14sd %xmm4,%xmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 50 */

	core::arch::asm!("vpdpbusd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpdpbusd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpdpbusd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpdpbusd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 51 */

	core::arch::asm!("vpdpbusds %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpdpbusds %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpdpbusds %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpdpbusds 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 52 */

	core::arch::asm!("vdpbf16ps %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vdpbf16ps %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vdpbf16ps %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vdpbf16ps 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vpdpwssd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpdpwssd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpdpwssd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpdpwssd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vp4dpwssd (%eax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("vp4dpwssd 0x12345678(%eax,%ecx,8),%zmm0,%zmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 53 */

	core::arch::asm!("vpdpwssds %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpdpwssds %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpdpwssds %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpdpwssds 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vp4dpwssds (%eax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("vp4dpwssds 0x12345678(%eax,%ecx,8),%zmm0,%zmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 54 */

	core::arch::asm!("vpopcntb %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpopcntb %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpopcntb %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpopcntb 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	core::arch::asm!("vpopcntw %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpopcntw %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpopcntw %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpopcntw 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	/* AVX-512: Op code 0f 38 55 */

	core::arch::asm!("vpopcntd %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpopcntd %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpopcntd %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpopcntd 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	core::arch::asm!("vpopcntq %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpopcntq %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpopcntq %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpopcntq 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	/* AVX-512: Op code 0f 38 59 */

	core::arch::asm!("vpbroadcastq %xmm4,%xmm6", options(att_syntax));
	core::arch::asm!("vbroadcasti32x2 %xmm7,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 5a */

	core::arch::asm!("vbroadcasti128 (%ecx),%ymm4", options(att_syntax));
	core::arch::asm!("vbroadcasti32x4 (%ecx),%zmm6", options(att_syntax));
	core::arch::asm!("vbroadcasti64x2 (%ecx),%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 5b */

	core::arch::asm!("vbroadcasti32x8 (%ecx),%zmm6", options(att_syntax));
	core::arch::asm!("vbroadcasti64x4 (%ecx),%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 62 */

	core::arch::asm!("vpexpandb %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpexpandb %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpexpandb %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpexpandb 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	core::arch::asm!("vpexpandw %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpexpandw %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpexpandw %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpexpandw 0x12345678(%eax,%ecx,8),%zmm2", options(att_syntax));

	/* AVX-512: Op code 0f 38 63 */

	core::arch::asm!("vpcompressb %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpcompressb %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpcompressb %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpcompressb %zmm2,0x12345678(%eax,%ecx,8)", options(att_syntax));

	core::arch::asm!("vpcompressw %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vpcompressw %ymm1, %ymm2", options(att_syntax));
	core::arch::asm!("vpcompressw %zmm1, %zmm2", options(att_syntax));
	core::arch::asm!("vpcompressw %zmm2,0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* AVX-512: Op code 0f 38 64 */

	core::arch::asm!("vpblendmd %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpblendmq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 65 */

	core::arch::asm!("vblendmps %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vblendmpd %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 66 */

	core::arch::asm!("vpblendmb %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpblendmw %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 68 */

	core::arch::asm!("vp2intersectd %xmm1, %xmm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectd %ymm1, %ymm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectd %zmm1, %zmm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectd 0x12345678(%eax,%ecx,8),%zmm2,%k3", options(att_syntax));

	core::arch::asm!("vp2intersectq %xmm1, %xmm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectq %ymm1, %ymm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectq %zmm1, %zmm2, %k3", options(att_syntax));
	core::arch::asm!("vp2intersectq 0x12345678(%eax,%ecx,8),%zmm2,%k3", options(att_syntax));

	/* AVX-512: Op code 0f 38 70 */

	core::arch::asm!("vpshldvw %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshldvw %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshldvw %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshldvw 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 71 */

	core::arch::asm!("vpshldvd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshldvd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshldvd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshldvd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vpshldvq %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshldvq %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshldvq %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshldvq 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 72 */

	core::arch::asm!("vcvtne2ps2bf16 %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vcvtne2ps2bf16 %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vcvtne2ps2bf16 %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vcvtne2ps2bf16 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vcvtneps2bf16 %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("vcvtneps2bf16 %ymm1, %xmm2", options(att_syntax));
	core::arch::asm!("vcvtneps2bf16 %zmm1, %ymm2", options(att_syntax));
	core::arch::asm!("vcvtneps2bf16 0x12345678(%eax,%ecx,8),%ymm2", options(att_syntax));

	core::arch::asm!("vpshrdvw %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshrdvw %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshrdvw %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshrdvw 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 73 */

	core::arch::asm!("vpshrdvd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshrdvd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshrdvd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshrdvd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vpshrdvq %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vpshrdvq %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vpshrdvq %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vpshrdvq 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 75 */

	core::arch::asm!("vpermi2b %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpermi2w %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 76 */

	core::arch::asm!("vpermi2d %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpermi2q %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 77 */

	core::arch::asm!("vpermi2ps %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpermi2pd %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 7a */

	core::arch::asm!("vpbroadcastb %eax,%xmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 7b */

	core::arch::asm!("vpbroadcastw %eax,%xmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 7c */

	core::arch::asm!("vpbroadcastd %eax,%xmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 7d */

	core::arch::asm!("vpermt2b %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpermt2w %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 7e */

	core::arch::asm!("vpermt2d %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpermt2q %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 7f */

	core::arch::asm!("vpermt2ps %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpermt2pd %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 83 */

	core::arch::asm!("vpmultishiftqb %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 88 */

	core::arch::asm!("vexpandps (%ecx),%zmm6", options(att_syntax));
	core::arch::asm!("vexpandpd (%ecx),%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 89 */

	core::arch::asm!("vpexpandd (%ecx),%zmm6", options(att_syntax));
	core::arch::asm!("vpexpandq (%ecx),%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 8a */

	core::arch::asm!("vcompressps %zmm6,(%ecx)", options(att_syntax));
	core::arch::asm!("vcompresspd %zmm6,(%ecx)", options(att_syntax));

	/* AVX-512: Op code 0f 38 8b */

	core::arch::asm!("vpcompressd %zmm6,(%ecx)", options(att_syntax));
	core::arch::asm!("vpcompressq %zmm6,(%ecx)", options(att_syntax));

	/* AVX-512: Op code 0f 38 8d */

	core::arch::asm!("vpermb %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpermw %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 8f */

	core::arch::asm!("vpshufbitqmb %xmm1, %xmm2, %k3", options(att_syntax));
	core::arch::asm!("vpshufbitqmb %ymm1, %ymm2, %k3", options(att_syntax));
	core::arch::asm!("vpshufbitqmb %zmm1, %zmm2, %k3", options(att_syntax));
	core::arch::asm!("vpshufbitqmb 0x12345678(%eax,%ecx,8),%zmm2,%k3", options(att_syntax));

	/* AVX-512: Op code 0f 38 90 */

	core::arch::asm!("vpgatherdd %xmm2,0x02(%ebp,%xmm7,2),%xmm1", options(att_syntax));
	core::arch::asm!("vpgatherdq %xmm2,0x04(%ebp,%xmm7,2),%xmm1", options(att_syntax));
	core::arch::asm!("vpgatherdd 0x7b(%ebp,%zmm7,8),%zmm6{{%k1}}", options(att_syntax));
	core::arch::asm!("vpgatherdq 0x7b(%ebp,%ymm7,8),%zmm6{{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 91 */

	core::arch::asm!("vpgatherqd %xmm2,0x02(%ebp,%xmm7,2),%xmm1", options(att_syntax));
	core::arch::asm!("vpgatherqq %xmm2,0x02(%ebp,%xmm7,2),%xmm1", options(att_syntax));
	core::arch::asm!("vpgatherqd 0x7b(%ebp,%zmm7,8),%ymm6{{%k1}}", options(att_syntax));
	core::arch::asm!("vpgatherqq 0x7b(%ebp,%zmm7,8),%zmm6{{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 9a */

	core::arch::asm!("vfmsub132ps %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub132ps %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vfmsub132ps %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vfmsub132ps 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vfmsub132pd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub132pd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vfmsub132pd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vfmsub132pd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("v4fmaddps (%eax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("v4fmaddps 0x12345678(%eax,%ecx,8),%zmm0,%zmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 9b */

	core::arch::asm!("vfmsub132ss %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub132ss 0x12345678(%eax,%ecx,8),%xmm2,%xmm3", options(att_syntax));

	core::arch::asm!("vfmsub132sd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub132sd 0x12345678(%eax,%ecx,8),%xmm2,%xmm3", options(att_syntax));

	core::arch::asm!("v4fmaddss (%eax), %xmm0, %xmm4", options(att_syntax));
	core::arch::asm!("v4fmaddss 0x12345678(%eax,%ecx,8),%xmm0,%xmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 a0 */

	core::arch::asm!("vpscatterdd %zmm6,0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vpscatterdq %zmm6,0x7b(%ebp,%ymm7,8){{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 a1 */

	core::arch::asm!("vpscatterqd %ymm6,0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vpscatterqq %ymm6,0x7b(%ebp,%ymm7,8){{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 a2 */

	core::arch::asm!("vscatterdps %zmm6,0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterdpd %zmm6,0x7b(%ebp,%ymm7,8){{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 a3 */

	core::arch::asm!("vscatterqps %ymm6,0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterqpd %zmm6,0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 aa */

	core::arch::asm!("vfmsub213ps %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub213ps %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vfmsub213ps %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vfmsub213ps 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vfmsub213pd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub213pd %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vfmsub213pd %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vfmsub213pd 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("v4fnmaddps (%eax), %zmm0, %zmm4", options(att_syntax));
	core::arch::asm!("v4fnmaddps 0x12345678(%eax,%ecx,8),%zmm0,%zmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 ab */

	core::arch::asm!("vfmsub213ss %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub213ss 0x12345678(%eax,%ecx,8),%xmm2,%xmm3", options(att_syntax));

	core::arch::asm!("vfmsub213sd %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vfmsub213sd 0x12345678(%eax,%ecx,8),%xmm2,%xmm3", options(att_syntax));

	core::arch::asm!("v4fnmaddss (%eax), %xmm0, %xmm4", options(att_syntax));
	core::arch::asm!("v4fnmaddss 0x12345678(%eax,%ecx,8),%xmm0,%xmm4", options(att_syntax));

	/* AVX-512: Op code 0f 38 b4 */

	core::arch::asm!("vpmadd52luq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 b5 */

	core::arch::asm!("vpmadd52huq %zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 c4 */

	core::arch::asm!("vpconflictd %zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vpconflictq %zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 38 c8 */

	core::arch::asm!("vexp2ps %zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("vexp2pd %zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 38 ca */

	core::arch::asm!("vrcp28ps %zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("vrcp28pd %zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 38 cb */

	core::arch::asm!("vrcp28ss %xmm5,%xmm6,%xmm7{{%k7}}", options(att_syntax));
	core::arch::asm!("vrcp28sd %xmm5,%xmm6,%xmm7{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 cc */

	core::arch::asm!("vrsqrt28ps %zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("vrsqrt28pd %zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 38 cd */

	core::arch::asm!("vrsqrt28ss %xmm5,%xmm6,%xmm7{{%k7}}", options(att_syntax));
	core::arch::asm!("vrsqrt28sd %xmm5,%xmm6,%xmm7{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 cf */

	core::arch::asm!("gf2p8mulb %xmm1, %xmm3", options(att_syntax));
	core::arch::asm!("gf2p8mulb 0x12345678(%eax,%ecx,8),%xmm3", options(att_syntax));

	core::arch::asm!("vgf2p8mulb %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vgf2p8mulb %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vgf2p8mulb %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vgf2p8mulb 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 dc */

	core::arch::asm!("vaesenc %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vaesenc %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vaesenc %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vaesenc 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 dd */

	core::arch::asm!("vaesenclast %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vaesenclast %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vaesenclast %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vaesenclast 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 de */

	core::arch::asm!("vaesdec %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vaesdec %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vaesdec %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vaesdec 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 38 df */

	core::arch::asm!("vaesdeclast %xmm1, %xmm2, %xmm3", options(att_syntax));
	core::arch::asm!("vaesdeclast %ymm1, %ymm2, %ymm3", options(att_syntax));
	core::arch::asm!("vaesdeclast %zmm1, %zmm2, %zmm3", options(att_syntax));
	core::arch::asm!("vaesdeclast 0x12345678(%eax,%ecx,8),%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 3a 03 */

	core::arch::asm!("valignd $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("valignq $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 3a 08 */

	core::arch::asm!("vroundps $0x5,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vrndscaleps $0x12,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 3a 09 */

	core::arch::asm!("vroundpd $0x5,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vrndscalepd $0x12,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 3a 0a */

	core::arch::asm!("vroundss $0x5,%xmm4,%xmm6,%xmm2", options(att_syntax));
	core::arch::asm!("vrndscaless $0x12,%xmm4,%xmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 0b */

	core::arch::asm!("vroundsd $0x5,%xmm4,%xmm6,%xmm2", options(att_syntax));
	core::arch::asm!("vrndscalesd $0x12,%xmm4,%xmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 18 */

	core::arch::asm!("vinsertf128 $0x5,%xmm4,%ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vinsertf32x4 $0x12,%xmm4,%zmm5,%zmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vinsertf64x2 $0x12,%xmm4,%zmm5,%zmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 19 */

	core::arch::asm!("vextractf128 $0x5,%ymm4,%xmm4", options(att_syntax));
	core::arch::asm!("vextractf32x4 $0x12,%zmm5,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vextractf64x2 $0x12,%zmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 1a */

	core::arch::asm!("vinsertf32x8 $0x12,%ymm5,%zmm6,%zmm7{{%k7}}", options(att_syntax));
	core::arch::asm!("vinsertf64x4 $0x12,%ymm5,%zmm6,%zmm7{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 1b */

	core::arch::asm!("vextractf32x8 $0x12,%zmm6,%ymm7{{%k7}}", options(att_syntax));
	core::arch::asm!("vextractf64x4 $0x12,%zmm6,%ymm7{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 1e */

	core::arch::asm!("vpcmpud $0x12,%zmm6,%zmm7,%k5", options(att_syntax));
	core::arch::asm!("vpcmpuq $0x12,%zmm6,%zmm7,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 1f */

	core::arch::asm!("vpcmpd $0x12,%zmm6,%zmm7,%k5", options(att_syntax));
	core::arch::asm!("vpcmpq $0x12,%zmm6,%zmm7,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 23 */

	core::arch::asm!("vshuff32x4 $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("vshuff64x2 $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 3a 25 */

	core::arch::asm!("vpternlogd $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("vpternlogq $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 3a 26 */

	core::arch::asm!("vgetmantps $0x12,%zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("vgetmantpd $0x12,%zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 3a 27 */

	core::arch::asm!("vgetmantss $0x12,%xmm5,%xmm6,%xmm7{{%k7}}", options(att_syntax));
	core::arch::asm!("vgetmantsd $0x12,%xmm5,%xmm6,%xmm7{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 38 */

	core::arch::asm!("vinserti128 $0x5,%xmm4,%ymm4,%ymm6", options(att_syntax));
	core::arch::asm!("vinserti32x4 $0x12,%xmm4,%zmm5,%zmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vinserti64x2 $0x12,%xmm4,%zmm5,%zmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 39 */

	core::arch::asm!("vextracti128 $0x5,%ymm4,%xmm6", options(att_syntax));
	core::arch::asm!("vextracti32x4 $0x12,%zmm5,%xmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vextracti64x2 $0x12,%zmm5,%xmm6{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 3a */

	core::arch::asm!("vinserti32x8 $0x12,%ymm5,%zmm6,%zmm7{{%k7}}", options(att_syntax));
	core::arch::asm!("vinserti64x4 $0x12,%ymm5,%zmm6,%zmm7{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 3b */

	core::arch::asm!("vextracti32x8 $0x12,%zmm6,%ymm7{{%k7}}", options(att_syntax));
	core::arch::asm!("vextracti64x4 $0x12,%zmm6,%ymm7{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 3e */

	core::arch::asm!("vpcmpub $0x12,%zmm6,%zmm7,%k5", options(att_syntax));
	core::arch::asm!("vpcmpuw $0x12,%zmm6,%zmm7,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 3f */

	core::arch::asm!("vpcmpb $0x12,%zmm6,%zmm7,%k5", options(att_syntax));
	core::arch::asm!("vpcmpw $0x12,%zmm6,%zmm7,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 42 */

	core::arch::asm!("vmpsadbw $0x5,%ymm4,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vdbpsadbw $0x12,%zmm4,%zmm5,%zmm6", options(att_syntax));

	/* AVX-512: Op code 0f 3a 43 */

	core::arch::asm!("vshufi32x4 $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("vshufi64x2 $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 3a 44 */

	core::arch::asm!("vpclmulqdq $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpclmulqdq $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpclmulqdq $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 3a 50 */

	core::arch::asm!("vrangeps $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("vrangepd $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 3a 51 */

	core::arch::asm!("vrangess $0x12,%xmm5,%xmm6,%xmm7", options(att_syntax));
	core::arch::asm!("vrangesd $0x12,%xmm5,%xmm6,%xmm7", options(att_syntax));

	/* AVX-512: Op code 0f 3a 54 */

	core::arch::asm!("vfixupimmps $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("vfixupimmpd $0x12,%zmm5,%zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 3a 55 */

	core::arch::asm!("vfixupimmss $0x12,%xmm5,%xmm6,%xmm7{{%k7}}", options(att_syntax));
	core::arch::asm!("vfixupimmsd $0x12,%xmm5,%xmm6,%xmm7{{%k7}}", options(att_syntax));

	/* AVX-512: Op code 0f 3a 56 */

	core::arch::asm!("vreduceps $0x12,%zmm6,%zmm7", options(att_syntax));
	core::arch::asm!("vreducepd $0x12,%zmm6,%zmm7", options(att_syntax));

	/* AVX-512: Op code 0f 3a 57 */

	core::arch::asm!("vreducess $0x12,%xmm5,%xmm6,%xmm7", options(att_syntax));
	core::arch::asm!("vreducesd $0x12,%xmm5,%xmm6,%xmm7", options(att_syntax));

	/* AVX-512: Op code 0f 3a 66 */

	core::arch::asm!("vfpclassps $0x12,%zmm7,%k5", options(att_syntax));
	core::arch::asm!("vfpclasspd $0x12,%zmm7,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 67 */

	core::arch::asm!("vfpclassss $0x12,%xmm7,%k5", options(att_syntax));
	core::arch::asm!("vfpclasssd $0x12,%xmm7,%k5", options(att_syntax));

	/* AVX-512: Op code 0f 3a 70 */

	core::arch::asm!("vpshldw $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshldw $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshldw $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 3a 71 */

	core::arch::asm!("vpshldd $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshldd $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshldd $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vpshldq $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshldq $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshldq $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 3a 72 */

	core::arch::asm!("vpshrdw $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshrdw $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshrdw $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 3a 73 */

	core::arch::asm!("vpshrdd $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshrdd $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshrdd $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));

	core::arch::asm!("vpshrdq $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpshrdq $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vpshrdq $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 3a ce */

	core::arch::asm!("gf2p8affineqb $0x12,%xmm1,%xmm3", options(att_syntax));

	core::arch::asm!("vgf2p8affineqb $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vgf2p8affineqb $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vgf2p8affineqb $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 3a cf */

	core::arch::asm!("gf2p8affineinvqb $0x12,%xmm1,%xmm3", options(att_syntax));

	core::arch::asm!("vgf2p8affineinvqb $0x12,%xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vgf2p8affineinvqb $0x12,%ymm1,%ymm2,%ymm3", options(att_syntax));
	core::arch::asm!("vgf2p8affineinvqb $0x12,%zmm1,%zmm2,%zmm3", options(att_syntax));

	/* AVX-512: Op code 0f 72 (Grp13) */

	core::arch::asm!("vprord $0x12,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vprorq $0x12,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vprold $0x12,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vprolq $0x12,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("psrad  $0x2,%mm6", options(att_syntax));
	core::arch::asm!("vpsrad $0x5,%ymm6,%ymm2", options(att_syntax));
	core::arch::asm!("vpsrad $0x5,%zmm6,%zmm2", options(att_syntax));
	core::arch::asm!("vpsraq $0x5,%zmm6,%zmm2", options(att_syntax));

	/* AVX-512: Op code 0f 38 c6 (Grp18) */

	core::arch::asm!("vgatherpf0dps 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf0dpd 0x7b(%ebp,%ymm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf1dps 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf1dpd 0x7b(%ebp,%ymm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf0dps 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf0dpd 0x7b(%ebp,%ymm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf1dps 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf1dpd 0x7b(%ebp,%ymm7,8){{%k1}}", options(att_syntax));

	/* AVX-512: Op code 0f 38 c7 (Grp19) */

	core::arch::asm!("vgatherpf0qps 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf0qpd 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf1qps 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vgatherpf1qpd 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf0qps 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf0qpd 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf1qps 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));
	core::arch::asm!("vscatterpf1qpd 0x7b(%ebp,%zmm7,8){{%k1}}", options(att_syntax));

	/* AVX-512: Examples */

	core::arch::asm!("vaddpd %zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vaddpd %zmm4,%zmm5,%zmm6{{%k7}}", options(att_syntax));
	core::arch::asm!("vaddpd %zmm4,%zmm5,%zmm6{{%k7}}{{z}}", options(att_syntax));
	core::arch::asm!("vaddpd {{rn-sae}},%zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vaddpd {{ru-sae}},%zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vaddpd {{rd-sae}},%zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vaddpd {{rz-sae}},%zmm4,%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vaddpd (%ecx),%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vaddpd 0x123(%eax,%ecx,8),%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vaddpd (%ecx){{1to8}},%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vaddpd 0x1fc0(%edx),%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vaddpd 0x3f8(%edx){{1to8}},%zmm5,%zmm6", options(att_syntax));
	core::arch::asm!("vcmpeq_uqps 0x1fc(%edx){{1to16}},%zmm6,%k5", options(att_syntax));
	core::arch::asm!("vcmpltsd 0x123(%eax,%ecx,8),%xmm3,%k5{{%k7}}", options(att_syntax));
	core::arch::asm!("vcmplesd {{sae}},%xmm4,%xmm5,%k5{{%k7}}", options(att_syntax));
	core::arch::asm!("vgetmantss $0x5b,0x123(%eax,%ecx,8),%xmm4,%xmm5{{%k7}}", options(att_syntax));

	/* bndmk m32, bnd */

	core::arch::asm!("bndmk (%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk (%eax), %bnd3", options(att_syntax));
	core::arch::asm!("bndmk (%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk (%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk (%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12(%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmk 0x12345678(%eax,%ecx,8), %bnd0", options(att_syntax));

	/* bndcl r/m32, bnd */

	core::arch::asm!("bndcl (%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl (%eax), %bnd3", options(att_syntax));
	core::arch::asm!("bndcl (%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl (%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl (%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12(%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl 0x12345678(%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcl %eax, %bnd0", options(att_syntax));

	/* bndcu r/m32, bnd */

	core::arch::asm!("bndcu (%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu (%eax), %bnd3", options(att_syntax));
	core::arch::asm!("bndcu (%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu (%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu (%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12(%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu 0x12345678(%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcu %eax, %bnd0", options(att_syntax));

	/* bndcn r/m32, bnd */

	core::arch::asm!("bndcn (%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn (%eax), %bnd3", options(att_syntax));
	core::arch::asm!("bndcn (%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn (%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn (%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12(%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn 0x12345678(%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndcn %eax, %bnd0", options(att_syntax));

	/* bndmov m64, bnd */

	core::arch::asm!("bndmov (%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov (%eax), %bnd3", options(att_syntax));
	core::arch::asm!("bndmov (%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov (%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov (%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12(%eax,%ecx,8), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndmov 0x12345678(%eax,%ecx,8), %bnd0", options(att_syntax));

	/* bndmov bnd, m64 */

	core::arch::asm!("bndmov %bnd0, (%eax)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, (0x12345678)", options(att_syntax));
	core::arch::asm!("bndmov %bnd3, (%eax)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, (%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(,%eax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, (%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, (%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%eax)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%ebp)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%ebp,%eax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%ebp)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%ebp,%eax,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bndmov %bnd0, 0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* bndmov bnd2, bnd1 */

	core::arch::asm!("bndmov %bnd0, %bnd1", options(att_syntax));
	core::arch::asm!("bndmov %bnd1, %bnd0", options(att_syntax));

	/* bndldx mib, bnd */

	core::arch::asm!("bndldx (%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx (0x12345678), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx (%eax), %bnd3", options(att_syntax));
	core::arch::asm!("bndldx (%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx (%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12(%eax,%ecx,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(%eax), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(%ebp), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(%ecx,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(%ebp,%eax,1), %bnd0", options(att_syntax));
	core::arch::asm!("bndldx 0x12345678(%eax,%ecx,1), %bnd0", options(att_syntax));

	/* bndstx bnd, mib */

	core::arch::asm!("bndstx %bnd0, (%eax)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, (0x12345678)", options(att_syntax));
	core::arch::asm!("bndstx %bnd3, (%eax)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, (%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(,%eax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, (%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12(%eax)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12(%ebp)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12(%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12(%ebp,%eax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12(%eax,%ecx,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(%eax)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(%ebp)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(%ecx,%eax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(%ebp,%eax,1)", options(att_syntax));
	core::arch::asm!("bndstx %bnd0, 0x12345678(%eax,%ecx,1)", options(att_syntax));

	/* bnd prefix on call, ret, jmp and all jcc */

	core::arch::asm!("bnd call label1", options(att_syntax));  /* Expecting: call unconditional 0xfffffffc */
	core::arch::asm!("bnd call *(%eax)", options(att_syntax)); /* Expecting: call indirect      0 */
	core::arch::asm!("bnd ret", options(att_syntax));          /* Expecting: ret  indirect      0 */
	core::arch::asm!("bnd jmp label1", options(att_syntax));   /* Expecting: jmp  unconditional 0xfffffffc */
	core::arch::asm!("bnd jmp label1", options(att_syntax));   /* Expecting: jmp  unconditional 0xfffffffc */
	core::arch::asm!("bnd jmp *(%ecx)", options(att_syntax));  /* Expecting: jmp  indirect      0 */
	core::arch::asm!("bnd jne label1", options(att_syntax));   /* Expecting: jcc  conditional   0xfffffffc */

	/* sha1rnds4 imm8, xmm2/m128, xmm1 */

	core::arch::asm!("sha1rnds4 $0x0, %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%eax), %xmm3", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, (%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12(%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1rnds4 $0x91, 0x12345678(%eax,%ecx,8), %xmm0", options(att_syntax));

	/* sha1nexte xmm2/m128, xmm1 */

	core::arch::asm!("sha1nexte %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha1nexte (%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte (%eax), %xmm3", options(att_syntax));
	core::arch::asm!("sha1nexte (%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte (%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte (%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12(%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1nexte 0x12345678(%eax,%ecx,8), %xmm0", options(att_syntax));

	/* sha1msg1 xmm2/m128, xmm1 */

	core::arch::asm!("sha1msg1 %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha1msg1 (%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 (%eax), %xmm3", options(att_syntax));
	core::arch::asm!("sha1msg1 (%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 (%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 (%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12(%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg1 0x12345678(%eax,%ecx,8), %xmm0", options(att_syntax));

	/* sha1msg2 xmm2/m128, xmm1 */

	core::arch::asm!("sha1msg2 %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha1msg2 (%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 (%eax), %xmm3", options(att_syntax));
	core::arch::asm!("sha1msg2 (%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 (%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 (%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12(%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha1msg2 0x12345678(%eax,%ecx,8), %xmm0", options(att_syntax));

	/* sha256rnds2 <XMM0>, xmm2/m128, xmm1 */
	/* Note sha256rnds2 has an implicit operand 'xmm0' */

	core::arch::asm!("sha256rnds2 %xmm4, %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%eax), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 (0x12345678), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%eax), %xmm3", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%ecx,%eax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(,%eax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%eax,%ecx,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 (%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%eax), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%ebp), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%ecx,%eax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%ebp,%eax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%eax,%ecx,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%eax), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%ebp), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%ecx,%eax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%ebp,%eax,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%eax,%ecx,1), %xmm1", options(att_syntax));
	core::arch::asm!("sha256rnds2 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));

	/* sha256msg1 xmm2/m128, xmm1 */

	core::arch::asm!("sha256msg1 %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha256msg1 (%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 (%eax), %xmm3", options(att_syntax));
	core::arch::asm!("sha256msg1 (%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 (%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 (%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12(%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg1 0x12345678(%eax,%ecx,8), %xmm0", options(att_syntax));

	/* sha256msg2 xmm2/m128, xmm1 */

	core::arch::asm!("sha256msg2 %xmm1, %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 %xmm7, %xmm2", options(att_syntax));
	core::arch::asm!("sha256msg2 (%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 (0x12345678), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 (%eax), %xmm3", options(att_syntax));
	core::arch::asm!("sha256msg2 (%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 (%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 (%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12(%eax,%ecx,8), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%eax), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%ebp), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%ecx,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%ebp,%eax,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%eax,%ecx,1), %xmm0", options(att_syntax));
	core::arch::asm!("sha256msg2 0x12345678(%eax,%ecx,8), %xmm0", options(att_syntax));

	/* clflushopt m8 */

	core::arch::asm!("clflushopt (%eax)", options(att_syntax));
	core::arch::asm!("clflushopt (0x12345678)", options(att_syntax));
	core::arch::asm!("clflushopt 0x12345678(%eax,%ecx,8)", options(att_syntax));
	/* Also check instructions in the same group encoding as clflushopt */
	core::arch::asm!("clflush (%eax)", options(att_syntax));
	core::arch::asm!("sfence", options(att_syntax));

	/* clwb m8 */

	core::arch::asm!("clwb (%eax)", options(att_syntax));
	core::arch::asm!("clwb (0x12345678)", options(att_syntax));
	core::arch::asm!("clwb 0x12345678(%eax,%ecx,8)", options(att_syntax));
	/* Also check instructions in the same group encoding as clwb */
	core::arch::asm!("xsaveopt (%eax)", options(att_syntax));
	core::arch::asm!("mfence", options(att_syntax));

	/* cldemote m8 */

	core::arch::asm!("cldemote (%eax)", options(att_syntax));
	core::arch::asm!("cldemote (0x12345678)", options(att_syntax));
	core::arch::asm!("cldemote 0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* xsavec mem */

	core::arch::asm!("xsavec (%eax)", options(att_syntax));
	core::arch::asm!("xsavec (0x12345678)", options(att_syntax));
	core::arch::asm!("xsavec 0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* xsaves mem */

	core::arch::asm!("xsaves (%eax)", options(att_syntax));
	core::arch::asm!("xsaves (0x12345678)", options(att_syntax));
	core::arch::asm!("xsaves 0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* xrstors mem */

	core::arch::asm!("xrstors (%eax)", options(att_syntax));
	core::arch::asm!("xrstors (0x12345678)", options(att_syntax));
	core::arch::asm!("xrstors 0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* ptwrite */

	core::arch::asm!("ptwrite (%eax)", options(att_syntax));
	core::arch::asm!("ptwrite (0x12345678)", options(att_syntax));
	core::arch::asm!("ptwrite 0x12345678(%eax,%ecx,8)", options(att_syntax));

	core::arch::asm!("ptwritel (%eax)", options(att_syntax));
	core::arch::asm!("ptwritel (0x12345678)", options(att_syntax));
	core::arch::asm!("ptwritel 0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* tpause */

	core::arch::asm!("tpause %ebx", options(att_syntax));

	/* umonitor */

	core::arch::asm!("umonitor %ax", options(att_syntax));
	core::arch::asm!("umonitor %eax", options(att_syntax));

	/* umwait */

	core::arch::asm!("umwait %eax", options(att_syntax));

	/* movdiri */

	core::arch::asm!("movdiri %eax,(%ebx)", options(att_syntax));
	core::arch::asm!("movdiri %ecx,0x12345678(%eax)", options(att_syntax));

	/* movdir64b */

	core::arch::asm!("movdir64b (%eax),%ebx", options(att_syntax));
	core::arch::asm!("movdir64b 0x12345678(%eax),%ecx", options(att_syntax));
	core::arch::asm!("movdir64b (%si),%bx", options(att_syntax));
	core::arch::asm!("movdir64b 0x1234(%si),%cx", options(att_syntax));

	/* enqcmd */

	core::arch::asm!("enqcmd (%eax),%ebx", options(att_syntax));
	core::arch::asm!("enqcmd 0x12345678(%eax),%ecx", options(att_syntax));
	core::arch::asm!("enqcmd (%si),%bx", options(att_syntax));
	core::arch::asm!("enqcmd 0x1234(%si),%cx", options(att_syntax));

	/* enqcmds */

	core::arch::asm!("enqcmds (%eax),%ebx", options(att_syntax));
	core::arch::asm!("enqcmds 0x12345678(%eax),%ecx", options(att_syntax));
	core::arch::asm!("enqcmds (%si),%bx", options(att_syntax));
	core::arch::asm!("enqcmds 0x1234(%si),%cx", options(att_syntax));

	/* incsspd */

	core::arch::asm!("incsspd %eax", options(att_syntax));
	/* Also check instructions in the same group encoding as incsspd */
	core::arch::asm!("xrstor (%eax)", options(att_syntax));
	core::arch::asm!("xrstor (0x12345678)", options(att_syntax));
	core::arch::asm!("xrstor 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("lfence", options(att_syntax));

	/* rdsspd */

	core::arch::asm!("rdsspd %eax", options(att_syntax));

	/* saveprevssp */

	core::arch::asm!("saveprevssp", options(att_syntax));

	/* rstorssp */

	core::arch::asm!("rstorssp (%eax)", options(att_syntax));
	core::arch::asm!("rstorssp (0x12345678)", options(att_syntax));
	core::arch::asm!("rstorssp 0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* wrssd */

	core::arch::asm!("wrssd %ecx,(%eax)", options(att_syntax));
	core::arch::asm!("wrssd %edx,(0x12345678)", options(att_syntax));
	core::arch::asm!("wrssd %edx,0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* wrussd */

	core::arch::asm!("wrussd %ecx,(%eax)", options(att_syntax));
	core::arch::asm!("wrussd %edx,(0x12345678)", options(att_syntax));
	core::arch::asm!("wrussd %edx,0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* setssbsy */

	core::arch::asm!("setssbsy", options(att_syntax));
	/* Also check instructions in the same group encoding as setssbsy */
	core::arch::asm!("rdpkru", options(att_syntax));
	core::arch::asm!("wrpkru", options(att_syntax));

	/* clrssbsy */

	core::arch::asm!("clrssbsy (%eax)", options(att_syntax));
	core::arch::asm!("clrssbsy (0x12345678)", options(att_syntax));
	core::arch::asm!("clrssbsy 0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* endbr32/64 */

	core::arch::asm!("endbr32", options(att_syntax));
	core::arch::asm!("endbr64", options(att_syntax));

	/* call with/without notrack prefix */

	core::arch::asm!("call *%eax", options(att_syntax));				/* Expecting: call indirect 0 */
	core::arch::asm!("call *(%eax)", options(att_syntax));				/* Expecting: call indirect 0 */
	core::arch::asm!("call *(0x12345678)", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("call *0x12345678(%eax,%ecx,8)", options(att_syntax));		/* Expecting: call indirect 0 */

	core::arch::asm!("bnd call *%eax", options(att_syntax));				/* Expecting: call indirect 0 */
	core::arch::asm!("bnd call *(%eax)", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("bnd call *(0x12345678)", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("bnd call *0x12345678(%eax,%ecx,8)", options(att_syntax));	/* Expecting: call indirect 0 */

	core::arch::asm!("notrack call *%eax", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("notrack call *(%eax)", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("notrack call *(0x12345678)", options(att_syntax));		/* Expecting: call indirect 0 */
	core::arch::asm!("notrack call *0x12345678(%eax,%ecx,8)", options(att_syntax));	/* Expecting: call indirect 0 */

	core::arch::asm!("notrack bnd call *%eax", options(att_syntax));			/* Expecting: call indirect 0 */
	core::arch::asm!("notrack bnd call *(%eax)", options(att_syntax));		/* Expecting: call indirect 0 */
	core::arch::asm!("notrack bnd call *(0x12345678)", options(att_syntax));		/* Expecting: call indirect 0 */
	core::arch::asm!("notrack bnd call *0x12345678(%eax,%ecx,8)", options(att_syntax)); /* Expecting: call indirect 0 */

	/* jmp with/without notrack prefix */

	core::arch::asm!("jmp *%eax", options(att_syntax));				/* Expecting: jmp indirect 0 */
	core::arch::asm!("jmp *(%eax)", options(att_syntax));				/* Expecting: jmp indirect 0 */
	core::arch::asm!("jmp *(0x12345678)", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("jmp *0x12345678(%eax,%ecx,8)", options(att_syntax));		/* Expecting: jmp indirect 0 */

	core::arch::asm!("bnd jmp *%eax", options(att_syntax));				/* Expecting: jmp indirect 0 */
	core::arch::asm!("bnd jmp *(%eax)", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("bnd jmp *(0x12345678)", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("bnd jmp *0x12345678(%eax,%ecx,8)", options(att_syntax));	/* Expecting: jmp indirect 0 */

	core::arch::asm!("notrack jmp *%eax", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack jmp *(%eax)", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack jmp *(0x12345678)", options(att_syntax));		/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack jmp *0x12345678(%eax,%ecx,8)", options(att_syntax));	/* Expecting: jmp indirect 0 */

	core::arch::asm!("notrack bnd jmp *%eax", options(att_syntax));			/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack bnd jmp *(%eax)", options(att_syntax));		/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack bnd jmp *(0x12345678)", options(att_syntax));		/* Expecting: jmp indirect 0 */
	core::arch::asm!("notrack bnd jmp *0x12345678(%eax,%ecx,8)", options(att_syntax)); /* Expecting: jmp indirect 0 */

	/* AVX512-FP16 */

	core::arch::asm!("vaddph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vaddph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vaddph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vaddph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vaddph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vaddph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vaddsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vaddsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, %zmm3, %zmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, 0x12345678(%eax,%ecx,8), %zmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, %xmm3, %xmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, 0x12345678(%eax,%ecx,8), %xmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, %ymm3, %ymm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpph $0x12, 0x12345678(%eax,%ecx,8), %ymm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpsh $0x12, %xmm3, %xmm2, %k5", options(att_syntax));
	core::arch::asm!("vcmpsh $0x12, 0x12345678(%eax,%ecx,8), %xmm2, %k5", options(att_syntax));
	core::arch::asm!("vcomish %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcomish 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtdq2ph %zmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtdq2ph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtdq2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtdq2ph %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtpd2ph %zmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtpd2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtpd2ph %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2dq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd %xmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2pd 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2ps 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2psx 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq %xmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2qq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2udq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq %xmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2uqq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2uw 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2w %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2w 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtph2w %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2w 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtph2w %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtph2w 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %zmm1, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %zmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %ymm1, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %xmm1, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %ymm2, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtps2ph $0x12, %xmm2, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vcvtps2phx %zmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtps2phx 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtps2phx %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtps2phx %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtqq2ph %zmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtqq2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtqq2ph %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsd2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsh2sd 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsh2si 0x12345678(%eax,%ecx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvtsh2ss 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsh2usi %xmm1, %eax", options(att_syntax));
	core::arch::asm!("vcvtsh2usi 0x12345678(%eax,%ecx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvtsi2sh %eax, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsi2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtsi2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtss2sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtss2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2dq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq %xmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2qq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq %ymm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2udq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq %xmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq %xmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2uqq 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2uw 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2w %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2w 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvttph2w %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2w 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvttph2w %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvttph2w 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvttsh2si %xmm1, %eax", options(att_syntax));
	core::arch::asm!("vcvttsh2si 0x12345678(%eax,%ecx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvttsh2usi %xmm1, %eax", options(att_syntax));
	core::arch::asm!("vcvttsh2usi 0x12345678(%eax,%ecx,8), %eax", options(att_syntax));
	core::arch::asm!("vcvtudq2ph %zmm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtudq2ph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtudq2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtudq2ph %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuqq2ph %zmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuqq2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuqq2ph %ymm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtusi2sh %eax, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtusi2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtusi2sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtuw2ph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vcvtw2ph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vdivph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vdivph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vdivph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vdivph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vdivph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vdivph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vdivsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vdivsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfcmaddcph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfcmaddcsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmaddcsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmulcph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfcmulcph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfcmulcsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfcmulcsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd132sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd132sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd213sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd213sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmadd231sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmadd231sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddcph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddcph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddcsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddcsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmaddsub231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub132sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub132sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub213sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub213sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsub231sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsub231sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmsubadd231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmulcph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmulcph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfmulcph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmulcph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmulcph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmulcph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfmulcsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfmulcsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd132sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd132sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd213sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd213sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmadd231sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmadd231sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub132ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub132sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub132sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub213ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub213sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub213sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub231ph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vfnmsub231sh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfnmsub231sh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vfpclassph $0x12, %zmm1, %k5", options(att_syntax));
	core::arch::asm!("vfpclassph $0x12, %xmm1, %k5", options(att_syntax));
	core::arch::asm!("vfpclassph $0x12, %ymm1, %k5", options(att_syntax));
	core::arch::asm!("vfpclasssh $0x12, %xmm1, %k5", options(att_syntax));
	core::arch::asm!("vfpclasssh $0x12, 0x12345678(%eax,%ecx,8), %k5", options(att_syntax));
	core::arch::asm!("vgetexpph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vgetexpph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vgetexpph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetexpph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vgetexpph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vgetexpph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vgetexpsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetexpsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vgetmantph $0x12, 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vgetmantsh $0x12, %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vgetmantsh $0x12, 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmaxph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vmaxph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vmaxph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmaxph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmaxph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vmaxph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vmaxsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmaxsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vminph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vminph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vminph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vminph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vminph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vminph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vminsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vminsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmovsh %xmm1, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vmovsh 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vmovsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmovw %xmm1, %eax", options(att_syntax));
	core::arch::asm!("vmovw %xmm1, 0x12345678(%eax,%ecx,8)", options(att_syntax));
	core::arch::asm!("vmovw %eax, %xmm1", options(att_syntax));
	core::arch::asm!("vmovw 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vmulph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vmulph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vmulph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmulph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmulph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vmulph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vmulsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vmulsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrcpph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vrcpph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vrcpph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrcpph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vrcpph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vrcpph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vrcpsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrcpsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vreduceph $0x12, 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vreducesh $0x12, %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vreducesh $0x12, 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vrndscaleph $0x12, 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vrndscalesh $0x12, %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrndscalesh $0x12, 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vrsqrtph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vrsqrtph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vrsqrtsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vrsqrtsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vscalefph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vscalefph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vscalefph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vscalefph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vscalefph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vscalefph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vscalefsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vscalefsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsqrtph %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vsqrtph 0x12345678(%eax,%ecx,8), %zmm1", options(att_syntax));
	core::arch::asm!("vsqrtph %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsqrtph 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));
	core::arch::asm!("vsqrtph %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vsqrtph 0x12345678(%eax,%ecx,8), %ymm1", options(att_syntax));
	core::arch::asm!("vsqrtsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsqrtsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsubph %zmm3, %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vsubph 0x12345678(%eax,%ecx,8), %zmm2, %zmm1", options(att_syntax));
	core::arch::asm!("vsubph %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsubph 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsubph %ymm3, %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vsubph 0x12345678(%eax,%ecx,8), %ymm2, %ymm1", options(att_syntax));
	core::arch::asm!("vsubsh %xmm3, %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vsubsh 0x12345678(%eax,%ecx,8), %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vucomish %xmm2, %xmm1", options(att_syntax));
	core::arch::asm!("vucomish 0x12345678(%eax,%ecx,8), %xmm1", options(att_syntax));

}

	/* Key Locker */

	core::arch::asm!("	loadiwkey %xmm1, %xmm2", options(att_syntax));
	core::arch::asm!("	encodekey128 %eax, %edx", options(att_syntax));
	core::arch::asm!("	encodekey256 %eax, %edx", options(att_syntax));
	core::arch::asm!("	aesenc128kl 0x77(%edx), %xmm3", options(att_syntax));
	core::arch::asm!("	aesenc256kl 0x77(%edx), %xmm3", options(att_syntax));
	core::arch::asm!("	aesdec128kl 0x77(%edx), %xmm3", options(att_syntax));
	core::arch::asm!("	aesdec256kl 0x77(%edx), %xmm3", options(att_syntax));
	core::arch::asm!("	aesencwide128kl	0x77(%edx)", options(att_syntax));
	core::arch::asm!("	aesencwide256kl	0x77(%edx)", options(att_syntax));
	core::arch::asm!("	aesdecwide128kl	0x77(%edx)", options(att_syntax));
	core::arch::asm!("	aesdecwide256kl	0x77(%edx)", options(att_syntax));

	/* Remote Atomic Operations */

	core::arch::asm!("aadd %ecx,(%eax)", options(att_syntax));
	core::arch::asm!("aadd %edx,(0x12345678)", options(att_syntax));
	core::arch::asm!("aadd %edx,0x12345678(%eax,%ecx,8)", options(att_syntax));

	core::arch::asm!("aand %ecx,(%eax)", options(att_syntax));
	core::arch::asm!("aand %edx,(0x12345678)", options(att_syntax));
	core::arch::asm!("aand %edx,0x12345678(%eax,%ecx,8)", options(att_syntax));

	core::arch::asm!("aor %ecx,(%eax)", options(att_syntax));
	core::arch::asm!("aor %edx,(0x12345678)", options(att_syntax));
	core::arch::asm!("aor %edx,0x12345678(%eax,%ecx,8)", options(att_syntax));

	core::arch::asm!("axor %ecx,(%eax)", options(att_syntax));
	core::arch::asm!("axor %edx,(0x12345678)", options(att_syntax));
	core::arch::asm!("axor %edx,0x12345678(%eax,%ecx,8)", options(att_syntax));

	/* AVX NE Convert */

	core::arch::asm!("vbcstnebf162ps (%ecx),%xmm6", options(att_syntax));
	core::arch::asm!("vbcstnesh2ps (%ecx),%xmm6", options(att_syntax));
	core::arch::asm!("vcvtneebf162ps (%ecx),%xmm6", options(att_syntax));
	core::arch::asm!("vcvtneeph2ps (%ecx),%xmm6", options(att_syntax));
	core::arch::asm!("vcvtneobf162ps (%ecx),%xmm6", options(att_syntax));
	core::arch::asm!("vcvtneoph2ps (%ecx),%xmm6", options(att_syntax));
	core::arch::asm!("vcvtneps2bf16 %xmm1,%xmm6", options(att_syntax));

	/* AVX VNNI INT16 */

	core::arch::asm!("vpdpbssd %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpbssds %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpbsud %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpbsuds %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpbuud %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpbuuds %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpwsud %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpwsuds %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpwusd %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpwusds %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpwuud %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpdpwuuds %xmm1,%xmm2,%xmm3", options(att_syntax));

	/* AVX IFMA */

	core::arch::asm!("vpmadd52huq %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vpmadd52luq %xmm1,%xmm2,%xmm3", options(att_syntax));

	/* AVX SHA512 */

	core::arch::asm!("vsha512msg1 %xmm1,%ymm2", options(att_syntax));
	core::arch::asm!("vsha512msg2 %ymm1,%ymm2", options(att_syntax));
	core::arch::asm!("vsha512rnds2 %xmm1,%ymm2,%ymm3", options(att_syntax));

	/* AVX SM3 */

	core::arch::asm!("vsm3msg1 %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vsm3msg2 %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vsm3rnds2 $0xa1,%xmm1,%xmm2,%xmm3", options(att_syntax));

	/* AVX SM4 */

	core::arch::asm!("vsm4key4 %xmm1,%xmm2,%xmm3", options(att_syntax));
	core::arch::asm!("vsm4rnds4 %xmm1,%xmm2,%xmm3", options(att_syntax));

	/* Pre-fetch */

	core::arch::asm!("prefetch (%eax)", options(att_syntax));
	core::arch::asm!("prefetcht0 (%eax)", options(att_syntax));
	core::arch::asm!("prefetcht1 (%eax)", options(att_syntax));
	core::arch::asm!("prefetcht2 (%eax)", options(att_syntax));
	core::arch::asm!("prefetchnta (%eax)", options(att_syntax));

	/* Non-serializing write MSR */

	core::arch::asm!("wrmsrns", options(att_syntax));

	/* Prediction history reset */

	core::arch::asm!("hreset $0", options(att_syntax));

	/* Serialize instruction execution */

	core::arch::asm!("serialize", options(att_syntax));

	/* TSX suspend load address tracking */

	core::arch::asm!("xresldtrk", options(att_syntax));
	core::arch::asm!("xsusldtrk", options(att_syntax));

	/* SGX */

	core::arch::asm!("encls", options(att_syntax));
	core::arch::asm!("enclu", options(att_syntax));
	core::arch::asm!("enclv", options(att_syntax));

	/* pconfig */

	core::arch::asm!("pconfig", options(att_syntax));

	/* wbnoinvd */

	core::arch::asm!("wbnoinvd", options(att_syntax));

	/* Following line is a marker for the awk script - do not change */
	core::arch::asm!("rdtsc", options(att_syntax)); /* Stop here */

	return 0;
	0
}
