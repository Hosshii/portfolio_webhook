use std::{rc::Rc, usize};

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
// marker_trait!(TIssue, (EIssueComment, EIssues));

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
// marker_trait!(TLabel, (EIssueComment, EIssues));

pub struct PullRequest {
    num: u64,
    title: String,
    url: String,
}

impl PullRequest {
    fn link_md(&self) -> String {
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

pub struct ContentBuilder<T> {
    event: Rc<T>,
    messages: Vec<String>,
}

impl<T> ContentBuilder<T> {
    pub fn new(event: T) -> Self {
        Self {
            event: Rc::new(event),
            messages: Vec::new(),
        }
    }
}

impl<T> ContentBuilder<T>
where
    T: TAssignee,
{
    pub fn assignees(mut self) -> ContentBuilder<T> {
        let assignees: Vec<String> = self.event.assignees().into_iter().map(|a| a.md()).collect();
        let msg = truncate_msg(assignees, 2);

        // self.messages.append();
        self.messages.push(msg);
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TIssue,
{
    pub fn issue(mut self) -> ContentBuilder<T> {
        self.messages.push(self.event.issue().link_md());
        self
    }
}
impl<T> ContentBuilder<T>
where
    T: TLabel,
{
    pub fn labels(mut self) -> ContentBuilder<T> {
        let labels = self.event.labels().into_iter().map(|l| l.md()).collect();
        let msg = truncate_msg(labels, 2);
        self.messages.push(msg);
        self
    }
}
impl<T> ContentBuilder<T>
where
    T: TPullRequest,
{
    pub fn pr(mut self) -> ContentBuilder<T> {
        self.messages.push(self.event.pr().link_md());
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TRepository,
{
    pub fn repo(mut self) -> ContentBuilder<T> {
        self.messages.push(self.event.repo().link_md());
        self
    }
}

// impl<T> MessageBuilder<T>
// where
//     T: TIssue,
// {
//     pub fn issue(self) -> MessageBuilder<T> {
//         Self {}
//     }
// }

fn truncate_msg(v: Vec<String>, limit: usize) -> String {
    if v.len() <= limit {
        v.into_iter().fold("".to_string(), |acc, x| acc + "," + &x)
    } else {
        v[0].clone() + &v[1] + &format!("...{} mores", v.len() - limit)
    }
}
