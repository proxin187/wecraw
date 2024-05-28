use crate::model::Model;

use tiny_http::{Server as HttpServer, Response, Request, Header, Method};


pub struct Server {
    model: Model,
    server: HttpServer,
}

impl Server {
    pub fn new(model: Model) -> Result<Server, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Server {
            model,
            server: HttpServer::http("127.0.0.1:8000")?,
        })
    }

    fn respond(
        &self,
        request: Request,
        data: &[u8],
        content_type: &str,
        status: u32
    ) -> Result<(), Box<dyn std::error::Error>> {
        let header = Header::from_bytes("Content-Type", content_type)
            .map_err(|_| String::from("failed to create header"))?;

        let response = Response::from_data(data)
            .with_header(header)
            .with_status_code(status);

        request.respond(response)?;

        Ok(())
    }

    fn serve(&self, mut request: Request) -> Result<(), Box<dyn std::error::Error>> {
        match (request.method(), request.url()) {
            (Method::Post, "/api/search") => {
                let mut body: Vec<u8> = Vec::new();

                request.as_reader().read_to_end(&mut body)?;

                let query = body.iter().map(|byte| *byte as char).collect::<String>();

                println!("[+] searching: {}", query);

                let result = self.model.search(query.as_str())?;
                let json = serde_json::to_vec(&result.iter().take(20).collect::<Vec<_>>())?;

                self.respond(
                    request,
                    &json,
                    "application/json",
                    200
                )?;
            },
            (Method::Get, "/" | "/index.html") => {
                self.respond(
                    request,
                    include_bytes!("../../webpage/index.html"),
                    "text/html; charset=utf-8",
                    200
                )?;
            },
            (Method::Get, "/index.js") => {
                self.respond(
                    request,
                    include_bytes!("../../webpage/index.js"),
                    "text/javascript; charset=utf-8",
                    200
                )?;
            },
            _ => {
                self.respond(
                    request,
                    include_bytes!("../../webpage/notfound.html"),
                    "text/html; charset=utf-8",
                    404
                )?;
            },
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("[+] serving at {}", self.server.server_addr());

        for request in self.server.incoming_requests() {
            self.serve(request)?;
        }

        Ok(())
    }
}

