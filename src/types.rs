#![allow(dead_code)]

use std::fmt::Display;

use colored::Colorize;

pub struct Project {
    primary: Task,
}

impl Project {
    pub fn new(name: String, description: String) -> Self {
        Project {
            primary: Task {
                name: name,
                description: description,
                complete: false,
                blocking: false,
                variant: TaskType::All,
                children: vec![],
            },
        }
    }
}

// Types of task.
#[derive(Debug)]
pub enum TaskType {
    /// All of the task's children must be completed for the task itself to be completed.
    /// This is suitable for most use cases.
    All,
    /// Any of the task's children must be completed for the task itself to be completed.
    /// This is useful for if there are multiple ways to complete the task and only one needs to succeed.
    Any,
}

impl Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::All => "All",
            Self::Any => "Any",
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum TaskStatus {
    ///This task will no longer influence the completability of parent tasks.
    Complete,
    /// This task is able to be done whenever.
    Ready,
    ///This task prevents its parent tasks from being completed, and cannot be done due to pending children.
    Incomplete,
    ///This task is impossible to do at this time, due to external factors.
    Blocking,
}

pub struct Task {
    name: String,
    description: String,
    blocking: bool,
    complete: bool,
    variant: TaskType,
    children: Vec<Task>,
}

impl Task {
    pub fn status(&self) -> TaskStatus {
        if self.complete {
            TaskStatus::Complete
        } else if self.blocking {
            TaskStatus::Blocking
        } else if self.ready() {
            TaskStatus::Ready
        // If we are not completed, not blocking, and not able to be completed, we are Incomplete.
        } else {
            TaskStatus::Incomplete
        }
    }
    pub fn display_short(&self) -> String {
        match self.status() {
            TaskStatus::Complete => format!("[X] {}", self.name.green()),
            TaskStatus::Incomplete => format!("[ ] {}", self.name.white()),
            TaskStatus::Blocking => format!("[!] {}", self.name.red()),
            TaskStatus::Ready => format!("[*] {}", self.name.blue()),
        }
    }
    pub fn display_long(&self) -> String {
        format!(
            "{}\n\tStatus: {:?}\n\tDescription: {}\n\tVariant: {:?}\n\tChildren: {}",
            self.display_short(),
            self.status(),
            self.description,
            self.variant,
            self.children.len()
        )
    }
    /// Returns whether the task is completed.
    pub fn complete(&self) -> bool {
        self.complete
    }
    /// Returns whether the task itself is blocking.
    pub fn blocking(&self) -> bool {
        self.blocking
    }
    /// Returns whether it is possible to complete the task.
    pub fn ready(&self) -> bool {
        match self.variant {
            TaskType::All => {
                // And type tasks are completable when all their children are complete.
                self.children.iter().all(|child| child.complete())
            }
            TaskType::Any => {
                // Any type tasks are completable when any of their children are complete.
                self.children.iter().any(|child| child.complete())
            }
        }
    }
    /// Get all the leaf tasks (tasks with no dependencies) that are blocking.
    pub fn get_blocking(&self) -> Vec<&Task> {
        let mut results: Vec<&Task> = vec![];
        // If the task itself has no children, it's worth considering if it is blocking.
        if self.children.is_empty() {
            if self.blocking() {
                results.push(self);
            }
        // If the task has children, it cannot be completed until all blocking children are completed.
        } else {
            self.children
                .iter()
                .for_each(|child| results.append(&mut child.get_blocking()));
        }
        results
    }
    /// Get all the leaf tasks that are completable.
    pub fn get_completable(&self) -> Vec<&Task> {
        let mut results: Vec<&Task> = vec![];
        if self.children.is_empty() {
            results.push(self);
        } else {
            self.children
                .iter()
                .for_each(|child| results.append(&mut child.get_completable()));
        }
        results
    }
    pub fn add_child(&mut self, address: &[usize], new_child: Task) -> Option<()> {
        if let Some(child_index) = address.first() {
            if let Some(child) = self.children.get_mut(*child_index) {
                child.add_child(&address[1..], new_child)
            } else {
                None
            }
        } else {
            self.children.push(new_child);
            Some(())
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_short())
    }
}
