use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use fake::faker::lorem::en::*;
use fake::Fake;
use rand::Rng;
use serde::{self, Deserialize, Serialize};
use uuid::Uuid;

use crate::user::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub status: TaskStatus,
    pub user: User,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub color: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TaskStatus {
    Done,
    InProgress,
    NotNedeed,
    ReadyToStart,
}

impl TaskStatus {
    fn get_random_task_status() -> TaskStatus {
        // TODO: use this when stable so we dont have to
        // hard code the enum length in the `gen_range`, that way we can
        // avoid breaking functionality with the enum changes.
        //
        // https://github.com/rust-lang/rust/issues/73662
        // let enum_length = mem::variant_count::<TaskStatus>();

        match rand::thread_rng().gen_range(0..=4) {
            0 => TaskStatus::Done,
            1 => TaskStatus::InProgress,
            2 => TaskStatus::NotNedeed,
            3 => TaskStatus::ReadyToStart,
            _ => TaskStatus::ReadyToStart,
        }
    }
}

pub const PROJECT_COLORS: [&str; 10] = [
    "#61e294ff",
    "#7bcdbaff",
    "#9799caff",
    "#bd93d8ff",
    "#b47aeaff",
    "#d3d5d4ff",
    "#a2c5acff",
    "#9db5b2ff",
    "#878e99ff",
    "#7f6a93ff",
];

pub const TIME_IN_SECONDS_OPTIONS: [i32; 7] = [3600, 1800, 5400, 3450, 1600, 1954, 7200];

impl Task {
    pub fn new_random_task(user: &Option<User>) -> Task {
        let user = match user {
            Some(u) => u.clone(),
            None => User::create_random_user(),
        };

        let random_amount_of_days = rand::thread_rng().gen_range(0..=10);
        let amount_of_days = Duration::days(random_amount_of_days);
        let random_time_in_seconds =
            TIME_IN_SECONDS_OPTIONS[rand::thread_rng().gen_range(0..TIME_IN_SECONDS_OPTIONS.len())];

        let random_date = Utc::now() - amount_of_days;

        let fake_initial_date = random_date - Duration::seconds(random_time_in_seconds as i64);
        let fake_end_date = random_date + Duration::seconds(random_time_in_seconds as i64);

        Task {
            id: Uuid::new_v4(),
            name: Words(3..5).fake::<Vec<String>>().join(" "),
            status: TaskStatus::get_random_task_status(),
            // TODO: Generate a separate table of users, and then reference the id instead.
            // generate a separate json file for the users too: users_json_db.json
            user,
            started_at: fake_initial_date,
            updated_at: fake_end_date,
            finished_at: fake_end_date,
            color: rand::thread_rng()
                .gen_range(0..(PROJECT_COLORS.len() - 1))
                .to_string(),
        }
    }
}