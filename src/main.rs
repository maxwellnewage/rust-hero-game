mod errors;
mod api;
mod models;

#[tokio::main]
async fn main() {
    match api::get_all_players().await {
        Ok(players) => {
            for player in players {
                println!("---- {} (@{}) ----", player.name, player.owner.username);
                println!("HP: {}", player.hp);
                println!("Dinero: {}", player.money);
                println!("Puntaje: {}", player.score);
            }
        }
        Err(e) => {
            eprintln!("Error al obtener los jugadores: {}", e);
        }
    }
}
