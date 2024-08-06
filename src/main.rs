#[macro_use]
extern crate rocket;

use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;

use advisory_lock::{AdvisoryFileLock, FileLockMode};
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

#[derive(FromForm)]
struct JobInput<'r> {
    title: &'r str,
    company: &'r str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Job {
    id: usize,
    // //  submit_date: ,
    title: String,
    company: String,
}

impl Job {
    fn from_input(input: &JobInput, id: usize) -> Self {
        Job {
            id,
            title: input.title.to_owned(),
            company: input.company.to_owned(),
        }
    }
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            name: "Rocket with Tera"
        },
    )
}

#[get("/add")]
fn add_get() -> Template {
    Template::render(
        "add",
        context! {
            title: "Add a Rust job"
        },
    )
}

#[post("/add", data = "<input>")]
fn add_post(input: Form<JobInput<'_>>) -> Template {
    rocket::info!("title: {}", input.title);
    rocket::info!("company: {}", input.company);

    save(&input).unwrap();

    Template::render(
        "add",
        context! {
            title: "Add a Rust job"
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, add_get, add_post])
        .attach(Template::fairing())
}

fn get_db() -> PathBuf {
    let path = match env::var("DB_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => env::current_dir().unwrap().join("db"),
    };
    fs::create_dir_all(&path).unwrap();

    path
}

fn save(input: &JobInput) -> Result<(), Box<dyn Error>> {
    let db = get_db();
    let filename = db.join("my.db");

    let mut fh = fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(filename)?;
    fh.lock(FileLockMode::Exclusive)?;

    let mut buffer = [0; 1000000];

    let res = fh.read(&mut buffer)?;

    let content = String::from_utf8(buffer[0..res].to_vec())?;
    //println!("old: {content:?}");

    let mut jobs: Vec<Job> = if content.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&content).unwrap()
    };

    let job = Job::from_input(input, jobs.len() + 1);

    jobs.push(job.to_owned());

    fh.rewind()?;
    fh.set_len(0)?; // truncate

    let content = serde_json::to_string(&jobs).unwrap();
    fh.write_all(content.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests;
