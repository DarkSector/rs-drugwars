struct Player {
    name: String,
    health: i8,
    money: i64,
    debt: i64,
}


impl Player {
    fn new_player(player_name: String) -> Player {
        Player {
            name: player_name,
            health: 100,
            money: 1000,
            debt: 2000,
        }
    }
}
