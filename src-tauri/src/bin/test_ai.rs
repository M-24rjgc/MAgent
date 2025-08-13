// 抑制开发期间的未使用代码警告
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::sync::Arc;
use tokio::sync::RwLock;
use magent_lib::config::AppConfig;
use magent_lib::models::{ModelManager, ChatRequest, ChatMessage};
use magent_lib::agents::clarifier::ClarifierAgent;
use magent_lib::core::data_structures::IdeaSeed;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🤖 MAgent AI Integration Test");
    
    // 创建配置
    let config = Arc::new(RwLock::new(AppConfig::new()));
    
    // 创建模型管理器
    let model_manager = Arc::new(ModelManager::new(config.clone()));
    
    // 测试基本聊天功能
    println!("\n1. 测试基本聊天功能...");
    let request = ChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            ChatMessage {
                role: "user".to_string(),
                content: "你好，请用中文回答：你是什么AI模型？".to_string(),
            }
        ],
        temperature: Some(0.7),
        max_tokens: Some(100),
    };
    
    match model_manager.chat(request).await {
        Ok(response) => {
            println!("✅ 聊天成功:");
            println!("   响应: {}", response.content);
            if let Some(usage) = response.usage {
                println!("   Token使用: {} prompt + {} completion = {} total", 
                    usage.prompt_tokens, usage.completion_tokens, usage.total_tokens);
            }
        }
        Err(e) => {
            println!("❌ 聊天失败: {}", e);
            println!("   这可能是因为未配置API密钥或网络问题");
        }
    }
    
    // 测试ClarifierAgent
    println!("\n2. 测试ClarifierAgent...");
    let clarifier = ClarifierAgent::new(config.clone(), model_manager.clone()).await?;
    
    let test_idea = IdeaSeed {
        raw_text: "我想做一个在线教育平台".to_string(),
        context_hints: vec!["教育".to_string(), "技术".to_string()],
        domain: Some("教育科技".to_string()),
    };
    
    match clarifier.analyze_and_clarify(&test_idea).await {
        Ok(clarification) => {
            println!("✅ 澄清分析成功:");
            println!("   开放槽位数量: {}", clarification.open_slots.len());
            println!("   问答对数量: {}", clarification.qa_pairs.len());
            println!("   置信度: {:.2}", clarification.confidence);
            
            for (i, qa) in clarification.qa_pairs.iter().enumerate() {
                println!("   问题 {}: {}", i + 1, qa.question);
                println!("     槽位类型: {:?}", qa.slot_type);
            }
        }
        Err(e) => {
            println!("❌ 澄清分析失败: {}", e);
        }
    }
    
    // 测试不同AI提供商
    println!("\n3. 测试不同AI提供商...");
    let providers = vec![
        ("OpenAI GPT", "gpt-3.5-turbo"),
        ("Claude", "claude-3-haiku-20240307"),
        ("DeepSeek", "deepseek-chat"),
    ];
    
    for (name, model) in providers {
        println!("\n   测试 {name}...");
        let request = ChatRequest {
            model: model.to_string(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: "请用一句话介绍你自己".to_string(),
                }
            ],
            temperature: Some(0.7),
            max_tokens: Some(50),
        };
        
        match model_manager.chat(request).await {
            Ok(response) => {
                println!("     ✅ {name}: {}", response.content.chars().take(50).collect::<String>());
            }
            Err(e) => {
                println!("     ❌ {name} 失败: {}", e);
            }
        }
    }
    
    println!("\n🎉 AI集成测试完成！");
    println!("\n📝 注意事项:");
    println!("   - 如果看到API错误，请确保在配置中设置了正确的API密钥");
    println!("   - 支持的提供商: OpenAI, Claude, DeepSeek");
    println!("   - 所有agent现在都支持真实AI驱动的分析");
    
    Ok(())
}
