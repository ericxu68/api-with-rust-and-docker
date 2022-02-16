# Building APIs with Rust

Includes Actix, serde, Env logger and chrono. Also Docker-compose to build it as a container.

## Usage

Locally for development:
```
    $ cargo run
```

Creating the docker image and running it:
```
    $ docker-compose build && docker-compose up
```

Any of both options (by default) will run over: `http://localhost:8080/`.

Check the `.env.template` to tweak any of these options.

Includes `http://localhost:8080/health` as a JSON-responder example.

### Todo

- Testing
- Benchmarking
- Auto-reloading during dev (cargo watch -x 'run --bin app')
- Healthchecking
- Web Sockets Example
- Review error handling
- Database Connection: This is very project dependent, so idk
- Disel ORM: idem as previous
- Rocket Example, some day when i actually decide to try it
- Server side events: probably not, but maybe yes, we got WS now so why bother?
