use std::ops::{Deref, DerefMut};

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

pub struct MessageBuilder<Title, Repo> {
    title: Title,
    msgs: Vec<String>,
    repo: Repo,
}

impl MessageBuilder<(), ()> {
    pub fn new() -> Self {
        Self {
            title: (),
            msgs: Vec::new(),
            repo: (),
        }
    }
}

impl MessageBuilder<String, Repository> {
    pub fn build(self) -> Message {
        use std::fmt::Write;

        let mut buf = String::new();
        writeln!(buf, "### {}", self.title).expect("buf error");

        for m in self.msgs {
            writeln!(buf, "{}", m).expect("buf error");
        }

        writeln!(
            buf,
            "#### [{}/{}]({})",
            self.repo.owner, self.repo.name, self.repo.url
        )
        .expect("buf error");

        Message(buf)
    }
}

impl<Title, Repo> MessageBuilder<Title, Repo> {
    pub fn title(self, title: impl Into<String>) -> MessageBuilder<String, Repo> {
        MessageBuilder {
            title: title.into(),
            msgs: self.msgs,
            repo: self.repo,
        }
    }

    pub fn msgs(mut self, mut msgs: Vec<String>) -> MessageBuilder<Title, Repo> {
        self.msgs.append(&mut msgs);
        MessageBuilder {
            title: self.title,
            msgs: self.msgs,
            repo: self.repo,
        }
    }

    pub fn msg(mut self, msg: impl Into<String>) -> MessageBuilder<Title, Repo> {
        self.msgs.push(msg.into());
        MessageBuilder {
            title: self.title,
            msgs: self.msgs,
            repo: self.repo,
        }
    }

    pub fn repo(self, repo: Repository) -> MessageBuilder<Title, Repository> {
        MessageBuilder {
            title: self.title,
            msgs: self.msgs,
            repo: repo,
        }
    }
}

pub struct Repository {
    url: String,
    owner: String,
    name: String,
}

impl Repository {
    pub fn new(url: impl Into<String>, owner: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            name: name.into(),
            owner: owner.into(),
        }
    }
}
