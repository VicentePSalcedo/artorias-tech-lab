use leptos::prelude::*;

#[component]
pub fn ProductsIndex() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-16 pb-24">
            // Hero Section
            <section class="relative min-h-[72vh] flex flex-col justify-center items-center text-center px-4">
                <div class="max-w-4xl mx-auto flex flex-col items-center">
                    <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                        "Internal " <span class="animate-shimmer-cyan">"Products"</span>
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-400 max-w-2xl font-light leading-relaxed">
                        "High-performance SaaS platforms engineered in-house."
                    </p>
                </div>
                
                // Animated Scroll Indicator
                <div class="animate-scroll-cue flex flex-col items-center gap-2 text-slate-500 mt-12">
                    <span class="text-xs font-mono uppercase tracking-widest text-slate-600">"Scroll to explore"</span>
                    <svg class="w-5 h-5 text-cyan-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
                    </svg>
                </div>
            </section>

            <div class="max-w-4xl mx-auto px-4 w-full flex flex-col gap-16">
                <div class="grid grid-cols-1 max-w-2xl mx-auto gap-8">
                // Renivel Product Card
                <a href="/products/renivel" class="block rounded-2xl border border-slate-800/60 bg-slate-900/90 p-8 relative overflow-hidden group bento-card bento-scroll-card">
                    <div class="absolute inset-0 bg-gradient-to-br from-cyan-400/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                    <div class="relative z-10">
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="text-2xl font-bold text-slate-100">"RENIVEL"</h2>
                            <span class="px-3 py-1 rounded-full bg-cyan-900/30 text-cyan-400 text-xs font-mono border border-cyan-800/50">
                                "Production"
                            </span>
                        </div>
                        <p class="text-slate-300 mb-6">
                            "The 'Lite ERP' for residential remodelers. Stop doing math on napkins and instantly see if your jobs are profitable."
                        </p>
                        <div class="flex items-center text-cyan-400 font-bold text-sm group-hover:text-cyan-300 transition-colors">
                            "Explore Renivel →"
                        </div>
                    </div>
                </a>

                // Future Product Placeholder
                <div class="block rounded-2xl border border-slate-800/60 bg-slate-900/90 p-8 relative overflow-hidden bento-card no-hover bento-scroll-card">
                    <div class="relative z-10 opacity-60">
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="text-2xl font-bold text-slate-400">"Project V2"</h2>
                            <span class="px-3 py-1 rounded-full bg-slate-800 text-slate-500 text-xs font-mono border border-slate-700/50">
                                "In Development"
                            </span>
                        </div>
                        <p class="text-slate-500 mb-6">
                            "Next-generation infrastructure tools are currently being architected. Stay tuned."
                        </p>
                    </div>
                </div>
            </div>
            </div>
        </div>
    }
}
