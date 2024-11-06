//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::write_translated_byte,
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next,
        get_current_scheduled_time, get_current_syscall_counter, suspend_current_and_run_next,
        TaskStatus,
    },
    timer::{get_time_ms, get_time_us},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    write_translated_byte(current_user_token(), ts as *mut u8, unsafe {
        core::slice::from_raw_parts(
            &TimeVal {
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            } as *const TimeVal as *const u8,
            core::mem::size_of::<TimeVal>(),
        )
    });
    0
}

/// get information about the current running task
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let ms = get_time_ms();
    write_translated_byte(current_user_token(), ti as *mut u8, unsafe {
        core::slice::from_raw_parts(
            &TaskInfo {
                status: TaskStatus::Running,
                syscall_times: get_current_syscall_counter(),
                // since the status of current task should be `Running`, `unwarp()`
                // should not enconuter a `None` value
                time: ms - get_current_scheduled_time().unwrap(),
            } as *const TaskInfo as *const u8,
            core::mem::size_of::<TaskInfo>(),
        )
    });
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
