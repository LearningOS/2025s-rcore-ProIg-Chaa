//! Types related to task management

use super::TaskContext;
pub const MAX_SYSCALL_NUM: usize = 500;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The syscall times of a task
    pub syscall_times: [usize; MAX_SYSCALL_NUM],
    /// The start time of a task
    pub start_time:usize,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
