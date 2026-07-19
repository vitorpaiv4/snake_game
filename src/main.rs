use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
struct Ponto {
    x: f32,
    y: f32,
}

#[derive(PartialEq)]
enum Direcao {
    Cima,
    Baixo,
    Esquerda,
    Direita,
}

#[macroquad::main("Snake Game")]
async fn main() {
    let mut corpo = vec![
        Ponto { x: 10.0, y: 10.0 },
        Ponto { x: 9.0, y: 10.0 },
        Ponto { x: 8.0, y: 10.0 },
    ];
    let tamanho_bloco = 20.0;
    let mut maca = Ponto { x: 15.0, y: 10.0 };

    let mut direcao = Direcao::Direita;
    let mut ultimo_tempo = get_time();
    let velocidade = 0.15; 
    let mut game_over = false;
    let mut pontuacao = 0;

    loop {
        clear_background(DARKGRAY);

        if game_over {
            draw_text("GAME OVER", screen_width() / 2.0 - 100.0, screen_height() / 2.0, 50.0, RED);
            draw_text("Pressione ESPAÇO para recomeçar", screen_width() / 2.0 - 200.0, screen_height() / 2.0 + 40.0, 30.0, WHITE);
            draw_text("Desenvolvido por Vitor Paiva", screen_width() / 2.0 - 200.0, screen_height() / 2.0 + 80.0, 30.0, WHITE);

            if is_key_pressed(KeyCode::Space) {
                corpo = vec![
                    Ponto { x: 10.0, y: 10.0 },
                    Ponto { x: 9.0, y: 10.0 },
                    Ponto { x: 8.0, y: 10.0 },
                ];
                direcao = Direcao::Direita;
                game_over = false;
                pontuacao = 0; 
            }
        } else {
            if is_key_pressed(KeyCode::Right) && direcao != Direcao::Esquerda {
                direcao = Direcao::Direita;
            } else if is_key_pressed(KeyCode::Left) && direcao != Direcao::Direita {
                direcao = Direcao::Esquerda;
            } else if is_key_pressed(KeyCode::Down) && direcao != Direcao::Cima {
                direcao = Direcao::Baixo;
            } else if is_key_pressed(KeyCode::Up) && direcao != Direcao::Baixo {
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

                let max_x = screen_width() / tamanho_bloco;
                let max_y = screen_height() / tamanho_bloco;

                // 1. MODIFICADO: Lógica de Wrap-Around em vez de morte
                // Eixo X
                if nova_cabeca.x < 0.0 {
                    nova_cabeca.x = max_x - 1.0; // Saiu pela esquerda, entra na direita
                } else if nova_cabeca.x >= max_x {
                    nova_cabeca.x = 0.0; // Saiu pela direita, entra na esquerda
                }

                // Eixo Y
                if nova_cabeca.y < 0.0 {
                    nova_cabeca.y = max_y - 1.0; // Saiu por cima, entra por baixo
                } else if nova_cabeca.y >= max_y {
                    nova_cabeca.y = 0.0; // Saiu por baixo, entra por cima
                }

                // 2. Game Over APENAS se bater no próprio corpo
                if corpo.contains(&nova_cabeca) {
                    game_over = true;
                }
                
                if !game_over {
                    corpo.insert(0, nova_cabeca);

                    if nova_cabeca.x == maca.x && nova_cabeca.y == maca.y {
                        let max_grid_x = max_x as i32;
                        let max_grid_y = max_y as i32;
                        
                        maca.x = macroquad::rand::gen_range(0, max_grid_x) as f32;
                        maca.y = macroquad::rand::gen_range(0, max_grid_y) as f32;
                        pontuacao += 10;
                    } else {
                        corpo.pop(); 
                    }
                }

                ultimo_tempo = get_time();
            }
        }

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

        let placar_texto = format!("Pontos: {}", pontuacao);
        draw_text(&placar_texto, 10.0, 30.0, 30.0, WHITE);

        next_frame().await;
    }
}