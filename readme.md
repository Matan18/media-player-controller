# Spotify Controller

Eu queria aprender um pouco de [Rust](https://www.rust-lang.org/), e baseado em um stories no instagram de um amigo mexendo com [Go](https://go.dev/), eu decidi montar esse projetinho em [Rust](https://www.rust-lang.org/) (inicialmente queria fazer em [Go](https://go.dev/), mas não estava conseguindo usar corretamente).

O projeto interage com o controlle de PS4 para controlar o player do [spotify](https://developer.spotify.com/), utilizando [redis](https://redis.io/) para guardar estados (como o token do usuário) no momento as funcionalidades são:

- Próxima música (R1);
- Música anterior (L1);
- Play/Pause (Option);
- Definir aleatório (X);
- Cancelar aleatório (Square);
- Definir repetição (Círculo);
- Definir repetição de música (Triângulo);

## Requisitos

- Ter acesso bluetooth;
- Ter controle PS4;
- Ter Rust instalado;
- Ter um servidor redis rodando;
- Instalar as dependências do projeto através do comando `cargo build`;
- Configurar um serviço no [spotify developers](https://developer.spotify.com/) e criar um arquivo `.env` baseado no `.env.example` com os respectivos `CLIENT_ID` e `CLIENT_SECRET`;
- Conectar o PS4 antes de iniciar o projeto;
- Iniciar o projeto com o comando `cargo run`;
- Acessar a rota `localhost:8080` para fazer a autenticação do usuário (até onde sei, spotify não possui um serviço de acesso direto)(o usuário deve ter spotify premium (esse requisito não é meu));
- Após o fluxo de login você deve ver um json no browser com as informações do usuário logado, e o controle deve estar funcionando;

## Funcionamento

A base do funcionamento do aplicativo consiste em compreender o botão digitado pelo usuário no controle e enviar requisições http para o spotify, pra isso, eu separei o programa em 3 partes:

- Servidor http (pasta `token`): utilizado para pegar o token do usuário construído em [rocket](https://docs.rs/rocket/0.4.11/rocket/);
- Interpretador do controle (pasta `controll_handler`): utilizado para ler o buffer de informações do controle e disparar eventos utilizando uma biblioteca de eventos do rust [event-emitter-rs](https://docs.rs/event-emitter-rs/latest/event_emitter_rs/);
- Controlador do spotify (pasta `player_controller`): utilizado para atuar no spotify através das requisições http;

Para fazer isso, eu montei um `mapa` do que cada botão significa no buffer que o controle envia para o computador, e deixei registrado no arquivo [buttons.md](./buttons.md), não está completo, mas me ajudou a montar o projeto inicialmente, uma falha nesse mapeamento é que dependendo dos botões que são **apertados juntos**, a informação é somada, gerando uma resposta diferente (por exemplo, usando o mapeamento atual, ao apertar quadrado e triângulo juntos, não teria uma reação).

A aplicação também usa [tokio](https://tokio.rs/tokio/tutorial) para gerenciamento de processos, pois o servidor http responsável pela autenticação e o interpretador do controle funcionam em paralelo, também cada handler de evento inicia um processo paralelo para realizar funções **asynchronas**
