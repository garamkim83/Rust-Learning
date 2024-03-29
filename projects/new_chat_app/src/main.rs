// 출처
// https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/chat/src/main.rs
// https://www.youtube.com/watch?v=NS9Dh63i_Q4

//같은 컴퓨터의 여러 브라우저 창을 통해 채팅할 수 있는 사이트

#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};
/*
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}
*/

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

struct Message {
    #[field(validate=len(..30))]
    pub room: String,
    #[field(validate=len(..30))]
    pub username: String,
    pub message: String,
}

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // A send 'fails' if there are no active receivers.
    let _res = queue.send(form.into_inner());
}

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop{
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("static")))
}
