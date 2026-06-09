use leptos::prelude::*;
use crate::components::bento::{BentoGrid, BentoBlock};
use crate::components::terminal::TerminalMock;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero Section
            <section class="relative min-h-[72vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-4xl mx-auto flex flex-col items-center">
                    <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                        "Dominate "
                        <br class="md:hidden" />
                        <span class="animate-shimmer">
                            "AI Search"
                        </span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-3xl mx-auto leading-relaxed mb-8">
                        "AI Search Optimization & premium web systems to ensure your business and products are recommended by ChatGPT, Claude, and Perplexity."
                    </p>
                </div>
                
                // Animated Scroll Indicator
                <div class="animate-scroll-cue flex flex-col items-center gap-2 text-slate-500">
                    <span class="text-xs font-mono uppercase tracking-widest text-slate-600">"Scroll to explore"</span>
                    <svg class="w-5 h-5 text-cyan-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
                    </svg>
                </div>
            </section>

            <BentoGrid>
                // Module A: The "whoami" Chief Architect Card
                <BentoBlock class="md:col-span-2 lg:col-span-1 row-span-2 flex flex-col justify-between no-hover">
                    <div>
                        <div class="flex items-center gap-3 mb-4">
                            <img src="/pfp.JPG" alt="Vicente" class="h-10 w-10 rounded-full object-cover border border-cyan-500/30" />
                            <div>
                                <h3 class="text-xl font-bold text-slate-100">"Vicente"</h3>
                                <p class="text-sm text-cyan-400">"Chief Architect"</p>
                            </div>
                        </div>
                        <div class="inline-block px-3 py-1 bg-emerald-950/50 border border-emerald-800/50 rounded-full text-xs text-emerald-400 font-mono mb-4">
                            "whoami // AI Search Visibility"
                        </div>
                        <p class="text-slate-300 leading-relaxed mb-6">
                            "Artorias Tech Lab doesn't do cookie-cutter marketing. We structure your website, content, and digital presence so that AI search engines (like ChatGPT, Claude, and Perplexity) can easily read, trust, and recommend your business to potential customers."
                        </p>
                        <p class="text-slate-300 leading-relaxed mb-6">
                            "You work directly with a senior digital architect to format your brand's information and put your products in front of the AI search tools driving modern sales."
                        </p>
                    </div>
                    
                    <div class="mt-8 border-t border-slate-800/60 pt-6">
                        <div class="text-xs text-slate-500 font-mono mb-2">"// System Status"</div>
                        <div class="flex items-center gap-2 text-sm text-emerald-400">
                            <div class="h-2 w-2 rounded-full bg-emerald-400 animate-pulse"></div>
                            "Accepting new clients"
                        </div>
                    </div>
                </BentoBlock>

                // Module B: Dual-Track Renivel SaaS Feature
                <BentoBlock class="md:col-span-2 lg:col-span-2 row-span-2 no-hover">
                    <div class="flex flex-col md:flex-row justify-between items-start md:items-center mb-6">
                        <div>
                            <h3 class="text-2xl font-bold text-slate-100 mb-2">"Renivel SaaS // Custom Software Showcase"</h3>
                            <p class="text-slate-300 max-w-xl">"A custom web platform designed and engineered entirely in-house to demonstrate our full-stack software development capabilities."</p>
                        </div>
                        <div class="mt-4 md:mt-0 px-3 py-1 rounded-full bg-cyan-900/30 text-cyan-400 text-xs font-mono border border-cyan-800/50">
                            "Production SaaS"
                        </div>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
                        <div class="space-y-4">
                            <div class="p-4 rounded-xl bg-slate-950/50 border border-slate-800">
                                <h4 class="font-medium text-slate-200 mb-1">"Production-Grade Build"</h4>
                                <p class="text-sm text-slate-400">"Renivel is a custom web application built for contractor teams to track field logistics, capture receipt data on-site, and audit real-time profit margins."</p>
                            </div>
                            <div class="p-4 rounded-xl bg-slate-950/50 border border-slate-800">
                                <h4 class="font-medium text-slate-200 mb-1">"High-Performance Infrastructure"</h4>
                                <p class="text-sm text-slate-400">"Under the hood, it features automated data backup pipelines, real-time database synchronization, and ultra-fast page load speeds."</p>
                            </div>
                        </div>
                        <div class="flex flex-col items-center justify-center gap-6">
                            <TerminalMock 
                                command="remodeling_erp --status" 
                                output="[+] Real-time budget ledger sync: Active\n[+] Database backup pipelines: Healthy\n[+] High-performance client portals: Online\n\nAll platforms running at peak performance."
                            />
                            <a href="/products/renivel" class="text-sm font-bold text-cyan-400 hover:text-cyan-300 underline decoration-cyan-900 underline-offset-4 transition-colors">
                                "Explore Renivel ->"
                            </a>
                        </div>
                    </div>
                </BentoBlock>

                // Module C: Aggressive Signs "IT Rescue" Case Study
                <BentoBlock class="md:col-span-2 lg:col-span-3 no-hover">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 items-center">
                        <div class="md:col-span-1 flex justify-center">
                            // Grayscale SVG placeholder transitioning to brand color
                            <div class="h-32 w-32 rounded-2xl bg-rose-600 flex items-center justify-center">
                                <span class="font-black text-3xl text-white">"AS"</span>
                            </div>
                        </div>
                        <div class="md:col-span-2">
                            <div class="flex items-center gap-2 mb-3">
                                <span class="px-2 py-1 rounded text-[10px] uppercase tracking-wider font-bold bg-rose-500/20 text-rose-400">"The IT Rescue Routine"</span>
                                <h3 class="text-xl font-bold text-slate-100">"Aggressive Signs"</h3>
                            </div>
                            <p class="text-slate-300 leading-relaxed mb-4">
                                <span class="font-semibold text-white">"Your business data belongs to you—not your IT provider."</span>
                                " When we partnered with Aggressive Signs, they were held hostage by an unresponsive agency running their critical operations on a locked-down legacy server box."
                            </p>
                            <p class="text-slate-300 leading-relaxed mb-4">
                                "We completely migrated their infrastructure to a high-performance local NAS and VPN server that they own outright, transitioned their team seamlessly to Google Workspace, and optimized their local design workstation hardware."
                            </p>
                            <div class="mt-4 p-4 border border-rose-900/30 bg-rose-950/10 rounded-lg">
                                <p class="text-sm font-mono text-rose-300">
                                    "No artificial software locks. No 3-week wait times for a support ticket. Just bulletproof systems and direct, same-day responses straight from the engineer."
                                </p>
                            </div>
                        </div>
                    </div>
                </BentoBlock>

                // Module D: Celeritech "AI Invoice Processing" Case Study
                <BentoBlock class="md:col-span-2 lg:col-span-3 no-hover">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 items-center">
                        <div class="md:col-span-1 flex justify-center">
                            // Grayscale SVG placeholder transitioning to brand color
                            <div class="h-32 w-32 rounded-2xl bg-indigo-600 flex items-center justify-center">
                                <span class="font-black text-3xl text-white">"CT"</span>
                            </div>
                        </div>
                        <div class="md:col-span-2">
                            <div class="flex items-center gap-2 mb-3">
                                <span class="px-2 py-1 rounded text-[10px] uppercase tracking-wider font-bold bg-indigo-500/20 text-indigo-400">"AI Automation Case Study"</span>
                                <h3 class="text-xl font-bold text-slate-100">"Celeritech"</h3>
                            </div>
                            <p class="text-slate-300 leading-relaxed mb-4">
                                <span class="font-semibold text-white">"Manual invoice entry is a slow, costly bottleneck for growing companies."</span>
                                " When we partnered with Celeritech, their accounting team was spending hours every day manually opening, reviewing, and typing details from vendor invoices into their databases."
                            </p>
                            <p class="text-slate-300 leading-relaxed mb-4">
                                "We engineered a secure, event-driven AI invoice processing platform. The system automatically reads incoming invoices (PDFs and images), uses advanced language models to instantly extract and validate structured line-item details, and securely synchronizes the results directly to their systems—cutting processing times by over 90%."
                            </p>
                            <div class="mt-4 p-4 border border-indigo-900/30 bg-indigo-950/10 rounded-lg">
                                <p class="text-sm font-mono text-indigo-300">
                                    "From hours of manual sorting to instant, hands-free extraction. Celeritech now processes hundreds of vendor invoices with complete accuracy and zero manual data entry."
                                </p>
                            </div>
                        </div>
                    </div>
                </BentoBlock>
            </BentoGrid>

            // FAQ / Explanation Section
            <section class="max-w-4xl mx-auto mt-24 px-4 bento-scroll-card">
                <div class="text-center mb-16">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-950/30 border border-cyan-900/50 text-cyan-400 text-sm font-mono mb-6">
                        <span class="w-2 h-2 rounded-full bg-cyan-400 animate-pulse"></span>
                        "THE NEW SEARCH PARADIGM"
                    </div>
                    <h2 class="text-3xl md:text-5xl font-extrabold text-slate-100 mb-6 uppercase tracking-tight">
                        "What is " <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-emerald-400">"AI Search Optimization"</span> "?"
                    </h2>
                    <p class="text-lg text-slate-400 max-w-2xl mx-auto leading-relaxed">
                        "As the web shifts from lists of links to direct AI answers, standard SEO is no longer enough. Here is how we ensure your business remains visible."
                    </p>
                </div>

                <div class="flex flex-col gap-6">
                    // Q1
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-cyan-500/30 transition-all duration-300">
                        <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-cyan-400">
                            "01 // What is AI Search Optimization?"
                        </h3>
                        <p class="text-slate-300 leading-relaxed">
                            "Traditional SEO ranks your website links on search engines like Google. AI Search Optimization is the process of formatting your website content and digital presence so AI engines (like ChatGPT, Claude, Gemini, and Perplexity) can easily understand, cite, and recommend your brand to users."
                        </p>
                    </div>

                    // Q2
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-cyan-500/30 transition-all duration-300">
                        <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-cyan-400">
                            "02 // Why is this the driving technology for modern sales?"
                        </h3>
                        <p class="text-slate-300 leading-relaxed">
                            "More and more customers are bypassing search engines to ask AI engines direct questions like: 'Who is the best local sign manufacturer?' or 'What is the easiest budgeting software for my team?' If your business isn't optimized for AI search, you won't be in the recommendation. Being recommended by AI is now the single most critical way to capture new sales."
                        </p>
                    </div>

                    // Q3
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-cyan-500/30 transition-all duration-300">
                        <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-cyan-400">
                            "03 // How do you optimize a website for AI models?"
                        </h3>
                        <p class="text-slate-300 leading-relaxed">
                            "AI engines require clean, fast, and structured information. We replace slow, bloated websites with high-performance pages and organize your data clearly. This ensures AI crawlers can scan and index your site instantly without getting stuck or timing out."
                        </p>
                    </div>

                    // Q4
                    <div class="border border-slate-800/80 bg-slate-900/40 rounded-2xl p-8 hover:border-cyan-500/30 transition-all duration-300">
                        <h3 class="text-xl font-bold text-slate-100 mb-3 font-mono text-cyan-400">
                            "04 // Does this replace traditional search optimization?"
                        </h3>
                        <p class="text-slate-300 leading-relaxed">
                            "No, it works hand-in-hand with it. The same improvements that make a website easy for AI engines to read—fast load speeds, clear layouts, and high-quality content—are exactly what Google looks for to rank you at the top. By optimizing for AI search, you win on both fronts."
                        </p>
                    </div>
                </div>
            </section>

            
        </div>
    }
}
