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
![Captura de Tela 2024-09-14 às 16 35 52](https://github.com/user-attachments/assets/ca516f4a-8543-4c15-a724-383b1cd22522)
TESTES UNITARIOS
![Captura de Tela 2024-09-29 às 12 33 48](https://github.com/user-attachments/assets/65329dda-f393-4346-b6f5-6201b636c7b7)
