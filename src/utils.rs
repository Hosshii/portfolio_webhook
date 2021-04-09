use std::{rc::Rc, usize};

use github_webhook::event::{
    self, Event, IssueCommentEvent, IssuesEvent, PullRequestEvent, PullRequestReviewCommentEvent,
    PullRequestReviewEvent, PushEvent,
};

pub(crate) mod hidden {
    pub trait Marker {}
}

pub mod prelude {
    pub use super::{
        action::TAction, assignee::TAssignee, commit::TCommit, issue::TIssue, label::TLabel,
        pull_request::TPullRequest, repository::TRepository,
    };
    pub use super::{
        EIssueComment, EIssues, EPullRequest, EPullRequestReview, EPullRequestReviewComment, EPush,
    };
}

macro_rules! newtype {
    ( $($id:ident, $ty:ty),* ) => {
        $(
            pub struct $id (pub $ty);
            impl std::ops::Deref for $id {
                type Target = $ty;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for $id {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl hidden::Marker for $id {}
        )*
    };
}

newtype! {
    EIssueComment, IssueCommentEvent,
    EIssues, IssuesEvent,
    EPullRequest, PullRequestEvent,
    EPullRequestReviewComment,  PullRequestReviewCommentEvent,
    EPullRequestReview, PullRequestReviewEvent,
    EPush, PushEvent
}

pub mod issue {
    use super::*;

    pub struct Issue {
        num: u64,
        title: String,
        url: String,
        assignees: Vec<String>,
    }

    impl Issue {
        pub fn link_md(&self) -> String {
            format!("[#{} {}]({})", self.num, self.title, self.url)
        }
    }

    pub trait TIssue {
        fn issue(&self) -> Issue;
    }

    impl TIssue for EIssueComment {
        fn issue(&self) -> Issue {
            Issue {
                assignees: self
                    .0
                    .issue
                    .assignees
                    .iter()
                    .map(|a| a.login.clone())
                    .collect(),
                num: self.0.issue.number,
                url: self.0.issue.html_url.clone(),
                title: self.0.issue.title.clone(),
            }
        }
    }

    impl TIssue for EIssues {
        fn issue(&self) -> Issue {
            Issue {
                assignees: self
                    .0
                    .issue
                    .assignees
                    .iter()
                    .map(|a| a.login.clone())
                    .collect(),
                num: self.0.issue.number,
                url: self.0.issue.html_url.clone(),
                title: self.0.issue.title.clone(),
            }
        }
    }
}

pub mod label {

    use super::issue::TIssue;
    use super::*;
    #[derive(Debug, Clone)]
    pub struct Label {
        pub color: String,
        pub name: String,
        pub url: String,
    }

    impl Label {
        pub fn md(&self) -> String {
            format!("[{}]({})", self.name, self.url)
        }
    }
    pub trait TLabel: TIssue {
        fn labels(&self) -> Vec<Label>;
    }

    impl From<&event::Label> for Label {
        fn from(from: &event::Label) -> Self {
            Label {
                color: from.color.clone(),
                name: from.name.clone(),
                url: from.url.clone(),
            }
        }
    }

    impl TLabel for EIssueComment {
        fn labels(&self) -> Vec<Label> {
            self.issue.labels.iter().map(|l| l.into()).collect()
        }
    }

    impl TLabel for EIssues {
        fn labels(&self) -> Vec<Label> {
            self.0.issue.labels.iter().map(|l| l.into()).collect()
        }
    }
}

pub mod pull_request {
    use super::*;

    pub struct PullRequest {
        num: u64,
        title: String,
        url: String,
    }

    impl PullRequest {
        pub fn link_md(&self) -> String {
            format!("[#{} {}]({})", self.num, self.title, self.url)
        }
    }

    pub trait TPullRequest {
        fn pr(&self) -> PullRequest;
    }

    impl TPullRequest for EPullRequest {
        fn pr(&self) -> PullRequest {
            PullRequest {
                num: self.number,
                title: self.0.pull_request.title.clone(),
                url: self.0.pull_request.url.clone(),
            }
        }
    }

    impl TPullRequest for EPullRequestReview {
        fn pr(&self) -> PullRequest {
            PullRequest {
                num: self.pull_request.number,
                title: self.pull_request.title.clone(),
                url: self.pull_request.url.clone(),
            }
        }
    }

    impl TPullRequest for EPullRequestReviewComment {
        fn pr(&self) -> PullRequest {
            PullRequest {
                num: self.pull_request.number,
                title: self.pull_request.title.clone(),
                url: self.pull_request.url.clone(),
            }
        }
    }
}

pub mod assignee {
    use super::*;

    pub struct Assignee {
        pub name: String,
    }
    impl Assignee {
        pub fn new(name: impl Into<String>) -> Self {
            Self { name: name.into() }
        }

        pub fn md(&self) -> String {
            self.name.clone()
        }
    }

    pub trait TAssignee {
        fn assignees(&self) -> Vec<Assignee>;
    }

    impl TAssignee for EIssues {
        fn assignees(&self) -> Vec<Assignee> {
            self.issue
                .assignees
                .iter()
                .map(|a| Assignee::new(a.login.clone()))
                .collect()
        }
    }

    impl TAssignee for EIssueComment {
        fn assignees(&self) -> Vec<Assignee> {
            self.issue
                .assignees
                .iter()
                .map(|a| Assignee::new(a.login.clone()))
                .collect()
        }
    }

    impl TAssignee for EPullRequest {
        fn assignees(&self) -> Vec<Assignee> {
            self.pull_request
                .assignee
                .iter()
                .map(|a| Assignee::new(a.login.clone()))
                .collect()
        }
    }

    impl TAssignee for EPullRequestReview {
        fn assignees(&self) -> Vec<Assignee> {
            self.pull_request
                .assignee
                .iter()
                .map(|a| Assignee::new(a.login.clone()))
                .collect()
        }
    }

    impl TAssignee for EPullRequestReviewComment {
        fn assignees(&self) -> Vec<Assignee> {
            self.pull_request
                .assignee
                .iter()
                .map(|a| Assignee::new(a.login.clone()))
                .collect()
        }
    }
}

pub mod repository {
    use super::*;

    pub struct Repository {
        name: String,
        owner: String,
        url: String,
    }

    impl Repository {
        pub fn link_md(&self) -> String {
            format!("[{}/{}]({})", self.owner, self.name, self.url)
        }
    }

    pub trait TRepository {
        fn repo(&self) -> Repository;
    }

    impl TRepository for EIssues {
        fn repo(&self) -> Repository {
            Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.login.clone(),
                url: self.repository.url.clone(),
            }
        }
    }

    impl TRepository for EIssueComment {
        fn repo(&self) -> Repository {
            Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.login.clone(),
                url: self.repository.url.clone(),
            }
        }
    }

    impl TRepository for EPullRequest {
        fn repo(&self) -> Repository {
            Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.login.clone(),
                url: self.repository.url.clone(),
            }
        }
    }

    impl TRepository for EPullRequestReview {
        fn repo(&self) -> Repository {
            Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.login.clone(),
                url: self.repository.url.clone(),
            }
        }
    }

    impl TRepository for EPullRequestReviewComment {
        fn repo(&self) -> Repository {
            Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.login.clone(),
                url: self.repository.url.clone(),
            }
        }
    }
    impl TRepository for EPush {
        fn repo(&self) -> Repository {
            Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.name.clone(),
                url: self.repository.url.clone(),
            }
        }
    }
}

pub mod action {
    use super::*;

    pub struct Action {
        action: String,
        sender: String,
        assignee: Option<String>,
    }

    impl Action {
        pub fn md(&self) -> String {
            if let Some(ref assignee) = self.assignee {
                format!("{} to {} by {}", self.action, assignee, self.sender)
            } else {
                format!("{} by {}", self.action, self.sender)
            }
        }
    }

    pub trait TAction {
        fn action(&self) -> Action;
    }

    impl TAction for EIssues {
        fn action(&self) -> Action {
            let assignee = self.issue.assignee.as_ref().map(|v| v.login.clone());
            Action {
                action: format!("{:?}", self.action),
                sender: self.sender.login.clone(),
                assignee,
            }
        }
    }

    impl TAction for EIssueComment {
        fn action(&self) -> Action {
            let assignee = self.issue.assignee.as_ref().map(|v| v.login.clone());
            Action {
                action: format!("{:?}", self.action),
                sender: self.sender.login.clone(),
                assignee,
            }
        }
    }

    impl TAction for EPullRequest {
        fn action(&self) -> Action {
            let assignee = self.pull_request.assignee.as_ref().map(|v| v.login.clone());
            Action {
                action: format!("{:?}", self.action),
                sender: self.sender.login.clone(),
                assignee,
            }
        }
    }

    impl TAction for EPullRequestReview {
        fn action(&self) -> Action {
            let assignee = self.pull_request.assignee.as_ref().map(|v| v.login.clone());
            Action {
                action: format!("{:?}", self.action),
                sender: self.sender.login.clone(),
                assignee,
            }
        }
    }

    impl TAction for EPullRequestReviewComment {
        fn action(&self) -> Action {
            let assignee = self.pull_request.assignee.as_ref().map(|v| v.login.clone());
            Action {
                action: format!("{:?}", self.action),
                sender: self.sender.login.clone(),
                assignee,
            }
        }
    }

    impl TAction for EPush {
        fn action(&self) -> Action {
            Action {
                action: "Pushed".to_string(),
                sender: self.sender.login.clone(),
                assignee: None,
            }
        }
    }
}

pub mod commit {
    use super::*;

    pub struct Commit {
        committer: String,
        time: String,
        message: String,
        id: String,
        url: String,
    }

    impl Commit {
        pub fn md(&self) -> String {
            let id: String = self.id.chars().take(7).collect();
            format!(
                "[{}]({}) - {} {} {}",
                id, self.url, self.message, self.time, self.committer
            )
        }
    }

    pub trait TCommit {
        fn commits(&self) -> Vec<Commit>;
    }

    impl TCommit for EPush {
        fn commits(&self) -> Vec<Commit> {
            use chrono::DateTime;

            let mut commits = Vec::with_capacity(self.commits.len());
            for commit in &self.commits {
                let time = DateTime::parse_from_rfc3339(&commit.timestamp)
                    .map(|time| time.format("%a %b %e %T %Y %z").to_string())
                    .unwrap_or("time parse error".to_string());
                commits.push(Commit {
                    committer: commit.committer.name.clone(),
                    time,
                    message: commit.message.clone(),
                    id: commit.id.clone(),
                    url: commit.url.clone(),
                });
            }
            commits
        }
    }
}
