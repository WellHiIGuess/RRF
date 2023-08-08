# OOGA BOOGA RS
Ooga Booga rs is a Front end Web framework that is written in Rust. Ooga Booga rs allows you to make elements and pages that compile to HTML and are sent through ```actix-web```.

## How to use
To use Ooga Booga rs simply clone this repository with the command:

```git clone https://github.com/WellHiIGuess/ooga-booga-rs.git```

This will give you a sample project to work with. All of your pages are in the ```/app``` directory and any reusable elements that you make will be in the ```/elements``` directory. Any Javascript/Typescript or CSS you use will be in the respective directories.

To make a new route you will first make a new file in ```/app``` and in it create a function that return a page like so:

```
use crate::library::page::{page, Page};

pub fn route() -> Page {
    // page is a function that takes in a Vector of elements and will automatically make a Page for you
    page(vec![
        // elements
    ])
}
```

You'll notice if you visit this page you still won't be able so see anything because you need to tell Actix to serve this page in the main file:

```
#[get("/route")]
async fn route_page() -> impl Responder {
    HttpResponse::Ok().body(route().serve())
}
```

And tell Actix to serve this function.

## Contributing
Ooga Booga is open source and as well is incomplete so contributing is welcomed!

## Documentation
Right now Ooga Booga has no documnetation but you're a programmer you can figure it out. Good luck!
