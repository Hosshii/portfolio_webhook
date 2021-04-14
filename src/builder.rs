use crate::utils::{hidden, prelude::*};
use std::{
    fmt,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct Message(String);

impl Deref for Message {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl AsRef<String> for Message {
    fn as_ref(&self) -> &String {
        self
    }
}

pub struct MessageBuilder<Title, Footer> {
    title: Title,
    msgs: Vec<String>,
    footer: Footer,
}

impl MessageBuilder<(), ()> {
    pub fn new() -> Self {
        Self {
            title: (),
            msgs: Vec::new(),
            footer: (),
        }
    }
}

impl MessageBuilder<Option<String>, Option<String>> {
    pub fn build(self) -> Option<Message> {
        use std::fmt::Write;

        if let (Some(title), Some(footer)) = (self.title, self.footer) {
            let mut buf = String::new();
            writeln!(buf, "### {}", title).expect("buf error");
            writeln!(buf, "---").expect("buf error");

            for m in self.msgs {
                writeln!(buf, "{}", m).expect("buf error");
            }

            writeln!(buf, "##### {}", footer).expect("buf error");

            Some(Message(buf))
        } else {
            None
        }
    }
}

impl<Title, Footer> MessageBuilder<Title, Footer> {
    pub fn title(self, title: Option<impl Into<String>>) -> MessageBuilder<Option<String>, Footer> {
        MessageBuilder {
            title: title.map(|v| v.into()),
            msgs: self.msgs,
            footer: self.footer,
        }
    }

    pub fn msgs(mut self, mut msgs: Vec<String>) -> MessageBuilder<Title, Footer> {
        self.msgs.append(&mut msgs);
        MessageBuilder {
            title: self.title,
            msgs: self.msgs,
            footer: self.footer,
        }
    }

    pub fn msg(mut self, msg: Option<String>) -> MessageBuilder<Title, Footer> {
        if let Some(msg) = msg {
            self.msgs.push(msg.into());
        }
        MessageBuilder {
            title: self.title,
            msgs: self.msgs,
            footer: self.footer,
        }
    }

    pub fn repo(self, footer: Option<String>) -> MessageBuilder<Title, Option<String>> {
        MessageBuilder {
            title: self.title,
            msgs: self.msgs,
            footer,
        }
    }
}

pub struct ContentBuilder<T> {
    event: Rc<T>,
    messages: Option<Vec<String>>,
}

impl<T> ContentBuilder<T>
where
    T: hidden::Marker,
{
    pub fn new(event: Rc<T>) -> Self {
        Self {
            event: event,
            messages: Some(Vec::new()),
        }
    }

    fn and_then<F: FnOnce(&mut Vec<String>) -> ()>(&mut self, f: F) {
        match self.messages.as_mut() {
            Some(v) => f(v),
            None => {}
        }
    }

    fn push_msg(&mut self, msg: impl Into<String>) {
        self.and_then(|v| v.push(msg.into()));
    }

    fn push_some_msg(&mut self, msg: Option<String>) {
        if let Some(msg) = msg {
            self.push_msg(msg);
        }
    }

    fn push_msg_or_none(&mut self, msg: Option<String>) {
        if let Some(msg) = msg {
            self.push_msg(msg);
        } else {
            self.messages = None;
        }
    }

    fn append_msg(&mut self, mut msgs: Vec<String>) {
        self.and_then(|v| v.append(&mut msgs));
    }

    fn none(&mut self) {
        self.messages = None;
    }

    pub fn msg(mut self, msg: impl Into<String>) -> Self {
        self.and_then(|v| v.push(msg.into()));

        Self {
            event: self.event,
            messages: self.messages,
        }
    }

    pub fn build_with_separator(self, separator: &str) -> Option<String> {
        let msg = self.messages.map(|v| v.join(separator));
        msg
    }

    pub fn build(self) -> Option<String> {
        let msg = self.messages.map(|v| v.join(" "));
        msg
    }
    pub fn build_trim(self) -> Option<String> {
        let msg = self.messages.map(|v| v.join(""));
        msg
    }

    pub fn build_lines(self) -> Option<String> {
        let msg = self.messages.map(|v| v.join("\n"));
        msg
    }

    // pub fn take(self) -> Vec<String> {
    //     self.messages
    // }

    pub fn clean(mut self) -> ContentBuilder<T> {
        self.and_then(|v| v.clear());
        self
    }

    pub fn group(mut self, f: fn(_self: ContentBuilder<T>) -> String) -> ContentBuilder<T> {
        let t = f(ContentBuilder::new(Rc::clone(&self.event)));
        self.and_then(|v| v.push(t));
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TAssignee,
{
    pub fn assignees(mut self) -> ContentBuilder<T> {
        let assignees: Vec<String> = self.event.assignees().into_iter().map(|a| a.md()).collect();
        let msg = truncate_msg(assignees, 2);

        self.and_then(|v| v.push(msg));
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TIssue,
{
    pub fn issue(mut self) -> ContentBuilder<T> {
        match self.event.issue() {
            Some(i) => self.and_then(|v| v.push(i.link_md())),
            None => self.messages = None,
        }
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
        self.push_msg(msg);
        self
    }
}
impl<T> ContentBuilder<T>
where
    T: TPullRequest,
{
    pub fn pr(mut self) -> ContentBuilder<T> {
        let msg = self.event.pr().map(|v| v.link_md());
        self.push_some_msg(msg);
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TRepository,
{
    pub fn repo(mut self) -> ContentBuilder<T> {
        let msg = self.event.repo().map(|v| v.link_md());
        self.push_msg_or_none(msg);
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TAction,
{
    pub fn action(mut self) -> ContentBuilder<T> {
        let msg = self.event.action().map(|v| v.md());
        self.push_msg_or_none(msg);
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TCommit,
{
    pub fn commit(mut self) -> ContentBuilder<T> {
        let commits: Vec<String> = self.event.commits().into_iter().map(|c| c.md()).collect();
        if commits.len() < 1 {
            self.none();
        } else {
            self.append_msg(commits);
        }
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TComment,
{
    pub fn comment(mut self) -> ContentBuilder<T> {
        let comment = self.event.comment().map(|v| v.comment());
        self.push_some_msg(comment);
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TReview,
{
    pub fn review_url(mut self) -> ContentBuilder<T> {
        let url = self.event.review().map(|v| v.url());
        self.push_some_msg(url);
        self
    }

    pub fn review_md(mut self) -> ContentBuilder<T> {
        let msg = self.event.review().map(|v| v.review("Review Comment"));
        self.push_some_msg(msg);
        self
    }
}

fn truncate_msg(v: Vec<String>, limit: usize) -> String {
    if v.len() <= limit {
        v.into_iter().fold("".to_string(), |acc, x| acc + "," + &x)
    } else {
        v[0].clone() + &v[1] + &format!("...{} mores", v.len() - limit)
    }
}
