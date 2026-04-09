use anyhow::Error;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;

trait RefineService {
    async fn refine(&mut self, src: &str, prompt: &str) -> Result<String, Error>;
}

struct LlamaRefineService {
    model: LlamaModel,
    backend: LlamaBackend,
}
impl RefineService for LlamaRefineService {
    async fn refine(&mut self, src: &str, prompt: &str) -> Result<String, Error> {
        let ctx_params = LlamaContextParams::default();
        let mut ctx = self
            .model
            .new_context(&self.backend, ctx_params)
            .expect("unable to create the llama_context");

        let prompt =
            format!("<|im_start|>user\n/no_think\n{prompt}####{src}<|im_end|>\n<|im_start|>assistant\n").to_string();
        let tokens_list = self
            .model
            .str_to_token(&prompt, AddBos::Always)
            .unwrap_or_else(|_| panic!("failed to tokenize {prompt}"));
        let n_len = 1024;

        // create a llama_batch with size 512
        // we use this object to submit token data for decoding
        let mut batch = LlamaBatch::new(512, 1);

        let last_index = tokens_list.len() as i32 - 1;
        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            // llama_decode will output logits only for the last token of the prompt
            let is_last = i == last_index;
            batch.add(token, i, &[0], is_last)?;
        }
        ctx.decode(&mut batch).expect("llama_decode() failed");

        let mut n_cur = batch.n_tokens();

        // The `Decoder`
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut sampler = LlamaSampler::greedy();

        let mut result = String::new();
        while n_cur <= n_len {
            // sample the next token
            {
                let token = sampler.sample(&ctx, batch.n_tokens() - 1);

                sampler.accept(token);

                // is it an end of stream?
                if token == self.model.token_eos() {
                    eprintln!();
                    break;
                }

                let output_string = self.model.token_to_piece(token, &mut decoder, true, None)?;
                result.push_str(&output_string);

                batch.clear();
                batch.add(token, n_cur, &[0], true)?;
            }

            n_cur += 1;

            ctx.decode(&mut batch).expect("failed to eval");
        }
        Ok(result)
    }
}
impl LlamaRefineService {
    pub fn new(model: Option<&str>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let model_path = model.unwrap_or("models/llm/Qwen3-0.6B-Q8_0.gguf");
        let backend = LlamaBackend::init()?;
        let params = LlamaModelParams::default();
        LlamaContextParams::default();
        let model = LlamaModel::load_from_file(&backend, model_path, &params)
            .expect("unable to load model");
        Ok(Self { model, backend })
    }
}
