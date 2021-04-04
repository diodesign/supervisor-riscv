# supervisor-level entry point for diosix system service applications on RV32I/RV64I platforms
#
# diosix system services running in supervisor-level capsules are
# entered here. the hypervisor treats guest OSes and system services
# the same in terms of initial environment, so a system service should
# play along as a capsule, initialize itself, and then communicate
# with the hypervisor to register itself as a service.
#
# this also means a guest OS can provide a system service, if allowed,
# meaning services aren't limited to monolithic native applications.
# 
# All values are little endian unless otherwise specified
#
# (c) Chris Williams, 2020-2021.
# See LICENSE for usage and copying.

.section .entry
.align 8

.global _start

# each thread has its own stack that's 1 << SV_THREAD_STACK_SIZE_SHIFT in size
# 18 => 256KiB stack
.equ SV_THREAD_STACK_SIZE_SHIFT, (18)

# the memory map for the system service is laid out as follows, ascending:
#
# . --- start of memory ---
# . application executable
# . --- start of heap ---
# .
# . --- end of heap ---
# . thread ID N stack
# . thread ID 1 stack
# . thread ID 0 stack
# . device tree structure
# . --- end of memory ---

# => a0 = scheduler thread ID. this ID number is guaranteed to start at 0
#         and count upwards. thread ID 0 will create the heap
#    a1 = pointer to device tree and top of available memory from which we build
#         descending per-thread stacks and initial heap areas
#    a2 = max number of threads assigned to this application service
# <= nothing to return to
_start:
# calculate top of thread stack for thread ID N where N is in a0
# stack top is in a1. store result in sp
  slli      t0, a0, SV_THREAD_STACK_SIZE_SHIFT
  sub       sp, a1, t0

# set up interrupt and exception handling
  la        t0, supervisor_irq_handler
  csrrw     x0, stvec, t0

# enable supervisor interrupts and exceptions by setting bit 1
  csrrsi    x0, sstatus, 1 << 1
# enable supervisor software interrupts by setting bit 1
  csrrsi    x0, sie, 1 << 1

  # thread 0 needs to zero the BSS */
  la        t0, clear_bss_finished
  beq       x0, a0, clear_bss

# other threads need to wait for clear_bss_finished
# to change from zero to non-zero to indicate the BSS is clear
clear_bss_wait_loop:
  amoswap.w t1, x0, (t0)
  beq       x0, t1, clear_bss_wait_loop
  j         clear_bss_loop_end

clear_bss:
  la        t1, __bss_start
  la        t2, __bss_end
  bgeu      t1, t2, clear_bss_loop_end # avoid empty or malformed bss 
clear_bss_loop:
  sd        x0, (t1)
  addi      t1, t1, 8
  bltu      t1, t2, clear_bss_loop
clear_bss_loop_end:
  li        t1, 1        # set clear_bss_finished to 1 now we're done
  amoswap.w x0, t1, (t0) # t0 = clear_bss_finished

# every core doesn't need to do this but the entry conditions require it
# TODO: relax sventry entry conditions for non-boot thread?
# redefine a1, a2 as start and end of heap memory space
# using a2 as the max CPU count and a1 as the top of the stack space
  slli      t0, a2, SV_THREAD_STACK_SIZE_SHIFT
  sub       a2, a1, t0
  la        a1, __application_end

# call sventry with:
# a0 = runtime-assigned scheduler thread ID number
# a1 = start of heap memory
# a2 = end of heap memory
  la        t0, sventry
  jalr      ra, t0

# fall through to loop rather than crash into random instructions/data
infinite_loop:
  j         infinite_loop

# variables
.align 4
clear_bss_finished:
.word 0