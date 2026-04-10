use crate::refine::RefineService;
use anyhow::{bail, Error};
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;
use std::num::NonZeroU32;

pub struct LlamaRefineService {
    model: LlamaModel,
    backend: LlamaBackend,
}

impl RefineService for LlamaRefineService {
    fn refine(&mut self, src: &str, prompt: &str) -> Result<String, Error> {
        let ctx_params =
            LlamaContextParams::default().with_n_ctx(NonZeroU32::new(self.model.n_ctx_train()));
        let mut ctx = self.model.new_context(&self.backend, ctx_params)?;

        let prompt = format!(
            "<|im_start|>user\n/no_think\n{prompt}####{src}<|im_end|>\n<|im_start|>assistant\n"
        )
        .to_string();
        let tokens_list = self.model.str_to_token(&prompt, AddBos::Always)?;
        let n_ctx = ctx.n_ctx() as usize;
        if tokens_list.len() >= n_ctx {
            bail!(
                "prompt uses {} tokens but context window is {}",
                tokens_list.len(),
                n_ctx
            );
        }
        let max_new_tokens = 1024usize.min(n_ctx - tokens_list.len());

        // create a llama_batch with size 512
        // we use this object to submit token data for decoding
        let mut batch = LlamaBatch::new(tokens_list.len(), 1);

        let last_index = tokens_list.len() as i32 - 1;
        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            // llama_decode will output logits only for the last token of the prompt
            let is_last = i == last_index;
            batch.add(token, i, &[0], is_last)?;
        }
        ctx.decode(&mut batch)?;

        let mut n_cur = batch.n_tokens();

        // The `Decoder`
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut sampler =
            LlamaSampler::chain_simple([LlamaSampler::temp(0.0), LlamaSampler::greedy()]);

        let mut result = String::new();
        let mut generated_tokens = 0usize;
        while generated_tokens < max_new_tokens {
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
            generated_tokens += 1;

            ctx.decode(&mut batch)?;
        }
        Ok(result)
    }
}

impl LlamaRefineService {
    pub fn new(model: Option<&str>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let model_path = model.unwrap_or("models/llm/qwen3.5-0.8b.gguf");
        let backend = LlamaBackend::init()?;
        let params = LlamaModelParams::default();
        LlamaContextParams::default();
        let model = LlamaModel::load_from_file(&backend, model_path, &params)
            .expect("unable to load model");
        Ok(Self { model, backend })
    }
}
