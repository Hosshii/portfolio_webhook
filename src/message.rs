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

        for m in self.msgs {
            writeln!(buf, "{}", m).expect("buf error");
        }

        writeln!(buf, "#### {}", self.footer).expect("buf error");

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
