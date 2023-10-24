//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,

    /// 记录status
    pub is_running: bool,
    ///记录task 启动的时间 
    pub start_time: usize,
    ///记录system调用task的次数 
    pub syscall_cnt: [u32;MAX_SYSCALL_NUM], 
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit, //未初始化
    /// ready to run
    Ready,  //准备运行
    /// running
    Running,    //正在运行
    /// exited
    Exited, //已退出
}
