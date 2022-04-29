use tokio::io::AsyncReadExt;
use chrono::Utc;
use clap::Parser;
use std::{net::SocketAddr, sync::Arc};
use hyper::{Body, Request, Response, Method, StatusCode};
use hyper::{Server, service::{make_service_fn, service_fn}};
use tokio::{fs::OpenOptions, io::{AsyncWriteExt, self}, sync::Mutex};

#[derive(Parser, Debug)]
#[clap(name="Skittles", version = "1.0")]
struct Args {
    #[clap(short, long, default_value_t = 7000)]
    port: u16,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let arg = Args::parse();
    let addr = SocketAddr::from(([127, 0, 0, 1], arg.port));

    let f = Arc::new(Mutex::new(1));

    let make_svc = make_service_fn(move |_conn| {        
        let f = Arc::clone(&f);

        async move {
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                let f = f.clone();

                async move {
                    let r: Result<Response<Body>, hyper::Error> = match (req.method(), req.uri().path()) {
                        (&Method::POST, "/log") => {
                            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

                            {
                                let _ = f.lock().await;
                                let mut file = OpenOptions::new().create(true).write(true).append(true).read(true).open("logger.txt").await.unwrap();
                                let _ = file.write(Utc::now().date().to_string().as_bytes()).await;
                                let _ = file.write(b" ").await;
                                let _ = file.write(&whole_body).await;
                                let _ = file.write(b"\n").await;
                            }

                            let mut ok = Response::default();
                            *ok.status_mut() = StatusCode::OK;
                            Ok(ok)
                        }
                        (&Method::POST, "/query") => {
                            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
                            let v = whole_body.to_vec();
                            let query_date = match std::str::from_utf8(&v) {
                                Ok(e) => e,
                                Err(_) => {
                                    let mut not_found = Response::default();
                                    *not_found.status_mut() = StatusCode::NOT_FOUND;
                                    return Ok(not_found)
                                }

                            };

                            let mut ok = Response::default();
                            let mut vv = String::new();
                            
                            {
                                let _ = f.lock().await;
                                let mut d = String::new();
                                let mut file = OpenOptions::new().create(true).write(true).append(true).read(true).open("logger.txt").await.unwrap();
                                let _i = file.read_to_string(&mut d).await;
                                let nsplit = d.split("\n");

                                for row in nsplit {
                                    match row.split_once(" ") {
                                        Some((a, _b)) => {
                                            if a == query_date {
                                                vv.push_str(&row);
                                                vv.push_str("\n");
                                            }
                                        },
                                        None => break,
                                    }
                                }

                            }
                            *ok.body_mut() = Body::from(vv);
                            *ok.status_mut() = StatusCode::OK;
                            Ok(ok)
                        },
                        _ => {
                            let mut not_found = Response::default();
                            *not_found.status_mut() = StatusCode::NOT_FOUND;
                            Ok(not_found)
                        }
                    };

                    return r;
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}