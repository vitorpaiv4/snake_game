use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)] // Adicionado PartialEq para podermos comparar Pontos facilmente
struct Ponto {
    x: f32,
    y: f32,
}

enum Direcao {
    Cima,
    Baixo,
    Esquerda,
    Direita,
}

#[macroquad::main("Snake Game")]
async fn main() {
    let mut corpo = vec![
        Ponto { x: 10.0, y: 10.0 }, // cabeca
        Ponto { x: 9.0, y: 10.0 },  // corpo
        Ponto { x: 8.0, y: 10.0 },  // rabo
    ];
    let tamanho_bloco = 20.0;
    let mut maca = Ponto { x: 15.0, y: 10.0 };

    let mut direcao = Direcao::Direita;
    let mut ultimo_tempo = get_time();
    let velocidade = 0.15; 
    
    // 1. ADICIONADO: Variável de estado
    let mut game_over = false;

    loop {
        clear_background(DARKGRAY);

        // 2. Se o jogo acabou, desenhamos o texto e travamos a lógica
        if game_over {
            // Desenha o texto vermelho no meio da tela
            draw_text("GAME OVER", screen_width() / 2.0 - 100.0, screen_height() / 2.0, 50.0, RED);
            draw_text("Pressione ESPAÇO para recomeçar", screen_width() / 2.0 - 200.0, screen_height() / 2.0 + 40.0, 30.0, WHITE);

            // Reinicia o jogo se apertar espaço
            if is_key_pressed(KeyCode::Space) {
                corpo = vec![
                    Ponto { x: 10.0, y: 10.0 },
                    Ponto { x: 9.0, y: 10.0 },
                    Ponto { x: 8.0, y: 10.0 },
                ];
                direcao = Direcao::Direita;
                game_over = false;
            }
        } 
        // 3. Se NÃO for game over, o jogo roda normalmente
        else {
            if is_key_pressed(KeyCode::Right) {
                direcao = Direcao::Direita;
            } else if is_key_pressed(KeyCode::Left) {
                direcao = Direcao::Esquerda;
            } else if is_key_pressed(KeyCode::Down) {
                direcao = Direcao::Baixo;
            } else if is_key_pressed(KeyCode::Up) {
                direcao = Direcao::Cima;
            }

            if get_time() - ultimo_tempo > velocidade {
                let mut nova_cabeca = corpo[0];

                match direcao {
                    Direcao::Cima => nova_cabeca.y -= 1.0,
                    Direcao::Baixo => nova_cabeca.y += 1.0,
                    Direcao::Esquerda => nova_cabeca.x -= 1.0,
                    Direcao::Direita => nova_cabeca.x += 1.0,
                }

                // 4. VERIFICAÇÃO DE COLISÃO COM A PAREDE
                // A tela começa no (0,0). O limite direito é a largura total dividida pelo bloco.
                let max_x = screen_width() / tamanho_bloco;
                let max_y = screen_height() / tamanho_bloco;

                if nova_cabeca.x < 0.0 || nova_cabeca.x >= max_x || nova_cabeca.y < 0.0 || nova_cabeca.y >= max_y {
                    game_over = true;
                }

                // 5. VERIFICAÇÃO DE COLISÃO COM O PRÓPRIO CORPO
                // O .contains() verifica se a nova cabeça já existe dentro do vetor do corpo
                if corpo.contains(&nova_cabeca) {
                    game_over = true;
                }
                
                // Só atualiza a cobra se ela não bateu
                if !game_over {
                    corpo.insert(0, nova_cabeca);

                    if nova_cabeca.x == maca.x && nova_cabeca.y == maca.y {
                        let max_grid_x = (screen_width() / tamanho_bloco) as i32;
                        let max_grid_y = (screen_height() / tamanho_bloco) as i32;
                        
                        maca.x = macroquad::rand::gen_range(0, max_grid_x) as f32;
                        maca.y = macroquad::rand::gen_range(0, max_grid_y) as f32;
                    } else {
                        corpo.pop(); 
                    }
                }

                ultimo_tempo = get_time();
            }
        }

        // --- DESENHOS (sempre desenha, mesmo no game over) ---
        draw_rectangle(
            maca.x * tamanho_bloco,
            maca.y * tamanho_bloco,
            tamanho_bloco,
            tamanho_bloco,
            RED,
        );

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