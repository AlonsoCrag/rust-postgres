use postgres::{ Client, Error, NoTls };
// use std::num::ParseIntError;
mod actions;

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://userPostgres:userPassword123@localhost/company",
        NoTls
    )?;
    // This will panic an error of type postgres, thats because we have to use that kind of Error here
    // If we are parsing, the kind of error will be ParseIntError

    let username = String::from("Jess");
    let user_id: i32 = 11; 

    // CRUD ACTIONS
    // try out each one of them :)
    actions::create::Select(&mut client);
    // actions::create::Insert(&mut client, username, user_id);
    // actions::create::Drop(&mut client, username, user_id);
    // actions::create::Update(&mut client, username, user_id);

    Ok(())
}
