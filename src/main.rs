mod models;
mod schema;
mod data;
mod setup;
use std::env;
use dotenvy::dotenv;
use mysql::*;

fn handle_setup(args: Vec<String>) {
 if args.len() > 1 && args[2] == "setup" {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env file. Example:  DATABASE_URL=mysql://uname:pass@localhost:3306/dbname");
        let opts = Opts::from_url(&db_url).unwrap();
        let pool = Pool::new(opts);
        let conn = pool.expect("could not establish mysql connection pool").get_conn().unwrap();
        let res = setup::setup_msql(conn).err();
        if res.is_some() {
            panic!("Error setting up database: {}", res.unwrap());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    handle_setup(args);
    data::analytics(&models::SurveyResponse::new(0,0,0,0,0,0,0),Some(1000));
    data::result_distribution(1000, Some(1000));
}
