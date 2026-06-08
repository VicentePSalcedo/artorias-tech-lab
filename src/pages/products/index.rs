use leptos::prelude::*;

#[component]
pub fn ProductsIndex() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto py-12">
            <div class="text-center mb-16">
                <h1 class="text-4xl md:text-5xl font-bold text-slate-100 mb-4">"Our Products"</h1>
                <p class="text-slate-400 text-lg">"High-performance SaaS platforms engineered in-house."</p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                // Renivel Product Card
                <a href="/products/renivel" class="block border border-slate-800/80 bg-slate-900/50 rounded-2xl p-8 backdrop-blur-sm relative overflow-hidden group hover:border-cyan-500/50 transition-colors bento-scroll-card">
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
                <div class="block border border-slate-800/40 bg-slate-900/20 rounded-2xl p-8 backdrop-blur-sm relative overflow-hidden bento-scroll-card">
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
