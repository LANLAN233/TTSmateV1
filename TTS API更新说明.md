TTSmate V1 TTS API更新说明

更新概述
根据最新的ttsServer.md文档，TTS服务器使用Gradio框架，API调用方式与原设计有所不同。本文档说明了API调用逻辑的更新内容。

API架构变更

原设计 vs 新架构：
- 原设计：基于RESTful API的直接调用
- 新架构：基于Gradio API的两步调用模式

Gradio API调用模式：
1. POST请求到 /gradio_api/call/{endpoint} 获取EVENT_ID
2. GET请求到 /gradio_api/call/{endpoint}/{EVENT_ID} 获取结果

已确认的API端点

根据ttsServer.md文档，当前可用的API端点包括：

1. 语音选择 (/on_voice_change)
   - 参数：vocie_selection (string)
   - 可选值：Default, Timbre1-9
   - 返回：音频种子数值 (float)

2. 音频种子生成 (/generate_seed)
   - 参数：无
   - 返回：音频种子数值 (float)

3. 文本种子生成 (/generate_seed_1)
   - 参数：无
   - 返回：文本种子数值 (float)

4. 音频种子变更 (/on_audio_seed_change)
   - 参数：audio_seed_input (float)
   - 返回：说话人嵌入字符串 (str)

5. 样本音频上传 (/on_upload_sample_audio)
   - 参数：sample_audio_input (文件)
   - 返回：处理结果字符串 (str)

6. DVAE系数配置 (/reload_chat)
   - 参数：coef (string)
   - 返回：配置结果字符串 (str)

7. 中断生成 (/interrupt_generate)
   - 参数：无
   - 返回：中断结果

缺失的关键API

重要发现：当前API文档中缺少最关键的文本转语音生成端点！

预期需要的API端点：
- 文本输入端点：接收要转换的文本内容
- 语音生成端点：执行实际的TTS转换
- 音频输出端点：返回生成的音频数据

这些端点在当前的ttsServer.md文档中未找到，可能的原因：
1. API文档不完整
2. 端点名称不明显
3. 需要特定的调用序列

代码更新内容

已更新的文件：

1. API文档.md
   - 更新了7.1 TTS服务器API接口部分
   - 添加了基于Gradio的API调用示例
   - 使用Python gradio_client库的调用方式

2. Rust开发指南.md
   - 更新了TTSClient的synthesize方法
   - 添加了Gradio API的辅助方法
   - 新增了GradioRequest、GradioEventResponse等数据结构

实现策略

当前实现策略：
1. 实现已知API端点的调用逻辑
2. 建立Gradio API的通用调用框架
3. 为缺失的TTS生成端点预留接口
4. 添加详细的日志和错误处理

代码示例：

Rust实现框架：
```rust
impl TTSClient {
    pub async fn synthesize(&self, text: &str, options: Option<SynthesizeOptions>) -> Result<AudioData, TTSError> {
        // 1. 设置语音类型
        let voice = options.as_ref().and_then(|o| o.voice.clone()).unwrap_or_else(|| self.config.default_voice.clone());
        let audio_seed = self.setup_voice_and_seeds(&voice).await?;
        
        // 2. TODO: 调用实际的文本转语音生成API
        // 当前缺少这个关键端点
        
        // 3. 临时返回空数据
        Ok(AudioData { data: vec![], format: AudioFormat::Wav, duration: Duration::from_secs(0), sample_rate: 44100 })
    }
    
    async fn setup_voice_and_seeds(&self, voice: &str) -> Result<f64, TTSError> {
        // 调用已知的配置API端点
        // ...
    }
    
    async fn call_gradio_api(&self, endpoint: &str, data: Vec<serde_json::Value>) -> Result<String, TTSError> {
        // Gradio API通用调用方法
        // ...
    }
}
```

Python调用示例：
```python
from gradio_client import Client

client = Client("http://192.168.11.153:8080/")

# 设置语音类型
result = client.predict(vocie_selection="Default", api_name="/on_voice_change")
audio_seed = result

# 生成种子
seed_result = client.predict(api_name="/generate_seed")
text_seed_result = client.predict(api_name="/generate_seed_1")

# 设置音频种子
embedding_result = client.predict(audio_seed_input=audio_seed, api_name="/on_audio_seed_change")
```

下一步行动

立即需要解决的问题：
1. 确认完整的API端点列表
2. 找到文本转语音生成的主要API
3. 确认API调用的正确序列
4. 测试实际的API调用流程

建议的解决方案：
1. 直接访问TTS服务器的Web界面，观察网络请求
2. 查看Gradio应用的完整API文档
3. 联系TTS服务器的部署人员获取完整API信息
4. 进行实际的API测试和验证

风险评估

技术风险：
- 主要TTS功能无法实现（高风险）
- API调用序列不正确（中风险）
- 性能问题（低风险）

缓解措施：
- 尽快获取完整API文档
- 实现渐进式的API集成
- 建立完善的错误处理和日志记录
- 准备备用的TTS解决方案

项目影响

对项目的影响：
- 核心TTS功能的实现被阻塞
- 需要额外的API调研时间
- 可能需要调整项目时间计划
- 其他模块的开发可以继续进行

建议的应对策略：
1. 优先解决API文档问题
2. 并行开发其他模块（AI文案生成、音效板等）
3. 建立模拟的TTS接口用于测试
4. 与TTS服务器团队建立沟通渠道

总结

TTS API的更新揭示了一个重要问题：当前的API文档不完整，缺少核心的文本转语音生成功能。虽然我们已经更新了代码框架以适配Gradio API，但仍需要获取完整的API信息才能实现完整的TTS功能。

建议立即采取行动获取完整的API文档，并与TTS服务器团队建立有效的沟通渠道，确保项目能够顺利进行。
