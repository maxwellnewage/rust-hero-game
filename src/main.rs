mod errors;
mod api;
mod models;

#[tokio::main]
async fn main() {
    match api::get_all_players().await {
        Ok(players) => {
            for player in players {
                println!("Jugador: {:?}", player);
            }
        }
        Err(e) => {
            eprintln!("Error al obtener los jugadores: {}", e);
        }
    }
}
