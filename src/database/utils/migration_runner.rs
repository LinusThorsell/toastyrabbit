use refinery::{Error, Runner};

pub fn runner_from_fs() -> Result<Runner, Error> {
    let migrations = refinery::load_sql_migrations("./migrations")?;
    Ok(Runner::new(&migrations))
}
