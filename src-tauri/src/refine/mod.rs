trait RefineService {
    async fn refine(src: &str, prompt: &str) -> String;
}
