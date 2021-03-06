use github_webhook::event::{
    self, IssueCommentEvent, IssuesEvent, PullRequestEvent, PullRequestReviewCommentEvent,
    PullRequestReviewEvent, PushEvent,
};

pub(crate) mod hidden {
    pub trait Marker {}
}

pub mod prelude {
    pub use super::{
        action::TAction, assignee::TAssignee, comment::TComment, commit::TCommit, issue::TIssue,
        label::TLabel, pull_request::TPullRequest, repository::TRepository, review::TReview,
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
    use super::{hidden::Marker, *};

    pub struct Issue {
        num: u64,
        title: String,
        url: String,
        _assignees: Vec<String>,
    }

    impl Issue {
        pub fn link_md(&self) -> String {
            format!("[#{} {}]({})", self.num, self.title, self.url)
        }
    }

    pub trait TIssue: Marker {
        fn issue(&self) -> Option<Issue>;
    }

    impl TIssue for EIssueComment {
        fn issue(&self) -> Option<Issue> {
            let issue = Issue {
                _assignees: self
                    .0
                    .issue
                    .assignees
                    .iter()
                    .map(|a| a.login.clone())
                    .collect(),
                num: self.0.issue.number,
                url: self.0.issue.html_url.clone(),
                title: self.0.issue.title.clone(),
            };
            Some(issue)
        }
    }

    impl TIssue for EIssues {
        fn issue(&self) -> Option<Issue> {
            let issue = Issue {
                _assignees: self
                    .0
                    .issue
                    .assignees
                    .iter()
                    .map(|a| a.login.clone())
                    .collect(),
                num: self.0.issue.number,
                url: self.0.issue.html_url.clone(),
                title: self.0.issue.title.clone(),
            };
            Some(issue)
        }
    }
}

pub mod label {
    use super::{hidden::Marker, *};
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
    pub trait TLabel: Marker {
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

    impl TLabel for EPullRequest {
        fn labels(&self) -> Vec<Label> {
            // todo
            Vec::new()
        }
    }
}

pub mod pull_request {
    use super::{hidden::Marker, *};

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

    pub trait TPullRequest: Marker {
        fn pr(&self) -> Option<PullRequest>;
    }

    impl TPullRequest for EPullRequest {
        fn pr(&self) -> Option<PullRequest> {
            let pr = PullRequest {
                num: self.number,
                title: self.0.pull_request.title.clone(),
                url: self.0.pull_request.url.clone(),
            };
            Some(pr)
        }
    }

    impl TPullRequest for EPullRequestReview {
        fn pr(&self) -> Option<PullRequest> {
            let pr = PullRequest {
                num: self.pull_request.number,
                title: self.pull_request.title.clone(),
                url: self.pull_request.html_url.clone(),
            };
            Some(pr)
        }
    }

    impl TPullRequest for EPullRequestReviewComment {
        fn pr(&self) -> Option<PullRequest> {
            let pr = PullRequest {
                num: self.pull_request.number,
                title: self.pull_request.title.clone(),
                url: self.pull_request.html_url.clone(),
            };
            Some(pr)
        }
    }
}

pub mod assignee {
    use super::{hidden::Marker, *};

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

    pub trait TAssignee: Marker {
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
    use super::{hidden::Marker, *};

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

    pub trait TRepository: Marker {
        fn repo(&self) -> Option<Repository>;
    }

    impl TRepository for EIssues {
        fn repo(&self) -> Option<Repository> {
            let repo = Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.login.clone(),
                url: self.repository.html_url.clone(),
            };
            Some(repo)
        }
    }

    impl TRepository for EIssueComment {
        fn repo(&self) -> Option<Repository> {
            let repo = Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.login.clone(),
                url: self.repository.html_url.clone(),
            };
            Some(repo)
        }
    }

    impl TRepository for EPullRequest {
        fn repo(&self) -> Option<Repository> {
            let repo = Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.login.clone(),
                url: self.repository.html_url.clone(),
            };
            Some(repo)
        }
    }

    impl TRepository for EPullRequestReview {
        fn repo(&self) -> Option<Repository> {
            let repo = Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.login.clone(),
                url: self.repository.html_url.clone(),
            };
            Some(repo)
        }
    }

    impl TRepository for EPullRequestReviewComment {
        fn repo(&self) -> Option<Repository> {
            let repo = Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.login.clone(),
                url: self.repository.html_url.clone(),
            };
            Some(repo)
        }
    }
    impl TRepository for EPush {
        fn repo(&self) -> Option<Repository> {
            let repo = Repository {
                name: self.repository.name.clone(),
                owner: self.repository.owner.name.clone(),
                url: self.repository.html_url.clone(),
            };
            Some(repo)
        }
    }
}

pub mod action {
    use super::{hidden::Marker, *};

    pub struct Action {
        action: String,
        sender: String,
        assignee: Option<String>,
    }

    impl Action {
        pub fn md(&self) -> String {
            if let Some(ref assignee) = self.assignee {
                format!("{} to `{}` by `{}`", self.action, assignee, self.sender)
            } else {
                format!("{} by `{}`", self.action, self.sender)
            }
        }
    }

    pub trait TAction: Marker {
        fn action(&self) -> Option<Action>;
    }

    impl TAction for EIssues {
        fn action(&self) -> Option<Action> {
            use github_webhook::event::IssuesAction::*;
            let assignee = match self.action {
                Assigned | Unassigned => self.issue.assignee.as_ref().map(|v| v.login.clone()),
                _ => None,
            };
            let action = Action {
                action: format!("{:?}", self.action),
                sender: self.sender.login.clone(),
                assignee,
            };
            Some(action)
        }
    }

    impl TAction for EIssueComment {
        fn action(&self) -> Option<Action> {
            let assignee = self.issue.assignee.as_ref().map(|v| v.login.clone());
            let action = Action {
                action: format!("{:?}", self.action),
                sender: self.sender.login.clone(),
                assignee,
            };
            Some(action)
        }
    }

    impl TAction for EPullRequest {
        fn action(&self) -> Option<Action> {
            use github_webhook::event::PullRequestAction::*;
            match self.action {
                Opened | Edited | Closed | Reopened | Assigned | Unassigned | ReviewRequested
                | ReviewRequestRemoved | ReadyForReview | Labeled | Unlabeled | Locked
                | Unlocked => {
                    let assignee = self.pull_request.assignee.as_ref().map(|v| v.login.clone());
                    let action = if let event::PullRequestAction::Closed = self.action {
                        if self.pull_request.merged {
                            "Merged".to_owned()
                        } else {
                            "Closed".to_owned()
                        }
                    } else {
                        format!("{:?}", self.action)
                    };
                    let action = Action {
                        action,
                        sender: self.sender.login.clone(),
                        assignee,
                    };
                    Some(action)
                }
                _ => None,
            }
        }
    }

    impl TAction for EPullRequestReview {
        fn action(&self) -> Option<Action> {
            let assignee = self.pull_request.assignee.as_ref().map(|v| v.login.clone());
            let action = Action {
                action: format!("{:?}", self.action),
                sender: self.sender.login.clone(),
                assignee,
            };
            Some(action)
        }
    }

    impl TAction for EPullRequestReviewComment {
        fn action(&self) -> Option<Action> {
            let assignee = self.pull_request.assignee.as_ref().map(|v| v.login.clone());
            let action = Action {
                action: format!("{:?}", self.action),
                sender: self.sender.login.clone(),
                assignee,
            };
            Some(action)
        }
    }

    impl TAction for EPush {
        fn action(&self) -> Option<Action> {
            if self.commits.len() < 1 {
                return None;
            }

            let action = if self.commits.len() > 1 {
                format!("{} commits pushed to {}", self.commits.len(), self._ref)
            } else {
                format!("{} commit pushed to {}", self.commits.len(), self._ref)
            };

            let action = Action {
                action: action,
                sender: self.sender.login.clone(),
                assignee: None,
            };
            Some(action)
        }
    }
}

pub mod commit {
    use super::{hidden::Marker, *};

    pub struct Commit {
        author: String,
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
                id, self.url, self.message, self.time, self.author
            )
        }
    }

    pub trait TCommit: Marker {
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
                    author: commit.author.name.clone(),
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

pub mod comment {
    use super::{hidden::Marker, *};

    pub struct Comment {
        comment: String,
        sender: String,
    }

    impl Comment {
        pub fn comment(&self) -> String {
            self.comment.clone()
        }

        pub fn comment_with_sender(&self) -> String {
            format!("{}: {}", self.sender, self.comment)
        }
    }

    pub trait TComment: Marker {
        fn comment(&self) -> Option<Comment>;
    }

    impl TComment for EIssues {
        fn comment(&self) -> Option<Comment> {
            self.issue.body.as_ref().map(|body| Comment {
                comment: body.clone(),
                sender: self.sender.login.clone(),
            })
        }
    }

    impl TComment for EIssueComment {
        fn comment(&self) -> Option<Comment> {
            Some(Comment {
                comment: self.comment.body.clone(),
                sender: self.sender.login.clone(),
            })
        }
    }

    impl TComment for EPullRequest {
        fn comment(&self) -> Option<Comment> {
            self.pull_request.body.as_ref().map(|body| Comment {
                comment: body.clone(),
                sender: self.sender.login.clone(),
            })
        }
    }

    impl TComment for EPullRequestReview {
        fn comment(&self) -> Option<Comment> {
            self.review.body.as_ref().map(|body| Comment {
                comment: body.clone(),
                sender: self.sender.login.clone(),
            })
        }
    }

    impl TComment for EPullRequestReviewComment {
        fn comment(&self) -> Option<Comment> {
            Some(Comment {
                comment: self.comment.body.clone(),
                sender: self.sender.login.clone(),
            })
        }
    }
}

pub mod review {
    use super::{hidden::Marker, *};

    pub struct Review {
        _comment: String,
        _reviewer: String,
        url: String,
    }

    impl Review {
        pub fn url(&self) -> String {
            self.url.clone()
        }

        pub fn review(&self, title: impl Into<String>) -> String {
            format!("[{}]({})", title.into(), self.url)
        }
    }

    pub trait TReview: Marker {
        fn review(&self) -> Option<Review>;
    }

    impl TReview for EPullRequestReviewComment {
        fn review(&self) -> Option<Review> {
            let review = Review {
                _comment: self.comment.body.clone(),
                _reviewer: self.sender.login.clone(),
                url: self.comment.html_url.clone(),
            };
            Some(review)
        }
    }
}
