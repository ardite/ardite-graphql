use iron::prelude::*;
use iron::headers::{Date, HttpDate, CacheControl, CacheDirective, ContentType};
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::modifiers::Header;
use iron::status::Status;
use iron::Handler;

static GRAPHQL_ENDPOINT: &'static str = "graphql";
static GRAPHIQL_ENDPOINT: &'static str = "graphiql";
static GRAPHIQL_HTML: &'static str = include_str!("../graphiql.html");

pub struct Server {
  pub enable_graphiql: bool
}

impl Handler for Server {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    let mut path = req.url.path.clone();

    res.set_mut(Header(CacheControl(vec![CacheDirective::NoCache, CacheDirective::Private])));

    if path.len() == 1 {
      // All of our endpoints are only one key.
      let endpoint = path.pop().unwrap();

      if endpoint == GRAPHQL_ENDPOINT {
        // If this is our GraphQL endpoint, let’s run the query.

        res.set_mut(Header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)]))));
        res.set_mut(Status::InternalServerError);
        res.set_mut("{}");

        return Ok(res);
      }

      if self.enable_graphiql && endpoint == GRAPHIQL_ENDPOINT {
        // If the developer has enabled GraphiQL and this is GraphiQL’s
        // endpoint, return the page.

        res.set_mut(Header(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![(Attr::Charset, Value::Utf8)]))));
        res.set_mut(Status::Ok);
        res.set_mut(GRAPHIQL_HTML);

        return Ok(res);
      }
    }

    res.set_mut(Header(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![(Attr::Charset, Value::Utf8)]))));
    res.set_mut(Status::NotFound);
    res.set_mut("Not Found");

    Ok(res)
  }
}
