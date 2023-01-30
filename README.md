<h1 align="center">WARP REST API</h1>

**A REST API created with warp framework that lets it users to ask and answer questions**

## SETUP

Clone the repository

```sh
git clone https://github.com/royrustdev/warp-rest-api.git
```

### SETUP THE DATABASE

I am going to use `podman` to setup database in container. As a Fedora Linux user, I prefer podman over docker, but all the commands can be run using `docker` as well by raplacing podman with docker.

#### STEP ONE - START THE CONTAINER

```sh
podman run --name postgres -e POSTGRES_PASSWORD=1234 -p 5432:5432 -d postgres:14.4
```

The command `podman run --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres:14.4` is used to start a PostgreSQL container with Podman. Here's what each part of the command does:

- `podman run`: This is the command to run a container with Podman.

- `--name postgres`: This option gives the container a name of "postgres", which makes it easier to identify and manage the container in the future.

- `-e POSTGRES_PASSWORD=password`: This option sets an environment variable named _POSTGRES_PASSWORD_ to the value of "password". This password will be used as the password for the PostgreSQL user "postgres".

- `-p 5432:5432`: This option maps the container's port 5432 to the host's port 5432. This makes it possible to connect to the PostgreSQL database running in the container from the host machine.

- `-d`: This option runs the container in the background, so the command prompt returns immediately after starting the container.

- `postgres:14.4`: This is the name of the PostgreSQL image to use for the container. The image will be pulled from a registry if it doesn't exist locally. Using the 14.4 version of PostgreSQL.

**With this command, Podman will start a PostgreSQL container with the specified environment variable and port mapping, and run it in the background.**

#### STEP 2 - CREATE A DATABASE NAMED `warp_rest_api` IN THE RUNNING CONTAINER

To create a "warp_rest_api" database in the PostgreSQL container, you can use the `psql` command-line tool that's included with the PostgreSQL image.

Here's an example of how to create the database:

1. Connect to the PostgreSQL container:

```sh
podman exec -it postgres psql -U postgres
```

2. Create the "warp_rest_api" database:

```sh
CREATE DATABASE warp_rest_api;
```

3. Exit the `psql` command-line tool:

```sh
\q
```

With these steps, you will have created a `warp_rest_api` database in the PostgreSQL container.

##### Database URL: `postgresql://postgres:1234@localhost:5432/warp_rest_api`

<br />

#### STEP 3 - DATABASE MIGRATION

Run these command from within the root of project directory.

##### SETUP MIGRATION FOR QUESTIONS TABLE

Add questions table migration

```sh
sqlx migrate add -r questions_table
```

###### THIS COMMAND SHOULD RETURN

```sh
Creating migrations/20230130045110_questions_table.up.sql
Creating migrations/20230130045110_questions_table.down.sql
```

##### SETUP MIGRATION FOR ANSWERS TABLE

Add answers table migration

```sh
sqlx migrate add -r answers_table
```

###### THIS COMMAND SHOULD RETURN

```sh
Creating migrations/20230129151005_answers_table.up.sql
Creating migrations/20230129151005_answers_table.down.sql
```

##### RUN THE MIGRATION

```sh
sqlx migrate run --database-url postgresql://postgres:1234@localhost:5432/warp_rest_api
```

###### THIS COMMAND SHOULD RETURN

```sh
Applied 20230129140342/migrate questions table (19.393543ms)
Applied 20230129151005/migrate answers table (6.661552ms)
```

The command `sqlx migrate run --database-url postgresql://postgres:1234@localhost:5432/warp_rest_api` is used to run database migrations using sqlx-cli. Here's what each part of the command does:

- `sqlx migrate run`: This is the command to run database migrations with sqlx-cli.

- `--database-url`: This option specifies the database URL to connect to.

- `postgresql://postgres:1234@localhost:5432/warp_rest_api`: This is the database URL that provides the connection information for sqlx-cli.

- `postgresql://`: This part of the URL specifies that the database is a PostgreSQL database.

- `postgres:1234@`: This part of the URL provides the username and password to use for the connection. In this case, the username is "postgres" and the password is "1234".

- `localhost:5432/`: This part of the URL provides the host and port information for the database. In this case, the host is "localhost" and the port is "5432".

- `warp_rest_api`: This part of the URL specifies the name of the database to connect to. In this case, it's the "warp_rest_api" database.

**With this command, sqlx-cli will connect to the specified PostgreSQL database and run any pending migrations. The migrations are specified in the code and executed in the specified order to make changes to the database schema.**

##### LOG INTO THE RUNNING POSTGRESQL DATABSE CONTAINER

```sh
# Connect to the `warp_rest_api` database
podman exec -it postgres psql -U postgres -d warp_rest_api

# List all tables
\dt

# Quit psql
\q
```

###### THIS COMMAND SHOULD RETURN

```sh
warp_rest_api=# \dt
              List of relations
 Schema |       Name       | Type  |  Owner
--------+------------------+-------+----------
 public | _sqlx_migrations | table | postgres
 public | answers          | table | postgres
 public | questions        | table | postgres
(3 rows)
```

### HOW TO REVERT DATABASE CHANGES ?

We try to revert our changes after migration. 
Each revert will trigger the latest migration and try to run the `*.down.sql` script:

```sh
sqlx migrate revert --database-url postgresql://postgres:1234@localhost:5432/warp_rest_api

# Sould return :
# Applied 20230129151005/revert answers table (11.308705ms)

sqlx migrate revert --database-url postgresql://postgres:1234@localhost:5432/warp_rest_api

# Sould return :
# Applied 20230129140342/revert questions table (11.24183ms)
```

### STEP 4 - RUN THE APPLICATION

```sh
cargo run --release
```

## USAGE

All the examples shown here are by using `curl`.
### CREATE A NEW QUESTION

```sh
curl -X POST \
  'http://127.0.0.1:8080/questions' \
  --header 'Content-Type: application/json' \
  --data-raw '  {
    "title": "How you doing?",
    "content": "What's up?",
    "tags": [
      "faq"
    ]
  }'
```

### GET ALL QUESTIONS

```sh
curl -X GET 'http://127.0.0.1:8080/questions'
```

### GET QUESTION BY ID

Request format
```sh
curl -X GET 'http://host:port/questions/:question_id'
```

Get the question with id 1
```sh
curl -X GET 'http://127.0.0.1:8080/questions/1'
```

### UPDATE QUESTION BY ID

```sh
curl -X PUT \
  'http://127.0.0.1:8080/questions/1' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "id": 1,
  "title": "new title",
  "content": "is it working?",
  "tags": [
    "faq", "test"
  ]
}'
```

### DELETE QUESTION BY ID

Request format
```sh
curl -X DELETE 'http://host:port/questions/:question_id'
```

Delete the question with id 1
```sh
curl -X DELETE 'http://127.0.0.1:8080/questions/1'
```

### ADD ANSWER TO A QUESTION

```sh
curl --location --request POST 'localhost:8080/answers' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'question_id=1' \
--data-urlencode 'content=This is the question I had.'
```
