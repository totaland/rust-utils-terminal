use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use tabled::Tabled;

/// Chrome bookmarks file location on macOS
const CHROME_BOOKMARKS_PATH: &str = "Library/Application Support/Google/Chrome/Default/Bookmarks";

/// Bookmark categories for auto-organization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BookmarkCategory {
    // AI/ML Categories
    AIGeneral,
    AILLMs,
    AIPromptEngineering,
    AIAgents,
    AIRAG,
    AIContext,
    AIFineTuning,
    AIEmbeddings,
    AIVectorDB,
    AIMLOps,
    AIComputerVision,
    AINLP,
    AIResearch,
    // General Categories
    Development,
    Social,
    News,
    Shopping,
    Entertainment,
    Education,
    Reference,
    Tools,
    Finance,
    Health,
    Travel,
    Food,
    Sports,
    Gaming,
    Music,
    Video,
    Other,
}

impl std::fmt::Display for BookmarkCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // AI/ML Categories
            BookmarkCategory::AIGeneral => write!(f, "🤖 AI/ML General"),
            BookmarkCategory::AILLMs => write!(f, "🧠 LLMs & Models"),
            BookmarkCategory::AIPromptEngineering => write!(f, "✍️  Prompt Engineering"),
            BookmarkCategory::AIAgents => write!(f, "🤝 AI Agents"),
            BookmarkCategory::AIRAG => write!(f, "📚 RAG"),
            BookmarkCategory::AIContext => write!(f, "🔗 Context & Memory"),
            BookmarkCategory::AIFineTuning => write!(f, "🎯 Fine-Tuning"),
            BookmarkCategory::AIEmbeddings => write!(f, "📊 Embeddings"),
            BookmarkCategory::AIVectorDB => write!(f, "🗄️  Vector Databases"),
            BookmarkCategory::AIMLOps => write!(f, "⚙️  MLOps"),
            BookmarkCategory::AIComputerVision => write!(f, "👁️  Computer Vision"),
            BookmarkCategory::AINLP => write!(f, "💬 NLP"),
            BookmarkCategory::AIResearch => write!(f, "🔬 AI Research"),
            // General Categories
            BookmarkCategory::Development => write!(f, "🛠️  Development"),
            BookmarkCategory::Social => write!(f, "👥 Social"),
            BookmarkCategory::News => write!(f, "📰 News"),
            BookmarkCategory::Shopping => write!(f, "🛒 Shopping"),
            BookmarkCategory::Entertainment => write!(f, "🎬 Entertainment"),
            BookmarkCategory::Education => write!(f, "📚 Education"),
            BookmarkCategory::Reference => write!(f, "📖 Reference"),
            BookmarkCategory::Tools => write!(f, "🔧 Tools"),
            BookmarkCategory::Finance => write!(f, "💰 Finance"),
            BookmarkCategory::Health => write!(f, "🏥 Health"),
            BookmarkCategory::Travel => write!(f, "✈️  Travel"),
            BookmarkCategory::Food => write!(f, "🍕 Food"),
            BookmarkCategory::Sports => write!(f, "⚽ Sports"),
            BookmarkCategory::Gaming => write!(f, "🎮 Gaming"),
            BookmarkCategory::Music => write!(f, "🎵 Music"),
            BookmarkCategory::Video => write!(f, "📹 Video"),
            BookmarkCategory::Other => write!(f, "📁 Other"),
        }
    }
}

impl BookmarkCategory {
    pub fn folder_name(&self) -> &str {
        match self {
            // AI/ML Categories
            BookmarkCategory::AIGeneral => "AI-ML/General",
            BookmarkCategory::AILLMs => "AI-ML/LLMs & Models",
            BookmarkCategory::AIPromptEngineering => "AI-ML/Prompt Engineering",
            BookmarkCategory::AIAgents => "AI-ML/Agents",
            BookmarkCategory::AIRAG => "AI-ML/RAG",
            BookmarkCategory::AIContext => "AI-ML/Context & Memory",
            BookmarkCategory::AIFineTuning => "AI-ML/Fine-Tuning",
            BookmarkCategory::AIEmbeddings => "AI-ML/Embeddings",
            BookmarkCategory::AIVectorDB => "AI-ML/Vector Databases",
            BookmarkCategory::AIMLOps => "AI-ML/MLOps",
            BookmarkCategory::AIComputerVision => "AI-ML/Computer Vision",
            BookmarkCategory::AINLP => "AI-ML/NLP",
            BookmarkCategory::AIResearch => "AI-ML/Research",
            // General Categories
            BookmarkCategory::Development => "Development",
            BookmarkCategory::Social => "Social Media",
            BookmarkCategory::News => "News",
            BookmarkCategory::Shopping => "Shopping",
            BookmarkCategory::Entertainment => "Entertainment",
            BookmarkCategory::Education => "Education",
            BookmarkCategory::Reference => "Reference",
            BookmarkCategory::Tools => "Tools & Utilities",
            BookmarkCategory::Finance => "Finance",
            BookmarkCategory::Health => "Health",
            BookmarkCategory::Travel => "Travel",
            BookmarkCategory::Food => "Food & Recipes",
            BookmarkCategory::Sports => "Sports",
            BookmarkCategory::Gaming => "Gaming",
            BookmarkCategory::Music => "Music",
            BookmarkCategory::Video => "Video",
            BookmarkCategory::Other => "Other",
        }
    }

    /// Categorize a bookmark based on its URL and title
    pub fn from_url_and_title(url: &str, title: &str) -> Self {
        let url_lower = url.to_lowercase();
        let title_lower = title.to_lowercase();
        let combined = format!("{} {}", url_lower, title_lower);

        // ============================================
        // AI/ML Categories (check first for specificity)
        // ============================================

        // RAG (Retrieval Augmented Generation)
        if combined.contains("retrieval augmented")
            || combined.contains("rag ")
            || combined.contains(" rag")
            || combined.contains("langchain") && combined.contains("retriev")
            || combined.contains("llamaindex")
            || combined.contains("llama-index")
            || combined.contains("llama_index")
            || combined.contains("haystack") && combined.contains("ai")
            || combined.contains("document retrieval")
            || combined.contains("semantic search") && combined.contains("llm")
            || combined.contains("knowledge base") && combined.contains("ai")
            || combined.contains("chunking")
                && (combined.contains("llm") || combined.contains("embedding"))
        {
            return BookmarkCategory::AIRAG;
        }

        // Context & Memory
        if combined.contains("context window")
            || combined.contains("context length")
            || combined.contains("long context")
            || combined.contains("memory")
                && (combined.contains("llm")
                    || combined.contains("agent")
                    || combined.contains("ai"))
            || combined.contains("conversation memory")
            || combined.contains("chat history")
            || combined.contains("mem0")
            || combined.contains("memgpt")
            || combined.contains("context management")
            || combined.contains("token limit")
            || combined.contains("context compression")
            || combined.contains("sliding window") && combined.contains("context")
        {
            return BookmarkCategory::AIContext;
        }

        // AI Agents
        if combined.contains("ai agent")
            || combined.contains("autonomous agent")
            || combined.contains("langchain agent")
            || combined.contains("autogpt")
            || combined.contains("auto-gpt")
            || combined.contains("babyagi")
            || combined.contains("crewai")
            || combined.contains("crew ai")
            || combined.contains("autogen")
            || combined.contains("agent framework")
            || combined.contains("multi-agent")
            || combined.contains("multiagent")
            || combined.contains("tool use") && combined.contains("llm")
            || combined.contains("function calling") && combined.contains("ai")
            || combined.contains("agentic")
            || combined.contains("agent orchestration")
            || combined.contains("smolagent")
            || combined.contains("phidata")
            || combined.contains("swarm") && combined.contains("agent")
            || url_lower.contains("mcp")
                && (combined.contains("protocol") || combined.contains("context"))
            || combined.contains("model context protocol")
        {
            return BookmarkCategory::AIAgents;
        }

        // Prompt Engineering
        if combined.contains("prompt engineering")
            || combined.contains("prompt template")
            || combined.contains("prompting")
            || combined.contains("chain of thought")
            || combined.contains("cot prompting")
            || combined.contains("few-shot")
            || combined.contains("zero-shot")
            || combined.contains("in-context learning")
            || combined.contains("prompt injection")
            || combined.contains("jailbreak") && combined.contains("llm")
            || combined.contains("system prompt")
            || combined.contains("prompt optimization")
            || combined.contains("dspy")
            || combined.contains("promptfoo")
            || combined.contains("prompt testing")
        {
            return BookmarkCategory::AIPromptEngineering;
        }

        // Vector Databases
        if url_lower.contains("pinecone.io")
            || url_lower.contains("weaviate.io")
            || url_lower.contains("milvus.io")
            || url_lower.contains("qdrant")
            || url_lower.contains("chroma") && combined.contains("vector")
            || url_lower.contains("chromadb")
            || combined.contains("vector database")
            || combined.contains("vector db")
            || combined.contains("vectorstore")
            || combined.contains("vector store")
            || combined.contains("pgvector")
            || combined.contains("faiss") && combined.contains("vector")
            || combined.contains("annoy") && combined.contains("vector")
            || combined.contains("similarity search") && combined.contains("vector")
            || url_lower.contains("lancedb")
            || url_lower.contains("vespa.ai")
        {
            return BookmarkCategory::AIVectorDB;
        }

        // Embeddings
        if combined.contains("embedding")
            || combined.contains("sentence transformer")
            || combined.contains("text-embedding")
            || combined.contains("ada-002")
            || combined.contains("openai embedding")
            || combined.contains("cohere embed")
            || combined.contains("word2vec")
            || combined.contains("doc2vec")
            || combined.contains("semantic similarity")
            || url_lower.contains("huggingface") && combined.contains("embed")
            || combined.contains("voyage ai")
            || combined.contains("jina embedding")
        {
            return BookmarkCategory::AIEmbeddings;
        }

        // Fine-Tuning
        if combined.contains("fine-tun")
            || combined.contains("finetun")
            || combined.contains("lora")
            || combined.contains("qlora")
            || combined.contains("peft")
            || combined.contains("adapter") && combined.contains("llm")
            || combined.contains("instruction tuning")
            || combined.contains("rlhf")
            || combined.contains("dpo") && combined.contains("training")
            || combined.contains("sft")
                && (combined.contains("llm") || combined.contains("training"))
            || combined.contains("training data") && combined.contains("llm")
            || combined.contains("axolotl")
            || combined.contains("unsloth")
            || url_lower.contains("predibase")
            || url_lower.contains("together.ai") && combined.contains("fine")
        {
            return BookmarkCategory::AIFineTuning;
        }

        // LLMs & Models
        if url_lower.contains("openai.com")
            || url_lower.contains("anthropic.com")
            || url_lower.contains("claude.ai")
            || url_lower.contains("chat.openai.com")
            || url_lower.contains("gemini.google")
            || url_lower.contains("bard.google")
            || url_lower.contains("mistral.ai")
            || url_lower.contains("cohere.com")
            || url_lower.contains("huggingface.co")
            || url_lower.contains("ollama")
            || url_lower.contains("replicate.com")
            || url_lower.contains("together.ai")
            || url_lower.contains("groq.com")
            || url_lower.contains("anyscale.com")
            || url_lower.contains("perplexity.ai")
            || url_lower.contains("deepseek")
            || url_lower.contains("meta.ai")
            || combined.contains("llama")
                && (combined.contains("model")
                    || combined.contains("meta")
                    || combined.contains("ai"))
            || combined.contains("gpt-4")
            || combined.contains("gpt-3")
            || combined.contains("chatgpt")
            || combined.contains("claude") && combined.contains("anthropic")
            || combined.contains("gemini") && combined.contains("google")
            || combined.contains("mistral") && combined.contains("model")
            || combined.contains("mixtral")
            || combined.contains("phi-") && combined.contains("microsoft")
            || combined.contains("falcon") && combined.contains("model")
            || combined.contains("qwen")
            || combined.contains("yi model")
            || combined.contains("command-r")
            || combined.contains("large language model")
            || combined.contains("foundation model")
        {
            return BookmarkCategory::AILLMs;
        }

        // MLOps
        if url_lower.contains("mlflow")
            || url_lower.contains("wandb.ai")
            || url_lower.contains("weights-and-biases")
            || url_lower.contains("neptune.ai")
            || url_lower.contains("comet.ml")
            || url_lower.contains("dagshub")
            || url_lower.contains("dvc.org")
            || url_lower.contains("kubeflow")
            || url_lower.contains("bentoml")
            || url_lower.contains("seldon")
            || url_lower.contains("ray.io")
            || url_lower.contains("modal.com")
            || combined.contains("mlops")
            || combined.contains("ml ops")
            || combined.contains("model deployment")
            || combined.contains("model serving")
            || combined.contains("model monitoring")
            || combined.contains("experiment tracking")
            || combined.contains("model registry")
            || combined.contains("feature store")
            || combined.contains("ml pipeline")
        {
            return BookmarkCategory::AIMLOps;
        }

        // Computer Vision
        if combined.contains("computer vision")
            || combined.contains("image recognition")
            || combined.contains("object detection")
            || combined.contains("image segmentation")
            || combined.contains("yolo") && combined.contains("detection")
            || combined.contains("opencv")
            || combined.contains("stable diffusion")
            || combined.contains("midjourney")
            || combined.contains("dall-e")
            || combined.contains("imagen")
            || combined.contains("diffusion model")
            || combined.contains("image generation")
            || combined.contains("text-to-image")
            || combined.contains("image-to-image")
            || combined.contains("inpainting")
            || combined.contains("controlnet")
            || combined.contains("comfyui")
            || url_lower.contains("civitai")
            || url_lower.contains("stability.ai")
            || url_lower.contains("runway")
            || combined.contains("vision model")
            || combined.contains("multimodal") && combined.contains("vision")
        {
            return BookmarkCategory::AIComputerVision;
        }

        // NLP
        if combined.contains("natural language processing")
            || combined.contains("nlp ")
            || combined.contains(" nlp")
            || combined.contains("text classification")
            || combined.contains("named entity")
            || combined.contains("ner ")
            || combined.contains("sentiment analysis")
            || combined.contains("text mining")
            || combined.contains("spacy")
            || combined.contains("nltk")
            || combined.contains("tokeniz")
            || combined.contains("part-of-speech")
            || combined.contains("dependency parsing")
            || combined.contains("text extraction")
            || combined.contains("information extraction")
        {
            return BookmarkCategory::AINLP;
        }

        // AI Research
        if url_lower.contains("arxiv.org") && combined.contains("ai")
            || url_lower.contains("arxiv.org") && combined.contains("machine learning")
            || url_lower.contains("arxiv.org") && combined.contains("llm")
            || url_lower.contains("arxiv.org") && combined.contains("neural")
            || url_lower.contains("arxiv.org") && combined.contains("transformer")
            || url_lower.contains("paperswithcode.com")
            || url_lower.contains("semanticscholar.org") && combined.contains("ai")
            || url_lower.contains("connectedpapers.com")
            || combined.contains("research paper") && combined.contains("ai")
            || combined.contains("ai research")
            || combined.contains("ml research")
            || url_lower.contains("deepmind.com")
            || url_lower.contains("research.google") && combined.contains("ai")
            || url_lower.contains("ai.meta.com")
            || url_lower.contains("research.microsoft.com") && combined.contains("ai")
        {
            return BookmarkCategory::AIResearch;
        }

        // General AI/ML (catch-all for AI content not fitting specific subcategories)
        if combined.contains("artificial intelligence")
            || combined.contains("machine learning")
            || combined.contains("deep learning")
            || combined.contains("neural network")
            || combined.contains("transformer")
                && (combined.contains("ai") || combined.contains("model"))
            || combined.contains("tensorflow")
            || combined.contains("pytorch")
            || combined.contains("keras")
            || combined.contains("scikit-learn")
            || combined.contains("sklearn")
            || url_lower.contains("kaggle.com")
            || url_lower.contains("fast.ai")
            || url_lower.contains("deeplearning.ai")
            || combined.contains("ai tool")
            || combined.contains("ml tool")
            || combined.contains("generative ai")
            || combined.contains("gen ai")
            || combined.contains("langchain")
            || combined.contains("llamaindex")
            || combined.contains("inference")
                && (combined.contains("model") || combined.contains("ai"))
        {
            return BookmarkCategory::AIGeneral;
        }

        // ============================================
        // General Categories
        // ============================================

        // Development
        if url_lower.contains("github.com")
            || url_lower.contains("gitlab.com")
            || url_lower.contains("bitbucket.org")
            || url_lower.contains("stackoverflow.com")
            || url_lower.contains("stackexchange.com")
            || url_lower.contains("developer.")
            || url_lower.contains("docs.")
            || url_lower.contains("npmjs.com")
            || url_lower.contains("crates.io")
            || url_lower.contains("pypi.org")
            || url_lower.contains("hub.docker.com")
            || url_lower.contains("kubernetes.io")
            || url_lower.contains("rust-lang.org")
            || url_lower.contains("python.org")
            || url_lower.contains("nodejs.org")
            || url_lower.contains("typescriptlang.org")
            || url_lower.contains("reactjs.org")
            || url_lower.contains("vuejs.org")
            || url_lower.contains("angular.io")
            || url_lower.contains("vercel.com")
            || url_lower.contains("netlify.com")
            || url_lower.contains("heroku.com")
            || url_lower.contains("aws.amazon.com")
            || url_lower.contains("cloud.google.com")
            || url_lower.contains("azure.microsoft.com")
            || url_lower.contains("codepen.io")
            || url_lower.contains("codesandbox.io")
            || url_lower.contains("replit.com")
            || url_lower.contains("jsfiddle.net")
            || url_lower.contains("medium.com") && combined.contains("programming")
            || url_lower.contains("dev.to")
            || url_lower.contains("hashnode.com")
            || combined.contains("api")
            || combined.contains("documentation")
            || combined.contains("tutorial")
        {
            return BookmarkCategory::Development;
        }

        // Social Media
        if url_lower.contains("facebook.com")
            || url_lower.contains("twitter.com")
            || url_lower.contains("x.com")
            || url_lower.contains("instagram.com")
            || url_lower.contains("linkedin.com")
            || url_lower.contains("reddit.com")
            || url_lower.contains("discord.com")
            || url_lower.contains("slack.com")
            || url_lower.contains("telegram.org")
            || url_lower.contains("whatsapp.com")
            || url_lower.contains("snapchat.com")
            || url_lower.contains("tiktok.com")
            || url_lower.contains("pinterest.com")
            || url_lower.contains("tumblr.com")
            || url_lower.contains("mastodon")
            || url_lower.contains("threads.net")
            || url_lower.contains("bluesky")
        {
            return BookmarkCategory::Social;
        }

        // News
        if url_lower.contains("news.")
            || url_lower.contains("bbc.com")
            || url_lower.contains("cnn.com")
            || url_lower.contains("nytimes.com")
            || url_lower.contains("washingtonpost.com")
            || url_lower.contains("theguardian.com")
            || url_lower.contains("reuters.com")
            || url_lower.contains("apnews.com")
            || url_lower.contains("bloomberg.com")
            || url_lower.contains("techcrunch.com")
            || url_lower.contains("theverge.com")
            || url_lower.contains("wired.com")
            || url_lower.contains("arstechnica.com")
            || url_lower.contains("engadget.com")
            || url_lower.contains("hackernews")
            || url_lower.contains("news.ycombinator.com")
            || combined.contains("breaking news")
        {
            return BookmarkCategory::News;
        }

        // Shopping
        if url_lower.contains("amazon.")
            || url_lower.contains("ebay.")
            || url_lower.contains("etsy.com")
            || url_lower.contains("aliexpress.com")
            || url_lower.contains("walmart.com")
            || url_lower.contains("target.com")
            || url_lower.contains("bestbuy.com")
            || url_lower.contains("newegg.com")
            || url_lower.contains("shopify")
            || url_lower.contains("/cart")
            || url_lower.contains("/checkout")
            || url_lower.contains("/shop")
            || combined.contains("buy")
            || combined.contains("deal")
            || combined.contains("discount")
        {
            return BookmarkCategory::Shopping;
        }

        // Video
        if url_lower.contains("youtube.com")
            || url_lower.contains("youtu.be")
            || url_lower.contains("vimeo.com")
            || url_lower.contains("dailymotion.com")
            || url_lower.contains("twitch.tv")
        {
            return BookmarkCategory::Video;
        }

        // Music
        if url_lower.contains("spotify.com")
            || url_lower.contains("soundcloud.com")
            || url_lower.contains("music.apple.com")
            || url_lower.contains("bandcamp.com")
            || url_lower.contains("last.fm")
            || url_lower.contains("pandora.com")
            || url_lower.contains("deezer.com")
            || url_lower.contains("tidal.com")
        {
            return BookmarkCategory::Music;
        }

        // Gaming
        if url_lower.contains("steam")
            || url_lower.contains("epicgames.com")
            || url_lower.contains("gog.com")
            || url_lower.contains("playstation.com")
            || url_lower.contains("xbox.com")
            || url_lower.contains("nintendo.com")
            || url_lower.contains("ign.com")
            || url_lower.contains("gamespot.com")
            || url_lower.contains("kotaku.com")
            || url_lower.contains("polygon.com")
        {
            return BookmarkCategory::Gaming;
        }

        // Entertainment (Netflix, etc.)
        if url_lower.contains("netflix.com")
            || url_lower.contains("hulu.com")
            || url_lower.contains("disneyplus.com")
            || url_lower.contains("hbomax.com")
            || url_lower.contains("primevideo.com")
            || url_lower.contains("crunchyroll.com")
            || url_lower.contains("imdb.com")
            || url_lower.contains("rottentomatoes.com")
            || url_lower.contains("letterboxd.com")
        {
            return BookmarkCategory::Entertainment;
        }

        // Education
        if url_lower.contains("coursera.org")
            || url_lower.contains("udemy.com")
            || url_lower.contains("edx.org")
            || url_lower.contains("khanacademy.org")
            || url_lower.contains("skillshare.com")
            || url_lower.contains("pluralsight.com")
            || url_lower.contains("lynda.com")
            || url_lower.contains("codecademy.com")
            || url_lower.contains("freecodecamp.org")
            || url_lower.contains("w3schools.com")
            || url_lower.contains("mdn")
            || url_lower.contains(".edu")
            || url_lower.contains("learn.")
            || combined.contains("course")
            || combined.contains("lesson")
            || combined.contains("tutorial")
        {
            return BookmarkCategory::Education;
        }

        // Reference (Wikipedia, dictionaries, etc.)
        if url_lower.contains("wikipedia.org")
            || url_lower.contains("wikimedia.org")
            || url_lower.contains("wiktionary.org")
            || url_lower.contains("britannica.com")
            || url_lower.contains("merriam-webster.com")
            || url_lower.contains("dictionary.com")
            || url_lower.contains("thesaurus.com")
            || url_lower.contains("translate.google")
            || url_lower.contains("deepl.com")
            || url_lower.contains("wolframalpha.com")
        {
            return BookmarkCategory::Reference;
        }

        // Tools & Utilities
        if url_lower.contains("notion.so")
            || url_lower.contains("trello.com")
            || url_lower.contains("asana.com")
            || url_lower.contains("monday.com")
            || url_lower.contains("figma.com")
            || url_lower.contains("canva.com")
            || url_lower.contains("drive.google.com")
            || url_lower.contains("dropbox.com")
            || url_lower.contains("box.com")
            || url_lower.contains("1password.com")
            || url_lower.contains("lastpass.com")
            || url_lower.contains("bitwarden.com")
            || url_lower.contains("grammarly.com")
            || url_lower.contains("calendly.com")
            || url_lower.contains("zoom.us")
            || url_lower.contains("meet.google.com")
            || url_lower.contains("teams.microsoft.com")
            || combined.contains("converter")
            || combined.contains("generator")
            || combined.contains("calculator")
        {
            return BookmarkCategory::Tools;
        }

        // Finance
        if url_lower.contains("bank")
            || url_lower.contains("paypal.com")
            || url_lower.contains("venmo.com")
            || url_lower.contains("coinbase.com")
            || url_lower.contains("binance.com")
            || url_lower.contains("robinhood.com")
            || url_lower.contains("fidelity.com")
            || url_lower.contains("schwab.com")
            || url_lower.contains("vanguard.com")
            || url_lower.contains("mint.com")
            || url_lower.contains("ynab.com")
            || url_lower.contains("creditkarma.com")
            || url_lower.contains("finance.")
            || combined.contains("invest")
            || combined.contains("stock")
            || combined.contains("crypto")
        {
            return BookmarkCategory::Finance;
        }

        // Health
        if url_lower.contains("webmd.com")
            || url_lower.contains("mayoclinic.org")
            || url_lower.contains("healthline.com")
            || url_lower.contains("nih.gov")
            || url_lower.contains("cdc.gov")
            || url_lower.contains("who.int")
            || url_lower.contains("myfitnesspal.com")
            || url_lower.contains("strava.com")
            || url_lower.contains("fitbit.com")
            || combined.contains("health")
            || combined.contains("fitness")
            || combined.contains("workout")
            || combined.contains("diet")
        {
            return BookmarkCategory::Health;
        }

        // Travel
        if url_lower.contains("booking.com")
            || url_lower.contains("airbnb.com")
            || url_lower.contains("expedia.com")
            || url_lower.contains("kayak.com")
            || url_lower.contains("tripadvisor.com")
            || url_lower.contains("skyscanner.com")
            || url_lower.contains("google.com/flights")
            || url_lower.contains("google.com/maps")
            || url_lower.contains("maps.google")
            || url_lower.contains("hotels.com")
            || url_lower.contains("vrbo.com")
            || combined.contains("travel")
            || combined.contains("flight")
            || combined.contains("hotel")
            || combined.contains("vacation")
        {
            return BookmarkCategory::Travel;
        }

        // Food & Recipes
        if url_lower.contains("allrecipes.com")
            || url_lower.contains("foodnetwork.com")
            || url_lower.contains("epicurious.com")
            || url_lower.contains("bonappetit.com")
            || url_lower.contains("seriouseats.com")
            || url_lower.contains("tasty.co")
            || url_lower.contains("doordash.com")
            || url_lower.contains("ubereats.com")
            || url_lower.contains("grubhub.com")
            || url_lower.contains("postmates.com")
            || url_lower.contains("yelp.com")
            || combined.contains("recipe")
            || combined.contains("cooking")
            || combined.contains("restaurant")
        {
            return BookmarkCategory::Food;
        }

        // Sports
        if url_lower.contains("espn.com")
            || url_lower.contains("sports.")
            || url_lower.contains("nfl.com")
            || url_lower.contains("nba.com")
            || url_lower.contains("mlb.com")
            || url_lower.contains("nhl.com")
            || url_lower.contains("fifa.com")
            || url_lower.contains("uefa.com")
            || url_lower.contains("olympics.com")
            || combined.contains("score")
            || combined.contains("league")
            || combined.contains("team")
        {
            return BookmarkCategory::Sports;
        }

        BookmarkCategory::Other
    }
}

/// A Chrome bookmark entry
#[derive(Debug, Clone)]
pub struct Bookmark {
    pub id: String,
    pub name: String,
    pub url: String,
    pub date_added: Option<String>,
    pub folder_path: String,
    pub category: BookmarkCategory,
}

/// A bookmark folder
#[derive(Debug, Clone)]
pub struct BookmarkFolder {
    pub id: String,
    pub name: String,
    pub path: String,
    pub children_count: usize,
}

/// Statistics about bookmarks
#[derive(Debug, Clone, Default)]
pub struct BookmarkStats {
    pub total_bookmarks: usize,
    pub total_folders: usize,
    pub duplicates: usize,
    pub by_domain: HashMap<String, usize>,
    pub by_category: HashMap<String, usize>,
    pub empty_folders: usize,
    pub deep_nesting_count: usize,
}

/// Entry for duplicate bookmarks table
#[derive(Tabled, Clone)]
pub struct DuplicateEntry {
    #[tabled(rename = "URL")]
    pub url: String,
    #[tabled(rename = "Occurrences")]
    pub count: usize,
    #[tabled(rename = "Titles")]
    pub titles: String,
}

/// Entry for domain statistics table
#[derive(Tabled, Clone)]
pub struct DomainEntry {
    #[tabled(rename = "Domain")]
    pub domain: String,
    #[tabled(rename = "Count")]
    pub count: usize,
    #[tabled(rename = "Percentage")]
    pub percentage: String,
}

/// Entry for category statistics table
#[derive(Tabled, Clone)]
pub struct CategoryEntry {
    #[tabled(rename = "Category")]
    pub category: String,
    #[tabled(rename = "Count")]
    pub count: usize,
    #[tabled(rename = "Percentage")]
    pub percentage: String,
}

/// Entry for bookmarks table
#[derive(Tabled, Clone)]
pub struct BookmarkTableEntry {
    #[tabled(rename = "Title")]
    pub title: String,
    #[tabled(rename = "URL")]
    pub url: String,
    #[tabled(rename = "Category")]
    pub category: String,
    #[tabled(rename = "Folder")]
    pub folder: String,
}

/// Entry for organization suggestions
#[derive(Tabled, Clone)]
pub struct OrganizeSuggestion {
    #[tabled(rename = "Bookmark")]
    pub bookmark: String,
    #[tabled(rename = "Current Folder")]
    pub current_folder: String,
    #[tabled(rename = "Suggested Folder")]
    pub suggested_folder: String,
    #[tabled(rename = "Category")]
    pub category: String,
}

/// Get the Chrome bookmarks file path
pub fn get_chrome_bookmarks_path() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME environment variable not set")?;
    let path = PathBuf::from(home).join(CHROME_BOOKMARKS_PATH);

    if !path.exists() {
        anyhow::bail!(
            "Chrome bookmarks file not found at: {}\n\
             Make sure Chrome is installed and you have bookmarks saved.",
            path.display()
        );
    }

    Ok(path)
}

/// Parse the Chrome bookmarks JSON file
pub fn parse_bookmarks() -> Result<(Vec<Bookmark>, Vec<BookmarkFolder>)> {
    let path = get_chrome_bookmarks_path()?;
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read bookmarks file: {}", path.display()))?;

    let json: serde_json::Value =
        serde_json::from_str(&content).context("Failed to parse bookmarks JSON")?;

    let mut bookmarks = Vec::new();
    let mut folders = Vec::new();

    // Parse the "roots" object which contains bookmark_bar, other, synced
    if let Some(roots) = json.get("roots").and_then(|r| r.as_object()) {
        for (root_name, root_value) in roots {
            if root_name == "sync_transaction_version" {
                continue;
            }
            parse_bookmark_node(root_value, root_name, &mut bookmarks, &mut folders);
        }
    }

    Ok((bookmarks, folders))
}

/// Recursively parse a bookmark node
fn parse_bookmark_node(
    node: &serde_json::Value,
    current_path: &str,
    bookmarks: &mut Vec<Bookmark>,
    folders: &mut Vec<BookmarkFolder>,
) {
    let node_type = node.get("type").and_then(|t| t.as_str()).unwrap_or("");

    if node_type == "url" {
        // This is a bookmark
        let name = node
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string();
        let url = node
            .get("url")
            .and_then(|u| u.as_str())
            .unwrap_or("")
            .to_string();
        let id = node
            .get("id")
            .and_then(|i| i.as_str())
            .unwrap_or("")
            .to_string();
        let date_added = node
            .get("date_added")
            .and_then(|d| d.as_str())
            .map(|s| s.to_string());

        let category = BookmarkCategory::from_url_and_title(&url, &name);

        bookmarks.push(Bookmark {
            id,
            name,
            url,
            date_added,
            folder_path: current_path.to_string(),
            category,
        });
    } else if node_type == "folder" {
        // This is a folder
        let name = node
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string();
        let id = node
            .get("id")
            .and_then(|i| i.as_str())
            .unwrap_or("")
            .to_string();
        let folder_path = if current_path.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", current_path, name)
        };

        let mut children_count = 0;

        if let Some(children) = node.get("children").and_then(|c| c.as_array()) {
            children_count = children.len();
            for child in children {
                parse_bookmark_node(child, &folder_path, bookmarks, folders);
            }
        }

        folders.push(BookmarkFolder {
            id,
            name,
            path: folder_path,
            children_count,
        });
    }
}

/// Extract domain from URL
pub fn extract_domain(url: &str) -> String {
    let url_lower = url.to_lowercase();

    // Remove protocol
    let without_protocol = url_lower
        .strip_prefix("https://")
        .or_else(|| url_lower.strip_prefix("http://"))
        .or_else(|| url_lower.strip_prefix("file://"))
        .unwrap_or(&url_lower);

    // Get domain part (before first /)
    let domain = without_protocol
        .split('/')
        .next()
        .unwrap_or(without_protocol);

    // Remove www. prefix
    domain.strip_prefix("www.").unwrap_or(domain).to_string()
}

/// Find duplicate bookmarks
pub fn find_duplicates(bookmarks: &[Bookmark]) -> Vec<DuplicateEntry> {
    let mut url_map: HashMap<String, Vec<&Bookmark>> = HashMap::new();

    for bookmark in bookmarks {
        url_map
            .entry(bookmark.url.clone())
            .or_default()
            .push(bookmark);
    }

    let mut duplicates: Vec<DuplicateEntry> = url_map
        .into_iter()
        .filter(|(_, bms)| bms.len() > 1)
        .map(|(url, bms)| {
            let titles: Vec<String> = bms.iter().map(|b| b.name.clone()).collect();
            let unique_titles: HashSet<String> = titles.into_iter().collect();
            DuplicateEntry {
                url: truncate_string(&url, 60),
                count: bms.len(),
                titles: unique_titles.into_iter().collect::<Vec<_>>().join(", "),
            }
        })
        .collect();

    duplicates.sort_by(|a, b| b.count.cmp(&a.count));
    duplicates
}

/// Get domain statistics
pub fn get_domain_stats(bookmarks: &[Bookmark]) -> Vec<DomainEntry> {
    let mut domain_counts: HashMap<String, usize> = HashMap::new();

    for bookmark in bookmarks {
        let domain = extract_domain(&bookmark.url);
        *domain_counts.entry(domain).or_insert(0) += 1;
    }

    let total = bookmarks.len() as f64;
    let mut entries: Vec<DomainEntry> = domain_counts
        .into_iter()
        .map(|(domain, count)| {
            let percentage = count as f64 / total * 100.0;
            DomainEntry {
                domain: truncate_string(&domain, 40),
                count,
                percentage: format!("{:.1}%", percentage),
            }
        })
        .collect();

    entries.sort_by(|a, b| b.count.cmp(&a.count));
    entries
}

/// Get category statistics
pub fn get_category_stats(bookmarks: &[Bookmark]) -> Vec<CategoryEntry> {
    let mut category_counts: HashMap<String, usize> = HashMap::new();

    for bookmark in bookmarks {
        let category = bookmark.category.to_string();
        *category_counts.entry(category).or_insert(0) += 1;
    }

    let total = bookmarks.len() as f64;
    let mut entries: Vec<CategoryEntry> = category_counts
        .into_iter()
        .map(|(category, count)| {
            let percentage = count as f64 / total * 100.0;
            CategoryEntry {
                category,
                count,
                percentage: format!("{:.1}%", percentage),
            }
        })
        .collect();

    entries.sort_by(|a, b| b.count.cmp(&a.count));
    entries
}

/// Get organization suggestions
pub fn get_organize_suggestions(bookmarks: &[Bookmark]) -> Vec<OrganizeSuggestion> {
    let mut suggestions = Vec::new();

    for bookmark in bookmarks {
        let suggested_folder = bookmark.category.folder_name();
        let current_folder = &bookmark.folder_path;

        // Only suggest if the bookmark is not already in a well-organized folder
        // and the category is not "Other"
        if bookmark.category != BookmarkCategory::Other
            && !current_folder
                .to_lowercase()
                .contains(&suggested_folder.to_lowercase())
        {
            suggestions.push(OrganizeSuggestion {
                bookmark: truncate_string(&bookmark.name, 40),
                current_folder: truncate_string(current_folder, 30),
                suggested_folder: suggested_folder.to_string(),
                category: bookmark.category.to_string(),
            });
        }
    }

    suggestions
}

/// Get bookmark statistics
pub fn get_bookmark_stats(bookmarks: &[Bookmark], folders: &[BookmarkFolder]) -> BookmarkStats {
    let mut stats = BookmarkStats::default();

    stats.total_bookmarks = bookmarks.len();
    stats.total_folders = folders.len();

    // Count duplicates
    let mut url_counts: HashMap<String, usize> = HashMap::new();
    for bookmark in bookmarks {
        *url_counts.entry(bookmark.url.clone()).or_insert(0) += 1;
    }
    stats.duplicates = url_counts.values().filter(|&&count| count > 1).count();

    // Count by domain
    for bookmark in bookmarks {
        let domain = extract_domain(&bookmark.url);
        *stats.by_domain.entry(domain).or_insert(0) += 1;
    }

    // Count by category
    for bookmark in bookmarks {
        let category = bookmark.category.to_string();
        *stats.by_category.entry(category).or_insert(0) += 1;
    }

    // Count empty folders
    stats.empty_folders = folders.iter().filter(|f| f.children_count == 0).count();

    // Count deep nesting (more than 3 levels)
    stats.deep_nesting_count = folders
        .iter()
        .filter(|f| f.path.matches('/').count() > 3)
        .count();

    stats
}

/// Search bookmarks by query
pub fn search_bookmarks(bookmarks: &[Bookmark], query: &str) -> Vec<BookmarkTableEntry> {
    let query_lower = query.to_lowercase();

    bookmarks
        .iter()
        .filter(|b| {
            b.name.to_lowercase().contains(&query_lower)
                || b.url.to_lowercase().contains(&query_lower)
                || b.folder_path.to_lowercase().contains(&query_lower)
        })
        .map(|b| BookmarkTableEntry {
            title: truncate_string(&b.name, 40),
            url: truncate_string(&b.url, 50),
            category: b.category.to_string(),
            folder: truncate_string(&b.folder_path, 30),
        })
        .collect()
}

/// Filter bookmarks by category
pub fn filter_by_category(bookmarks: &[Bookmark], category: &str) -> Vec<BookmarkTableEntry> {
    let category_lower = category.to_lowercase();

    bookmarks
        .iter()
        .filter(|b| {
            b.category
                .to_string()
                .to_lowercase()
                .contains(&category_lower)
        })
        .map(|b| BookmarkTableEntry {
            title: truncate_string(&b.name, 40),
            url: truncate_string(&b.url, 50),
            category: b.category.to_string(),
            folder: truncate_string(&b.folder_path, 30),
        })
        .collect()
}

/// Filter bookmarks by domain
pub fn filter_by_domain(bookmarks: &[Bookmark], domain: &str) -> Vec<BookmarkTableEntry> {
    let domain_lower = domain.to_lowercase();

    bookmarks
        .iter()
        .filter(|b| extract_domain(&b.url).contains(&domain_lower))
        .map(|b| BookmarkTableEntry {
            title: truncate_string(&b.name, 40),
            url: truncate_string(&b.url, 50),
            category: b.category.to_string(),
            folder: truncate_string(&b.folder_path, 30),
        })
        .collect()
}

/// Export bookmarks to markdown
pub fn export_to_markdown(bookmarks: &[Bookmark], output_path: Option<&str>) -> Result<String> {
    let mut md = String::new();

    md.push_str("# Chrome Bookmarks Export\n\n");
    md.push_str(&format!("*Exported on: {}*\n\n", chrono_lite_now()));
    md.push_str(&format!("**Total bookmarks: {}**\n\n", bookmarks.len()));

    // Group by category
    let mut by_category: HashMap<String, Vec<&Bookmark>> = HashMap::new();
    for bookmark in bookmarks {
        by_category
            .entry(bookmark.category.folder_name().to_string())
            .or_default()
            .push(bookmark);
    }

    // Sort categories
    let mut categories: Vec<_> = by_category.keys().cloned().collect();
    categories.sort();

    for category in categories {
        if let Some(bms) = by_category.get(&category) {
            md.push_str(&format!("## {}\n\n", category));
            for bookmark in bms {
                md.push_str(&format!("- [{}]({})\n", bookmark.name, bookmark.url));
            }
            md.push('\n');
        }
    }

    if let Some(path) = output_path {
        fs::write(path, &md).with_context(|| format!("Failed to write to {}", path))?;
        println!("{} Exported to: {}", "✅".green(), path.cyan());
    }

    Ok(md)
}

/// Simple date/time function (avoiding chrono dependency)
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Simple date calculation (approximate, good enough for display)
    let days = secs / 86400;
    let years = 1970 + (days / 365);
    let remaining_days = days % 365;
    let months = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;

    format!("{}-{:02}-{:02}", years, months.min(12), day.min(31))
}

/// Truncate a string to a maximum length
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Interactive bookmark selector for organization
pub fn interactive_organize(bookmarks: &[Bookmark]) -> Result<Vec<OrganizeSuggestion>> {
    let suggestions = get_organize_suggestions(bookmarks);

    if suggestions.is_empty() {
        println!("{}", "All bookmarks are already well-organized!".green());
        return Ok(vec![]);
    }

    println!(
        "\n{} Found {} bookmarks that could be better organized\n",
        "📋".cyan(),
        suggestions.len().to_string().yellow()
    );

    // For now, return all suggestions - interactive mode would be implemented similarly to organizer.rs
    Ok(suggestions)
}
