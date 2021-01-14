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
# thread ID 0 gets a block of memory to initialize the heap
.equ SV_THREAD_BASE_HEAP_SIZE, (4 * 1024 * 1024)

# the memory map for the system service is laid out as follows, ascending:
#
# . < start of memory >
# . application executable
# .
# . contiguous RAM reserved for the application
# . 
# . thread ID N stack
# . thread ID 1 stack
# . thread ID 0 stack
# . initial heap block
# . device tree structure
# . < end of memory >

# => a0 = scheduler thread ID. the app can spawn as many local threads as it likes,
#         though ultimately the scheduler is going to run a maximum of N threads
#         at once on physical CPU cores. this ID number is guaranteed to start at 0
#         and count upwards. thread ID 0 will manage the heap
#    a1 = pointer to device tree and top of available memory from which we build
#         descending per-thread stacks and initial heap areas
# nothing to return to
_start:
  # set up stack pointer by first skipping over the initial heap block
  la        t0, SV_THREAD_BASE_HEAP_SIZE
  sub       t1, a1, t0
  # calculate top of thread stack for thread ID N where N is in a0
  # and store in sp
  slli      t0, a0, SV_THREAD_STACK_SIZE_SHIFT
  sub       sp, t1, t0

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
.if ptrwidth == 32
  sw        x0, (t1)
  addi      t1, t1, 4
.else
  sd        x0, (t1)
  addi      t1, t1, 8
.endif
  bltu      t1, t2, clear_bss_loop
clear_bss_loop_end:
  li        t1, 1        # set clear_bss_finished to 1 now we're done
  amoswap.w x0, t1, (t0) # t0 = clear_bss_finished

  # call sventry with:
  # a0 = runtime-assigned scheduler thread ID number
  # a1 = pointer to start of devicetree / end of heap structure
  # a2 = big-endian length of the devicetree
  # a3 = little-endian length of the heap structure (mixing of endianness is ew)
  lw        a2, 4(a1)       # 32-bit size of tree stored from byte 4 in tree blob
  la        a3, SV_THREAD_BASE_HEAP_SIZE
  la        t0, sventry
  jalr      ra, t0, 0

# fall through to loop rather than crash into random instructions/data
infinite_loop:
  j         infinite_loop

# variables
.align 4
clear_bss_finished:
.word 0