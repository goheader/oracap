use std::fs;
use std::str::FromStr;
use oracle::{Connection, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SqlType {
    ALL,
    UserAndDBCap,
    OnlyDBCap,
    ArchAndDBCap,
}

impl FromStr for SqlType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(SqlType::ALL),
            "user_and_db_cap" => Ok(SqlType::UserAndDBCap),
            "only_db_cap" => Ok(SqlType::OnlyDBCap),
            "arch_and_db_cap" => Ok(SqlType::ArchAndDBCap),
            _ => Err(()),
        }
    }
}

const LISTENER_PORT: i16 = 1521;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub owner: Owner,
    pub databases: Vec<Databases>,
    pub sql: SQLs,
}


pub fn new() -> Result<Config, Error> {
    let config_file = fs::read_to_string("config.toml").expect("open file failed");
    let config: Config = toml::from_str(&config_file).expect("parse file failed");
    Ok(config)
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Owner {
    pub name: String,
    pub password: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Databases {
    pub host: String,
    pub sid: String,
    pub sql_type: SqlType,
}

impl Databases {
    pub fn get_conn(&self, owner: &Owner) -> Result<Connection, Error> {
        let conn_url = format!("{}:{}/{}", self.host, LISTENER_PORT, self.sid);
        let conn = Connection::connect(&owner.name, &owner.password, conn_url)?;
        Ok(conn)
    }


    pub fn exec_db_sql(&self, sql_s: &SQLs, conn: &Connection) {
        let db_row = conn.query_row(&sql_s.db_cap, &[]).expect("get data failed");
        let db_cap: f64 = db_row.get("total_gb").expect("can't get any data");

        println!("{:?}\t\t{:?}", self.sid, db_cap);
    }

    pub fn exec_user_sql(&self, sql_s: &SQLs, conn: &Connection) {
        let user_rows = conn.query(&sql_s.user_cap, &[]).expect("get data failed");
        for user_result in user_rows {
            let row = user_result.expect("no data found");
            let username = row.get(0).expect("no data get");
            let user_cap = row.get(1).expect("no data");
            println!("{:?}\t\t{:?}", username, user_cap)
        }
    }

    pub fn exec_arch_sql(&self, sql_s: &SQLs, conn: &Connection) {
        let arch = conn.query_row(&sql_s.arch_avg_cap, &[]).unwrap();
        let arch_avg = arch.get("arch_avg_cap").unwrap();
        println!("arch_avg: {:?}\t\t{:?}", self.sid, arch_avg);
    }

    pub fn exec_all_sql(&self, sql_s: &SQLs, conn: &Connection) {
        self.exec_db_sql(sql_s, conn);
        self.exec_user_sql(sql_s, conn);
        self.exec_arch_sql(sql_s, conn);
    }

    pub fn exec_user_db_sql(&self, sql_s: &SQLs, conn: &Connection) {
        self.exec_db_sql(sql_s, conn);
        self.exec_user_sql(sql_s, conn);
    }

    pub fn exec_arch_db_sql(&self, sql_s: &SQLs, conn: &Connection) {
        self.exec_db_sql(sql_s, conn);
        self.exec_arch_sql(sql_s, conn);
    }
}


#[derive(Deserialize, Serialize, Debug)]
pub struct SQLs {
    pub user_cap: String,
    pub db_cap: String,
    pub arch_avg_cap: String,
}















