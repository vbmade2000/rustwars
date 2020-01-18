pub mod user {

    #[derive(Debug)] // Required to print struct
    pub struct OverallRank {
        pub rank: i32,
        pub name: String,
        pub color: String,
        pub score: i32,
    }

    pub struct LanguangeRank {
        pub language: String,
        pub rank: i32,
        pub name: String,
        pub color: String,
        pub score: i32,
    }

    #[derive(Debug)] // Required to print struct
    pub struct Ranks {
        pub overall_rank: OverallRank,
    }

    #[derive(Debug)] // Required to print struct
    pub struct CodeChallanges {
        total_authore: u32,
        total_completed: u32,
    }

    #[derive(Debug)] // Required to print struct
    pub struct User {
        pub username: String,
        pub name: String,
        pub clan: String,
        pub leader_board_position: String,
        pub skills: String,
        pub code_challanges: CodeChallanges,
    }
    pub fn print_mac() {
        println!("This is from mod")
    }
}
