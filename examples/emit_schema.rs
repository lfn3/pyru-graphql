use pyru_graphql::make_schema;

#[tokio::main]
async fn main() {
    let s = make_schema();

    println!("{}", &s.sdl());
}
