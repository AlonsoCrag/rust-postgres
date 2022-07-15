
pub mod create {

    use postgres::{ Client, Error, NoTls };
    use postgres::types::Type;

    #[derive(Debug)]
    struct User {
        username: String,
        user_id: i32
    }
    

    pub fn Select(client: &mut Client) -> Result<(), Error> {

        for row in client.query("SELECT * FROM users", &[])? {
            let user = User {
                username: row.get(0),
                user_id: row.get(1) 
            };
            println!("Record: {:?}", &user);
        }
        Ok(())

    }

    pub fn Insert(client: &mut Client, username: String, user_id: i32) -> Result<(), Error> {

        let statement = client.prepare_typed(
            "INSERT INTO users (username, user_id) VALUES ($1, $2)",
            &[Type::VARCHAR, Type::INT4],
        )?;
    
        let res = client.execute(
            &statement,
            &[&username, &user_id]
        )?;
    
        print!("Result while INSERT -> {}", &res);
        Ok(())
    }
    
    pub fn Drop(client: &mut Client, username: String, user_id: i32) -> Result<(), Error> {
    
        let statement = client.prepare_typed(
            "DELETE FROM users WHERE user_id = $1",
            &[Type::INT4],
        )?;
    
        let res = client.execute(
            &statement,
            &[&user_id]
        )?;
    
        print!("Result while INSERT -> {}", &res);
    
        Ok(())
    }

    pub fn Update(client: &mut Client, username: String, user_id: i32) -> Result<(), Error> {
    
        let statement = client.prepare_typed(
            "UPDATE users SET username = 'Bearz' WHERE user_id = $1",
            &[Type::INT4],
        )?;
    
        let res = client.execute(
            &statement,
            &[&user_id]
        )?;
    
        print!("Result while INSERT -> {}", &res);
    
        Ok(())
    }

}