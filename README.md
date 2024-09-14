# DevOps
-------------
Resumo da Configuração do Docker para um Projeto Rust
Criação do Projeto Rust:
Estrutura básica com src/main.rs e Cargo.toml.
Criação do Dockerfile:
Imagem Base: Utiliza rust:1.73 para construir o projeto.
Construção: Copia arquivos necessários, baixa dependências, compila o código e cria uma imagem de execução com debian:buster-slim.
Execução: O Dockerfile define o comando padrão para rodar o binário Rust.
Passos de Construção e Execução:
Construir Imagem: docker build -t hello 
Executar Contêiner: docker run -p 3000:3000 hello
