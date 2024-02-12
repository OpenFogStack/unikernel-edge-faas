use crate::dispatcher::Invocation;
use axum::extract::Path;
use axum::{extract::State, http, routing, Router};
use hyper::body::Body;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProxyServer {
    server_task: tokio::task::JoinHandle<()>,
    pub receiver: mpsc::Receiver<Invocation>,
}

impl ProxyServer {
    pub fn listen(port: u16, max_waiters: usize, shutdown: CancellationToken) -> Self {
        let (sender, receiver) = mpsc::channel(max_waiters);
        let server_task = tokio::spawn(async move {
            Self::start(sender, port, shutdown).await;
        });

        Self {
            server_task,
            receiver,
        }
    }

    async fn start(sender: mpsc::Sender<Invocation>, port: u16, shutdown: CancellationToken) {
        let path = "/invoke/:function_name/:req_path";
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let app = Router::new()
            .route(path, routing::get(Self::invoke_handler))
            .with_state(sender);

        info!("Proxy server listening on 0.0.0.0:{}", port);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(async move {
                shutdown.cancelled().await;
                info!("Proxy server shutting down");
            })
            .await
            .expect("Proxy server exited with error");
    }

    fn error_response(code: http::StatusCode, msg: &str) -> http::Response<Body> {
        info!("Failure! {}", msg);
        http::Response::builder()
            .status(code)
            .body(Body::from(format!("error: {}", msg)))
            .unwrap()
    }

    async fn invoke_handler(
        Path((function, path)): Path<(String, String)>,
        State(sender): State<mpsc::Sender<Invocation>>,
        request: http::Request<Body>,
    ) -> http::Response<Body> {
        let (tx, rx) = tokio::sync::oneshot::channel();

        let iv = Invocation {
            function,
            path,
            request,
            response: tx,
        };

        info!("Received invocation request {}:{}", iv.function, iv.path);

        // Enqueue invocation request to be picked up by the dispatcher.
        // This channel is bounded and if we are unable to send the invocation
        // request within a certain time i.e. if the load is too high,
        // abort and return error to client.
        if let Err(_) = sender
            .send_timeout(iv, tokio::time::Duration::from_millis(5000))
            .await
        {
            return Self::error_response(http::StatusCode::GATEWAY_TIMEOUT, "too many requests");
        }

        // The invocation request passed to the dispatcher will notify us
        // through this channel once the request has been processed and pass
        // the http response or an error.
        // TODO: handle channel closures
        // TODO: we should just pass an http response in the error case as well
        match rx.await.unwrap() {
            Ok(response) => {
                info!("Success!");
                return response;
            }
            Err(e) => {
                return Self::error_response(http::StatusCode::INTERNAL_SERVER_ERROR, &e);
            }
        }
    }
}
