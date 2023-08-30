use warp::Filter;

#[tokio::main]
async fn main() {
    // Defina o conteúdo HTML
    let html_content = warp::path::end()
        .map(|| {
            warp::reply::html(include_str!("./index.html"))
        })
        .with(warp::cors().allow_any_origin());  // Adicione esta linha para permitir qualquer origem

    // Inicie o servidor
    warp::serve(html_content)
        .run(([0, 0, 0, 0], 3000))
        .await;
}
