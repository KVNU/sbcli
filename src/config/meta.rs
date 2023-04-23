use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::tasks::{files::make_task_path, Task};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Meta {
    pub total_tasks: usize,
    pub solved_tasks: usize,
    /// ID of the next task to be solved
    pub next_task_id: usize,
    // TODO only use getters and setters for these, since we need to regenerate the Meta struct if these change
    tasks: Vec<Task>,
    solved_tasks_ids: Vec<usize>,
    /// <task_id, order_by>
    order: HashMap<usize, usize>,
    /// <task_id, path>
    directory: HashMap<usize, PathBuf>,
}

impl Meta {
    pub fn new(tasks: &Vec<Task>) -> Self {
        let tasks = tasks.clone();
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

    pub fn get_task_path(&self, task_id: usize) -> Option<&PathBuf> {
        self.directory.get(&task_id)
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

    pub fn load() -> anyhow::Result<Self> {
        let cfg = crate::config::Config::load()?;
        let progress_path = cfg.meta_path;
        let progress_json = std::fs::read_to_string(progress_path)?;
        let progress: Self = serde_json::from_str(&progress_json)?;
        Ok(progress)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let cfg = crate::config::Config::load()?;
        let progress_path = cfg.meta_path;
        let progress_json = serde_json::to_string(&self)?;
        std::fs::write(progress_path, progress_json)?;
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
