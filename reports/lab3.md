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
