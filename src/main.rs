use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use time::{Duration, OffsetDateTime};

fn access_archive_path() -> Result<PathBuf> {
    let mut file_path = env::home_dir().context("Failed to get home directory!")?;
    file_path.push("slime_archive");
    file_path.push("Task.json");
    Ok(file_path)
}

fn create_file_with_dirs(path: impl AsRef<Path>) -> std::io::Result<fs::File> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?; // 自动递归创建所有目录
        }
    }
    fs::File::create(path)
}

fn first_run() -> Result<()> {
    let file_path = access_archive_path().context("Failed to get archive path!")?;
    if !file_path.exists() {
        if let Ok(_) = create_file_with_dirs(file_path) {
            println!("File created successfully!");
        } else {
            println!("Failed to create file!");
        }
    }
    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add { content: String },
    List,
    Delete { id: usize },
    Clear,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    content: String,
    create_date: OffsetDateTime,
    time_limit: Duration,
}
impl Task {
    fn new(content: String) -> Self {
        Task {
            content,
            create_date: OffsetDateTime::now_local().unwrap(),
            time_limit: Duration::days(7),
        }
    }
    fn display(&self) {
        let now = OffsetDateTime::now_local().unwrap();
        let remaining_time = self.time_limit
            - Duration::seconds(now.unix_timestamp() - self.create_date.unix_timestamp());
        println!("{}", self.content.cyan());
        println!("  {}", self.create_date.to_string().green());
        println!("  {}", remaining_time.to_string().green());
    }
}

fn load_tasks() -> Result<Vec<Task>> {
    let file_path = access_archive_path()?;
    if !file_path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read(&file_path).context("Failed to read file!")?;
    if content.is_empty() {
        return Ok(vec![]);
    }
    let tasks: Vec<Task> =
        serde_json::from_slice(&content).context("Failed to deserialize tasks!")?;
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    let archive_path = access_archive_path()?;
    fs::write(&archive_path, json).context("Failed to write file!")?;
    Ok(())
}

fn add_task(content: String) -> Result<()> {
    let current_task = Task::new(content);
    let mut tasks = load_tasks().context("Failed to load tasks")?;
    tasks.push(current_task);
    save_tasks(&tasks).context("Failed to save tasks!")?;
    Ok(())
}

fn check_tasks() -> Result<()> {
    let tasks = load_tasks().context("Failed to load tasks")?;
    let mut index = 0;
    for task in tasks {
        index += 1;
        print!("{}:", index);
        task.display();
    }
    Ok(())
}

fn delete_task(index: usize) -> Result<()> {
    let mut tasks = load_tasks().context("Failed to load tasks")?;

    if index <= 0 || index > tasks.len() {
        return Err(anyhow!("Invalid index!"));
    }
    tasks.remove((index - 1) as usize);
    save_tasks(&tasks).context("Failed to save tasks!")?;
    Ok(())
}

fn clear_tasks() -> Result<()> {
    let empty_list = Vec::<Task>::new();
    save_tasks(&empty_list).context("Failed to save tasks!")?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    first_run().context("Failed to initialize slime!")?;
    match args.command {
        Command::Add { content } => add_task(content)?,
        Command::List => check_tasks()?,
        Command::Delete { id } => delete_task(id)?,
        Command::Clear => clear_tasks()?,
    }
    Ok(())
}
