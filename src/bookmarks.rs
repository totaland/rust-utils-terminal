use anyhow::{Context, Result};
use colored::Colorize;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
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
    // Development Subcategories
    DevGeneral,
    DevReact,
    DevPython,
    DevJava,
    DevRust,
    DevJavaScript,
    DevTypeScript,
    DevCSS,
    DevKubernetes,
    DevDocker,
    DevPostgres,
    DevDatabase,
    DevAWS,
    DevServerless,
    DevWebTech,
    DevMobile,
    DevGit,
    DevDevOps,
    DevAPI,
    // Finance Subcategories
    FinanceGeneral,
    FinanceCrypto,
    FinanceTrading,
    FinancePersonal,
    // Personal Development
    PersonalDevelopment,
    // General Categories
    Social,
    News,
    Shopping,
    Entertainment,
    Education,
    Reference,
    Tools,
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
            // Development Subcategories
            BookmarkCategory::DevGeneral => write!(f, "🛠️  Dev/General"),
            BookmarkCategory::DevReact => write!(f, "⚛️  Dev/React"),
            BookmarkCategory::DevPython => write!(f, "🐍 Dev/Python"),
            BookmarkCategory::DevJava => write!(f, "☕ Dev/Java"),
            BookmarkCategory::DevRust => write!(f, "🦀 Dev/Rust"),
            BookmarkCategory::DevJavaScript => write!(f, "🟨 Dev/JavaScript"),
            BookmarkCategory::DevTypeScript => write!(f, "🔷 Dev/TypeScript"),
            BookmarkCategory::DevCSS => write!(f, "🎨 Dev/CSS"),
            BookmarkCategory::DevKubernetes => write!(f, "☸️  Dev/Kubernetes"),
            BookmarkCategory::DevDocker => write!(f, "🐳 Dev/Docker"),
            BookmarkCategory::DevPostgres => write!(f, "🐘 Dev/PostgreSQL"),
            BookmarkCategory::DevDatabase => write!(f, "🗃️  Dev/Database"),
            BookmarkCategory::DevAWS => write!(f, "☁️  Dev/AWS"),
            BookmarkCategory::DevServerless => write!(f, "⚡ Dev/Serverless"),
            BookmarkCategory::DevWebTech => write!(f, "🌐 Dev/WebTech"),
            BookmarkCategory::DevMobile => write!(f, "📱 Dev/Mobile"),
            BookmarkCategory::DevGit => write!(f, "📦 Dev/Git"),
            BookmarkCategory::DevDevOps => write!(f, "🔧 Dev/DevOps"),
            BookmarkCategory::DevAPI => write!(f, "🔌 Dev/API"),
            // Finance Subcategories
            BookmarkCategory::FinanceGeneral => write!(f, "💰 Finance/General"),
            BookmarkCategory::FinanceCrypto => write!(f, "₿ Finance/Crypto"),
            BookmarkCategory::FinanceTrading => write!(f, "📈 Finance/Trading"),
            BookmarkCategory::FinancePersonal => write!(f, "💵 Finance/Personal"),
            // Personal Development
            BookmarkCategory::PersonalDevelopment => write!(f, "🌱 Personal Development"),
            // General Categories
            BookmarkCategory::Social => write!(f, "👥 Social"),
            BookmarkCategory::News => write!(f, "📰 News"),
            BookmarkCategory::Shopping => write!(f, "🛒 Shopping"),
            BookmarkCategory::Entertainment => write!(f, "🎬 Entertainment"),
            BookmarkCategory::Education => write!(f, "📚 Education"),
            BookmarkCategory::Reference => write!(f, "📖 Reference"),
            BookmarkCategory::Tools => write!(f, "🔧 Tools"),
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
            // Development Subcategories
            BookmarkCategory::DevGeneral => "Development/General",
            BookmarkCategory::DevReact => "Development/React",
            BookmarkCategory::DevPython => "Development/Python",
            BookmarkCategory::DevJava => "Development/Java",
            BookmarkCategory::DevRust => "Development/Rust",
            BookmarkCategory::DevJavaScript => "Development/JavaScript",
            BookmarkCategory::DevTypeScript => "Development/TypeScript",
            BookmarkCategory::DevCSS => "Development/CSS",
            BookmarkCategory::DevKubernetes => "Development/Kubernetes",
            BookmarkCategory::DevDocker => "Development/Docker",
            BookmarkCategory::DevPostgres => "Development/PostgreSQL",
            BookmarkCategory::DevDatabase => "Development/Database",
            BookmarkCategory::DevAWS => "Development/AWS",
            BookmarkCategory::DevServerless => "Development/Serverless",
            BookmarkCategory::DevWebTech => "Development/WebTech",
            BookmarkCategory::DevMobile => "Development/Mobile",
            BookmarkCategory::DevGit => "Development/Git",
            BookmarkCategory::DevDevOps => "Development/DevOps",
            BookmarkCategory::DevAPI => "Development/API",
            // Finance Subcategories
            BookmarkCategory::FinanceGeneral => "Finance/General",
            BookmarkCategory::FinanceCrypto => "Finance/Crypto",
            BookmarkCategory::FinanceTrading => "Finance/Trading",
            BookmarkCategory::FinancePersonal => "Finance/Personal",
            // Personal Development
            BookmarkCategory::PersonalDevelopment => "Personal Development",
            // General Categories
            BookmarkCategory::Social => "Social Media",
            BookmarkCategory::News => "News",
            BookmarkCategory::Shopping => "Shopping",
            BookmarkCategory::Entertainment => "Entertainment",
            BookmarkCategory::Education => "Education",
            BookmarkCategory::Reference => "Reference",
            BookmarkCategory::Tools => "Tools & Utilities",
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
        // Finance Subcategories (check before Development)
        // ============================================

        // Crypto (check first - most specific)
        if url_lower.contains("coinbase.com")
            || url_lower.contains("binance.com")
            || url_lower.contains("kraken.com")
            || url_lower.contains("gemini.com")
            || url_lower.contains("ftx.com")
            || url_lower.contains("kucoin.com")
            || url_lower.contains("huobi")
            || url_lower.contains("okx.com")
            || url_lower.contains("bybit.com")
            || url_lower.contains("bitstamp")
            || url_lower.contains("bitfinex")
            || url_lower.contains("bitmex")
            || url_lower.contains("coinmarketcap.com")
            || url_lower.contains("coingecko.com")
            || url_lower.contains("tradingview.com")
            || url_lower.contains("dextools.io")
            || url_lower.contains("etherscan.io")
            || url_lower.contains("bscscan.com")
            || url_lower.contains("polygonscan.com")
            || url_lower.contains("uniswap")
            || url_lower.contains("sushiswap")
            || url_lower.contains("pancakeswap")
            || url_lower.contains("metamask.io")
            || url_lower.contains("opensea.io")
            || url_lower.contains("rarible.com")
            || url_lower.contains("looksrare")
            || combined.contains("bitcoin")
            || combined.contains("btc ")
            || combined.contains("ethereum")
            || combined.contains("eth ")
            || combined.contains("crypto")
            || combined.contains("blockchain")
            || combined.contains("defi")
            || combined.contains("nft")
            || combined.contains("ico ")
            || combined.contains("token sale")
            || combined.contains("airdrop")
            || combined.contains("staking")
            || combined.contains("yield farming")
            || combined.contains("liquidity pool")
            || combined.contains("smart contract")
            || combined.contains("wallet")
                && (combined.contains("crypto")
                    || combined.contains("bitcoin")
                    || combined.contains("ethereum"))
            || combined.contains("exchange")
                && (combined.contains("crypto")
                    || combined.contains("coin")
                    || combined.contains("token"))
            || combined.contains("altcoin")
            || combined.contains("memecoin")
            || combined.contains("chart pattern")
            || combined.contains("candlestick")
            || combined.contains("trading signal")
            || combined.contains("technical analysis")
                && (combined.contains("crypto") || combined.contains("coin"))
            || combined.contains("solana")
            || combined.contains("cardano")
            || combined.contains("polkadot")
            || combined.contains("avalanche")
            || combined.contains("polygon") && !combined.contains("css")
            || combined.contains("arbitrum")
            || combined.contains("optimism")
            || combined.contains("layer 2")
            || combined.contains("web3")
            || combined.contains("dapp")
            || combined.contains("decentralized")
        {
            return BookmarkCategory::FinanceCrypto;
        }

        // Trading (stocks, forex, etc.)
        if url_lower.contains("robinhood.com")
            || url_lower.contains("etrade.com")
            || url_lower.contains("tdameritrade.com")
            || url_lower.contains("thinkorswim")
            || url_lower.contains("interactivebrokers")
            || url_lower.contains("stockcharts.com")
            || url_lower.contains("finviz.com")
            || url_lower.contains("yahoo.com/finance")
            || url_lower.contains("finance.yahoo.com")
            || url_lower.contains("marketwatch.com")
            || url_lower.contains("seekingalpha.com")
            || url_lower.contains("investopedia.com")
            || url_lower.contains("morningstar.com")
            || combined.contains("stock market")
            || combined.contains("stock trading")
            || combined.contains("forex")
            || combined.contains("options trading")
            || combined.contains("futures trading")
            || combined.contains("dividend")
            || combined.contains("portfolio") && combined.contains("invest")
            || combined.contains("market analysis")
            || combined.contains("bull market")
            || combined.contains("bear market")
            || combined.contains("earnings report")
            || combined.contains("etf ")
            || combined.contains("index fund")
        {
            return BookmarkCategory::FinanceTrading;
        }

        // Personal Finance
        if url_lower.contains("mint.com")
            || url_lower.contains("ynab.com")
            || url_lower.contains("personalcapital.com")
            || url_lower.contains("creditkarma.com")
            || url_lower.contains("nerdwallet.com")
            || url_lower.contains("bankrate.com")
            || combined.contains("budget")
            || combined.contains("saving money")
            || combined.contains("retirement")
            || combined.contains("401k")
            || combined.contains("ira ")
            || combined.contains("credit score")
            || combined.contains("credit card") && !combined.contains("api")
            || combined.contains("mortgage")
            || combined.contains("debt")
            || combined.contains("tax return")
            || combined.contains("net worth")
            || combined.contains("financial planning")
            || combined.contains("emergency fund")
        {
            return BookmarkCategory::FinancePersonal;
        }

        // General Finance (catch-all)
        if url_lower.contains("bank")
            || url_lower.contains("paypal.com")
            || url_lower.contains("venmo.com")
            || url_lower.contains("fidelity.com")
            || url_lower.contains("schwab.com")
            || url_lower.contains("vanguard.com")
            || url_lower.contains("finance.")
            || combined.contains("invest") && !combined.contains("investigate")
            || combined.contains("financial")
        {
            return BookmarkCategory::FinanceGeneral;
        }

        // ============================================
        // Personal Development (check before Development)
        // ============================================
        if combined.contains("habit")
            || combined.contains("productivity")
                && !combined.contains("developer")
                && !combined.contains("tool")
            || combined.contains("self improvement")
            || combined.contains("self-improvement")
            || combined.contains("personal growth")
            || combined.contains("motivation")
            || combined.contains("mindset")
            || combined.contains("goal setting")
            || combined.contains("time management") && !combined.contains("project")
            || combined.contains("life hack")
            || combined.contains("morning routine")
            || combined.contains("meditation")
            || combined.contains("mindfulness")
            || combined.contains("journaling")
            || combined.contains("gratitude")
            || combined.contains("stoicism")
            || combined.contains("atomic habits")
            || combined.contains("deep work")
            || combined.contains("getting things done")
            || combined.contains("gtd ")
            || combined.contains("pomodoro")
            || combined.contains("procrastination")
            || combined.contains("discipline")
            || combined.contains("self help")
            || combined.contains("self-help")
            || combined.contains("memory technique")
            || combined.contains("speed reading")
            || combined.contains("learning how to learn")
            || combined.contains("career growth")
            || combined.contains("public speaking")
            || combined.contains("emotional intelligence")
        {
            return BookmarkCategory::PersonalDevelopment;
        }

        // ============================================
        // Other General Categories (check before Development)
        // ============================================

        // Shopping (check early to avoid catching in Development)
        if url_lower.contains("amazon.")
            || url_lower.contains("ebay.")
            || url_lower.contains("etsy.com")
            || url_lower.contains("aliexpress.com")
            || url_lower.contains("walmart.com")
            || url_lower.contains("target.com")
            || url_lower.contains("bestbuy.com")
            || url_lower.contains("newegg.com")
            || url_lower.contains("/cart")
            || url_lower.contains("/checkout")
            || combined.contains("buy now")
            || combined.contains("add to cart")
            || combined.contains("shopping")
            || combined.contains("discount code")
            || combined.contains("coupon")
        {
            return BookmarkCategory::Shopping;
        }

        // Video (check early)
        if url_lower.contains("youtube.com")
            || url_lower.contains("youtu.be")
            || url_lower.contains("vimeo.com")
            || url_lower.contains("dailymotion.com")
            || url_lower.contains("twitch.tv")
        {
            return BookmarkCategory::Video;
        }

        // Social Media (check early)
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
            || url_lower.contains(".edu")
            || url_lower.contains("learn.")
            || combined.contains("online course")
            || combined.contains("free course")
        {
            return BookmarkCategory::Education;
        }

        // ============================================
        // Development Subcategories
        // ============================================

        // React / React Native
        if url_lower.contains("reactjs.org")
            || url_lower.contains("react.dev")
            || url_lower.contains("reactnative.dev")
            || combined.contains("react")
                && (combined.contains("component")
                    || combined.contains("hook")
                    || combined.contains("redux")
                    || combined.contains("nextjs")
                    || combined.contains("next.js")
                    || combined.contains("gatsby")
                    || combined.contains("jsx")
                    || combined.contains("state management"))
            || combined.contains("react native")
            || combined.contains("expo")
            || url_lower.contains("nextjs.org")
            || combined.contains("use effect")
            || combined.contains("usestate")
            || combined.contains("usememo")
            || combined.contains("zustand")
            || combined.contains("tanstack")
            || combined.contains("react query")
        {
            return BookmarkCategory::DevReact;
        }

        // Python
        if url_lower.contains("python.org")
            || url_lower.contains("pypi.org")
            || combined.contains("python")
                && (combined.contains("pip")
                    || combined.contains("django")
                    || combined.contains("flask")
                    || combined.contains("fastapi")
                    || combined.contains("pandas")
                    || combined.contains("numpy")
                    || combined.contains("jupyter")
                    || combined.contains("anaconda")
                    || combined.contains("virtualenv")
                    || combined.contains("poetry"))
            || url_lower.contains("django")
            || url_lower.contains("flask")
            || url_lower.contains("fastapi")
            || combined.contains("pydantic")
            || combined.contains("pytest")
        {
            return BookmarkCategory::DevPython;
        }

        // Rust
        if url_lower.contains("rust-lang.org")
            || url_lower.contains("crates.io")
            || combined.contains("rust")
                && (combined.contains("cargo")
                    || combined.contains("rustup")
                    || combined.contains("tokio")
                    || combined.contains("actix")
                    || combined.contains("wasm")
                    || combined.contains("serde"))
            || combined.contains("rustacean")
        {
            return BookmarkCategory::DevRust;
        }

        // Java / Kotlin / JVM
        if combined.contains("java")
            && (combined.contains("spring")
                || combined.contains("maven")
                || combined.contains("gradle")
                || combined.contains("jvm")
                || combined.contains("hibernate")
                || combined.contains("junit"))
            || combined.contains("kotlin")
            || url_lower.contains("spring.io")
            || combined.contains("springboot")
            || combined.contains("spring boot")
        {
            return BookmarkCategory::DevJava;
        }

        // TypeScript
        if url_lower.contains("typescriptlang.org")
            || combined.contains("typescript")
                && (combined.contains("type")
                    || combined.contains("interface")
                    || combined.contains("generic")
                    || combined.contains("tsc"))
            || combined.contains(".ts ")
            || combined.contains(".tsx")
        {
            return BookmarkCategory::DevTypeScript;
        }

        // JavaScript (general)
        if url_lower.contains("nodejs.org")
            || url_lower.contains("npmjs.com")
            || combined.contains("javascript")
            || combined.contains("node.js")
            || combined.contains("nodejs")
            || combined.contains("npm ")
            || combined.contains("yarn ")
            || combined.contains("pnpm")
            || combined.contains("deno")
            || combined.contains("bun ")
            || combined.contains("express.js")
            || combined.contains("expressjs")
            || combined.contains("es6")
            || combined.contains("ecmascript")
            || combined.contains("async await")
            || combined.contains("promise")
        {
            return BookmarkCategory::DevJavaScript;
        }

        // CSS / Styling
        if combined.contains("css")
            || combined.contains("tailwind")
            || combined.contains("sass")
            || combined.contains("scss")
            || combined.contains("less ")
            || combined.contains("styled-component")
            || combined.contains("bootstrap")
            || combined.contains("material ui")
            || combined.contains("chakra ui")
            || combined.contains("flexbox")
            || combined.contains("grid layout")
            || combined.contains("animation")
            || combined.contains("responsive design")
            || url_lower.contains("csswizardry")
            || url_lower.contains("css-tricks")
        {
            return BookmarkCategory::DevCSS;
        }

        // Kubernetes
        if url_lower.contains("kubernetes.io")
            || combined.contains("kubernetes")
            || combined.contains("k8s")
            || combined.contains("kubectl")
            || combined.contains("helm ")
            || combined.contains("helm chart")
            || combined.contains("minikube")
            || combined.contains("kind cluster")
            || combined.contains("pod ")
            || combined.contains("deployment") && combined.contains("container")
            || combined.contains("service mesh")
            || combined.contains("istio")
            || combined.contains("ingress")
        {
            return BookmarkCategory::DevKubernetes;
        }

        // Docker
        if url_lower.contains("docker.com")
            || url_lower.contains("hub.docker.com")
            || combined.contains("docker")
            || combined.contains("dockerfile")
            || combined.contains("container") && !combined.contains("kubernetes")
            || combined.contains("docker-compose")
            || combined.contains("podman")
        {
            return BookmarkCategory::DevDocker;
        }

        // PostgreSQL
        if url_lower.contains("postgresql.org")
            || combined.contains("postgresql")
            || combined.contains("postgres")
            || combined.contains("psql")
            || combined.contains("pg_")
        {
            return BookmarkCategory::DevPostgres;
        }

        // Database (general)
        if combined.contains("mysql")
            || combined.contains("mongodb")
            || combined.contains("redis")
            || combined.contains("elasticsearch")
            || combined.contains("sqlite")
            || combined.contains("dynamodb")
            || combined.contains("cassandra")
            || combined.contains("sql ")
            || combined.contains("nosql")
            || combined.contains("database")
            || combined.contains("query optimization")
            || combined.contains("orm ")
            || combined.contains("prisma")
            || combined.contains("drizzle")
        {
            return BookmarkCategory::DevDatabase;
        }

        // AWS
        if url_lower.contains("aws.amazon.com")
            || combined.contains("aws ")
            || combined.contains("amazon web services")
            || combined.contains("lambda") && combined.contains("aws")
            || combined.contains("ec2")
            || combined.contains("s3 bucket")
            || combined.contains("cloudformation")
            || combined.contains("cloudwatch")
            || combined.contains("dynamodb")
            || combined.contains("sqs ")
            || combined.contains("sns ")
            || combined.contains("iam ") && combined.contains("aws")
            || combined.contains("cdk") && combined.contains("aws")
        {
            return BookmarkCategory::DevAWS;
        }

        // Serverless
        if combined.contains("serverless")
            || combined.contains("lambda function")
            || combined.contains("cloud function")
            || combined.contains("azure function")
            || combined.contains("vercel") && combined.contains("function")
            || combined.contains("netlify function")
            || combined.contains("edge function")
            || combined.contains("faas")
            || url_lower.contains("serverless.com")
        {
            return BookmarkCategory::DevServerless;
        }

        // Git
        if url_lower.contains("github.com")
            || url_lower.contains("gitlab.com")
            || url_lower.contains("bitbucket.org")
            || combined.contains("git ")
            || combined.contains("gitflow")
            || combined.contains("pull request")
            || combined.contains("merge conflict")
            || combined.contains("branch") && combined.contains("git")
            || combined.contains("commit") && combined.contains("git")
            || combined.contains("rebase")
            || combined.contains("cherry-pick")
        {
            return BookmarkCategory::DevGit;
        }

        // DevOps / CI/CD
        if combined.contains("devops")
            || combined.contains("ci/cd")
            || combined.contains("cicd")
            || combined.contains("jenkins")
            || combined.contains("github actions")
            || combined.contains("gitlab ci")
            || combined.contains("circleci")
            || combined.contains("travis ci")
            || combined.contains("argo")
            || combined.contains("terraform")
            || combined.contains("ansible")
            || combined.contains("puppet")
            || combined.contains("chef ")
            || combined.contains("infrastructure as code")
            || combined.contains("monitoring")
            || combined.contains("prometheus")
            || combined.contains("grafana")
            || combined.contains("datadog")
            || combined.contains("sonarqube")
        {
            return BookmarkCategory::DevDevOps;
        }

        // Mobile Development
        if combined.contains("ios ")
            || combined.contains("android ")
            || combined.contains("swift")
            || combined.contains("swiftui")
            || combined.contains("xcode")
            || combined.contains("flutter")
            || combined.contains("dart ")
            || combined.contains("mobile app")
            || combined.contains("app store")
            || combined.contains("play store")
            || url_lower.contains("developer.apple.com")
            || url_lower.contains("developer.android.com")
        {
            return BookmarkCategory::DevMobile;
        }

        // Web Tech (general web development)
        if combined.contains("html")
            || combined.contains("dom ")
            || combined.contains("web component")
            || combined.contains("pwa")
            || combined.contains("progressive web")
            || combined.contains("service worker")
            || combined.contains("websocket")
            || combined.contains("http")
            || combined.contains("cors")
            || combined.contains("oauth")
            || combined.contains("jwt ")
            || combined.contains("rest api")
            || combined.contains("graphql")
            || combined.contains("grpc")
            || combined.contains("webpack")
            || combined.contains("vite")
            || combined.contains("esbuild")
            || combined.contains("rollup")
            || combined.contains("babel")
            || url_lower.contains("vuejs.org")
            || url_lower.contains("angular.io")
            || url_lower.contains("svelte.dev")
            || combined.contains("vue ")
            || combined.contains("angular")
            || combined.contains("svelte")
        {
            return BookmarkCategory::DevWebTech;
        }

        // API Development
        if combined.contains("api ")
            || combined.contains("rest ")
            || combined.contains("openapi")
            || combined.contains("swagger")
            || combined.contains("postman")
            || combined.contains("insomnia")
            || combined.contains("endpoint")
            || combined.contains("webhook")
        {
            return BookmarkCategory::DevAPI;
        }

        // General Development (catch-all)
        if url_lower.contains("stackoverflow.com")
            || url_lower.contains("stackexchange.com")
            || url_lower.contains("developer.")
            || url_lower.contains("docs.")
            || url_lower.contains("vercel.com")
            || url_lower.contains("netlify.com")
            || url_lower.contains("heroku.com")
            || url_lower.contains("cloud.google.com")
            || url_lower.contains("azure.microsoft.com")
            || url_lower.contains("codepen.io")
            || url_lower.contains("codesandbox.io")
            || url_lower.contains("replit.com")
            || url_lower.contains("jsfiddle.net")
            || url_lower.contains("medium.com") && combined.contains("programming")
            || url_lower.contains("dev.to")
            || url_lower.contains("hashnode.com")
            || combined.contains("documentation")
            || combined.contains("tutorial")
            || combined.contains("programming")
            || combined.contains("coding")
            || combined.contains("developer")
        {
            return BookmarkCategory::DevGeneral;
        }

        // ============================================
        // Remaining General Categories
        // ============================================

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

/// Export bookmarks to Chrome-compatible HTML format (Netscape Bookmark format)
/// This creates an organized bookmark file that can be imported into Chrome
pub fn export_to_chrome_html(bookmarks: &[Bookmark], output_path: Option<&str>) -> Result<String> {
    use std::collections::BTreeMap;

    // Group by category
    let mut by_category: BTreeMap<String, Vec<&Bookmark>> = BTreeMap::new();

    for bm in bookmarks {
        let folder = bm.category.folder_name().to_string();
        by_category.entry(folder).or_default().push(bm);
    }

    let mut html = String::new();

    // Netscape bookmark file header (required for Chrome import)
    html.push_str("<!DOCTYPE NETSCAPE-Bookmark-file-1>\n");
    html.push_str("<!-- This is an automatically generated file.\n");
    html.push_str("     It will be read and overwritten.\n");
    html.push_str("     DO NOT EDIT! -->\n");
    html.push_str("<META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=UTF-8\">\n");
    html.push_str("<TITLE>Bookmarks</TITLE>\n");
    html.push_str("<H1>Bookmarks</H1>\n");
    html.push_str("<DL><p>\n");

    // Bookmarks Bar folder (main import target)
    html.push_str("    <DT><H3 ADD_DATE=\"1\" LAST_MODIFIED=\"1\" PERSONAL_TOOLBAR_FOLDER=\"true\">Bookmarks bar</H3>\n");
    html.push_str("    <DL><p>\n");

    // Sort categories for consistent output
    let mut categories: Vec<_> = by_category.keys().collect();
    categories.sort();

    for category in categories {
        let bms = &by_category[category];

        // Create a folder for each category
        html.push_str(&format!(
            "        <DT><H3 ADD_DATE=\"1\" LAST_MODIFIED=\"1\">{}</H3>\n",
            html_escape(category)
        ));
        html.push_str("        <DL><p>\n");

        // Sort bookmarks within category by name
        let mut sorted_bms = bms.clone();
        sorted_bms.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        for bm in sorted_bms {
            let escaped_name = html_escape(&bm.name);
            let escaped_url = html_escape(&bm.url);

            html.push_str(&format!(
                "            <DT><A HREF=\"{}\" ADD_DATE=\"1\">{}</A>\n",
                escaped_url, escaped_name
            ));
        }

        html.push_str("        </DL><p>\n");
    }

    // Close Bookmarks bar folder
    html.push_str("    </DL><p>\n");

    // Close root
    html.push_str("</DL><p>\n");

    // Statistics comment
    let total_bookmarks: usize = by_category.values().map(|v| v.len()).sum();
    let total_categories = by_category.len();
    html.push_str(&format!(
        "<!-- Organized {} bookmarks into {} categories -->\n",
        total_bookmarks, total_categories
    ));

    if let Some(path) = output_path {
        fs::write(path, &html).with_context(|| format!("Failed to write to {}", path))?;
        println!(
            "\n{} Exported organized bookmarks to: {}",
            "✅".green(),
            path.cyan()
        );
        println!("\n{} To import into Chrome:", "📋".cyan());
        println!(
            "   1. Open Chrome and go to {} (or Cmd+Shift+O)",
            "chrome://bookmarks".yellow()
        );
        println!("   2. Click the three-dot menu (⋮) in the top right");
        println!(
            "   3. Select {} → {}",
            "Import bookmarks".yellow(),
            "Choose file".yellow()
        );
        println!("   4. Select the exported HTML file");
        println!(
            "\n{} Your bookmarks will be organized into {} category folders",
            "💡".yellow(),
            total_categories.to_string().green()
        );
    }

    Ok(html)
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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

/// Truncate a string to a maximum length (handles UTF-8 properly)
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len.saturating_sub(3)).collect();
        format!("{}...", truncated)
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

/// Entry for dead links table
#[derive(Tabled, Clone)]
pub struct DeadLinkEntry {
    #[tabled(rename = "Title")]
    pub title: String,
    #[tabled(rename = "URL")]
    pub url: String,
    #[tabled(rename = "Status")]
    pub status: String,
    #[tabled(rename = "Folder")]
    pub folder: String,
}

/// Check if a URL is dead (returns status code or error)
pub fn check_url_status(url: &str) -> (bool, String) {
    // Skip non-http URLs
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return (true, "skipped".to_string());
    }

    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
    {
        Ok(c) => c,
        Err(e) => return (false, format!("client error: {}", e)),
    };

    match client.head(url).send() {
        Ok(response) => {
            let status = response.status();
            if status.is_success() || status.is_redirection() {
                (true, status.to_string())
            } else if status == reqwest::StatusCode::METHOD_NOT_ALLOWED {
                // Some servers don't allow HEAD, try GET
                match client.get(url).send() {
                    Ok(resp) => {
                        let s = resp.status();
                        (s.is_success() || s.is_redirection(), s.to_string())
                    }
                    Err(e) => (false, format!("error: {}", e)),
                }
            } else {
                (false, status.to_string())
            }
        }
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("dns") || err_str.contains("resolve") {
                (false, "DNS error".to_string())
            } else if err_str.contains("timeout") {
                (false, "timeout".to_string())
            } else if err_str.contains("connection") {
                (false, "connection error".to_string())
            } else {
                (false, format!("error: {}", truncate_string(&err_str, 30)))
            }
        }
    }
}

/// Check for dead links in bookmarks (with parallel processing)
pub fn find_dead_links(bookmarks: &[Bookmark], verbose: bool) -> Vec<DeadLinkEntry> {
    let total = bookmarks.len();
    let checked = Arc::new(AtomicUsize::new(0));
    let dead_count = Arc::new(AtomicUsize::new(0));

    println!(
        "{} Checking {} bookmarks for dead links (this may take a while)...\n",
        "🔍".cyan(),
        total.to_string().yellow()
    );

    let dead_links: Vec<DeadLinkEntry> = bookmarks
        .par_iter()
        .filter_map(|bookmark| {
            let current = checked.fetch_add(1, Ordering::SeqCst) + 1;

            // Progress indicator every 50 bookmarks
            if current % 50 == 0 || current == total {
                print!(
                    "\r{} Progress: {}/{} checked, {} dead found",
                    "⏳".cyan(),
                    current.to_string().yellow(),
                    total.to_string().yellow(),
                    dead_count.load(Ordering::SeqCst).to_string().red()
                );
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }

            let (is_alive, status) = check_url_status(&bookmark.url);

            if verbose && !is_alive {
                println!(
                    "\n  {} {} - {}",
                    "❌".red(),
                    truncate_string(&bookmark.name, 40),
                    status.red()
                );
            }

            if !is_alive && status != "skipped" {
                dead_count.fetch_add(1, Ordering::SeqCst);
                Some(DeadLinkEntry {
                    title: truncate_string(&bookmark.name, 40),
                    url: truncate_string(&bookmark.url, 50),
                    status,
                    folder: truncate_string(&bookmark.folder_path, 25),
                })
            } else {
                None
            }
        })
        .collect();

    println!("\n"); // Clear the progress line
    dead_links
}

/// Duplicate bookmark info for removal
#[derive(Clone, Debug)]
pub struct DuplicateGroup {
    pub url: String,
    pub bookmarks: Vec<BookmarkInfo>,
}

#[derive(Clone, Debug)]
pub struct BookmarkInfo {
    pub id: String,
    pub name: String,
    pub folder_path: String,
}

/// Find duplicate bookmark groups with full info
pub fn find_duplicate_groups(bookmarks: &[Bookmark]) -> Vec<DuplicateGroup> {
    let mut url_map: HashMap<String, Vec<BookmarkInfo>> = HashMap::new();

    for bookmark in bookmarks {
        url_map
            .entry(bookmark.url.clone())
            .or_default()
            .push(BookmarkInfo {
                id: bookmark.id.clone(),
                name: bookmark.name.clone(),
                folder_path: bookmark.folder_path.clone(),
            });
    }

    url_map
        .into_iter()
        .filter(|(_, bms)| bms.len() > 1)
        .map(|(url, bookmarks)| DuplicateGroup { url, bookmarks })
        .collect()
}

/// Remove duplicates from the bookmarks file (keeps the first occurrence)
pub fn remove_duplicates(dry_run: bool, interactive: bool) -> Result<usize> {
    use crossterm::{
        cursor,
        event::{self, Event, KeyCode},
        execute,
        terminal::{self, ClearType},
    };
    use std::io::{Write, stdout};

    let path = get_chrome_bookmarks_path()?;
    let content = fs::read_to_string(&path)?;
    let mut json: serde_json::Value = serde_json::from_str(&content)?;

    // Parse bookmarks to find duplicates
    let (bookmarks, _) = parse_bookmarks()?;
    let duplicate_groups = find_duplicate_groups(&bookmarks);

    if duplicate_groups.is_empty() {
        println!("{}", "No duplicate bookmarks found!".green());
        return Ok(0);
    }

    println!(
        "\n{} Found {} duplicate URL groups ({} total duplicate entries)\n",
        "🔍".cyan(),
        duplicate_groups.len().to_string().yellow(),
        duplicate_groups
            .iter()
            .map(|g| g.bookmarks.len() - 1)
            .sum::<usize>()
            .to_string()
            .yellow()
    );

    // Collect IDs to remove (keep first occurrence, remove rest)
    let mut ids_to_remove: HashSet<String> = HashSet::new();
    let mut removal_details: Vec<(String, String, String)> = Vec::new(); // (url, name, folder)

    for group in &duplicate_groups {
        // Skip the first one (keep it), remove the rest
        for bookmark in group.bookmarks.iter().skip(1) {
            ids_to_remove.insert(bookmark.id.clone());
            removal_details.push((
                truncate_string(&group.url, 50),
                truncate_string(&bookmark.name, 30),
                truncate_string(&bookmark.folder_path, 25),
            ));
        }
    }

    if interactive {
        println!("{}", "Duplicates to be removed:".bold().cyan());
        println!("{}", "─".repeat(80).dimmed());

        for (i, (url, name, folder)) in removal_details.iter().enumerate().take(20) {
            println!(
                "  {}. {} - {} ({})",
                (i + 1).to_string().yellow(),
                name.cyan(),
                url.dimmed(),
                folder.magenta()
            );
        }

        if removal_details.len() > 20 {
            println!(
                "  ... and {} more",
                (removal_details.len() - 20).to_string().yellow()
            );
        }

        println!("\n{}", "─".repeat(80).dimmed());
        println!(
            "\n{} {} duplicates will be removed (keeping first occurrence of each URL)",
            "⚠️".yellow(),
            ids_to_remove.len().to_string().red()
        );
        print!(
            "\n{} Are you sure you want to proceed? [y/N]: ",
            "❓".cyan()
        );
        stdout().flush()?;

        terminal::enable_raw_mode()?;
        let confirmed = loop {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => break true,
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Enter => {
                        break false;
                    }
                    _ => {}
                }
            }
        };
        terminal::disable_raw_mode()?;
        println!();

        if !confirmed {
            println!("{}", "Operation cancelled.".yellow());
            return Ok(0);
        }
    }

    if dry_run {
        println!("\n{} Dry run - no changes made", "📋".cyan());
        println!("Would remove {} duplicate bookmarks:", ids_to_remove.len());
        for (url, name, folder) in removal_details.iter().take(10) {
            println!("  {} {} ({})", "•".red(), name, folder);
        }
        if removal_details.len() > 10 {
            println!("  ... and {} more", removal_details.len() - 10);
        }
        return Ok(ids_to_remove.len());
    }

    // Create backup
    let backup_path = format!("{}.backup", path.display());
    fs::copy(&path, &backup_path)?;
    println!("{} Backup created: {}", "💾".green(), backup_path.cyan());

    // Remove duplicates from JSON structure
    let removed_count = remove_bookmarks_by_id(&mut json, &ids_to_remove);

    // Write back to file
    let new_content = serde_json::to_string_pretty(&json)?;
    fs::write(&path, new_content)?;

    println!(
        "\n{} Removed {} duplicate bookmarks",
        "✅".green(),
        removed_count.to_string().yellow()
    );
    println!("{} Restart Chrome to see the changes", "💡".yellow());

    Ok(removed_count)
}

/// Recursively remove bookmarks by ID from JSON structure
fn remove_bookmarks_by_id(json: &mut serde_json::Value, ids_to_remove: &HashSet<String>) -> usize {
    let mut removed = 0;

    if let Some(obj) = json.as_object_mut() {
        if let Some(roots) = obj.get_mut("roots") {
            if let Some(roots_obj) = roots.as_object_mut() {
                for (_key, value) in roots_obj.iter_mut() {
                    removed += remove_from_node(value, ids_to_remove);
                }
            }
        }
    }

    removed
}

fn remove_from_node(node: &mut serde_json::Value, ids_to_remove: &HashSet<String>) -> usize {
    let mut removed = 0;

    if let Some(obj) = node.as_object_mut() {
        if let Some(children) = obj.get_mut("children") {
            if let Some(children_arr) = children.as_array_mut() {
                // First, recursively process children of folders
                for child in children_arr.iter_mut() {
                    removed += remove_from_node(child, ids_to_remove);
                }

                // Then remove marked bookmarks
                let original_len = children_arr.len();
                children_arr.retain(|child| {
                    if let Some(id) = child.get("id").and_then(|i| i.as_str()) {
                        !ids_to_remove.contains(id)
                    } else {
                        true
                    }
                });
                removed += original_len - children_arr.len();
            }
        }
    }

    removed
}

/// Remove dead links from bookmarks
pub fn remove_dead_links(
    dead_links: &[DeadLinkEntry],
    dry_run: bool,
    interactive: bool,
) -> Result<usize> {
    use crossterm::{
        event::{self, Event, KeyCode},
        terminal,
    };
    use std::io::{Write, stdout};

    if dead_links.is_empty() {
        println!("{}", "No dead links to remove!".green());
        return Ok(0);
    }

    let path = get_chrome_bookmarks_path()?;
    let content = fs::read_to_string(&path)?;
    let mut json: serde_json::Value = serde_json::from_str(&content)?;

    // We need to find bookmark IDs by URL
    let (bookmarks, _) = parse_bookmarks()?;
    let dead_urls: HashSet<String> = dead_links.iter().map(|d| d.url.clone()).collect();

    let ids_to_remove: HashSet<String> = bookmarks
        .iter()
        .filter(|b| {
            dead_urls.iter().any(|dead_url| {
                b.url.contains(dead_url.trim_end_matches("..."))
                    || dead_url.contains(
                        &truncate_string(&b.url, 50)
                            .trim_end_matches("...")
                            .to_string(),
                    )
            })
        })
        .map(|b| b.id.clone())
        .collect();

    if interactive {
        println!("\n{}", "Dead links to be removed:".bold().cyan());
        println!("{}", "─".repeat(80).dimmed());

        for (i, entry) in dead_links.iter().enumerate().take(20) {
            println!(
                "  {}. {} - {} [{}]",
                (i + 1).to_string().yellow(),
                entry.title.cyan(),
                entry.status.red(),
                entry.folder.dimmed()
            );
        }

        if dead_links.len() > 20 {
            println!(
                "  ... and {} more",
                (dead_links.len() - 20).to_string().yellow()
            );
        }

        println!("\n{}", "─".repeat(80).dimmed());
        println!(
            "\n{} {} dead links will be removed",
            "⚠️".yellow(),
            dead_links.len().to_string().red()
        );
        print!(
            "\n{} Are you sure you want to proceed? [y/N]: ",
            "❓".cyan()
        );
        stdout().flush()?;

        terminal::enable_raw_mode()?;
        let confirmed = loop {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => break true,
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Enter => {
                        break false;
                    }
                    _ => {}
                }
            }
        };
        terminal::disable_raw_mode()?;
        println!();

        if !confirmed {
            println!("{}", "Operation cancelled.".yellow());
            return Ok(0);
        }
    }

    if dry_run {
        println!("\n{} Dry run - no changes made", "📋".cyan());
        println!("Would remove {} dead links", dead_links.len());
        return Ok(dead_links.len());
    }

    // Create backup
    let backup_path = format!("{}.backup", path.display());
    fs::copy(&path, &backup_path)?;
    println!("{} Backup created: {}", "💾".green(), backup_path.cyan());

    // Remove dead links from JSON structure
    let removed_count = remove_bookmarks_by_id(&mut json, &ids_to_remove);

    // Write back to file
    let new_content = serde_json::to_string_pretty(&json)?;
    fs::write(&path, new_content)?;

    println!(
        "\n{} Removed {} dead links",
        "✅".green(),
        removed_count.to_string().yellow()
    );
    println!("{} Restart Chrome to see the changes", "💡".yellow());

    Ok(removed_count)
}
