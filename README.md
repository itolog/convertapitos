# ConvertApiTos

## Preliminary requirements

Add `public/uploads` folder if it does not exist in the project root

## Local development

run the command

```bash
make
```

or

```bash
cargo watch -x run
```

## Launching the project in production mode

```bash
make prod
```

or

```bash
docker compose up -d --build
```

### project shutdown

```bash
make prod-stop
```

or

```bash
docker compose stop
```