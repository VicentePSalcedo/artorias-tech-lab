use leptos::prelude::*;
use crate::components::bento::{BentoGrid, BentoBlock};
use crate::components::terminal::TerminalMock;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero Section
            <section class="relative min-h-[72vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-3xl mx-auto flex flex-col items-center">
                    <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-6 leading-tight">
                        "Engineering "
                        <br class="md:hidden" />
                        <span class="animate-shimmer">
                            "Autonomy"
                        </span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-2xl mx-auto leading-relaxed mb-8">
                        "Elite, custom Rust and TypeScript infrastructure for businesses that refuse to rent their operational capacity."
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
                <BentoBlock class="md:col-span-2 lg:col-span-1 row-span-2 flex flex-col justify-between">
                    <div>
                        <div class="flex items-center gap-3 mb-4">
                            <img src="/pfp.JPG" alt="Vicente" class="h-10 w-10 rounded-full object-cover border border-cyan-500/30" />
                            <div>
                                <h3 class="text-xl font-bold text-slate-100">"Vicente"</h3>
                                <p class="text-sm text-cyan-400">"Chief Architect"</p>
                            </div>
                        </div>
                        <div class="inline-block px-3 py-1 bg-emerald-950/50 border border-emerald-800/50 rounded-full text-xs text-emerald-400 font-mono mb-4">
                            "whoami // Elite Engineering. Zero Bureaucracy."
                        </div>
                        <p class="text-slate-300 leading-relaxed mb-6">
                            "Artorias Tech Lab doesn't have an account management team, a sales department, or a bloated corporate hierarchy. You work directly with a full-stack digital architect who designs, writes, and deploys every single line of code and infrastructure."
                        </p>
                        <p class="text-slate-300 leading-relaxed mb-6">
                            "From bare-metal local network servers to full-stack Next.js platforms and cloud-native Rust applications, we deliver enterprise-grade engineering with absolute operational agility."
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
                <BentoBlock class="md:col-span-2 lg:col-span-2 row-span-2">
                    <div class="flex flex-col md:flex-row justify-between items-start md:items-center mb-6">
                        <div>
                            <h3 class="text-2xl font-bold text-slate-100 mb-2">"Renivel SaaS"</h3>
                            <p class="text-slate-300 max-w-xl">"We don't just build websites; we engineer highly scalable, multi-tenant software platforms from the ground up."</p>
                        </div>
                        <div class="mt-4 md:mt-0 px-3 py-1 rounded-full bg-cyan-900/30 text-cyan-400 text-xs font-mono border border-cyan-800/50">
                            "Production"
                        </div>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
                        <div class="space-y-4">
                            <div class="p-4 rounded-xl bg-slate-950/50 border border-slate-800">
                                <h4 class="font-medium text-slate-200 mb-1">"Production-Grade Architecture"</h4>
                                <p class="text-sm text-slate-400">"Renivel is built for remodeling teams to track field logistics, capture receipt data from a dusty truck dashboard, and audit real-time profit margins."</p>
                            </div>
                            <div class="p-4 rounded-xl bg-slate-950/50 border border-slate-800">
                                <h4 class="font-medium text-slate-200 mb-1">"Cloud-Native Infrastructure"</h4>
                                <p class="text-sm text-slate-400">"Under the hood, it features automated infrastructure-as-code pipelines via AWS Terraform and secure secrets handling inside AWS Secrets Manager."</p>
                            </div>
                        </div>
                        <div class="flex flex-col items-center justify-center gap-6">
                            <TerminalMock 
                                command="nu ./inspect_infrastructure.nu" 
                                output="[+] Multi-tenant architecture verified\n[+] Terraform pipelines executed\n[+] AWS Secrets Manager integrated\n\nAll systems operational."
                            />
                            <a href="/products/renivel" class="text-sm font-bold text-cyan-400 hover:text-cyan-300 underline decoration-cyan-900 underline-offset-4 transition-colors">
                                "Explore Renivel ->"
                            </a>
                        </div>
                    </div>
                </BentoBlock>

                // Module C: Aggressive Signs "IT Rescue" Case Study
                <BentoBlock class="md:col-span-2 lg:col-span-3">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 items-center">
                        <div class="md:col-span-1 flex justify-center">
                            // Grayscale SVG placeholder transitioning to brand color
                            <div class="h-32 w-32 rounded-2xl bg-slate-800 flex items-center justify-center transition-all duration-500 hover:bg-rose-600 group cursor-pointer">
                                <span class="font-black text-3xl text-slate-600 group-hover:text-white transition-colors duration-500">"AS"</span>
                            </div>
                        </div>
                        <div class="md:col-span-2">
                            <div class="flex items-center gap-2 mb-3">
                                <span class="px-2 py-1 rounded text-[10px] uppercase tracking-wider font-bold bg-rose-500/20 text-rose-400">"The IT Rescue Routine"</span>
                                <h3 class="text-xl font-bold text-slate-100">"Aggressive Signs"</h3>
                            </div>
                            <p class="text-slate-300 leading-relaxed mb-4">
                                <span class="font-semibold text-white">"Your business data belongs to you—not your IT provider. "</span>
                                "When we partnered with Aggressive Signs, they were held hostage by an unresponsive agency running their critical operations on a locked-down legacy server box."
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
            </BentoGrid>

            
        </div>
    }
}
