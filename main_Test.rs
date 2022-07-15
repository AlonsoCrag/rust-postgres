use postgres::{ Client, Error, NoTls };
use std::fs::File;

fn Create() -> Result<i32, Error> {
    Ok(99)
}

fn Request() -> Result<(), Error> {
    let data = Create()?;
    print!("Result in request from create {}", &data);
    Ok(())
}

// QUestion operator is used to unwrap() functions that return Result<T, E>

fn main() -> Result<(), Error> {
    let data = Create()?;
    print!("{}", &data);
    

    let mut conn: Option<String> = Some(String::from(""));
    conn = Some(String::from("chochis pechochis"));

    let res = &conn.unwrap(); // unwrap moves the owner 

    // println!("{:?}", &conn);
    println!("{:?}", res.to_owned() + &String::from("jejejej"));

    Ok(())
    
}
