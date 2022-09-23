# Shiny-RS Yew

This is a set of UI components to build Shiny UIs with Yew!
It is still a work in progress, but it is already usable. For
an example on how this crate can be used to build Shiny UIs
checkout the [Shiny-RS Example](https://github.com/andyquinterom/shiny-rs-example).

## Including dependencies

In order to use this with a Shiny server (R, Python or Rust) you will need to include
the JavaScript dependencies in your `index.html`. For a deeper explanation on how to
to do it for R checkout this [this article](https://shiny.rstudio.com/articles/html-ui.html)
from RStudio.

### With shiny-rs-server

In order to run the app with the pre-built components you will need to include the
following dependencies in the `static` directory at the root of your project:

- jquery.min.js
- shiny.min.js
- shiny.min.css
- bootstrap.min.css
- bootstrap.bundle.min.js

To automatically download these dependencies run the following commands:

```bash
wget https://code.jquery.com/jquery-3.6.1.min.js -O static/jquery.min.js
wget https://raw.githubusercontent.com/rstudio/shiny/v1.7.2/inst/www/shared/shiny.min.js -O static/shiny.min.js
wget https://raw.githubusercontent.com/rstudio/shiny/v1.7.2/inst/www/shared/shiny.min.js.map -O static/shiny.min.js.map
wget https://raw.githubusercontent.com/rstudio/shiny/v1.7.2/inst/www/shared/shiny.min.css -O static/shiny.min.css
wget https://cdn.jsdelivr.net/npm/bootstrap@5.2.1/dist/css/bootstrap.min.css -O static/bootstrap.min.css
wget https://cdn.jsdelivr.net/npm/bootstrap@5.2.1/dist/js/bootstrap.bundle.min.js -O static/bootstrap.bundle.min.js
```

We will also need to expose the `static` folder we created in our server to the `/lib` path. We can easily
add a service that looks something like this:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/lib", "./static").show_files_listing())
            .service(web::resource("/websocket/").route(web::get().to(server)))
            .service(Files::new("/", "./dist").index_file("index.html"))
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
```

## Build an app

To build an app that is 100% Rust (meaning the backend is also built in rust with `shiny-rs-server`)
you will need to devide your project into two parts:

- The `frontend` part, which will be the UI of your app.
- The `backend` part, which will be the server of your app.

A folder structure can look like this:

```
├── backend
│   ├── src
│   │   ├── main.rs
│   ├── Cargo.toml
│   └── ...
├── frontend
│   ├── src
│   │   ├── main.rs
│   ├── Cargo.toml
│   ├── Trunk.toml
│   └── index.html
├── src (shared logic)
│   └── lib.rs
├── Cargo.toml
├── dist (the html and wasm files)
└── .gitignore
```

We can then build our front end with `trunk`:

```bash
cd frontend
trunk build --release
```

And our backend with `cargo`:

```bash
cd backend
cargo build --release
```

We have a template left on [shiny-rs-template](https://github.com/andyquinterom/shiny-rs-template).
