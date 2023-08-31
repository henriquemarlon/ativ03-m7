# Notex Projeto

Este projeto consiste em três serviços principais: um banco de dados PostgreSQL, um backend e um frontend, todos orquestrados usando Docker Compose.

## Pré-requisitos

- Docker
- Docker Compose

## Configuração

1. **Arquivo .env**: Certifique-se de ter um arquivo `.env` dentro do diretório `notex`. Para isso, rode:

```bash
make env
```

2. Adicione como exemplo as seguintes variáveis ao arquivo ```.env``` criado dentro da pasta ```/notex```:

```bash
POSTGRES_DB="db"
POSTGRES_USER="user"
JWT_SECRET_KEY="secret"
POSTGRES_PASSWORD="password"
DATABASE_URL="postgres://user:password@postgres:5432/db"
```

Você pode ajustar os valores conforme necessário.

2. **Imagens Docker**: Este projeto utiliza as seguintes imagens Docker:

- `postgres:latest`
- `henriquemarlon/notex-backend:1.0.1`
- `henriquemarlon/notex-frontend:1.0.0`

Certifique-se de que você tem acesso a essas imagens ou ajuste o arquivo `docker-compose.yml` conforme necessário.

## Como Rodar

1. Navegue até o diretório raiz do projeto.

2. Inicie os serviços usando Docker Compose:

```bash
docker compose up
```

3. Após iniciar os serviços, você pode acessar:

- **Frontend**: `http://localhost:3000`
- **Backend**: `http://localhost:8080`
- **Banco de Dados**: `http://localhost:5432`

4. Para parar os serviços:

```bash
docker compose down -v
```

## Volumes

Este projeto utiliza um volume chamado `postgres_data` para persistir os dados do banco de dados PostgreSQL.

## Arquitetura da Aplicação:
<p align="center">
  <img src="URL_DA_IMAGEM" alt="Descrição da Imagem">
</p>