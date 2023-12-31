pub mod default;
pub mod utils;
pub mod api_client;
pub mod fb;


pub mod db_connection;
pub mod db;
pub mod models;
pub mod providers;
pub mod providers_ref;
pub mod endpoint;
pub mod app;
pub mod start;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
