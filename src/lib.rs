mod authentication;
pub use authentication::*;

mod client;
pub use client::*;

mod message;
pub use message::*;

mod notification;
pub use notification::*;

mod settings;
pub use settings::*;

mod utils;
pub use utils::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
