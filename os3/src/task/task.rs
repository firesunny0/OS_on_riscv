/*
 * @Author: firesunny
 * @Date: 2022-11-26 20:15:49
 * @LastEditTime: 2022-11-26 20:25:58
 * @FilePath: /lab1-os3-firesunny0/os3/src/task/task.rs
 * @Description:
 */
use super::TaskContext;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
}
