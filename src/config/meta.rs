use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::tasks::{files::make_task_path, Task};

const META_FILE_NAME: &str = ".meta.json";
const DIRECTORY_DIR_NAME_BASE: &str = "./tasks";

/// Meta is a struct that contains information about the current state of the exercises directory
/// It is serialized to a file in the exercises directory
/// It is used to keep track of the order of the tasks, the solved tasks, and the next task to be solved
/// It is also used to keep track of the paths to the tasks
/// TODO: this is rather fragmented, and should be refactored. In particular, it would be nice to sync it tightly with the Config.
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub total_tasks: usize,
    pub solved_tasks: usize,
    /// ID of the next task to be solved
    pub next_task_id: usize,
    directory_dir: PathBuf,
    meta_path: PathBuf,
    // TODO only use getters and setters for these, since we need to regenerate the Meta struct if these change
    tasks: Vec<Task>,
    solved_tasks_ids: Vec<usize>,
    /// <task_id, order_by>
    order: HashMap<usize, usize>,
    /// <task_id, path>
    directory: HashMap<usize, PathBuf>,
}

impl Default for Meta {
    fn default() -> Self {
        // on windows, the default exercises directory is %USERPROFILE%\sbcli
        // on linux, the default exercises directory is $HOME/sbcli
        // let exercises_dir = dirs::home_dir()
        //     .map(|mut p| {
        //         p.push(APP_NAME);
        //         p
        //     })
        //     .unwrap_or_else(|| PathBuf::from(APP_NAME));
        let directory_dir = PathBuf::from(DIRECTORY_DIR_NAME_BASE);
        let meta_path = directory_dir.join(META_FILE_NAME);

        Self {
            total_tasks: 0,
            solved_tasks: 0,
            next_task_id: 0,
            directory_dir,
            meta_path,
            tasks: Vec::new(),
            solved_tasks_ids: Vec::new(),
            order: HashMap::new(),
            directory: HashMap::new(),
        }
    }
}

impl Meta {
    pub fn new(tasks: &[Task]) -> Self {
        let tasks = tasks.to_owned();
        let order = Self::get_order(&tasks);
        let directory = tasks
            .iter()
            .map(|task| {
                let path = make_task_path(task).unwrap();
                (task.taskid, path)
            })
            .collect::<HashMap<usize, PathBuf>>();

        Self {
            total_tasks: tasks.len(),
            tasks,
            order,
            directory,
            ..Default::default()
        }
    }

    pub fn directory_dir(&self) -> &PathBuf {
        &self.directory_dir
    }
    pub fn meta_path(&self) -> &PathBuf {
        &self.meta_path
    }

    pub fn get_task_path(&self, task_id: usize) -> Option<&PathBuf> {
        self.directory.get(&task_id)
    }

    pub fn get_task_id(&self, task_path: &Path) -> Option<usize> {
        self.directory
            .iter()
            .find(|(_, path)| {
                // TODO this is a bit hacky, but it works. Fix it tho.
                path.canonicalize().expect("Failed to get canonical path")
                    == task_path
                        .canonicalize()
                        .expect("Failed to get canonical path")
            })
            .map(|(task_id, _)| *task_id)
    }

    pub fn get_order(tasks: &Vec<Task>) -> HashMap<usize, usize> {
        let mut order = HashMap::new();
        for task in tasks {
            order.insert(task.taskid, task.order_by);
        }

        order
    }

    pub fn solved_task_ids(&self) -> &Vec<usize> {
        &self.solved_tasks_ids
    }

    /// Updates the list of solved tasks and the next task to be solved
    pub fn set_solved_tasks_ids(&mut self, solved_tasks_ids: Vec<usize>) {
        self.solved_tasks_ids = solved_tasks_ids;
        self.update();
    }

    pub fn add_solved_task_id(&mut self, task_id: usize) {
        self.solved_tasks_ids.push(task_id);
        self.update();
    }

    /// Initializes the meta file
    pub fn init() -> anyhow::Result<Self> {
        let meta = Self::default();
        if !meta.directory_dir.exists() {
            std::fs::create_dir_all(&meta.directory_dir)?;
        }

        meta.save()?;
        Ok(meta)
    }

    pub fn load() -> anyhow::Result<Self> {
        let path = Self::default().meta_path; // TODO this is a bit hacky, but we can't configure the path for now, so whatever
        let meta_json = std::fs::read_to_string(path)?;
        let progress: Self = serde_json::from_str(&meta_json)?;
        Ok(progress)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let progress_json = serde_json::to_string(&self)?;
        std::fs::write(&self.meta_path, progress_json)?;
        Ok(())
    }

    pub fn update(&mut self) {
        self.solved_tasks = self.solved_tasks_ids.len();
        self.next_task_id = self.get_next_task();
    }

    /// Returns the ID of the next task to be solved
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

    pub fn tasks(&self) -> &[Task] {
        self.tasks.as_ref()
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

        let mut progress = Meta::new(&tasks);

        assert_eq!(progress.total_tasks, 7);
        assert!(progress.solved_tasks_ids.is_empty());

        progress.set_solved_tasks_ids(vec![1, 2, 3]);
        assert_eq!(progress.get_next_task(), 4);

        progress.set_solved_tasks_ids(vec![1, 2, 3, 4, 6]);
        assert!(progress.next_task_id == 7 || progress.solved_tasks == 6);
    }
}
