pub mod bot;
pub mod handler;


pub use bot::CommonBot;
pub use bot::Bot;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
