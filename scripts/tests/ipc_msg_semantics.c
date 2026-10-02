// SPDX-License-Identifier: GPL-2.0-only
/* Supplemental syscall coverage for the Rust provider. Upstream msgque.c
 * remains the unchanged primary acceptance test; this adds blocking paths.
 */
#define _GNU_SOURCE
#include <assert.h>
#include <errno.h>
#include <limits.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/msg.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

struct message { long type; char text[32]; };
static int count;
static void passed(const char *name) { printf("ok %d - %s\n", ++count, name); fflush(stdout); }
static int queue(void) { int id = msgget(IPC_PRIVATE, IPC_CREAT | 0666); assert(id >= 0); return id; }
static void remove_queue(int id) { assert(msgctl(id, IPC_RMID, NULL) == 0); }
static void send_message(int id, long type, const char *text, size_t size)
{
 struct message m = { .type = type };
 assert(size <= sizeof(m.text));
 if (size) memcpy(m.text, text, size);
 assert(msgsnd(id, &m, size, IPC_NOWAIT) == 0);
}
static struct msqid_ds stats(int id) { struct msqid_ds ds; assert(msgctl(id, IPC_STAT, &ds) == 0); return ds; }
static void capacity(int id, unsigned long bytes)
{
 struct msqid_ds ds = stats(id);
 ds.msg_qbytes = bytes;
 assert(msgctl(id, IPC_SET, &ds) == 0);
}
static void receive_message(int id, long wanted, int flags, long type, const char *text, size_t size)
{
 struct message m;
 assert(msgrcv(id, &m, sizeof(m.text), wanted, flags | IPC_NOWAIT) == (ssize_t)size);
 assert(m.type == type);
 assert(memcmp(m.text, text, size) == 0);
}
static void selection(void)
{
 int id = queue();
 struct message m;
 send_message(id, 5, "a", 1); send_message(id, 3, "b", 1);
 send_message(id, 8, "c", 1); send_message(id, 3, "d", 1);
 receive_message(id, 3, MSG_EXCEPT, 5, "a", 1);
 receive_message(id, -LONG_MAX, 0, 3, "b", 1);
 receive_message(id, LONG_MIN, 0, 3, "d", 1);
 receive_message(id, 0, 0, 8, "c", 1);
 assert(msgrcv(id, &m, sizeof(m.text), 0, IPC_NOWAIT) == -1 && errno == ENOMSG);
 remove_queue(id); passed("FIFO, MSG_EXCEPT and negative-type minimum selection");
}
static void bounds(void)
{
 int id = queue();
 struct message m = { .type = 0 };
 assert(msgsnd(id, &m, 0, IPC_NOWAIT) == -1 && errno == EINVAL);
 m.type = 1;
 assert(msgsnd(id, &m, SIZE_MAX, IPC_NOWAIT) == -1 && errno == EINVAL);
 assert(msgrcv(id, &m, SIZE_MAX, 0, IPC_NOWAIT) == -1 && errno == EINVAL);
 assert(msgrcv(-1, &m, 0, 0, IPC_NOWAIT) == -1 && errno == EINVAL);
 assert(msgsnd(id, (void *)(uintptr_t)1, 0, IPC_NOWAIT) == -1 && errno == EFAULT);
 send_message(id, 1, "abcd", 4);
 assert(msgrcv(id, &m, 1, 0, IPC_NOWAIT) == -1 && errno == E2BIG);
 assert(stats(id).msg_qnum == 1);
 assert(msgrcv(id, &m, 1, 0, IPC_NOWAIT | MSG_NOERROR) == 1 && m.text[0] == 'a');
 assert(stats(id).msg_qnum == 0);
 send_message(id, 1, "abcd", 4);
 assert(msgrcv(id, (void *)(uintptr_t)1, 4, 0, IPC_NOWAIT) == -1 && errno == EFAULT);
 assert(stats(id).msg_qnum == 0); /* Removal precedes user-copy failure. */
 remove_queue(id); passed("validation, EFAULT, E2BIG and truncation removal");
}
static void copying(void)
{
 int id = queue(); struct message m;
 send_message(id, 9, "copy", 4);
 assert(msgrcv(id, &m, sizeof(m.text), 0, MSG_COPY) == -1 && errno == EINVAL);
 assert(msgrcv(id, &m, sizeof(m.text), 0, MSG_COPY | MSG_EXCEPT | IPC_NOWAIT) == -1 && errno == EINVAL);
 receive_message(id, 0, MSG_COPY, 9, "copy", 4);
 assert(stats(id).msg_qnum == 1);
 assert(msgrcv(id, &m, sizeof(m.text), 1, MSG_COPY | IPC_NOWAIT) == -1 && errno == ENOMSG);
 receive_message(id, 0, 0, 9, "copy", 4);
 remove_queue(id); passed("MSG_COPY validation, ordinal lookup and queue retention");
}
static void zero_length(void)
{
 int id = queue(); struct message m = { .type = 2 };
 capacity(id, 1); send_message(id, 1, "", 0);
 assert(msgsnd(id, &m, 0, IPC_NOWAIT) == -1 && errno == EAGAIN);
 assert(stats(id).msg_qnum == 1);
 receive_message(id, 0, 0, 1, "", 0);
 send_message(id, 2, "", 0); receive_message(id, 0, 0, 2, "", 0);
 remove_queue(id); passed("zero-length message count enforces capacity");
}
static void child_done(pid_t pid)
{
 int status;
 assert(waitpid(pid, &status, 0) == pid);
 assert(WIFEXITED(status) && WEXITSTATUS(status) == 0);
}
static void sleeping(pid_t pid, int ready)
{
 char byte, path[64], line[1024];
 assert(read(ready, &byte, 1) == 1); close(ready);
 snprintf(path, sizeof(path), "/proc/%d/stat", pid);
 for (int i = 0; i < 50000; i++) {
  FILE *file = fopen(path, "r"); assert(file);
  assert(fgets(line, sizeof(line), file)); fclose(file);
  char *end = strrchr(line, ')'); assert(end);
  if (end[2] == 'S') return;
  assert(end[2] != 'Z');
  struct timespec delay = { .tv_nsec = 100000 }; nanosleep(&delay, NULL);
 }
 assert(!"child failed to enter blocking syscall");
}
static void signal_handler(int signal) { (void)signal; }
static void blocking_receive(int action)
{
 int id = queue(), ready[2]; assert(pipe(ready) == 0);
 pid_t pid = fork(); assert(pid >= 0);
 if (!pid) {
  struct message m; ssize_t result;
  close(ready[0]);
  if (action == 3) assert(setuid(65534) == 0);
  struct sigaction sa = { .sa_handler = signal_handler };
  assert(sigaction(SIGUSR1, &sa, NULL) == 0);
  assert(write(ready[1], "r", 1) == 1); close(ready[1]);
  result = msgrcv(id, &m, action == 4 || action == 5 ? 1 : sizeof(m.text), 7, action == 5 ? MSG_NOERROR : 0);
  if (action == 0) assert(result == 4 && m.type == 7 && memcmp(m.text, "wake", 4) == 0);
  else if (action == 1) assert(result == -1 && errno == EIDRM);
  else if (action == 2) assert(result == -1 && errno == EINTR);
  else if (action == 3) assert(result == -1 && errno == EACCES);
  else if (action == 4) assert(result == -1 && errno == E2BIG);
  else assert(result == 1 && m.type == 7 && m.text[0] == 'w');
  _exit(0);
 }
 close(ready[1]); sleeping(pid, ready[0]);
 if (action == 1) remove_queue(id);
 else if (action == 2) assert(kill(pid, SIGUSR1) == 0);
 else if (action == 3) {
  struct msqid_ds ds = stats(id); ds.msg_perm.mode = 0600;
  assert(msgctl(id, IPC_SET, &ds) == 0);
 } else send_message(id, 7, "wake", 4);
 child_done(pid);
 if (action == 4) receive_message(id, 0, 0, 7, "wake", 4);
 if (action != 1) remove_queue(id);
 const char *names[] = { "pipelined blocked receive", "receiver IPC_RMID wakeup", "receiver signal interruption",
  "permission change expunges blocked receiver", "pipelined E2BIG leaves message queued", "pipelined MSG_NOERROR truncates" };
 passed(names[action]);
}
static void blocking_send(int action)
{
 int id = queue(), ready[2]; assert(pipe(ready) == 0);
 capacity(id, 1); send_message(id, 1, "a", 1);
 pid_t pid = fork(); assert(pid >= 0);
 if (!pid) {
  struct message m = { .type = 2, .text = "b" };
  close(ready[0]);
  struct sigaction sa = { .sa_handler = signal_handler };
  assert(sigaction(SIGUSR1, &sa, NULL) == 0);
  assert(write(ready[1], "s", 1) == 1); close(ready[1]);
  int result = msgsnd(id, &m, 1, 0);
  if (action == 1) assert(result == -1 && errno == EIDRM);
  else if (action == 2) assert(result == -1 && errno == EINTR);
  else assert(result == 0);
  _exit(0);
 }
 close(ready[1]); sleeping(pid, ready[0]);
 if (action == 1) remove_queue(id);
 else if (action == 2) assert(kill(pid, SIGUSR1) == 0);
 else if (action == 3) capacity(id, 2);
 else receive_message(id, 0, 0, 1, "a", 1);
 child_done(pid);
 if (action == 0 || action == 3) {
  if (action == 3) receive_message(id, 0, 0, 1, "a", 1);
  receive_message(id, 0, 0, 2, "b", 1);
 }
 if (action != 1) remove_queue(id);
 const char *names[] = { "receive wakes blocked sender", "sender IPC_RMID wakeup", "sender signal interruption", "capacity increase wakes blocked sender" };
 passed(names[action]);
}
int main(void)
{
 alarm(60);
 puts("TAP version 13\n1..14");
 selection(); bounds(); copying(); zero_length();
 for (int i = 0; i < 6; i++) blocking_receive(i);
 for (int i = 0; i < 4; i++) blocking_send(i);
 return count == 14 ? 0 : 1;
}
