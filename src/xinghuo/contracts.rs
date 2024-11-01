
mod contracts {
    
    #[derive(Debug)]
    pub struct Content
    {
        role: String,
        content: String,
    }

    pub struct Message
    {
        text: Vec<Content>,
    }

    pub struct Payload
    {
        message: Message,
    }

    pub struct Chat
    {
        domain: String,
        temperature: f64,
        max_tokens: i32,
    }

    pub struct Parameter
    {
        chat: Chat,
    }

    pub struct Header
    {
        app_id: String,
        uid: String,
    }

    pub struct JsonRequest
    {
        hearder: Header,
        parameter: Parameter,
        payload: Payload,
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