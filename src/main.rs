use macroquad::prelude::*;

// Criando estrutura
struct Ponto {
    x: f32,
    y: f32,
}
// Criando um Enum para definir a direção da cobra
enum Direcao {
    Cima,
    Baixo, 
    Esquerda, 
    Direita,
}



#[macroquad::main("Snake Game")]
async fn main() {
    loop {
       // Definimos a posição inicial da cabeça da cobra
       let mut cabeca = Ponto {x: 10.0, y:10.0};
       //tamanho inicial de cada bloco de pixel
       let tamanho_bloco = 20.0;


       let mut direcao = Direcao::Direita;
       let mut ultimo_tempo = get_time();
       let velocidade = 0.15; // velocidade da cobra

       loop{
        clear_background(DARKGRAY);

        //Lendo o teclado para movimentar a cobra   
        if is_key_pressed(KeyCode::Right) {
            direcao = Direcao::Direita;
        }else if is_key_pressed(KeyCode::Left) {
            direcao = Direcao::Esquerda;
        }else if is_key_pressed(KeyCode::Down) {
            direcao = Direcao::Baixo;
        }
        else if is_key_pressed(KeyCode::Up) {
            direcao = Direcao::Cima;
        }
        
        // verificando se já passou tempo suficiente para mover a cobra
        if get_time() - ultimo_tempo > velocidade {

            //match do rust é tipo o switch
            match direcao {
                Direcao::Cima => cabeca.y -= 1.0,
                Direcao::Baixo => cabeca.y += 1.0,
                Direcao::Esquerda => cabeca.x -= 1.0,
                Direcao::Direita => cabeca.x += 1.0
            }
            //reinicia o cronograma 
            ultimo_tempo = get_time();
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
