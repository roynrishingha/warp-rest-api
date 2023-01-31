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

##### SETUP MIGRATION FOR ACCOUNTS TABLE

Add answers table migration

```sh
sqlx migrate add -r create_accounts_table
```

###### THIS COMMAND SHOULD RETURN

```sh
Creating migrations/20230131020159_create_accounts_table.up.sql
Creating migrations/20230131020159_create_accounts_table.down.sql
```

```sh
sqlx migrate add -r extend_questions_table;
# Creating migrations/20230131023048_extend_questions_table.up.sql
# Creating migrations/20230131023048_extend_questions_table.down.sql

sqlx migrate add -r extend_answers_table;
# Creating migrations/20230131023126_extend_answers_table.up.sql
# Creating migrations/20230131023126_extend_answers_table.down.sql
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
 public | accounts         | table | postgres
 public | answers          | table | postgres
 public | questions        | table | postgres
(4 rows)
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

In order CREATE, UPDATE, DELETE resources, 
User needs to Register and Login.

### REGISTER A NEW USER

```sh
curl -X POST \
  'http://127.0.0.1:8080/registration' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "email": "test1@gmail.com",
  "password": "1234"
}'
```

### LOGIN AN EXISTING USER

```sh
curl -X POST \
  'http://127.0.0.1:8080/login' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "email": "test1@gmail.com",
  "password": "1234"
}'

# Token looks like this:
#
# "v2.local.NLj-3SvcxmxFS0nvbqFNju1w-CwmhPd9oQMUaO7dgzg5L94YlO4kppfpQ1A0iYIBhxdyFGrbAmn4ASmBnCS9vYxu7Ku5iGZVHUjw5DjPvYZbcATvzbZ1p8lV2hCskReb-xX-DzULxH6qIBJFoYfgqwz6xv4YXdEv1nDwnQDuYdUW3WIXkmw"
#
# Note: The Token given above is just an example.
```

**IT RETURNS A TOKEN THAT WE NEED TO CREATE, UPDATE, DELETE RESOURCES. PASS THIS TOKEN AS AN AUTHORIZATION TOKEN IN HEADER OF EACH REQUEST.**

### CREATE A NEW QUESTION

```sh
curl -X POST \
  'http://127.0.0.1:8080/questions' \
  --header 'Authorization: "ATHORIZATION TOKEN THAT I GOT FROM LOGIN"' \
  --header 'Content-Type: application/json' \
  --data-raw '  {
    "title": "How you doing?",
    "content": "What's up?",
    "tags": [
      "faq"
    ]
  }'
```

**AUTHORIZATION IS NOT REQUIRED FOR GET REQUESTS**
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
  --header 'Authorization: "ATHORIZATION TOKEN THAT I GOT FROM LOGIN"' \
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
curl -X DELETE 'http://host:port/questions/:question_id' \
--header 'Authorization: ATHORIZATION TOKEN THAT I GOT FROM LOGIN'
```

Delete the question with id 1
```sh
curl -X DELETE \
  'http://127.0.0.1:8080/questions/1' \
  --header 'Authorization: ATHORIZATION TOKEN THAT I GOT FROM LOGIN'
```

### ADD ANSWER TO A QUESTION

```sh
curl --location --request POST 'localhost:8080/answers' \
--header 'Authorization: ATHORIZATION TOKEN THAT I GOT FROM LOGIN'
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'question_id=1' \
--data-urlencode 'content=This is the question I had.'
```

```sh
cargo run -- --db-host localhost --log-level info --db-name rustwebdev –
db-port 5432 --db-password password

```

```sh
podman-compose build

# STDOUT

['podman', '--version', '']
using podman version: 4.3.1
podman build -t warp-rest-api_server -f ./Dockerfile .
[1/2] STEP 1/13: FROM rust:latest AS builder
[1/2] STEP 2/13: RUN rustup target add x86_64-unknown-linux-musl
--> Using cache 7eda1f1ae95f797763830cb4f8867f61d45d7fd8f9cc6dad6330bb311737aeed
--> 7eda1f1ae95
[1/2] STEP 3/13: RUN apt -y update
--> Using cache ad1e534bbcfe808ec8c0f9c9c636d056e80c47ab047c27c4995a131ecc1e81a2
--> ad1e534bbcf
[1/2] STEP 4/13: RUN apt install -y musl-tools musl-dev
--> Using cache a48338d63854f5c7ddf703e943fe8f1c89bdc62cafa0c088218594fed58b983c
--> a48338d6385
[1/2] STEP 5/13: RUN apt-get install -y build-essential
--> Using cache 66e0c7d11088a044e3331bbef21ecbb326930f1a18408b713123e6792a4e7b10
--> 66e0c7d1108
[1/2] STEP 6/13: RUN apt install -y gcc-x86-64-linux-gnu
--> Using cache 2b3b6818787815a3efa1326297276977e790007c87f55e0ddf628afbf4bd70b9
--> 2b3b6818787
[1/2] STEP 7/13: WORKDIR /app
--> Using cache 1469885ed0ccd96ad7e16732d9593a2042299a6b105a635283617f8230841aa4
--> 1469885ed0c
[1/2] STEP 8/13: COPY ./ .
--> 03caa04ed54
[1/2] STEP 9/13: ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
--> 069bc62035f
[1/2] STEP 10/13: ENV CC='gcc'
--> 06e794241ba
[1/2] STEP 11/13: ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
--> adaa359ac58
[1/2] STEP 12/13: ENV CC_x86_64-unknown-linux-musl=x86_64-linux-gnu-gcc
--> e9c85d3c2a1
[1/2] STEP 13/13: RUN cargo build --target x86_64-unknown-linux-musl --release
    Updating crates.io index
 Downloading crates ...
# I didnot the stdout of Downloading crates.
# As it is a list of all the required crates to compile our crate.
#
# After downloading all required crates,
# all crates gets compiled in `release mode`
# Build time may vary...
    Finished release [optimized] target(s) in 3m 15s
--> 7624fe775a5
[2/2] STEP 1/5: FROM scratch
[2/2] STEP 2/5: WORKDIR /app
--> Using cache e64430571e07f72c6ab9fa16d788be5aff2438973d1fe6382bb4b944a1dc7b20
--> e64430571e0
[2/2] STEP 3/5: COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/warp-rest-api ./
--> 60501d37b0a
[2/2] STEP 4/5: COPY --from=builder /app/.env ./
--> ab7f7684adf
[2/2] STEP 5/5: CMD ["/app/warp-rest-api"]
[2/2] COMMIT warp-rest-api_server
--> 78b29aae138
Successfully tagged localhost/warp-rest-api_server:latest
78b29aae1389a5571e1150800aed1ded0af4dbee5c6e165dc0eb14b836d3b29c
exit code: 0
```

Now time to start the application
```sh
podman-compose up

# STDOUT

['podman', '--version', '']
using podman version: 4.3.1
** excluding:  set()
['podman', 'inspect', '-t', 'image', '-f', '{{.Id}}', 'warp-rest-api_server']
podman volume inspect warp-rest-api_data || podman volume create warp-rest-api_data
['podman', 'volume', 'inspect', 'warp-rest-api_data']
['podman', 'network', 'exists', 'warp-rest-api_default']
podman create --name=warp-rest-api_database_1 --label io.podman.compose.config-hash=123 --label io.podman.compose.project=warp-rest-api --label io.podman.compose.version=0.0.1 --label com.docker.compose.project=warp-rest-api --label com.docker.compose.project.working_dir=/home/royrustdev/dev/warp-rest-api --label com.docker.compose.project.config_files=docker-compose.yml --label com.docker.compose.container-number=1 --label com.docker.compose.service=database --env-file /home/royrustdev/dev/warp-rest-api/.env -v warp-rest-api_data:/var/lib/postgresql/data --net warp-rest-api_default --network-alias database -p 5432:5432 --restart always postgres:14.4
Error: creating container storage: the container name "warp-rest-api_database_1" is already in use by 6b2946699846ea4e4a7001dfaf4d1330a6419114c71e96ac5135588e6a01aa1c. You have to remove that container to be able to reuse that name: that name is already in use
exit code: 125
['podman', 'network', 'exists', 'warp-rest-api_default']
podman create --name=warp-rest-api_server_1 --label io.podman.compose.config-hash=123 --label io.podman.compose.project=warp-rest-api --label io.podman.compose.version=0.0.1 --label com.docker.compose.project=warp-rest-api --label com.docker.compose.project.working_dir=/home/royrustdev/dev/warp-rest-api --label com.docker.compose.project.config_files=docker-compose.yml --label com.docker.compose.container-number=1 --label com.docker.compose.service=server --env-file /home/royrustdev/dev/warp-rest-api/.env --net warp-rest-api_default --network-alias server -p 8080:8080 warp-rest-api_server
97cb5464f87d3c85df28620bd173e33baebb68c24b5a1fdd8ad677b667bb5294
exit code: 0
podman start -a warp-rest-api_database_1
The files belonging to this database system will be owned by user "postgres".
This user must also own the server process.

The database cluster will be initialized with locale "en_US.utf8".
The default database encoding has accordingly been set to "UTF8".
The default text search configuration will be set to "english".

Data page checksums are disabled.

fixing permissions on existing directory /var/lib/postgresql/data ... ok
creating subdirectories ... ok
selecting dynamic shared memory implementation ... posix
selecting default max_connections ... 100
selecting default shared_buffers ... 128MB
selecting default time zone ... Etc/UTC
creating configuration files ... ok
running bootstrap script ... ok
performing post-bootstrap initialization ... ok
syncing data to disk ... podman start -a warp-rest-api_server_1
error: The following required arguments were not provided:
    --db-password <DB_PASSWORD>

USAGE:
    warp-rest-api [OPTIONS] --db-password <DB_PASSWORD>

For more information try --help
ok


Success. You can now start the database server using:

    pg_ctl -D /var/lib/postgresql/data -l logfile start

initdb: warning: enabling "trust" authentication for local connections
You can change this by editing pg_hba.conf or using the option -A, or
--auth-local and --auth-host, the next time you run initdb.
exit code: 2
waiting for server to start....2023-01-31 06:58:19.835 UTC [43] LOG:  starting PostgreSQL 14.4 (Debian 14.4-1.pgdg110+1) on x86_64-pc-linux-gnu, compiled by gcc (Debian 10.2.1-6) 10.2.1 20210110, 64-bit
2023-01-31 06:58:19.836 UTC [43] LOG:  listening on Unix socket "/var/run/postgresql/.s.PGSQL.5432"
2023-01-31 06:58:19.843 UTC [44] LOG:  database system was shut down at 2023-01-31 06:58:19 UTC
2023-01-31 06:58:19.847 UTC [43] LOG:  database system is ready to accept connections
 done
server started
CREATE DATABASE


/usr/local/bin/docker-entrypoint.sh: ignoring /docker-entrypoint-initdb.d/*

waiting for server to shut down...2023-01-31 06:58:20.541 UTC [43] LOG:  received fast shutdown request
.2023-01-31 06:58:20.542 UTC [43] LOG:  aborting any active transactions
2023-01-31 06:58:20.543 UTC [43] LOG:  background worker "logical replication launcher" (PID 50) exited with exit code 1
2023-01-31 06:58:20.544 UTC [45] LOG:  shutting down
2023-01-31 06:58:20.560 UTC [43] LOG:  database system is shut down
 done
server stopped

PostgreSQL init process complete; ready for start up.

2023-01-31 06:58:20.664 UTC [1] LOG:  starting PostgreSQL 14.4 (Debian 14.4-1.pgdg110+1) on x86_64-pc-linux-gnu, compiled by gcc (Debian 10.2.1-6) 10.2.1 20210110, 64-bit
2023-01-31 06:58:20.664 UTC [1] LOG:  listening on IPv4 address "0.0.0.0", port 5432
2023-01-31 06:58:20.664 UTC [1] LOG:  listening on IPv6 address "::", port 5432
2023-01-31 06:58:20.667 UTC [1] LOG:  listening on Unix socket "/var/run/postgresql/.s.PGSQL.5432"
2023-01-31 06:58:20.672 UTC [57] LOG:  database system was shut down at 2023-01-31 06:58:20 UTC
2023-01-31 06:58:20.678 UTC [1] LOG:  database system is ready to accept connections
```
