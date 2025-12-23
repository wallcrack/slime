use anyhow::{Context, Result, anyhow};
use colored::*;
use humantime::parse_duration;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    content: String,
    create_date: OffsetDateTime,
    done_date: Option<OffsetDateTime>,
    last_active_date: Option<OffsetDateTime>,
    time_limit: Duration,
    used_time: Duration,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    is_focusing: bool,
    focused_on: usize,
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList {
            is_focusing: false,
            focused_on: 0,
            tasks: Vec::new(),
        }
    }
    pub fn is_focusing(&self) -> bool {
        return self.is_focusing;
    }
    pub fn display(&self) {
        let now = OffsetDateTime::now_local().unwrap();
        let formatter =
            time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                .unwrap();
        let mut index = 0;
        for task in &self.tasks {
            index += 1;
            let remaining_time = task.time_limit
                - Duration::seconds(now.unix_timestamp() - task.create_date.unix_timestamp());
            println!("{}:{}", index, task.content.cyan());
            println!(
                "  Created: {}",
                task.create_date
                    .format(&formatter)
                    .unwrap()
                    .to_string()
                    .green()
            );
            println!("  Remaining: {}", remaining_time.to_string().green());
        }
    }

    pub fn add(&mut self, content: String, duration_str: String) -> Result<()> {
        let time_limit = Duration::try_from(
            parse_duration(&duration_str).context("Failed to convert duration!")?,
        )
        .context("Failed to convert duration!")?;
        let current_task = Task::new(content, time_limit);
        self.tasks.push(current_task);
        Ok(())
    }

    pub fn delete(&mut self, index: usize) -> Result<Task> {
        if index >= self.tasks.len() {
            return Err(anyhow!("Invalid index!"));
        }
        if index == self.focused_on && self.is_focusing {
            self.unfocus()?;
        }
        Ok(self.tasks.remove(index))
    }

    /*
    #[warn(dead_code)]
    pub fn swap(&mut self, id1: usize, id2: usize) -> Result<()> {
        if id1 <= 0 || id1 > self.tasks.len() || id2 <= 0 || id2 > self.tasks.len() {
            return Err(anyhow!("Invalid index!"));
        }
        self.tasks.swap(id1 - 1, id2 - 1);
        Ok(())
    }
    */
    pub fn focus(&mut self, index: usize) -> Result<()> {
        if index >= self.tasks.len() {
            return Err(anyhow!("Invalid index!"));
        }
        self.is_focusing = true;
        self.focused_on = index;
        self.tasks[self.focused_on].inactivate();
        self.tasks[self.focused_on].activate();
        Ok(())
    }
    pub fn display_focusing(&mut self) {
        if !self.is_focusing {
            println!("No task focused");
            return;
        }
        let task = &mut self.tasks[self.focused_on];

        task.inactivate();
        task.activate();
        let formatter =
            time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                .unwrap();
        println!("🛠️  Focusing on: {}", task.content.cyan());
        println!(
            "🛠️  Create Date: {}",
            task.create_date
                .format(&formatter)
                .unwrap()
                .to_string()
                .green()
        );

        println!("🛠️  Used Time: {}", task.used_time.to_string().green());
    }
    pub fn unfocus(&mut self) -> Result<()> {
        self.is_focusing = false;
        self.tasks[self.focused_on].inactivate();
        Ok(())
    }

    pub fn pop_focused_task(&mut self) -> Result<Task> {
        Ok(self.delete(self.focused_on)?)
    }
}

impl Task {
    pub fn new(content: String, time_limit: Duration) -> Self {
        Task {
            content: content,
            create_date: OffsetDateTime::now_local().unwrap(),
            done_date: None,
            last_active_date: None,
            time_limit: time_limit,
            used_time: Duration::ZERO,
        }
    }
    pub fn activate(&mut self) {
        self.last_active_date = Some(OffsetDateTime::now_local().unwrap());
    }
    pub fn inactivate(&mut self) {
        let last_active_date = self.last_active_date.unwrap();
        self.used_time += OffsetDateTime::now_local().unwrap() - last_active_date;
    }
    pub fn done(&mut self) {
        self.inactivate();
        self.done_date = Some(OffsetDateTime::now_local().unwrap());
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DoneList {
    tasks: Vec<Task>,
}

impl DoneList {
    pub fn new() -> Self {
        DoneList { tasks: Vec::new() }
    }
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn display(&self) {
        let mut index = 0;
        let formatter =
            time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                .unwrap();
        for task in &self.tasks {
            index += 1;
            println!("✅{}:{}", index, task.content.cyan());
            println!(
                "  Created At: {}",
                task.create_date
                    .format(&formatter)
                    .unwrap()
                    .to_string()
                    .green()
            );
            println!(
                "  Done At: {}",
                task.done_date
                    .unwrap()
                    .format(&formatter)
                    .unwrap()
                    .to_string()
                    .green()
            );
            println!("  Used Time: {}", task.used_time.to_string().green());
        }
    }
}
