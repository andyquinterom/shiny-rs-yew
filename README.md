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

To include the dependencies using the `shiny-rs-server` crate you will need to include
the JavaScript and CSS dependencies. While we are working on setting up a CDN
to make this easier, for now you will have to manually download the dependencies
from the official Shiny repository.

In the following example we will download the dependencies and include them in
a `static` folder in the root of our project.

```bash
wget https://raw.githubusercontent.com/rstudio/shiny/v1.7.2/inst/www/shared/shiny.min.js -O static/shiny.min.js
wget https://raw.githubusercontent.com/rstudio/shiny/v1.7.2/inst/www/shared/shiny.min.js.map -O static/shiny.min.js.map
wget https://raw.githubusercontent.com/rstudio/shiny/v1.7.2/inst/www/shared/shiny.min.css -O static/shiny.min.css
```

Our `index.html` will something like this:

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <script src="https://code.jquery.com/jquery-3.6.1.min.js" integrity="sha256-o88AwQnZB+VDvE9tvIXrMQaPlFFSUTR+nldQm1LuPXQ=" crossorigin="anonymous"></script>
        <link href="lib/shiny.min.css" rel="stylesheet" />
        <script src="lib/shiny.min.js"></script>
        <link href="https://unpkg.com/leaflet@1.8.0/dist/leaflet.css" rel="stylesheet" />
        <script src="https://unpkg.com/leaflet@1.8.0/dist/leaflet.js"></script>
        <script src="https://cdn.plot.ly/plotly-2.14.0.min.js"></script>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.0/dist/css/bootstrap.min.css" integrity="sha384-gH2yIJqKdNHPEq0n4Mqa/HGKIhSkIHeL5AyhkYV8i59U5AR6csBvApHHNl/vI1Bx" crossorigin="anonymous">
        <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.0/dist/js/bootstrap.min.js" integrity="sha384-ODmDIVzN+pFdexxHEHFBQH3/9/vQ9uori45z4JjnFsRydbmQbmL5t1tQ0culUzyK" crossorigin="anonymous"></script>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
    </head>
    <body>
        <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.1/dist/js/bootstrap.bundle.min.js" integrity="sha384-u1OknCvxWvY5kfmNBILK2hRnQC3Pr17a+RTT6rIHI7NnikvbZlHgTPOOmMi466C8" crossorigin="anonymous"></script>
    </body>
</html>
```

We will also need to expose the `static` folder we created in our server. We can easily
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
