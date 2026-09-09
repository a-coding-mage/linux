// SPDX-License-Identifier: GPL-2.0
/*
 * Test cases for random32 functions.
 */

// <linux/prandom.h> and <kunit/test.h> provide the external definitions.

#[repr(C)]
pub struct RndState {
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
}

#[repr(C)]
pub struct Kunit {
    _private: [u8; 0],
}

extern "C" {
    pub fn prandom_warmup(state: *mut RndState);
    pub fn prandom_u32_state(state: *mut RndState) -> u32;
    pub fn __seed(x: u32, m: u32) -> u32;
}

#[derive(Copy, Clone)]
#[repr(C)]
struct PrandomTest1 {
    seed: u32,
    result: u32,
}

static TEST1: [PrandomTest1; 4] = [
    PrandomTest1 { seed: 1, result: 3484351685 },
    PrandomTest1 { seed: 2, result: 2623130059 },
    PrandomTest1 { seed: 3, result: 3125133893 },
    PrandomTest1 { seed: 4, result: 984847254 },
];

#[derive(Copy, Clone)]
#[repr(C)]
struct PrandomTest2 {
    seed: u32,
    iteration: u32,
    result: u32,
}

static TEST2: [PrandomTest2; 100] = [
    PrandomTest2 { seed: 931557656, iteration: 959, result: 2975593782 },
    PrandomTest2 { seed: 1339693295, iteration: 876, result: 3887776532 },
    PrandomTest2 { seed: 1545556285, iteration: 961, result: 1615538833 },
    PrandomTest2 { seed: 601730776, iteration: 723, result: 1776162651 },
    PrandomTest2 { seed: 1027516047, iteration: 687, result: 511983079 },
    PrandomTest2 { seed: 416526298, iteration: 700, result: 916156552 },
    PrandomTest2 { seed: 1395522032, iteration: 652, result: 2222063676 },
    PrandomTest2 { seed: 366221443, iteration: 617, result: 2992857763 },
    PrandomTest2 { seed: 1539836965, iteration: 714, result: 3783265725 },
    PrandomTest2 { seed: 556206671, iteration: 994, result: 799626459 },
    PrandomTest2 { seed: 684907218, iteration: 799, result: 367789491 },
    PrandomTest2 { seed: 2121230701, iteration: 931, result: 2115467001 },
    PrandomTest2 { seed: 1668516451, iteration: 644, result: 3620590685 },
    PrandomTest2 { seed: 768046066, iteration: 883, result: 2034077390 },
    PrandomTest2 { seed: 1989159136, iteration: 833, result: 1195767305 },
    PrandomTest2 { seed: 536585145, iteration: 996, result: 3577259204 },
    PrandomTest2 { seed: 1008129373, iteration: 642, result: 1478080776 },
    PrandomTest2 { seed: 1740775604, iteration: 939, result: 1264980372 },
    PrandomTest2 { seed: 1967883163, iteration: 508, result: 10734624 },
    PrandomTest2 { seed: 1923019697, iteration: 730, result: 3821419629 },
    PrandomTest2 { seed: 442079932, iteration: 560, result: 3440032343 },
    PrandomTest2 { seed: 1961302714, iteration: 845, result: 841962572 },
    PrandomTest2 { seed: 2030205964, iteration: 962, result: 1325144227 },
    PrandomTest2 { seed: 1160407529, iteration: 507, result: 240940858 },
    PrandomTest2 { seed: 635482502, iteration: 779, result: 4200489746 },
    PrandomTest2 { seed: 1252788931, iteration: 699, result: 867195434 },
    PrandomTest2 { seed: 1961817131, iteration: 719, result: 668237657 },
    PrandomTest2 { seed: 1071468216, iteration: 983, result: 917876630 },
    PrandomTest2 { seed: 1281848367, iteration: 932, result: 1003100039 },
    PrandomTest2 { seed: 582537119, iteration: 780, result: 1127273778 },
    PrandomTest2 { seed: 1973672777, iteration: 853, result: 1071368872 },
    PrandomTest2 { seed: 1896756996, iteration: 762, result: 1127851055 },
    PrandomTest2 { seed: 847917054, iteration: 500, result: 1717499075 },
    PrandomTest2 { seed: 1240520510, iteration: 951, result: 2849576657 },
    PrandomTest2 { seed: 1685071682, iteration: 567, result: 1961810396 },
    PrandomTest2 { seed: 1516232129, iteration: 557, result: 3173877 },
    PrandomTest2 { seed: 1208118903, iteration: 612, result: 1613145022 },
    PrandomTest2 { seed: 1817269927, iteration: 693, result: 4279122573 },
    PrandomTest2 { seed: 1510091701, iteration: 717, result: 638191229 },
    PrandomTest2 { seed: 365916850, iteration: 807, result: 600424314 },
    PrandomTest2 { seed: 399324359, iteration: 702, result: 1803598116 },
    PrandomTest2 { seed: 1318480274, iteration: 779, result: 2074237022 },
    PrandomTest2 { seed: 697758115, iteration: 840, result: 1483639402 },
    PrandomTest2 { seed: 1696507773, iteration: 840, result: 577415447 },
    PrandomTest2 { seed: 2081979121, iteration: 981, result: 3041486449 },
    PrandomTest2 { seed: 955646687, iteration: 742, result: 3846494357 },
    PrandomTest2 { seed: 1250683506, iteration: 749, result: 836419859 },
    PrandomTest2 { seed: 595003102, iteration: 534, result: 366794109 },
    PrandomTest2 { seed: 47485338, iteration: 558, result: 3521120834 },
    PrandomTest2 { seed: 619433479, iteration: 610, result: 3991783875 },
    PrandomTest2 { seed: 704096520, iteration: 518, result: 4139493852 },
    PrandomTest2 { seed: 1712224984, iteration: 606, result: 2393312003 },
    PrandomTest2 { seed: 1318233152, iteration: 922, result: 3880361134 },
    PrandomTest2 { seed: 855572992, iteration: 761, result: 1472974787 },
    PrandomTest2 { seed: 64721421, iteration: 703, result: 683860550 },
    PrandomTest2 { seed: 678931758, iteration: 840, result: 380616043 },
    PrandomTest2 { seed: 692711973, iteration: 778, result: 1382361947 },
    PrandomTest2 { seed: 677703619, iteration: 530, result: 2826914161 },
    PrandomTest2 { seed: 92393223, iteration: 586, result: 1522128471 },
    PrandomTest2 { seed: 1222592920, iteration: 743, result: 3466726667 },
    PrandomTest2 { seed: 358288986, iteration: 695, result: 1091956998 },
    PrandomTest2 { seed: 1935056945, iteration: 958, result: 514864477 },
    PrandomTest2 { seed: 735675993, iteration: 990, result: 1294239989 },
    PrandomTest2 { seed: 1560089402, iteration: 897, result: 2238551287 },
    PrandomTest2 { seed: 70616361, iteration: 829, result: 22483098 },
    PrandomTest2 { seed: 368234700, iteration: 731, result: 2913875084 },
    PrandomTest2 { seed: 20221190, iteration: 879, result: 1564152970 },
    PrandomTest2 { seed: 539444654, iteration: 682, result: 1835141259 },
    PrandomTest2 { seed: 1314987297, iteration: 840, result: 1801114136 },
    PrandomTest2 { seed: 2019295544, iteration: 645, result: 3286438930 },
    PrandomTest2 { seed: 469023838, iteration: 716, result: 1637918202 },
    PrandomTest2 { seed: 1843754496, iteration: 653, result: 2562092152 },
    PrandomTest2 { seed: 400672036, iteration: 809, result: 4264212785 },
    PrandomTest2 { seed: 404722249, iteration: 965, result: 2704116999 },
    PrandomTest2 { seed: 600702209, iteration: 758, result: 584979986 },
    PrandomTest2 { seed: 519953954, iteration: 667, result: 2574436237 },
    PrandomTest2 { seed: 1658071126, iteration: 694, result: 2214569490 },
    PrandomTest2 { seed: 420480037, iteration: 749, result: 3430010866 },
    PrandomTest2 { seed: 690103647, iteration: 969, result: 3700758083 },
    PrandomTest2 { seed: 1029424799, iteration: 937, result: 3787746841 },
    PrandomTest2 { seed: 2012608669, iteration: 506, result: 3362628973 },
    PrandomTest2 { seed: 1535432887, iteration: 998, result: 42610943 },
    PrandomTest2 { seed: 1330635533, iteration: 857, result: 3040806504 },
    PrandomTest2 { seed: 1223800550, iteration: 539, result: 3954229517 },
    PrandomTest2 { seed: 1322411537, iteration: 680, result: 3223250324 },
    PrandomTest2 { seed: 1877847898, iteration: 945, result: 2915147143 },
    PrandomTest2 { seed: 1646356099, iteration: 874, result: 965988280 },
    PrandomTest2 { seed: 805687536, iteration: 744, result: 4032277920 },
    PrandomTest2 { seed: 1948093210, iteration: 633, result: 1346597684 },
    PrandomTest2 { seed: 392609744, iteration: 783, result: 1636083295 },
    PrandomTest2 { seed: 690241304, iteration: 770, result: 1201031298 },
    PrandomTest2 { seed: 1360302965, iteration: 696, result: 1665394461 },
    PrandomTest2 { seed: 1220090946, iteration: 780, result: 1316922812 },
    PrandomTest2 { seed: 447092251, iteration: 500, result: 3438743375 },
    PrandomTest2 { seed: 1613868791, iteration: 592, result: 828546883 },
    PrandomTest2 { seed: 523430951, iteration: 548, result: 2552392304 },
    PrandomTest2 { seed: 726692899, iteration: 810, result: 1656872867 },
    PrandomTest2 { seed: 1364340021, iteration: 836, result: 3710513486 },
    PrandomTest2 { seed: 1986257729, iteration: 931, result: 935013962 },
    PrandomTest2 { seed: 407983964, iteration: 921, result: 728767059 },
];

unsafe fn prandom_state_test_seed(state: *mut RndState, seed: u32) {
    // LCG(x) = (x) * 69069U, the super-duper LCG.
    (*state).s1 = __seed(seed.wrapping_mul(69069), 2);
    (*state).s2 = __seed((*state).s1.wrapping_mul(69069), 8);
    (*state).s3 = __seed((*state).s2.wrapping_mul(69069), 16);
    (*state).s4 = __seed((*state).s3.wrapping_mul(69069), 128);
}

unsafe fn test_prandom_seed_boundary(_test: *mut Kunit) {
    let mut state = core::mem::MaybeUninit::<RndState>::uninit();
    for item in TEST1.iter() {
        prandom_state_test_seed(state.as_mut_ptr(), item.seed);
        prandom_warmup(state.as_mut_ptr());
        let result = prandom_u32_state(state.as_mut_ptr());
        assert_eq!(item.result, result);
    }
}

unsafe fn test_prandom_taus113(_test: *mut Kunit) {
    let mut state = core::mem::MaybeUninit::<RndState>::uninit();
    for item in TEST2.iter() {
        prandom_state_test_seed(state.as_mut_ptr(), item.seed);
        prandom_warmup(state.as_mut_ptr());
        for _ in 0..item.iteration - 1 {
            prandom_u32_state(state.as_mut_ptr());
        }
        let result = prandom_u32_state(state.as_mut_ptr());
        assert_eq!(item.result, result);
    }
}

// KUnit registration corresponding to KUNIT_CASE(test_prandom_seed_boundary),
// KUNIT_CASE(test_prandom_taus113), the prandom suite, and module metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
