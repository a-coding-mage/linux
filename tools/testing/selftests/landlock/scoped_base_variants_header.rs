/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Landlock scoped_domains test variant definition.
 *
 * This file defines a fixture variant "scoped_domains" that has all
 * permutations of parent/child process being in separate or shared
 * Landlock domain, or not being in a Landlock domain at all.
 *
 * Scoped access tests can include this file to avoid repeating these
 * combinations.
 *
 * Copyright © 2017-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2019-2020 ANSSI
 * Copyright © 2024 Tahera Fahimi <fahimitahera@gmail.com>
 */

/* C source uses FIXTURE_VARIANT(scoped_domains). */
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct scoped_domains {
	pub domain_both: bool,
	pub domain_parent: bool,
	pub domain_child: bool,
}

/*
 *        No domain
 *
 *   P1-.               P1 -> P2 : allow
 *       \              P2 -> P1 : allow
 *        'P2
 */
/* C source uses FIXTURE_VARIANT_ADD(scoped_domains, without_domain). */
pub const without_domain: scoped_domains = scoped_domains {
	domain_both: false,
	domain_parent: false,
	domain_child: false,
};

/*
 *        Child domain
 *
 *   P1--.              P1 -> P2 : allow
 *        \             P2 -> P1 : deny
 *        .'-----.
 *        |  P2  |
 *        '------'
 */
/* C source uses FIXTURE_VARIANT_ADD(scoped_domains, child_domain). */
pub const child_domain: scoped_domains = scoped_domains {
	domain_both: false,
	domain_parent: false,
	domain_child: true,
};

/*
 *        Parent domain
 * .------.
 * |  P1  --.           P1 -> P2 : deny
 * '------'  \          P2 -> P1 : allow
 *            '
 *            P2
 */
/* C source uses FIXTURE_VARIANT_ADD(scoped_domains, parent_domain). */
pub const parent_domain: scoped_domains = scoped_domains {
	domain_both: false,
	domain_parent: true,
	domain_child: false,
};

/*
 *        Parent + child domain (siblings)
 * .------.
 * |  P1  ---.          P1 -> P2 : deny
 * '------'   \         P2 -> P1 : deny
 *         .---'--.
 *         |  P2  |
 *         '------'
 */
/* C source uses FIXTURE_VARIANT_ADD(scoped_domains, sibling_domain). */
pub const sibling_domain: scoped_domains = scoped_domains {
	domain_both: false,
	domain_parent: true,
	domain_child: true,
};

/*
 *         Same domain (inherited)
 * .-------------.
 * | P1----.     |      P1 -> P2 : allow
 * |        \    |      P2 -> P1 : allow
 * |         '   |
 * |         P2  |
 * '-------------'
 */
/* C source uses FIXTURE_VARIANT_ADD(scoped_domains, inherited_domain). */
pub const inherited_domain: scoped_domains = scoped_domains {
	domain_both: true,
	domain_parent: false,
	domain_child: false,
};

/*
 *         Inherited + child domain
 * .-----------------.
 * |  P1----.        |  P1 -> P2 : allow
 * |         \       |  P2 -> P1 : deny
 * |        .-'----. |
 * |        |  P2  | |
 * |        '------' |
 * '-----------------'
 */
/* C source uses FIXTURE_VARIANT_ADD(scoped_domains, nested_domain). */
pub const nested_domain: scoped_domains = scoped_domains {
	domain_both: true,
	domain_parent: false,
	domain_child: true,
};

/*
 *         Inherited + parent domain
 * .-----------------.
 * |.------.         |  P1 -> P2 : deny
 * ||  P1  ----.     |  P2 -> P1 : allow
 * |'------'    \    |
 * |             '   |
 * |             P2  |
 * '-----------------'
 */
/* C source uses FIXTURE_VARIANT_ADD(scoped_domains, nested_and_parent_domain). */
pub const nested_and_parent_domain: scoped_domains = scoped_domains {
	domain_both: true,
	domain_parent: true,
	domain_child: false,
};

/*
 *         Inherited + parent and child domain (siblings)
 * .-----------------.
 * | .------.        |  P1 -> P2 : deny
 * | |  P1  .        |  P2 -> P1 : deny
 * | '------'\       |
 * |          \      |
 * |        .--'---. |
 * |        |  P2  | |
 * |        '------' |
 * '-----------------'
 */
/* C source uses FIXTURE_VARIANT_ADD(scoped_domains, forked_domains). */
pub const forked_domains: scoped_domains = scoped_domains {
	domain_both: true,
	domain_parent: true,
	domain_child: true,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
