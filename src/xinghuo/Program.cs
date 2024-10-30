using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;
using System.Collections;
using Newtonsoft.Json;
using System.Net.WebSockets;
using System.Threading;
using Newtonsoft.Json.Linq;
using System.Text.Json;

/**
 * 星火认知大模型 WebAPI 接口调用示例 接口文档（必看）：https://www.xfyun.cn/doc/spark/Web.html
 * 错误码链接：https://www.xfyun.cn/doc/spark/%E6%8E%A5%E5%8F%A3%E8%AF%B4%E6%98%8E.html （code返回错误码时必看）
 * @author iflytek
 */
namespace Webiat
{
    class Program
    {
        static ClientWebSocket webSocket0;
        static CancellationToken cancellation;
        // 应用APPID（必须为webapi类型应用，并开通星火认知大模型授权）
        const string x_appid = "XXXXXXXX";
        // 接口密钥（webapi类型应用开通星火认知大模型后，控制台--我的应用---星火认知大模型---相应服务的apikey）
        const string api_secret = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
        // 接口密钥（webapi类型应用开通星火认知大模型后，控制台--我的应用---星火认知大模型---相应服务的apisecret）
        const string api_key = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

        static string hostUrl = "https://spark-api.xf-yun.com/v1.1/chat";
        async public static void Tasker()
        {
            
            string authUrl = GetAuthUrl();
            string url = authUrl.Replace("http://", "ws://").Replace("https://", "wss://");
            using (webSocket0 = new ClientWebSocket())
            {
                try
                {
                    await webSocket0.ConnectAsync(new Uri(url), cancellation);

                    JsonRequest request = new JsonRequest();
                    request.header = new Header()
                                    {
                                        app_id = x_appid,
                                        uid = "12345"
                                    };
                    request.parameter = new Parameter()
                                        {
                                            chat = new Chat()
                                            {
                                                domain = "general",//模型领域，默认为星火通用大模型
                                                temperature = 0.5,//温度采样阈值，用于控制生成内容的随机性和多样性，值越大多样性越高；范围（0，1）
                                                max_tokens = 1024,//生成内容的最大长度，范围（0，4096）
                                            }
                                        };
                    request.payload = new Payload()
                                        {
                                            message = new Message()
                                            {
                                                text = new List<Content>
                                                {
                                                    new Content() { role = "user", content = "你是谁" },
                                                    // new Content() { role = "assistant", content = "....." }, // AI的历史回答结果，这里省略了具体内容，可以根据需要添加更多历史对话信息和最新问题的内容。
                                                }
                                            }
                                        };

                    string jsonString = JsonConvert.SerializeObject(request);
                    //连接成功，开始发送数据
                    

                    var frameData2 = System.Text.Encoding.UTF8.GetBytes(jsonString.ToString());

                    
                    webSocket0.SendAsync(new ArraySegment<byte>(frameData2), WebSocketMessageType.Text, true, cancellation);
                   
                    // 接收流式返回结果进行解析
                    byte[] receiveBuffer = new byte[1024];
                    WebSocketReceiveResult result = await webSocket0.ReceiveAsync(new ArraySegment<byte>(receiveBuffer), cancellation);
                    String resp = "";
                    while (!result.CloseStatus.HasValue)
                    {
                        if (result.MessageType == WebSocketMessageType.Text)
                        {
                            string receivedMessage = Encoding.UTF8.GetString(receiveBuffer, 0, result.Count);
                            //将结果构造为json
                            
                            JObject jsonObj = JObject.Parse(receivedMessage);
                            int code = (int)jsonObj["header"]["code"];
                            
                            
                            if(0==code){
                                int status = (int)jsonObj["payload"]["choices"]["status"];
                                

                                JArray textArray = (JArray)jsonObj["payload"]["choices"]["text"];
                                string content = (string)textArray[0]["content"];
                                resp += content;

                                if(status != 2){
                                    Console.WriteLine($"已接收到数据： {receivedMessage}");
                                }
                                else{
                                    Console.WriteLine($"最后一帧： {receivedMessage}");
                                    int totalTokens = (int)jsonObj["payload"]["usage"]["text"]["total_tokens"];
                                    Console.WriteLine($"整体返回结果： {resp}");
                                    Console.WriteLine($"本次消耗token数： {totalTokens}");
                                    break;
                                }

                            }else{
                                Console.WriteLine($"请求报错： {receivedMessage}");
                            }
                                

                        }
                        else if (result.MessageType == WebSocketMessageType.Close)
                        {
                            Console.WriteLine("已关闭WebSocket连接");
                            break;
                        }

                        result = await webSocket0.ReceiveAsync(new ArraySegment<byte>(receiveBuffer), cancellation);
                    }
                }
                catch (Exception e)
                {
                    Console.WriteLine(e.Message);
                }
            }
        }
        // 返回code为错误码时，请查询https://www.xfyun.cn/document/error-code解决方案
        static string GetAuthUrl()
        {
            string date = DateTime.UtcNow.ToString("r");

            Uri uri = new Uri(hostUrl);
            StringBuilder builder = new StringBuilder("host: ").Append(uri.Host).Append("\n").//
                                    Append("date: ").Append(date).Append("\n").//
                                    Append("GET ").Append(uri.LocalPath).Append(" HTTP/1.1");

            string sha = HMACsha256(api_secret, builder.ToString());
            string authorization = string.Format("api_key=\"{0}\", algorithm=\"{1}\", headers=\"{2}\", signature=\"{3}\"", api_key, "hmac-sha256", "host date request-line", sha);
            //System.Web.HttpUtility.UrlEncode

            string NewUrl = "https://" + uri.Host + uri.LocalPath;

            string path1 = "authorization" + "=" + Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(authorization));
            date = date.Replace(" ", "%20").Replace(":", "%3A").Replace(",", "%2C");
            string path2 = "date" + "=" + date;
            string path3 = "host" + "=" + uri.Host;

            NewUrl = NewUrl + "?" + path1 + "&" + path2 + "&" + path3;
            return NewUrl;
        }




        public static string HMACsha256(string apiSecretIsKey, string buider)
        {
            byte[] bytes = System.Text.Encoding.UTF8.GetBytes(apiSecretIsKey);
            System.Security.Cryptography.HMACSHA256 hMACSHA256 = new System.Security.Cryptography.HMACSHA256(bytes);
            byte[] date = System.Text.Encoding.UTF8.GetBytes(buider);
            date = hMACSHA256.ComputeHash(date);
            hMACSHA256.Clear();

            return Convert.ToBase64String(date);

        }
        
        
        static void Main(string[] args)
        {
            Tasker();
            Console.ReadLine();
        }

    }
}



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