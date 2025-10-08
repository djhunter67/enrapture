use std::task::Poll;

use actix_web::{
    Error, HttpRequest, HttpResponse, Responder, get,
    http::{
        self, StatusCode,
        header::{ContentEncoding, ContentType},
    },
    web,
};
use askama::Template;
use futures::stream;
use tracing::{info, instrument};

use crate::endpoints::templates::Thumbnails;

use super::templates::IndexTemplate;

#[instrument(
    name = "Serving main page",
    level = "debug",
    target = "web_app_bloodhound",
    fields(samples = 25, title = "Home")
)]
#[get("/")]
pub async fn index() -> HttpResponse {
    info!("Serving main page");
    let version: &str = env!("CARGO_PKG_VERSION");

    // temporary hardcoded thumbnail
    // TODO: replace with dynamic content from database
    let thumbnails = vec![
        Thumbnails {
            title: "The Matrix",
            img_url: "https://m.media-amazon.com/images/I/51oBxmV-dML._AC_.jpg",
            alt: "The Matrix Image",
            description: "The Matrix is a groundbreaking sci-fi film that explores the nature of reality and human existence. Directed by the Wachowskis, it follows Neo, a hacker who discovers that the world he knows is a simulated reality created by intelligent machines. With stunning visual effects and thought-provoking themes, The Matrix has become a cult classic, inspiring countless discussions about technology, freedom, and identity."
        },
        Thumbnails {
            title: "The Terminator",
            img_url: "https://m.media-amazon.com/images/M/MV5BZmE0YzIxM2QtMGNlMi00MjRmLWE3MWMtOWQzMGVjMmU0YTFmXkEyXkFqcGc@._V1_FMjpg_UX1000_.jpg",
            alt: "The Terminator Image",
            description: "The Terminator is a classic sci-fi action film directed by James Cameron. It stars Arnold Schwarzenegger as a cyborg assassin sent from the future to kill Sarah Connor, whose unborn son will lead humanity in a war against machines. With its thrilling action sequences and iconic catchphrases, The Terminator has become a cultural phenomenon, spawning multiple sequels and influencing the genre for decades."
        },
        Thumbnails {
            title: "Honey, I Shrunk the Kids",
            img_url: "https://lumiere-a.akamaihd.net/v1/images/p_honeyishrunkthekids_19900_19125f54.jpeg",
            alt: "Honey, I Shrunk the Kids Image",
            description: "Honey, I Shrunk the Kids is a beloved family comedy film directed by Joe Johnston. It tells the story of an eccentric inventor who accidentally shrinks his children and their friends to miniature size, leading to a series of hilarious and adventurous escapades in their own backyard. With its imaginative premise and heartwarming themes, the movie has become a nostalgic favorite for audiences of all ages."

        },
        Thumbnails {
            title: "28Days Later",
            img_url: "https://m.media-amazon.com/images/M/MV5BM2I4NTI0ZGQtNGQ2ZC00ODIxLWI2N2QtMDBkNzI1NDhjYjE5XkEyXkFqcGc@._V1_.jpg",
            alt: "28 Days Later Image",
            description: "28 Days Later is a gripping post-apocalyptic horror film directed by Danny Boyle. The story follows a group of survivors navigating a world devastated by a deadly virus that turns humans into rage-fueled zombies. With its intense atmosphere, social commentary, and innovative cinematography, 28 Days Later has become a landmark in the zombie genre, influencing countless films and TV shows that followed."
        },
        Thumbnails {
            title: "The Old Guard",
            img_url: "https://m.media-amazon.com/images/M/MV5BNTk5Y2NjNjktMzJjMS00ODZkLThlYzUtNmFmNDdmZWNjNTc2XkEyXkFqcGc@._V1_.jpg",
            alt: "The Old Guard Image",
            description: "The Old Guard is an action-packed fantasy film directed by Gina Prince-Bythewood. It stars Charlize Theron as the leader of a group of immortal warriors who have protected humanity for centuries. When their secret is exposed, they must fight to survive against those who seek to exploit their powers. With its thrilling action sequences and exploration of themes like immortality and sacrifice, The Old Guard has garnered praise for its fresh take on the superhero genre."
        },
    ];

    let var_name = IndexTemplate {
        title: "Home",
        content: vec!["friendly", "messages"],
        version,
        thumbnails,
        location: "Jacobson, MN",
    };

    let rendered = var_name.render().expect("Failed to render template");

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered)
}

#[allow(clippy::future_not_send)]
pub async fn sse(_req: HttpRequest) -> impl Responder {
    let mut counter: usize = 5;

    // yeilds `data N` whrere N in [5; 1]
    let server_events = stream::poll_fn(move |_cx| -> Poll<Option<Result<web::Bytes, Error>>> {
        if counter == 0 {
            return Poll::Ready(None);
        }
        let payload = format!("data: {counter}\n\n");
        counter -= 1;
        Poll::Ready(Some(Ok(web::Bytes::from(payload))))
    });

    HttpResponse::build(StatusCode::OK)
        .insert_header((http::header::CONTENT_TYPE, "text/event-stream"))
        .insert_header(ContentEncoding::Identity)
        .streaming(server_events)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::pin::pin;

    use actix_web::{
        App,
        body::{self, MessageBody},
        test,
        web::{self, Bytes},
    };
    use futures::future;

    use super::{index, sse};

    #[actix_web::test]
    async fn test_get_index() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_is_html() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();

        let resp = test::call_and_read_body(&app, req).await;
        assert!(!resp.is_empty());
        let first_letters: Bytes = resp.slice(0..15).iter().copied().collect();
        let conv_str = std::str::from_utf8(&first_letters).unwrap();
        assert!(conv_str == "<!DOCTYPE html>");
    }

    #[actix_web::test]
    async fn test_stream_chunk() {
        let app = test::init_service(App::new().route("/sse", web::get().to(sse))).await;
        let req = test::TestRequest::get().uri("/sse").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let mut body = pin!(body);

        // first chunk
        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        println!("byte 1: {bytes:#?}");

        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 5\n\n")
        );

        // Second chunk
        let bytes = future::poll_fn(|cx| body.as_pin_mut().poll_next(cx)).await;
        println!("byte 2: {bytes:#?}");
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 4\n\n")
        );

        // Remaining part
        for i in 0..3 {
            let expected_data = format!("data: {}\n\n", 3 - i);
            let bytes = future::poll_fn(|cx| body.as_pin_mut().poll_next(cx)).await;
            println!("rem bytes: {bytes:#?}");
            assert_eq!(bytes.unwrap().unwrap(), web::Bytes::from(expected_data));
        }
    }

    #[actix_web::test]
    async fn test_stream_full_payload() {
        let app = test::init_service(App::new().route("/sse", web::get().to(sse))).await;
        let req = test::TestRequest::get().uri("/sse").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert_eq!(
            bytes.unwrap(),
            web::Bytes::from_static(b"data: 5\n\ndata: 4\n\ndata: 3\n\ndata: 2\n\ndata: 1\n\n")
        );
    }
}
