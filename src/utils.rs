use std::str::FromStr;
use std::usize;

use github_webhook::event::{
    self, Event, IssueCommentEvent, IssuesEvent, PullRequestEvent, PullRequestReviewCommentEvent,
    PullRequestReviewEvent, PushEvent,
};

macro_rules! newtype {
    ( $($id:ident, $ty:ty),* ) => {
        $(
            pub struct $id ($ty);
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

// macro_rules! marker_trait {
//     ( $($trait:ty, $($type:ty),*), *) => {
//         $(
//             $(
//                 impl $trait for $type {}
//             )*
//         )*
//     };
// }

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
// marker_trait!(TIssue, (EIssueComment, EIssues));

#[derive(Debug, Clone)]
pub struct Label {
    pub color: String,
    pub name: String,
    pub url: String,
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
// marker_trait!(TLabel, (EIssueComment, EIssues));

pub struct PullRequest {
    num: u64,
    title: String,
    url: String,
}

impl PullRequest {
    pub fn name_md(&self) -> String {
        format!("[{}]({})", self.title, self.url)
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

// marker_trait!(
//     TPullRequest,
//     (EPullRequest, EPullRequestReviewComment, EPullRequestReview)
// );

pub struct Assignee {
    pub name: String,
}
impl Assignee {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
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

// marker_trait!(
//     TAssignee,
//     (
//         EIssues,
//         EIssueComment,
//         EPullRequest,
//         EPullRequestReview,
//         EPullRequestReviewComment
//     )
// );

pub struct Repository {
    name: String,
    owner: String,
    url: String,
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

// marker_trait!(
//     TRepository,
//     (
//         EIssues,
//         EIssueComment,
//         EPullRequest,
//         EPullRequestReview,
//         EPullRequestReviewComment,
//         EPush
//     )
// );

pub struct MessageBuilder<T> {
    event: T,
    issue: Option<Issue>,
}

pub struct Issue {
    num: u64,
    title: String,
    url: String,
    assignees: Vec<String>,
}

// impl<T> MessageBuilder<T>
// where
//     T: TIssue,
// {
//     pub fn issue(self) -> MessageBuilder<T> {
//         Self {}
//     }
// }
