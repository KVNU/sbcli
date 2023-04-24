use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use confy::ConfyError;
use serde::{Deserialize, Serialize};

use crate::tasks::{files::make_task_path, models::Task};

use super::{APP_NAME, DIRECTORY_DIR_NAME, META_FILE_NAME};

/// `TaskDirectory` maintains the task-related data for the CLI app.
/// It contains the tasks, the order in which they should be solved and/or displayed, their workspace paths, and the solved tasks.
///
/// This struct is designed to be part of the `Meta` struct, providing a clean separation of concerns
/// between the task-specific data and other application state.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDirectory {
    path: PathBuf,
    tasks: Vec<Task>,
    solved_tasks_ids: Vec<usize>,
    /// <task_id, order_by>
    order: HashMap<usize, usize>,
    /// <task_id, workspace_path>
    directory: HashMap<usize, PathBuf>,
}

impl Default for TaskDirectory {
    fn default() -> Self {
        // on windows, the default exercises directory is %USERPROFILE%\sbcli\<DIR_NAME>
        // on linux, the default exercises directory is $HOME/sbcli/
        let path = dirs::home_dir()
            .map(|mut p| {
                p.push(APP_NAME);
                p.push(DIRECTORY_DIR_NAME);
                p
            })
            .unwrap_or_else(|| PathBuf::from(APP_NAME));

        Self {
            path,
            tasks: Vec::new(),
            solved_tasks_ids: Vec::new(),
            order: HashMap::new(),
            directory: HashMap::new(),
        }
    }
}

impl TaskDirectory {
    pub fn new(tasks: &[Task]) -> Self {
        let d = Self::default();

        let tasks = tasks.to_owned();
        let order = Self::get_order(&tasks);
        let directory = tasks
            .iter()
            .map(|task| {
                let (dir_path, _) = make_task_path(task, &d.path).unwrap();
                (task.taskid, dir_path)
            })
            .collect::<HashMap<usize, PathBuf>>();

        Self {
            path: d.path,
            tasks,
            order,
            directory,
            ..d
        }
    }

    fn get_order(tasks: &[Task]) -> HashMap<usize, usize> {
        tasks
            .iter()
            .enumerate()
            .map(|(index, task)| (task.taskid, index))
            .collect()
    }

    /// Returns the ID of the next task to be solved
    /// If all tasks have been solved, returns 0
    fn get_next_task(&self) -> usize {
        let highest_solved_order = self
            .solved_tasks_ids
            .iter()
            .filter_map(|task_id| self.order.get(task_id))
            .max();

        if let Some(highest_solved_order) = highest_solved_order {
            let next_task = self
                .order
                .iter()
                .filter(|(_, order)| **order > *highest_solved_order)
                .min_by_key(|(_, order)| **order)
                .map(|(task_id, _)| *task_id);

            if let Some(next_task) = next_task {
                return next_task;
            }
        }

        0
    }
}

/// `Meta` contains information about the current state of the task directory and overall progress.
/// It includes the total number of tasks, the number of solved tasks, the ID of the next task to be solved,
/// and the base directory for the tasks.
///
/// The `Meta` struct also contains a `TaskDirectory`, which is responsible for maintaining task-specific data.
/// The `Meta` struct provides methods for loading, saving, and updating the state.
#[derive(Debug, Default, Serialize, Deserialize)]

pub struct Meta {
    pub total_tasks: usize,
    pub solved_tasks: usize,
    pub next_task_id: usize,
    task_directory: TaskDirectory,
}

impl Meta {
    pub fn new(tasks: &[Task]) -> Self {
        let task_directory = TaskDirectory::new(tasks);

        Self {
            total_tasks: tasks.len(),
            task_directory,
            ..Default::default()
        }
    }

    pub fn task_directory(&self) -> &TaskDirectory {
        &self.task_directory
    }

    pub fn load() -> Result<Self, ConfyError> {
        confy::load::<Self>(APP_NAME, META_FILE_NAME)
    }

    pub fn save(&self) -> Result<(), ConfyError> {
        confy::store(APP_NAME, META_FILE_NAME, self)
    }

    pub fn tasks(&self) -> &[Task] {
        self.task_directory.tasks.as_ref()
    }

    pub fn directory_dir(&self) -> &Path {
        &self.task_directory.path
    }

    pub fn get_task_path(&self, task_id: usize) -> Option<&PathBuf> {
        self.task_directory.directory.get(&task_id)
    }

    fn find_workspace_directory(path: &Path) -> Option<PathBuf> {
        let mut current_path = path.canonicalize().ok()?;

        loop {
            // Check if the current path's parent directory is named DIRECTORY_DIR_NAME
            // TODO: improve this check
            if let Some(parent) = current_path.parent() {
                if parent.file_name().unwrap_or_default() == DIRECTORY_DIR_NAME {
                    return Some(current_path);
                }
            }

            // Move up one level in the directory structure
            current_path = match current_path.parent() {
                Some(parent_path) => parent_path.to_path_buf(),
                None => break,
            };
        }

        None
    }

    /// TODO this is a bit hacky, but it works. Fix it tho.
    pub fn get_task_id_from_workspace(&self, task_path: &Path) -> Option<usize> {
        let task_path = task_path
            .canonicalize()
            .expect("Failed to get canonical path");

        let workspace_path =
            Self::find_workspace_directory(&task_path).expect("Failed to find workspace directory");

        self.task_directory
            .directory
            .iter()
            .find(|(_, path)| {
                path.canonicalize().expect("Failed to get canonical path") == workspace_path
            })
            .map(|(task_id, _)| *task_id)
    }

    pub fn solved_task_ids(&self) -> &Vec<usize> {
        &self.task_directory.solved_tasks_ids
    }

    /// Updates the list of solved tasks and the next task to be solved
    pub fn set_solved_tasks_ids(&mut self, solved_tasks_ids: Vec<usize>) {
        self.task_directory.solved_tasks_ids = solved_tasks_ids;
        self.update();
    }

    pub fn add_solved_task_id(&mut self, task_id: usize) {
        if self.task_directory.solved_tasks_ids.contains(&task_id) {
            return;
        }

        self.task_directory.solved_tasks_ids.push(task_id);
        self.solved_tasks += 1;

        self.update();
    }

    pub fn update(&mut self) {
        self.solved_tasks = self.task_directory.solved_tasks_ids.len();
        self.next_task_id = self.task_directory.get_next_task();
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_task() {
        let tasks = vec![
            Task {
                taskid: 1,
                order_by: 1,
                ..Default::default()
            },
            Task {
                taskid: 2,
                order_by: 2,
                ..Default::default()
            },
            Task {
                taskid: 3,
                order_by: 3,
                ..Default::default()
            },
            Task {
                taskid: 4,
                order_by: 4,
                ..Default::default()
            },
            Task {
                taskid: 5,
                order_by: 5,
                ..Default::default()
            },
            Task {
                taskid: 6,
                order_by: 6,
                ..Default::default()
            },
            Task {
                taskid: 7,
                order_by: 7,
                ..Default::default()
            },
        ];

        let mut meta = Meta::new(&tasks);

        assert_eq!(meta.total_tasks, 7);
        assert!(meta.task_directory.solved_tasks_ids.is_empty());

        meta.set_solved_tasks_ids(vec![1, 2, 3]);
        assert_eq!(meta.next_task_id, 4);

        meta.set_solved_tasks_ids(vec![1, 2, 3, 4, 6]);
        assert!(meta.next_task_id == 7 || meta.solved_tasks == 6);
    }
}
