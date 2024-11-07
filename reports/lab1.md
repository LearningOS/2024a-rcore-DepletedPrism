将任务使用的系统调用次数、第一次被调用的时刻记录在 `TaskControlBlock` 中，并在 `TaskManager` 中给出维护相关信息的接口，从而实现系统调用 `sys_task_info`，得以获取当前正在执行的任务使用的系统调用及调用次数、系统调用时刻距离任务第一次被调度时刻的时长。

# 简答题

1. 在 `os` 目录下使用 `make run CHAPTER=2` 运行可以得到的部分结果：

   ```plain
   [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
   [kernel] IllegalInstruction in application, kernel killed it.
   [kernel] IllegalInstruction in application, kernel killed it.
   ```

   输出结果中显示 RustSBI 版本为 RustSBI version 0.3.0-alpha.2（然而在后续的输出也有 `RustSBI-QEMU Version 0.2.0-alpha.2` 字样）。

   - ch2b_bad_address.rs：访问地址 0x0。
 
   - ch2b_bad_instructions.rs：在 U 态使用 `sret` 从 S 态返回 U 态。
 
   - ch2b_bad_register.rs：指令 `csrr` 从一个 CSR（sstatus）中读取内容到通用寄存器中。
 
2. 刚进入 `__resotre` 时，`a0` 保存的是当前任务 TaskContext 的地址。`__restore` 在 `__switch` 完成后执行，在内核第一次运行任务、在内核态完成当前运行任务切换时使用。

   处理了 TrapContext 中保存的寄存器 `sstatus`, `sepc`, `sp`。对于进入用户态而言，分别记录了当前 CPU 的状态（包括中断状态、权限模式等）、Trap 处理完后下一条指令的地址、用户栈地址。

   跳过 `x2`（sp, stack pointer）和 `x4`（tp, thread pointer）是因为内核栈/用户栈需要其他地方做额外处理。其中 `x2`（`sp`）通过 L48 和 L60 处理。

   该指令后，`sp` 指向用户栈、`sscratch` 指向内核栈。

   `__restore` 中状态切换发生在 `sret` 指令执行后。在执行 `sret` 后，由于恢复了在用户态时记录的 `sstatus`，`sret` 根据 `sstatus` 中 SPP 的值（SPP=0）切换至用户态。

   该指令后，`sp` 指向内核栈、`sscratch` 指向用户栈。

   在 Trap 触发后，CPU 由 U 态 切换到 S 态，并跳转到 `stvec` 所指位置。

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

2. 此外，我也参考了 **以下资料**，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
