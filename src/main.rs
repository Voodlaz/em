use actix_web::*;
use actix_web_actors::ws;

use actix_files as fs;
use tera::{Tera, Context};

use actix::StreamHandler;

use actix::*;

use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct Ws {
    hb: Instant,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl Ws {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(
        &mut self,msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            /*Ok(ws::Message::Text(text)) => {
                /*create_message(&PgConnection, text, 6);

                let results = posts.filter(publish_at.lt(now))
                    .load::<Message>(&PgConnection)
                    .expect("Error loading posts");*/
                ctx.text(text)
            },*/
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

async fn ws(req: HttpRequest,stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    let resp = ws::start(Ws::new(), &req, stream);
    println!("{:?}", resp);
    resp
}

async fn index() -> impl Responder {
    let tera =
        Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
        ).unwrap();

    let ctx = Context::new();
    let rendered = tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //это тоже нужно будет поставить
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();


    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()

            .data(pool.clone())
            .wrap(middleware::Logger::default())

            .service(
                fs::Files::new("/static", "./static")
                .show_files_listing()
            )

            .service(
                web::resource("/ws/").to(ws)
            )
            .service(
                web::resource("/").to(index)
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
