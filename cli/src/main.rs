use crate::errors::AppError;

mod errors;
#[tokio::main]
async fn main()-> Result <(), AppError> {
    println!("Hello from an async main function!");
    Ok(())
}