use oracap::{new, SqlType};

fn main() {
   let cfg_result = new();

    match cfg_result {
        Ok(ref conf) => {
            for db in conf.databases.iter() {
                let conn = db.get_conn(&conf.owner).expect("connect to db failed");

                match db.sql_type {
                    SqlType::ALL => db.exec_all_sql(&conf.sql,&conn),
                    SqlType::OnlyDBCap => db.exec_db_sql(&conf.sql,&conn),
                    SqlType::UserAndDBCap => db.exec_user_db_sql(&conf.sql,&conn),
                    SqlType::ArchAndDBCap => db.exec_arch_db_sql(&conf.sql,&conn),
                }

            }
        }
        Err(err) => println!("{:?}", err)
    }
}
