#[derive(Debug)]
enum Gender {
  Unspecified = 0,
  Female = 1,
  Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
  id: UserId,
  name: String,
  gender: Gender,
}

#[derive(Debug)]
struct Topic {
  id: TopicId,
  name: String,
  owner: UserId,
}

// 定义聊天室中可能发生的事件
#[derive(Debug)]
enum Event {
  Join((UserId, TopicId)),
  Leave((UserId, TopicId)),
  Message((UserId, TopicId, String)),
}

fn process_event(event: &Event) {
  match event {
      Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
      Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
      Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
  }
}

fn process_message(event: &Event) {
  if let Event::Message((_, _, msg)) = event {
      println!("boardcast : {}", msg);
  }
    
}

fn main() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };
    
    let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));
    let event4 = Event::Leave((alice.id, topic.id));
    
    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);

    println!("\nprocess_event");
    process_event(&event1);
    process_event(&event2);
    process_event(&event3);
    process_event(&event4);

    println!("\nprocess_message");
    process_message(&event1);
    process_message(&event2);
    process_message(&event3);
    process_message(&event4);
}