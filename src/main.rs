use macroquad::prelude::*;

// Criando estrutura
struct Ponto {
    x: f32,
    y: f32,
}

#[macroquad::main("Snake Game")]
async fn main() {
    loop {
       // Definimos a posição inicial da cabeça da cobra
       let mut cabeca = Ponto {x: 10.0, y:10.0};
       //tamanho inicial de cada bloco de pixel
       let tamanho_bloco = 20.0;

       loop{
        clear_background(DARKGRAY);

        //Lendo o teclado para movimentar a cobra   
        if is_key_pressed(KeyCode::Right) {
            cabeca.x += 1.0;
        }else if is_key_pressed(KeyCode::Left) {
            cabeca.x -= 1.0 ;
        }else if is_key_pressed(KeyCode::Down) {
            cabeca.y += 1.0;
        }
        else if is_key_pressed(KeyCode::Up) {
            cabeca.y -= 1.0;
        }
    



        //DESENHANDO O RETANGULO (CABEÇA DA COBRA)
        draw_rectangle(
            cabeca.x * tamanho_bloco,
            cabeca.y * tamanho_bloco, 
            tamanho_bloco,
            tamanho_bloco,
            GREEN
        
        );

        next_frame().await;
       }
    
    }
}
