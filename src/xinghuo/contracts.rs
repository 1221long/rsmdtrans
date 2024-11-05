pub const ROLE_USER: &str = "user";
pub mod request {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Content {
        pub role: String,
        pub content: String,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Message {
        pub text: Vec<Content>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Payload {
        pub message: Message,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Chat {
        pub domain: String,
        pub temperature: f64,
        pub max_tokens: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Parameter {
        pub chat: Chat,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Header {
        pub app_id: String,
        pub uid: String,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Body {
        pub header: Header,
        pub parameter: Parameter,
        pub payload: Payload,
    }
}

/* -- json example
{
  "header": {
    "code": 0,
    "message": "Success",
    "sid": "cht000b2fc1@dx192f9e970f3b8f2540",
    "status": 0
  },
  "payload": {
    "choices": {
      "status": 0,
      "seq": 0,
      "text": [
        {
          "content": "你好",
          "role": "assistant",
          "index": 0
        }
      ]
    }
  }
}
*/

pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Header {
        pub code: i8,
        pub message: String,
        pub sid: String,
        pub status: i8,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Payload {
        pub choices: Choices,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Choices {
        pub status: i8,
        pub seq: i8,
        pub text: Vec<Message>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Message {
        pub role: String,
        pub content: String,
        pub index: isize,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Body {
        pub header: Header,
        pub payload: Payload,
    }
}

/*
//构造请求体
public class JsonRequest
{
    public Header header { get; set; }
    public Parameter parameter { get; set; }
    public Payload payload { get; set; }
}

public class Header
{
    public string app_id { get; set; }
    public string uid { get; set; }
}

public class Parameter
{
    public Chat chat { get; set; }
}

public class Chat
{
    public string domain { get; set; }
    public double temperature { get; set; }
    public int max_tokens { get; set; }
}

public class Payload
{
    public Message message { get; set; }
}

public class Message
{
    public List<Content> text { get; set; }
}

public class Content
{
    public string role { get; set; }
    public string content { get; set; }
}
*/
