在 `TaskControlBlock` 中实现创建新子进程、根据 ELF 文件完成对应内容初始化的接口，并暴露给 `sys_spawn`；在 `TaskControlBlock` 中加入 `stride` 和 `priority` 以满足维护 Stride 调度算法的需要（没有考虑 `stride` 溢出的情况），并将 `TaskManager` 中的 `VecDeque` 对应替换为优先队列 `BinaryHeap`，并在取出任务时相应更新 `stride`。

# 问答题

实际情况中，p2.stride 可能溢出而导致 p2.stride 的值为 4，从而使得 p2 继续执行。

考虑归纳法。在所有进程初始化阶段，stride 的值均为 0，满足 max_stride - min_stride = 0 <= big_stride/2。当选择 min_stride 并更新 pass 时，若 min_stride + pass <= max_stride，更新后最小 min_stride' 仍满足 max_stride - min_stride' <= big_stride/2；若 min_stride + pass > max_stride，更新后 max_stride 发生变化，最小 min_stride' 满足 min_stride + pass - min_stride' <= big_stride/priority <= big_stride/2。

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.abs_diff(other.0) <= BIG_STRIDE / 2 {
            Some(self.0.cmp(&other.0))
        } else {
            Some(other.0.cmp(&self.0))
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

2. 此外，我也参考了 **以下资料**，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
