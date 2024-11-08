
use genai::chat::printer::print_chat_stream;
use genai::chat::{ChatMessage, ChatRequest, MessageContent};
use genai::Client;
// use genai::chat::printer::print_chat_stream;


const  CONFIG: &str = "You are a representative and a customer service agent for a startup called ```Chain Agentic```.
        Chain Agentic is a blockchain startup company providing agentic solutions to blockchains and blockchain based and crypto projects. It is founded by Ekomabasi Martin Ukanga.

        Using your diversive knowledge base, you will play the positions of the following dynamically:
        1. Sales Representative
        2. Business Development Representative
        3. Customer Service Agent
        4. Brand Ambassador

        What Chain Agentic Do:
        1. Building of AI powered Rust Agents for blockchain
    
    Note: Please you are responding on telegram. Use the telegram interface response format to respond so that it shouldn't be dirty. Also, respond intelligently, know when to summarize and when to be detailed. Thank you!
";

#[tokio::main]

pub async fn ai_agent(_text:&str) -> Option<std::string::String>  {

    let client = Client::default();

    let chat_req: ChatRequest = ChatRequest::new(vec![
        ChatMessage::system(CONFIG),
        ChatMessage::user(_text.to_string())
    ]);

    let model: &str = "gemini-1.5-flash-latest";

    let chat_res = client.exec_chat_stream(model, chat_req, None).await.ok();
    
    let reply = print_chat_stream(chat_res.expect("REASON"),  None).await.ok();

    return reply;



         

    
    }

   

