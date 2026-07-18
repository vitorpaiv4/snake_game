use macroquad::prelude::*;


#[macroquad::main("Snake Game")]
async fn main() {
    loop {
        //limpa a tela 
        clear_background(DARKGRAY);

        //Aguarda o próximo frame
        next_frame().await;
    }
}
