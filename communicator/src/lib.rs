pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::*;
    mod test2 {
        use super::*;
        #[test]
        fn it_works() {
            network::connect();
            client::connect();
        }
    }
}
