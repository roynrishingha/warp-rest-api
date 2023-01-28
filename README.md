<h1 align="center">WARP REST API</h1>

**A REST API created with warp framework that lets it users to ask and answer questions**

## Setup

1. Clone the repository locally
2. Run `cargo run --release`

## Usage

### Create a new question

```sh
curl -X POST \
  'http://127.0.0.1:8080/questions' \
  --header 'Content-Type: application/json' \
  --data-raw '  {
    "id": "4",
    "title": "How you doing?",
    "content": "What's up?",
    "tags": [
      "faq"
    ]
  }'
```

### Get all questions

```sh
curl -X GET 'http://127.0.0.1:8080/questions'
```

### Get one question

```sh
curl -X GET 'http://host:port/questions/:question_id'
```

```sh
curl -X GET 'http://127.0.0.1:8080/questions/2'
```

### Update a question

```sh
curl -X PUT \
  'http://127.0.0.1:8080/questions/4' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "id": "4",
  "title": "new title",
  "content": "is it working?",
  "tags": [
    "faq", "test"
  ]
}'
```

### Delete a question

```sh
curl -X DELETE 'http://host:port/questions/:question_id'
```

```sh
curl -X DELETE 'http://127.0.0.1:8080/questions/4'
```