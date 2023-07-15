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
