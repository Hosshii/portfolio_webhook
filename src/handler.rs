use crate::builder::ContentBuilder;
use crate::builder::MessageBuilder;
use crate::error::MyError;
use crate::utils::prelude::*;
use crate::webhook::WebHook;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use github_webhook::event::{
    Event, IssueCommentEvent, IssuesAction, IssuesEvent, PullRequestEvent,
    PullRequestReviewCommentEvent, PullRequestReviewEvent, PushEvent,
};
use log::info;
use std::rc::Rc;

pub async fn webhook(
    mut req: HttpRequest,
    hook: web::Data<WebHook>,
    body: String,
) -> Result<HttpResponse, MyError> {
    let result = hook.parse_and_authenticate(&mut req, &body);

    match result {
        Ok(event) => {
            match event {
                Event::Issues(e) => {
                    issue_handler(&hook, e).await?;
                }
                _ => unimplemented!(),
            }

            Ok(HttpResponse::Ok().body("correctly parsed"))
        }
        Err(e) => {
            println!("{}", e);
            println!("{:?}", e);
            Ok(HttpResponse::BadRequest().body(e.to_string()))
        }
    }
}

async fn issue_handler(hook: &WebHook, event: IssuesEvent) -> Result<HttpResponse, MyError> {
    let event = Rc::new(EIssues(event));

    let title = ContentBuilder::new(Rc::clone(&event))
        .issue()
        .assignees()
        .build();
    let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

    let message = MessageBuilder::new().title(title).repo(repo).build();

    info!("msg {}", message);
    // let res = hook.post_message(&message).await?;

    // info!("{:?}", res);

    Ok(HttpResponse::Ok().body("successfully posted"))
}
