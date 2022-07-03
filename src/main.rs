// Handle errors using the `https://crates.io/crates/anyhow` crate
use anyhow::Result;

// Convert this main function into async function

#[async_std::main]
async fn main() -> Result<()>{
	// Read the database environment from the `.env` file
	// let env_database_url: &str = include_str!("../.env").trim();
	// Split the env url
	// let split_url: Vec<&str> = env_database_url.split("=").collect();
	// Get item with the format `database_backend://username:password@localhost/database`
	// let database_url = split_url[1];
	let database_url: &str = "postgres://postgres:password@localhost/hcount";
	
	let db = Database::connect(database_url)
		.await
		.expect("Database connection failed");
	
	let builder = db.get_database_backend();
	let schema = Schema::new(builder);
	
	let _create_table_user =  db.execute(builder.build(&schema.create_table_from_entity(User))).await;
	let _create_table_expense =  db.execute(builder.build(&schema.create_table_from_entity(Expense))).await;
	
	println!("`CREATE TABLE User` {:?}", 
		match _create_table_user {
			Ok(_) => "Operation Successful".to_owned(),
			Err(e) => format!("Unsuccessful - Error {:?}", e),
		}
	);
	println!("`CREATE TABLE Expense` {:?}", 
		match _create_table_expense {
			Ok(_) => "Operation Successful".to_owned(),
			Err(e) => format!("Unsuccessful - Error {:?}", e),
		}
	);
	Ok(())
}
