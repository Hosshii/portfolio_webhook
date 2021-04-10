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
                Event::Issues(e) => issue_handler(&hook, e).await,
                Event::IssueComment(e) => issue_comment_handler(&hook, e).await,
                Event::PullRequest(e) => pull_request_handler(&hook, e).await,
                Event::PullRequestReview(e) => pull_request_review_handler(&hook, e).await,
                Event::PullRequestReviewComment(e) => {
                    pull_request_review_comment_handler(&hook, e).await
                }
                Event::Push(e) => push_handler(&hook, e).await,
                Event::Ping(_) => ping_handler().await,
                _ => unimplemented!(),
            }

            // Ok(HttpResponse::Ok().body("correctly parsed"))
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
    let msg = ContentBuilder::new(Rc::clone(&event)).comment().build();
    let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

    let message = MessageBuilder::new()
        .title(title)
        .msg(msg)
        .repo(repo)
        .build();

    let _ = hook.post_message(message.as_ref()).await?;

    Ok(HttpResponse::Ok().body("successfully posted"))
}

async fn issue_comment_handler(
    hook: &WebHook,
    event: IssueCommentEvent,
) -> Result<HttpResponse, MyError> {
    use github_webhook::event::IssueCommentAction;
    match &event.action {
        IssueCommentAction::Created | IssueCommentAction::Deleted | IssueCommentAction::Edited => {
            let event = Rc::new(EIssueComment(event));

            let title = ContentBuilder::new(Rc::clone(&event))
                .issue()
                .action()
                .build();
            let msg = ContentBuilder::new(Rc::clone(&event)).comment().build();
            let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

            let message = MessageBuilder::new()
                .title(title)
                .msg(msg)
                .repo(repo)
                .build();

            let _ = hook.post_message(message.as_ref()).await?;

            Ok(HttpResponse::Ok().body("successfully posted"))
        }
    }
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

    let _ = hook.post_message(message.as_ref()).await?;

    Ok(HttpResponse::Ok().body("successfully posted"))
}

async fn pull_request_handler(
    hook: &WebHook,
    event: PullRequestEvent,
) -> Result<HttpResponse, MyError> {
    let event = Rc::new(EPullRequest(event));

    let title = ContentBuilder::new(Rc::clone(&event)).pr().action().build();
    let msg = ContentBuilder::new(Rc::clone(&event))
        .comment()
        .assignees()
        .labels()
        .build_lines();
    let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

    let message = MessageBuilder::new()
        .title(title)
        .msg(msg)
        .repo(repo)
        .build();

    let _ = hook.post_message(message.as_ref()).await?;

    Ok(HttpResponse::Ok().body("successfully posted"))
}

async fn pull_request_review_handler(
    hook: &WebHook,
    event: PullRequestReviewEvent,
) -> Result<HttpResponse, MyError> {
    match event.review.state.as_str() {
        "approved" | "commented" | "changes_requested" => {
            let event = Rc::new(EPullRequestReview(event));

            let title = ContentBuilder::new(Rc::clone(&event)).pr().action().build();
            let title = title + "Pull Request";

            let msg = ContentBuilder::new(Rc::clone(&event))
                .comment()
                .assignees()
                .build_lines();

            let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

            let message = MessageBuilder::new()
                .title(title)
                .msg(msg)
                .repo(repo)
                .build();

            let _ = hook.post_message(message.as_ref()).await?;

            Ok(HttpResponse::Ok().body("successfully posted"))
        }
        _ => Ok(HttpResponse::Ok().body("successfully accepted, but not posted")),
    }
}

async fn pull_request_review_comment_handler(
    hook: &WebHook,
    event: PullRequestReviewCommentEvent,
) -> Result<HttpResponse, MyError> {
    let event = Rc::new(EPullRequestReviewComment(event));

    let title = ContentBuilder::new(Rc::clone(&event)).pr().action().build();
    let msg = ContentBuilder::new(Rc::clone(&event))
        .comment()
        .assignees()
        .build_lines();
    let repo = ContentBuilder::new(Rc::clone(&event)).repo().build();

    let message = MessageBuilder::new()
        .title(title)
        .msg(msg)
        .repo(repo)
        .build();

    let _ = hook.post_message(message.as_ref()).await?;

    Ok(HttpResponse::Ok().body("successfully posted"))
}

async fn ping_handler() -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok().body("pong!"))
}
