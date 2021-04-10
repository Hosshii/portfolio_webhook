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

impl MessageBuilder<String, String> {
    pub fn build(self) -> Message {
        use std::fmt::Write;

        let mut buf = String::new();
        writeln!(buf, "### {}", self.title).expect("buf error");
        writeln!(buf, "---").expect("buf error");

        for m in self.msgs {
            writeln!(buf, "{}", m).expect("buf error");
        }

        writeln!(buf, "##### {}", self.footer).expect("buf error");

        Message(buf)
    }
}

impl<Title, Footer> MessageBuilder<Title, Footer> {
    pub fn title(self, title: impl Into<String>) -> MessageBuilder<String, Footer> {
        MessageBuilder {
            title: title.into(),
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

    pub fn msg(mut self, msg: impl Into<String>) -> MessageBuilder<Title, Footer> {
        self.msgs.push(msg.into());
        MessageBuilder {
            title: self.title,
            msgs: self.msgs,
            footer: self.footer,
        }
    }

    pub fn repo(self, footer: String) -> MessageBuilder<Title, String> {
        MessageBuilder {
            title: self.title,
            msgs: self.msgs,
            footer: footer,
        }
    }
}

pub struct ContentBuilder<T> {
    event: Rc<T>,
    messages: Vec<String>,
}

impl<T> ContentBuilder<T>
where
    T: hidden::Marker,
{
    pub fn new(event: Rc<T>) -> Self {
        Self {
            event: event,
            messages: Vec::new(),
        }
    }

    pub fn msg(mut self, msg: impl Into<String>) -> Self {
        self.messages.push(msg.into());
        Self {
            event: self.event,
            messages: self.messages,
        }
    }

    pub fn build_with_separator(self, separator: &str) -> (String, ContentBuilder<T>) {
        let msg = self.messages.join(separator);
        (msg, self.clean())
    }

    pub fn build(self) -> String {
        let msg = self.messages.join(" ");
        msg
    }
    pub fn build_trim(self) -> String {
        let msg = self.messages.join("");
        msg
    }

    pub fn build_lines(self) -> String {
        let msg = self.messages.join("\n");
        msg
    }

    pub fn take(self) -> Vec<String> {
        self.messages
    }

    pub fn clean(mut self) -> ContentBuilder<T> {
        self.messages.clear();
        self
    }

    pub fn group(mut self, f: fn(_self: ContentBuilder<T>) -> String) -> ContentBuilder<T> {
        self.messages
            .push(f(ContentBuilder::new(Rc::clone(&self.event))));
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

impl<T> ContentBuilder<T>
where
    T: TAction,
{
    pub fn action(mut self) -> ContentBuilder<T> {
        self.messages.push(self.event.action().md());
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TCommit,
{
    pub fn commit(mut self) -> ContentBuilder<T> {
        let mut commits: Vec<String> = self.event.commits().into_iter().map(|c| c.md()).collect();
        self.messages.append(&mut commits);
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TComment,
{
    pub fn comment(mut self) -> ContentBuilder<T> {
        if let Some(comment) = self.event.comment() {
            self.messages.push(comment.comment());
        }
        self
    }
}

impl<T> ContentBuilder<T>
where
    T: TReview,
{
    pub fn review_url(mut self) -> ContentBuilder<T> {
        self.messages.push(self.event.review().url());
        self
    }

    pub fn review_md(mut self) -> ContentBuilder<T> {
        self.messages
            .push(self.event.review().review("Review Comment"));
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
