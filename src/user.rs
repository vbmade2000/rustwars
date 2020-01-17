pub mod user {

    pub fn print_mac() {
        println!("This is from mod")
    }

    #[derive(Debug)]
    pub struct User {
        pub username: String,
        pub name: String,
        pub clan: String,
        pub leader_board_position: String,
        pub skills: String
    }
}
