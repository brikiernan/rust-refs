#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let message1 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: "10:00 AM".to_string(),
    };

    let message2 = ChatMessage {
        content: DigitalContent::VideoFile,
        time: "10:05 AM".to_string(),
    };

    message1.consume_entertainment();
    message2.consume_entertainment();

    println!("Message 1 time: {}", message1.retrieve_time());
    println!("Message 2 time: {}", message2.retrieve_time());
    let message3 = ChatMessage {
        content: "Hello, World!",
        time: "10:10 AM".to_string(),
    };
    println!("Message 3 time: {}", message3.retrieve_time());
}
