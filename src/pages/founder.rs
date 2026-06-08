use leptos::prelude::*;
use crate::components::terminal::TerminalMock;

#[component]
pub fn FounderPage() -> impl IntoView {
    view! {
        <div class="max-w-6xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
            // Hero Section
            <section class="relative min-h-[72vh] flex flex-col justify-center items-center text-center px-4 mb-12">
                <div class="max-w-4xl mx-auto flex flex-col items-center">
                    <img src="/pfp.JPG" alt="Vicente" class="h-32 w-32 rounded-full object-cover border-2 border-emerald-500/30 shadow-[0_0_25px_rgba(52,211,153,0.15)] mb-6" />
                    <div class="inline-block px-4 py-1.5 bg-emerald-950/50 border border-emerald-800/50 rounded-full text-sm text-emerald-400 font-mono mb-8 shadow-[0_0_15px_rgba(52,211,153,0.1)]">
                        "whoami // Vicente - Elite Engineering. Zero Bureaucracy."
                    </div>
                    <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-8 leading-tight">
                        "I Build The Engine. "
                        <br class="hidden md:block" />
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-cyan-400">
                            "You Own The Keys."
                        </span>
                    </h1>
                    <p class="text-xl text-slate-300 max-w-3xl mx-auto leading-relaxed mb-8">
                        "Artorias Tech Lab doesn't have an account management team, a sales department, or a bloated corporate hierarchy. You work directly with a full-stack digital architect who designs, writes, and deploys every single line of code and infrastructure."
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

            // The Dual Pitch Grid
            <section class="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-20">
                // Local Business Pitch
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-3xl p-8 md:p-12 backdrop-blur-sm relative overflow-hidden group hover:border-slate-700/80 transition-all bento-scroll-card">
                    <div class="absolute inset-0 bg-gradient-to-br from-rose-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                    <div class="relative z-10">
                        <div class="flex items-center gap-3 mb-6">
                            <div class="h-12 w-12 rounded-xl bg-rose-950/50 border border-rose-900 flex items-center justify-center text-rose-400">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m3-4h1m-1 4h1m-5 8h8" />
                                </svg>
                            </div>
                            <h2 class="text-2xl font-bold text-slate-100">"For Local Businesses"</h2>
                        </div>
                        <p class="text-slate-300 leading-relaxed mb-6">
                            "I will clean up your messy Google Workspace, protect your design data, and give you control of your hardware so you never have to deal with a corporate IT headache again."
                        </p>
                        <ul class="space-y-4">
                            <li class="flex items-start gap-3">
                                <span class="text-rose-400 mt-1">"→"</span>
                                <div>
                                    <h4 class="font-bold text-slate-200">"Absolute Data Ownership"</h4>
                                    <p class="text-sm text-slate-400">"No more being held hostage by unresponsive agencies. Your hardware, your servers, your keys."</p>
                                </div>
                            </li>
                            <li class="flex items-start gap-3">
                                <span class="text-rose-400 mt-1">"→"</span>
                                <div>
                                    <h4 class="font-bold text-slate-200">"Zero Bureaucracy"</h4>
                                    <p class="text-sm text-slate-400">"Skip the 3-week support ticket wait times. Get direct, same-day responses straight from the engineer."</p>
                                </div>
                            </li>
                        </ul>
                    </div>
                </div>

                // Enterprise Pitch
                <div class="border border-slate-800/80 bg-slate-900/40 rounded-3xl p-8 md:p-12 backdrop-blur-sm relative overflow-hidden group hover:border-slate-700/80 transition-all bento-scroll-card">
                    <div class="absolute inset-0 bg-gradient-to-br from-cyan-400/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                    <div class="relative z-10">
                        <div class="flex items-center gap-3 mb-6">
                            <div class="h-12 w-12 rounded-xl bg-cyan-950/50 border border-cyan-800/50 flex items-center justify-center text-cyan-400">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                                </svg>
                            </div>
                            <h2 class="text-2xl font-bold text-slate-100">"For Enterprise Consulting"</h2>
                        </div>
                        <p class="text-slate-300 leading-relaxed mb-6">
                            "Look at the infrastructure behind Renivel. I architect highly scalable Next.js and NestJS platforms, write high-performance Rust services, manage secure AWS cloud pipelines, and can solve your most complex backend automation bottlenecks without hallucinating."
                        </p>
                        <ul class="space-y-4">
                            <li class="flex items-start gap-3">
                                <span class="text-cyan-400 mt-1">"→"</span>
                                <div>
                                    <h4 class="font-bold text-slate-200">"Full-Stack Ecosystems"</h4>
                                    <p class="text-sm text-slate-400">"End-to-end TypeScript architectures with Next.js, NestJS, and PostgreSQL, alongside memory-safe Rust backends."</p>
                                </div>
                            </li>
                            <li class="flex items-start gap-3">
                                <span class="text-cyan-400 mt-1">"→"</span>
                                <div>
                                    <h4 class="font-bold text-slate-200">"Infrastructure as Code"</h4>
                                    <p class="text-sm text-slate-400">"Reproducible, auditable cloud environments managed via Terraform and deployed with complete CI/CD automation."</p>
                                </div>
                            </li>
                        </ul>
                    </div>
                </div>
            </section>

            // Tech Stack & Capabilities Console
            <section class="max-w-4xl mx-auto bento-scroll-card no-hover">
                <div class="mb-6 text-center">
                    <h3 class="text-2xl font-bold text-slate-100">"Technical Competencies"</h3>
                    <p class="text-slate-400 mt-2">"The toolkit powering Artorias Tech Lab."</p>
                </div>
                <TerminalMock 
                    command="cat capabilities.json | jq" 
                    output="{
  \"languages\": [\"TypeScript\", \"Rust\", \"SQL\", \"Nushell\"],
  \"frameworks\": [\"Next.js\", \"NestJS\", \"Leptos\", \"React\"],
  \"cloud_infrastructure\": {
    \"provider\": \"AWS (IAM, ECS, RDS, Secrets Manager)\",
    \"iac\": \"Terraform\",
    \"networking\": \"VPC, Route53, VPN\"
  },
  \"local_infrastructure\": [
    \"Bare-Metal Server Deployment\",
    \"TrueNAS / ZFS Architecture\"
  ],
  \"status\": \"Ready to build.\"
}"
                />
            </section>
        </div>
    }
}
