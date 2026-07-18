use macroquad::prelude::*;

// 1. Nossa estrutura de Ponto com a permissão de cópia
#[derive(Clone, Copy)]
struct Ponto {
    x: f32,
    y: f32,
}

// 2. As direções possíveis
enum Direcao {
    Cima,
    Baixo,
    Esquerda,
    Direita,
}

// 3. A função principal
#[macroquad::main("Snake Game")]
async fn main() {
    let mut corpo = vec![
        Ponto { x: 10.0, y: 10.0 }, // cabeca
        Ponto { x: 9.0, y: 10.0 },  // corpo
        Ponto { x: 8.0, y: 10.0 },  // rabo
    ];
    let tamanho_bloco = 20.0;

    let mut maca = Ponto { x: 15.0, y: 10.0 }; // posição inicial da maçã

    let mut direcao = Direcao::Direita;
    let mut ultimo_tempo = get_time();
    let velocidade = 0.15; // velocidade da cobra

    loop {
        clear_background(DARKGRAY);

        // Lendo o teclado para movimentar a cobra   
        if is_key_pressed(KeyCode::Right) {
            direcao = Direcao::Direita;
        } else if is_key_pressed(KeyCode::Left) {
            direcao = Direcao::Esquerda;
        } else if is_key_pressed(KeyCode::Down) {
            direcao = Direcao::Baixo;
        } else if is_key_pressed(KeyCode::Up) {
            direcao = Direcao::Cima;
        }

        // Verificando se já passou tempo suficiente para mover a cobra
        if get_time() - ultimo_tempo > velocidade {
            
            let mut nova_cabeca = corpo[0];

            match direcao {
                Direcao::Cima => nova_cabeca.y -= 1.0,
                Direcao::Baixo => nova_cabeca.y += 1.0,
                Direcao::Esquerda => nova_cabeca.x -= 1.0,
                Direcao::Direita => nova_cabeca.x += 1.0,
            }
            
            // Sempre inserimos a nova cabeça primeiro
            corpo.insert(0, nova_cabeca);

            // 2. ADICIONADO: Lógica de comer a maçã e crescer
            if nova_cabeca.x == maca.x && nova_cabeca.y == maca.y {
                // Comeu! 
                // Sorteia uma nova posição para a maçã (vamos supor um grid de 40x30 blocos)
                // O "as f32" converte o número inteiro do sorteio para decimal
                maca.x = macroquad::rand::gen_range(0, 40) as f32;
                maca.y = macroquad::rand::gen_range(0, 30) as f32;
                
                // NOTA: Como não damos corpo.pop() aqui, a cobra cresce!
            } else {
                // Não comeu a maçã, então removemos o rabo para manter o tamanho
                corpo.pop(); 
            }

            ultimo_tempo = get_time();
        }
        //desenhando a maça
        draw_rectangle(
            maca.x * tamanho_bloco,
            maca.y * tamanho_bloco,
            tamanho_bloco,
            tamanho_bloco,
            RED,
        );
        // Desenhando o corpo da cobra
        for (i, parte) in corpo.iter().enumerate() {
            let cor = if i == 0 { DARKGREEN } else { GREEN };

            draw_rectangle(
                parte.x * tamanho_bloco,
                parte.y * tamanho_bloco,
                tamanho_bloco,
                tamanho_bloco,
                cor,
            );
        }

        next_frame().await;
    }
}