use std::path::Path;

pub fn open_task_in_editor(task_path: &Path) -> anyhow::Result<()> {
    if open::that(task_path).is_err() {
        println!(
            "Could not open task file: {}.\nPlease open it manually in your preferred editor.",
            task_path.display()
        );
    }

    Ok(())
}


