use std::time::Instant;
use talkful_lib::refine::{LlamaRefineService, RefineService};

fn main() {
    let src = r#"
开去所以这一切都是命中注定的那时我将卡车开到了一个三岔路口所看到一个路标朝右指着千米荡六十公里我的卡车便向右转弯接下来我就闯祸了这是我第二次闯祸第一次是在安徽皖南山区那是十多年前了那时候我的那辆解放牌不是后来这辆黄河 在一辆一条狭窄的盘山公路上把一个孩子撞到了十多丈下面的水库里我是没有办法才这样坐的那时候我的卡车正绕着公路往下滑在完成了一第七个急转弯后,不， 是第八个
    "#;
    let prompt = r#"
You are an expert editorial assistant. Your task is to transform raw speech-to-text transcripts into polished, professional written text.

Apply the following rules strictly:

1. CLEANSE: Remove all filler words (e.g., "um", "uh", "like", "you know"), stuttering, and false starts.
2. DEDUPLICATE: Remove unnecessary repetitions of words or phrases.
3. RESOLVE CORRECTIONS: If the speaker self-corrects (e.g., "go to the... no, wait, send the email"), keep only the final intended meaning ("send the email"). Discard the abandoned thought.
4. FORMAT: Structure the output logically. use proper paragraph breaks for topic shifts, and correct punctuation/capitalization.
5. CLARIFY: Rephrase awkward or ambiguous phrasing for clarity and conciseness without changing the original intent or tone.

Output ONLY the final polished text. Do not include explanations, notes, or markdown code blocks.    "#;
    let mut refiner = LlamaRefineService::new(Some("models/llm/qwen3-0.6b.gguf")).unwrap();

    let start = Instant::now();
    let result = refiner.refine(src, prompt).unwrap();
    eprintln!("refine took: {:?}", start.elapsed());
    println!("{result}");
}
