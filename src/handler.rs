use crate::error::MyError;
use crate::message::{MessageBuilder, Repository};
use crate::webhook::WebHook;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use github_webhook::event::{
    Event, IssueCommentEvent, IssuesAction, IssuesEvent, PullRequestEvent,
    PullRequestReviewCommentEvent, PullRequestReviewEvent, PushEvent,
};

pub async fn webhook(
    mut req: HttpRequest,
    hook: web::Data<WebHook>,
    body: String,
) -> impl Responder {
    let result = hook.parse_and_authenticate(&mut req, &body);

    match result {
        Ok(event) => {
            println!("{:?}", event);

            HttpResponse::Ok().body("correctly parsed")
        }
        Err(e) => {
            println!("{}", e);
            println!("{:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}

async fn issue_handler(hook: &WebHook, event: IssuesEvent) -> Result<HttpResponse, MyError> {
    use IssuesAction::*;

    let issue_name = format!(
        "[#{} {}]({})",
        event.issue.number, event.issue.title, event.issue.html_url
    );

    let title = match &event.action {
        Assigned | Unassigned => {
            format!(
                "Issue {} {:?} to `{}` by `{}`",
                issue_name,
                event.action,
                event.issue.assignee.ok_or(MyError::ReadPayloadError)?.login,
                event.sender.login
            )
        }
        action => {
            format!(
                "Issue {} {:?} by `{}`",
                issue_name, action, event.sender.login
            )
        }
    };

    let repo = Repository::new(
        &event.repository.html_url,
        &event.repository.owner.login,
        &event.repository.name,
    );
    let message = MessageBuilder::new()
        .repo(repo)
        .title(&title)
        .msg(title)
        .build();

    hook.post_message(&message).await?;

    Ok(HttpResponse::Ok().body("successfully posted"))
}
