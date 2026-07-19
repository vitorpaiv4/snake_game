# Jogo da Cobrinha em Rust (Snake Game)

Um projeto simples do clássico jogo da cobrinha construído do zero para aprendizado da linguagem Rust. Este projeto utiliza a biblioteca `macroquad` para a renderização gráfica 2D e captura de eventos do teclado.

## Funcionalidades

* Movimentação contínua da cobra com proteção contra movimento reverso.
* Sistema de pontuação que aumenta ao comer a maçã.
* Mecânica de travessia de tela (wrap-around): ao sair por um lado, a cobra reaparece no lado oposto.
* Tela de "Game Over" acionada caso a cobra colida com o próprio corpo.

## Pré-requisitos

Para rodar este projeto, você precisa ter a linguagem Rust e o gerenciador de pacotes Cargo instalados na sua máquina.

## Como executar

1. Abra o terminal e navegue até a pasta raiz do projeto.
2. Execute o comando abaixo para compilar e iniciar o jogo:

   cargo run

Nota: Na primeira execução, o Cargo fará o download e a compilação do motor gráfico, o que pode demorar alguns segundos. As execuções seguintes serão quase instantâneas.

## Controles

* Setas do teclado (Cima, Baixo, Esquerda, Direita): Guiar a cobra pelo tabuleiro.
* Barra de espaço: Recomeçar o jogo na tela de Game Over.