use leptos::prelude::*;

#[component]
pub fn ProductsIndex() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto py-12">
            <div class="mb-16 flex flex-col items-center text-center bento-scroll-card" style="--scroll-progress: 1">
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-950/30 border border-cyan-900/50 text-cyan-400 text-sm font-mono mb-8">
                    <span class="w-2 h-2 rounded-full bg-cyan-400 animate-pulse"></span>
                    "MODULE: PRODUCTS"
                </div>
                <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight mb-6 leading-tight text-slate-100 uppercase">
                    "Internal " <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-emerald-400">"Products"</span>
                </h1>
                <p class="text-xl md:text-2xl text-slate-400 max-w-2xl font-light">
                    "High-performance SaaS platforms engineered in-house."
                </p>
            </div>

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
    }
}
