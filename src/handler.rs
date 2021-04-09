use crate::builder::ContentBuilder;
use crate::builder::MessageBuilder;
use crate::error::MyError;
use crate::utils::prelude::*;
use crate::webhook::WebHook;
use actix_web::{web, HttpRequest, HttpResponse};
use github_webhook::event::{
    Event, IssueCommentEvent, IssuesEvent, PullRequestEvent, PullRequestReviewCommentEvent,
    PullRequestReviewEvent, PushEvent,
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
        .action()
        .build();
    let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

    let message = MessageBuilder::new().title(title).repo(repo).build();

    info!("msg {}", message);
    // let res = hook.post_message(&message).await?;

    // info!("{:?}", res);

    Ok(HttpResponse::Ok().body("successfully posted"))
}

async fn issue_comment_handler(
    hook: &WebHook,
    event: IssueCommentEvent,
) -> Result<HttpResponse, MyError> {
    let event = Rc::new(EIssueComment(event));

    let title = ContentBuilder::new(Rc::clone(&event))
        .issue()
        .action()
        .build();
    let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

    let message = MessageBuilder::new().title(title).repo(repo).build();

    info!("msg {}", message);
    // let res = hook.post_message(&message).await?;

    // info!("{:?}", res);

    Ok(HttpResponse::Ok().body("successfully posted"))
}

async fn push_handler(hook: &WebHook, event: PushEvent) -> Result<HttpResponse, MyError> {
    let event = Rc::new(EPush(event));

    let title = ContentBuilder::new(Rc::clone(&event)).action().build();
    let msg = ContentBuilder::new(Rc::clone(&event))
        .commit()
        .build_lines();
    let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

    let message = MessageBuilder::new()
        .title(title)
        .msg(msg)
        .repo(repo)
        .build();

    info!("msg {}", message);
    // let res = hook.post_message(&message).await?;

    // info!("{:?}", res);

    Ok(HttpResponse::Ok().body("successfully posted"))
}

async fn pull_request_handler(
    hook: &WebHook,
    event: PullRequestEvent,
) -> Result<HttpResponse, MyError> {
    let event = Rc::new(EPullRequest(event));

    let title = ContentBuilder::new(Rc::clone(&event)).pr().action().build();
    let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

    let message = MessageBuilder::new().title(title).repo(repo).build();

    info!("msg {}", message);
    // let res = hook.post_message(&message).await?;

    // info!("{:?}", res);

    Ok(HttpResponse::Ok().body("successfully posted"))
}

async fn pull_request_review_handler(
    hook: &WebHook,
    event: PullRequestReviewEvent,
) -> Result<HttpResponse, MyError> {
    let event = Rc::new(EPullRequestReview(event));

    let title = ContentBuilder::new(Rc::clone(&event)).pr().action().build();
    let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

    let message = MessageBuilder::new().title(title).repo(repo).build();

    info!("msg {}", message);
    // let res = hook.post_message(&message).await?;

    // info!("{:?}", res);

    Ok(HttpResponse::Ok().body("successfully posted"))
}

async fn pull_request_review_comment_handler(
    hook: &WebHook,
    event: PullRequestReviewCommentEvent,
) -> Result<HttpResponse, MyError> {
    let event = Rc::new(EPullRequestReviewComment(event));

    let title = ContentBuilder::new(Rc::clone(&event)).pr().action().build();
    let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

    let message = MessageBuilder::new().title(title).repo(repo).build();

    info!("msg {}", message);
    // let res = hook.post_message(&message).await?;

    // info!("{:?}", res);

    Ok(HttpResponse::Ok().body("successfully posted"))
}
