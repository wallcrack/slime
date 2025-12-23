use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json::{self};
use std::{fs, path::Path};
mod common;
use common::*;
mod task;
use task::*;

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
    let file_path = access_archive_path("tasks.json").context("Failed to get archive path!")?;
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
    Add {
        content: String,
        duration_str: String,
    },
    List,
    Delete {
        id: usize,
    },
    Done,
    Focus {
        id: usize,
    },
    Focusing,
    Unfocus,
    ListDone,
}

fn load_tasks() -> Result<TaskList> {
    let file_path = access_archive_path("tasks.json")?;
    if !file_path.exists() {
        return Ok(TaskList::new());
    }
    let content = fs::read(&file_path).context("Failed to read file!")?;
    if content.is_empty() {
        return Ok(TaskList::new());
    }
    let task_list: TaskList =
        serde_json::from_slice(&content).context("Failed to deserialize tasks!")?;
    Ok(task_list)
}

fn save_tasks(task_list: &TaskList) -> Result<()> {
    let json = serde_json::to_string_pretty(task_list)?;
    let archive_path = access_archive_path("tasks.json")?;
    fs::write(&archive_path, json).context("Failed to write file!")?;
    Ok(())
}

fn add_task(content: String, duration_str: String) -> Result<()> {
    let mut task_list = load_tasks().context("Failed to load tasks")?;
    task_list.add(content, duration_str)?;
    save_tasks(&task_list).context("Failed to save tasks!")?;
    Ok(())
}

fn delete_task(index: usize) -> Result<()> {
    let mut task_list = load_tasks().context("Failed to load tasks")?;
    task_list.delete(index - 1)?;
    save_tasks(&task_list).context("Failed to save tasks!")?;
    Ok(())
}
/*
fn swap_tasks(id1: usize, id2: usize) -> Result<()> {
    println!("Command swap has been blocked");
    return Ok(());
    let mut task_list = load_tasks().context("Failed to load tasks")?;
    task_list.swap(id1, id2)?;
    save_tasks(&task_list).context("Failed to save tasks!")?;
    Ok(())
}

fn clear_tasks() -> Result<()> {
    let empty_list = Vec::<Task>::new();
    save_tasks(&empty_list).context("Failed to save tasks!")?;
    Ok(())
}
*/

fn mark_task_done() -> Result<()> {
    let mut tasks_list = load_tasks().context("Failed to load tasks")?;
    let mut done_list = load_done_tasks().context("Failed to load done tasks")?;

    if !tasks_list.is_focusing() {
        println!("No task is focused!");
        return Ok(());
    }

    let mut done_task = tasks_list.pop_focused_task()?;
    done_task.done();
    done_list.add(done_task);
    save_tasks(&tasks_list).context("Failed to save tasks!")?;
    save_done_tasks(&done_list).context("Failed to save done tasks!")?;
    Ok(())
}

fn display_tasks() -> Result<()> {
    let task_list = load_tasks().context("Failed to load tasks")?;
    task_list.display();
    Ok(())
}

fn save_done_tasks(done_list: &DoneList) -> Result<()> {
    let json = serde_json::to_string_pretty(done_list)?;
    let archive_path = access_archive_path("done_tasks.json")?;
    fs::write(&archive_path, json).context("Failed to write file!")?;
    Ok(())
}

fn load_done_tasks() -> Result<DoneList> {
    let file_path = access_archive_path("done_tasks.json")?;
    if !file_path.exists() {
        return Ok(DoneList::new());
    }
    let content = fs::read(&file_path).context("Failed to read file!")?;
    if content.is_empty() {
        return Ok(DoneList::new());
    }
    let done_list: DoneList =
        serde_json::from_slice(&content).context("Failed to deserialize tasks!")?;
    Ok(done_list)
}

fn display_done_tasks() -> Result<()> {
    let done_list = load_done_tasks().context("Failed to load tasks")?;
    done_list.display();
    Ok(())
}

fn focus_task(id: usize) -> Result<()> {
    let mut task_list = load_tasks().context("Failed to load tasks")?;
    task_list.focus(id - 1)?;
    save_tasks(&task_list).context("Failed to save tasks")?;
    Ok(())
}
fn display_focusing() -> Result<()> {
    let mut task_list = load_tasks().context("Failed to load tasks")?;
    // 应该改为输出前刷新。
    task_list.display_focusing();
    Ok(())
}

fn unfocus_task() -> Result<()> {
    let mut task_list = load_tasks().context("Failed to load tasks")?;
    task_list.unfocus()?;
    save_tasks(&task_list).context("Failed to save tasks")?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    first_run().context("Failed to initialize slime!")?;
    match args.command {
        Command::Add {
            content,
            duration_str,
        } => add_task(content, duration_str)?,
        Command::List => display_tasks()?,
        Command::Delete { id } => delete_task(id)?,
        Command::Done => mark_task_done()?,
        Command::ListDone => display_done_tasks()?,
        Command::Focus { id } => focus_task(id)?,
        Command::Focusing => display_focusing()?,
        Command::Unfocus => unfocus_task()?,
    }
    Ok(())
}
