/* Private genuine C callback execution, original tree header and live objects. */
#include <linux/rbtree_augmented.h>
typedef void (*rotate_t)(struct rb_node *,struct rb_node *);
extern void oracle___rb_insert_augmented(struct rb_node *,struct rb_root *,rotate_t);
extern void oracle___rb_erase_color(struct rb_node *,struct rb_root *,rotate_t);
extern void rust_call(bool,bool,bool,struct rb_node *,struct rb_root *,rotate_t);
extern void rust_wrong_context(void);
extern void panic_control(void);
extern void build_error_control(void);
void proof_abort(void);
void callback_first(rotate_t);
int test_main(int,char **);
static unsigned rotations,first_calls;
static struct rb_node *expected_old,*expected_new;
static void rotate(struct rb_node *old,struct rb_node *new)
{
 if(old!=expected_old || new!=expected_new) proof_abort();
 ++rotations;
}
static void wrong_rotate(const struct rb_node *old,struct rb_node *new)
{
 if(old!=expected_old || new!=expected_new) proof_abort();
 ++rotations;
}
void callback_first(rotate_t cb) { if(cb) proof_abort(); ++first_calls; }
void proof_abort(void) { asm volatile("mov $97,%%edi; mov $60,%%eax; syscall; ud2" ::: "rax","rdi","rcx","r11","memory"); __builtin_unreachable(); }

int test_main(int argc,char **argv)
{
 if(argc!=4) return 95;
 unsigned mode=argv[1][0]-'0';
 bool oracle=argv[2][0]=='c',erase=argv[3][0]=='e';
 if(mode==5) { rust_wrong_context();return first_calls==1 ? 0:30; }
 if(mode==7) { panic_control(); return 98; }
 if(mode==8) { build_error_control(); return 98; }
 if(mode>8) return 96;
 struct rb_node a={0},b={0},c={0},d={0};
 struct rb_root root=RB_ROOT;
 bool needs_rotation=mode==2||mode==3||mode==4;
 struct rb_node *node;
 if(erase) {
   /* Missing left child under a; sibling b is black. Mode0/1 recolors
    * upward without rotation; mode2/3/4 have red far child c, case4. */
   a.rb_right=&b; a.__rb_parent_color=RB_BLACK;root.rb_node=&a;
   b.__rb_parent_color=(unsigned long)&a|RB_BLACK;
   if(needs_rotation) { b.rb_right=&c;c.__rb_parent_color=(unsigned long)&b; }
   node=&a; expected_old=&a;expected_new=&b;
 } else if(needs_rotation) {
   /* Root a black, left child b red, inserted left child c red: case3. */
   root.rb_node=&a;a.__rb_parent_color=RB_BLACK;a.rb_left=&b;
   b.__rb_parent_color=(unsigned long)&a;b.rb_left=&c;
   c.__rb_parent_color=(unsigned long)&b;node=&c;
   expected_old=&a;expected_new=&b;
 } else {
   rb_link_node(&d,NULL,&root.rb_node);node=&d;
 }
 rotate_t cb=needs_rotation ? rotate:NULL;
 if(mode==4) cb=(rotate_t)wrong_rotate;
 if(mode==6) {
   rust_call(oracle,erase,true,node,&root,cb);
 } else if(mode==1||mode==3) {
   rust_call(oracle,erase,false,node,&root,cb);
 } else if(mode==0||mode==2||mode==4) {
   void (*volatile f)(struct rb_node *,struct rb_root *,rotate_t)=erase ?
      (oracle?oracle___rb_erase_color:__rb_erase_color):
      (oracle?oracle___rb_insert_augmented:__rb_insert_augmented);
   f(node,&root,cb);
 }
 if(rotations!=(unsigned)needs_rotation) return 31;
 if(needs_rotation && (root.rb_node!=&b || (erase?b.rb_left:b.rb_right)!=&a || !rb_is_black(&b))) return 32;
 return 0;
}
asm(".globl _start\n_start:\n mov (%rsp),%edi\n lea 8(%rsp),%rsi\n call test_main\n mov %eax,%edi\n mov $60,%eax\n syscall\n ud2\n");
