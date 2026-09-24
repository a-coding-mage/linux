/* Full-provider differential fixture. Native C headers and native Rust types. */
#include <linux/rbtree_augmented.h>

void *memset(void *dest, int value, __kernel_size_t size)
{
    volatile unsigned char *p = dest;
    while (size--) *p++ = value;
    return dest;
}
void *memcpy(void *dest, const void *source, __kernel_size_t size)
{
    unsigned char *d = dest;
    const unsigned char *s = source;
    while (size--) *d++ = *s++;
    return dest;
}
int bcmp(const void *a, const void *b, __kernel_size_t size);
int bcmp(const void *a, const void *b, __kernel_size_t size)
{
    const unsigned char *p = a, *q = b;
    while (size--) if (*p++ != *q++) return 1;
    return 0;
}
/* Test-only fatal panic endpoint; a reached Rust panic fails the process. */
void rust_begin_unwind(void *info) __attribute__((noreturn));
void rust_begin_unwind(void *info) { __builtin_trap(); }

#define N 127
#define SLOTS (2*N)
struct item { struct rb_node_linked link; int id, key; unsigned int sum; };
static struct item nodes[2][SLOTS];
static struct rb_root_linked trees[2];
static int active[N];
static unsigned int rng = 0x9328175;
static unsigned long digest;
static unsigned int calls[2], trace[2][8192];
static int side;
static unsigned int random_word(void) { rng ^= rng << 13; rng ^= rng >> 17; rng ^= rng << 5; return rng; }
static struct item *entry(const struct rb_node *node) { return (struct item *)node; }
static int id(const struct rb_node *node) { return node ? entry(node)->id + 1 : 0; }
static unsigned int sum(const struct rb_node *node) { return node ? entry(node)->sum : 0; }
static void record(int kind, struct rb_node *a, struct rb_node *b)
{
    unsigned int *n = &calls[side];
    if (*n + 7 >= 8192) __builtin_trap();
    trace[side][(*n)++] = kind;
    trace[side][(*n)++] = id(a);
    trace[side][(*n)++] = id(b);
    trace[side][(*n)++] = a ? id(a->rb_left) : 0;
    trace[side][(*n)++] = a ? id(a->rb_right) : 0;
    trace[side][(*n)++] = a ? (a->__rb_parent_color & 3) : 0;
    trace[side][(*n)++] = b ? (b->__rb_parent_color & 3) : 0;
}
static void propagate(struct rb_node *node, struct rb_node *stop)
{
    record(1, node, stop);
    while (node != stop) {
        unsigned int value = 1 + sum(node->rb_left) + sum(node->rb_right);
        if (entry(node)->sum == value) break;
        entry(node)->sum = value;
        node = rb_parent(node);
    }
}
static void copy_cb(struct rb_node *old, struct rb_node *new)
{
    record(2, old, new); entry(new)->sum = entry(old)->sum;
}
static void rotate(struct rb_node *old, struct rb_node *new)
{
    record(3, old, new); entry(new)->sum = entry(old)->sum;
    entry(old)->sum = 1 + sum(old->rb_left) + sum(old->rb_right);
}
static const struct rb_augment_callbacks callbacks = { propagate, copy_cb, rotate };
static bool less(struct rb_node *a, const struct rb_node *b) { return entry(a)->key < entry(b)->key; }

extern void oracle_rb_insert_color(struct rb_node *, struct rb_root *);
extern void oracle___rb_insert_augmented(struct rb_node *, struct rb_root *, void (*)(struct rb_node *, struct rb_node *));
extern void oracle___rb_erase_color(struct rb_node *, struct rb_root *, void (*)(struct rb_node *, struct rb_node *));
extern void oracle_rb_erase(struct rb_node *, struct rb_root *);
extern bool oracle_rb_erase_linked(struct rb_node_linked *, struct rb_root_linked *);
extern struct rb_node *oracle_rb_next(const struct rb_node *);
extern struct rb_node *oracle_rb_prev(const struct rb_node *);
extern struct rb_node *oracle_rb_first_postorder(const struct rb_root *);
extern struct rb_node *oracle_rb_next_postorder(const struct rb_node *);
extern void oracle_rb_replace_node(struct rb_node *, struct rb_node *, struct rb_root *);
extern void oracle_rb_replace_node_rcu(struct rb_node *, struct rb_node *, struct rb_root *);
extern struct rb_node *oracle_edge(const struct rb_root *, bool);
extern struct rb_node *rust_edge(const struct rb_root *, bool);
extern bool oracle_add_linked(struct rb_node_linked *, struct rb_root_linked *, bool (*)(struct rb_node *, const struct rb_node *));
extern bool rust_add_linked(struct rb_node_linked *, struct rb_root_linked *, bool (*)(struct rb_node *, const struct rb_node *));
extern void oracle_erase_aug(struct rb_node *, struct rb_root *, const struct rb_augment_callbacks *);
extern void rust_erase_aug(struct rb_node *, struct rb_root *, const struct rb_augment_callbacks *);
extern void rust_link_rcu(struct rb_node *, struct rb_node *, struct rb_node **);
extern struct rb_node *rust_tags(struct rb_node *, struct rb_node *);
extern int rust_inline_checks(void);

#define CHECK(x) do { if (!(x)) return __LINE__; } while (0)

/* Same key/max layout as the Rust header fixture, with original C callbacks. */
struct max_item { struct rb_node node; int key, max; };
static int max_value(struct max_item *item) { return item->key; }
RB_DECLARE_CALLBACKS_MAX(static, named_callbacks, struct max_item, node, int, max, max_value);
extern const struct rb_augment_callbacks rust_generated_callbacks;
static int named_callback_tests(void)
{
    for (int side = 0; side < 2; side++) {
        const struct rb_augment_callbacks *cb = side ? &rust_generated_callbacks : &named_callbacks;
        struct max_item n[3] = {0};
        for (int i = 0; i < 3; i++) { n[i].key = i + 1; n[i].max = i + 1; }
        n[1].node.rb_left = &n[0].node; n[1].node.rb_right = &n[2].node;
        n[0].node.__rb_parent_color = (unsigned long)&n[1].node;
        n[2].node.__rb_parent_color = (unsigned long)&n[1].node;
        n[1].max = 3;
        /* vmalloc-style update followed by direct named-helper propagation. */
        n[2].key = 9;
        if (side) cb->propagate(&n[2].node, NULL);
        else named_callbacks_propagate(&n[2].node, NULL);
        CHECK(n[2].max == 9 && n[1].max == 9);
        n[2].key = 3; cb->propagate(&n[2].node, NULL);
        CHECK(n[2].max == 3 && n[1].max == 3);
        cb->copy(&n[1].node, &n[0].node); CHECK(n[0].max == 3);
        n[0].max = 1;
        n[1].node.rb_right = NULL; n[2].node.rb_left = &n[1].node;
        cb->rotate(&n[1].node, &n[2].node);
        CHECK(n[1].max == 2 && n[2].max == 3);
    }
    return 0;
}
static int verify_node(struct rb_node *n, struct rb_node *parent, int lo, int hi, bool aug)
{
    int l, r;
    if (!n) return 1;
    if (rb_parent(n) != parent || entry(n)->key <= lo || entry(n)->key >= hi) return -1;
    if (rb_is_red(n) && ((n->rb_left && rb_is_red(n->rb_left)) || (n->rb_right && rb_is_red(n->rb_right)))) return -1;
    if (aug && entry(n)->sum != 1 + sum(n->rb_left) + sum(n->rb_right)) return -1;
    l = verify_node(n->rb_left, n, lo, entry(n)->key, aug);
    r = verify_node(n->rb_right, n, entry(n)->key, hi, aug);
    if (l < 0 || l != r) return -1;
    return l + rb_is_black(n);
}
static int verify(bool aug, bool linked)
{
    struct rb_node *a, *b;
    int i, count = 0, prev = -1;
    CHECK(id(trees[0].rb_root.rb_node) == id(trees[1].rb_root.rb_node));
    CHECK(calls[0] == calls[1]);
    for (i = 0; i < calls[0]; i++) CHECK(trace[0][i] == trace[1][i]);
    for (i = 0; i < N; i++) {
        int slot = active[i];
        if (slot < 0) continue;
        a = &nodes[0][slot].link.node; b = &nodes[1][slot].link.node;
        CHECK(id(a->rb_left) == id(b->rb_left)); CHECK(id(a->rb_right) == id(b->rb_right));
        CHECK(id(rb_parent(a)) == id(rb_parent(b)));
        CHECK((a->__rb_parent_color & 3) == (b->__rb_parent_color & 3));
        if (aug) CHECK(entry(a)->sum == entry(b)->sum);
        if (linked) {
            CHECK(id((struct rb_node *)nodes[0][slot].link.prev) == id((struct rb_node *)nodes[1][slot].link.prev));
            CHECK(id((struct rb_node *)nodes[0][slot].link.next) == id((struct rb_node *)nodes[1][slot].link.next));
        }
    }
    CHECK(!trees[1].rb_root.rb_node || rb_is_black(trees[1].rb_root.rb_node));
    CHECK(verify_node(trees[1].rb_root.rb_node, NULL, -1, N, aug) > 0);
    a = oracle_edge(&trees[0].rb_root, false); b = rust_edge(&trees[1].rb_root, false);
    while (a || b) {
        CHECK(id(a) == id(b)); CHECK(entry(b)->key > prev); prev = entry(b)->key;
        a = oracle_rb_next(a); b = rb_next(b); CHECK(++count <= N);
    }
    a = oracle_edge(&trees[0].rb_root, true); b = rust_edge(&trees[1].rb_root, true); i = 0;
    while (a || b) { CHECK(id(a) == id(b)); a = oracle_rb_prev(a); b = rb_prev(b); CHECK(++i <= N); }
    CHECK(i == count);
    a = oracle_rb_first_postorder(&trees[0].rb_root); b = rb_first_postorder(&trees[1].rb_root); i = 0;
    while (a || b) { CHECK(id(a) == id(b)); digest = digest * 33 + id(b); a = oracle_rb_next_postorder(a); b = rb_next_postorder(b); CHECK(++i <= N); }
    CHECK(i == count);
    if (linked) {
        CHECK(id((struct rb_node *)trees[0].rb_leftmost) == id((struct rb_node *)trees[1].rb_leftmost));
        struct rb_node_linked *p = trees[1].rb_leftmost, *last = NULL; i = 0;
        while (p) { CHECK(p->prev == last); last = p; p = p->next; CHECK(++i <= N); }
        CHECK(i == count);
    }
    return 0;
}

static void insert(int s, int slot, bool aug, bool linked)
{
    struct rb_node *node = &nodes[s][slot].link.node, *parent = NULL;
    struct rb_root *root = &trees[s].rb_root;
    struct rb_node **link = &root->rb_node;
    side = s;
    if (linked) {
        if (s) rust_add_linked(&nodes[s][slot].link, &trees[s], less);
        else oracle_add_linked(&nodes[s][slot].link, &trees[s], less);
        return;
    }
    while (*link) { parent = *link; if (aug) entry(parent)->sum++; link = less(node, parent) ? &parent->rb_left : &parent->rb_right; }
    entry(node)->sum = 1;
    if (s) rust_link_rcu(node, parent, link); else rb_link_node_rcu(node, parent, link);
    if (aug) {
        if (s) __rb_insert_augmented(node, root, rotate); else oracle___rb_insert_augmented(node, root, rotate);
    } else {
        if (s) rb_insert_color(node, root); else oracle_rb_insert_color(node, root);
    }
}

static int exercise(bool aug, bool linked)
{
    int i, step, err;
    for (i = 0; i < N; i++) active[i] = -1;
    for (side = 0; side < 2; side++) {
        trees[side] = (struct rb_root_linked)RB_ROOT_LINKED;
        for (i = 0; i < SLOTS; i++) {
            nodes[side][i] = (struct item){ .id=i, .key=i%N };
            RB_CLEAR_LINKED_NODE(&nodes[side][i].link);
        }
    }
    for (step = 0; step < 50000; step++) {
        int key = random_word() % N, slot = active[key];
        unsigned int action = random_word();
        calls[0] = calls[1] = 0;
        if (slot < 0) {
            slot = key; active[key] = slot;
            for (i = 0; i < 2; i++) insert(i, slot, aug, linked);
        } else if ((action & 3) == 0 && !linked) {
            int new_slot = slot < N ? slot + N : slot - N;
            for (i = 0; i < 2; i++) {
                struct rb_node *old = &nodes[i][slot].link.node, *new = &nodes[i][new_slot].link.node;
                nodes[i][new_slot].sum = nodes[i][slot].sum;
                if (action & 4) { if (i) rb_replace_node_rcu(old, new, &trees[i].rb_root); else oracle_rb_replace_node_rcu(old, new, &trees[i].rb_root); }
                else { if (i) rb_replace_node(old, new, &trees[i].rb_root); else oracle_rb_replace_node(old, new, &trees[i].rb_root); }
            }
            active[key] = new_slot;
        } else {
            for (i = 0; i < 2; i++) {
                struct rb_node *node = &nodes[i][slot].link.node;
                side = i;
                if (linked) {
                    bool more = i ? rb_erase_linked(&nodes[i][slot].link, &trees[i]) : oracle_rb_erase_linked(&nodes[i][slot].link, &trees[i]);
                    CHECK(more == !!trees[i].rb_leftmost);
                    CHECK(RB_EMPTY_LINKED_NODE(&nodes[i][slot].link));
                    CHECK(!nodes[i][slot].link.prev && !nodes[i][slot].link.next);
                } else if (aug) {
                    if (i) rust_erase_aug(node, &trees[i].rb_root, &callbacks); else oracle_erase_aug(node, &trees[i].rb_root, &callbacks);
                } else { if (i) rb_erase(node, &trees[i].rb_root); else oracle_rb_erase(node, &trees[i].rb_root); }
            }
            active[key] = -1;
        }
        err = verify(aug, linked); if (err) return err;
    }
    return 0;
}

#include "domain_tail.c"

int test_main(void);
int test_main(void)
{
    int err;
    struct rb_node n = {0}, p = {0}, q = {0};
    struct rb_root r = RB_ROOT;
    err = rust_inline_checks(); if (err) return err;
    err = named_callback_tests(); if (err) return err;
    err = domain_tests(); if (err) return err;
    n.__rb_parent_color = (unsigned long)&p | 3;
    CHECK(rust_tags(&n, &q) == &p);
    CHECK(n.__rb_parent_color == ((unsigned long)&q | 1));
    RB_CLEAR_NODE(&n);
    CHECK(!rb_next(&n) && !rb_prev(&n));
    CHECK(!rb_next_postorder(NULL) && !rb_first_postorder(&r));
    {
        struct rb_node_linked lone = {0};
        struct rb_root_linked linked = RB_ROOT_LINKED;
        CHECK(rust_add_linked(&lone, &linked, NULL));
        CHECK(linked.rb_leftmost == &lone);
        CHECK(!rb_erase_linked(&lone, &linked));
        CHECK(!linked.rb_root.rb_node && !linked.rb_leftmost);
        CHECK(RB_EMPTY_LINKED_NODE(&lone) && !lone.prev && !lone.next);
    }
    /* Legal null callback on an empty-tree insertion: no callback is reached. */
    rb_link_node(&n, NULL, &r.rb_node);
    __rb_insert_augmented(&n, &r, NULL);
    CHECK(rb_is_black(&n));
    /* No-rotation deletion permits a null callback too. */
    {
        struct rb_node g = {0}, s = {0}, q2 = {0};
        r.rb_node = &g;
        g.rb_left = &n; g.rb_right = &q2; g.__rb_parent_color = RB_BLACK;
        n.rb_left = NULL; n.rb_right = &s; n.__rb_parent_color = (unsigned long)&g;
        s.__rb_parent_color = (unsigned long)&n | RB_BLACK;
        q2.__rb_parent_color = (unsigned long)&g | RB_BLACK;
        __rb_erase_color(&n, &r, NULL);
        CHECK(rb_is_black(&n) && rb_is_red(&s));
    }
    err = exercise(false, false); if (err) return err;
    err = exercise(true, false); if (err) return err;
    err = exercise(false, true); if (err) return err;
    return 0;
}

#if defined(__x86_64__)
asm(".globl _start\n_start:\n xor %rbp,%rbp\n call test_main\n mov %eax,%edi\n mov $60,%eax\n syscall\n ud2\n");
#elif defined(__i386__)
asm(".globl _start\n_start:\n call test_main\n mov %eax,%ebx\n mov $1,%eax\n int $0x80\n ud2\n");
#endif
