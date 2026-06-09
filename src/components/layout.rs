use leptos::prelude::*;

#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    let (is_mobile_menu_open, set_mobile_menu_open) = signal(false);

    let toggle_menu = move |_| set_mobile_menu_open.update(|n| *n = !*n);
    let close_menu = move |_| set_mobile_menu_open.set(false);

    view! {
        <div class="min-h-screen bg-slate-950 text-slate-200 selection:bg-cyan-400/30 bg-grid-pattern relative overflow-x-hidden">
            // Subtle Background Ambiance Blobs (Optimized for Mobile Scroll Performance)
            <div class="fixed inset-0 pointer-events-none overflow-hidden z-0 transform-gpu">
            </div>

            <header class="sticky top-0 z-50 w-full border-b border-slate-800/60 bg-slate-950/95">
                <div class="container mx-auto flex h-16 items-center justify-between px-4 sm:px-6 lg:px-8">
                    <a href="/" class="flex items-center gap-3 font-bold text-xl tracking-tight text-slate-100 hover:text-cyan-400 transition-colors" on:click=close_menu>
                        <img src="/icon-original.png" alt="Artorias Tech Lab Logo" class="h-6 w-6 object-contain" />
                        <div class="flex items-center gap-2">
                            <span class="animate-shimmer">"Artorias"</span>
                            "Tech Lab"
                        </div>
                    </a>
                    <nav class="hidden md:flex items-center gap-4 text-xs font-mono">
                        <a href="/" class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-slate-900/50 border border-slate-800 text-slate-400 hover:text-cyan-400 hover:border-cyan-500/50 transition-all duration-300">
                            <span class="w-1.5 h-1.5 rounded-full bg-slate-600"></span>
                            "HOME"
                        </a>
                        <a href="/products" class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-950/30 border border-cyan-900/40 text-cyan-400 hover:bg-cyan-900/20 hover:border-cyan-400/80 transition-all duration-300">
                            <span class="w-1.5 h-1.5 rounded-full bg-cyan-400 animate-pulse"></span>
                            "PRODUCTS"
                        </a>
                        <a href="/services" class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-950/30 border border-emerald-900/40 text-emerald-400 hover:bg-emerald-900/20 hover:border-emerald-400/80 transition-all duration-300">
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                            "SERVICES"
                        </a>
                        <a href="/founder" class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-rose-950/30 border border-rose-900/40 text-rose-400 hover:bg-rose-900/20 hover:border-rose-400/80 transition-all duration-300">
                            <span class="w-1.5 h-1.5 rounded-full bg-rose-400 animate-pulse"></span>
                            "FOUNDER"
                        </a>
                        <a href="/contact" class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-950/30 border border-cyan-900/40 text-cyan-400 hover:bg-cyan-900/20 hover:border-cyan-400/80 transition-all duration-300">
                            <span class="w-1.5 h-1.5 rounded-full bg-cyan-400 animate-pulse"></span>
                            "COMM_LINK"
                        </a>
                        <a href="tel:9042067198" class="ml-4 px-3 py-1 rounded-full bg-slate-900 border border-slate-800 text-slate-300 hover:text-cyan-400 hover:border-cyan-500 transition-colors">"904-206-7198"</a>
                    </nav>

                    // Mobile Toggle Button
                    <button class="md:hidden p-2 -mr-2 text-slate-300 hover:text-cyan-400 transition-colors" on:click=toggle_menu aria-label="Toggle menu">
                        <Show when=move || !is_mobile_menu_open.get() fallback=|| view! {
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        }>
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </Show>
                    </button>
                </div>

                // Mobile Navigation Dropdown
                <Show when=move || is_mobile_menu_open.get()>
                    <div class="md:hidden absolute w-full border-b border-slate-800/60 bg-slate-950 px-4 py-6 flex flex-col gap-4 shadow-2xl">
                        <a href="/" class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-slate-900/50 border border-slate-800 text-slate-400 text-sm font-mono w-fit" on:click=close_menu>
                            <span class="w-1.5 h-1.5 rounded-full bg-slate-600"></span>
                            "HOME"
                        </a>
                        <a href="/products" class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-cyan-950/30 border border-cyan-900/40 text-cyan-400 text-sm font-mono w-fit" on:click=close_menu>
                            <span class="w-1.5 h-1.5 rounded-full bg-cyan-400 animate-pulse"></span>
                            "PRODUCTS"
                        </a>
                        <a href="/services" class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-emerald-950/30 border border-emerald-900/40 text-emerald-400 text-sm font-mono w-fit" on:click=close_menu>
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                            "SERVICES"
                        </a>
                        <a href="/founder" class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-rose-950/30 border border-rose-900/40 text-rose-400 text-sm font-mono w-fit" on:click=close_menu>
                            <span class="w-1.5 h-1.5 rounded-full bg-rose-400 animate-pulse"></span>
                            "FOUNDER"
                        </a>
                        <a href="/contact" class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-cyan-950/30 border border-cyan-900/40 text-cyan-400 text-sm font-mono w-fit" on:click=close_menu>
                            <span class="w-1.5 h-1.5 rounded-full bg-cyan-400 animate-pulse"></span>
                            "COMM_LINK"
                        </a>
                        <div class="pt-4 mt-2 border-t border-slate-800/60">
                            <a href="tel:9042067198" class="inline-block px-5 py-2 rounded-full bg-slate-900 border border-slate-800 text-slate-300 hover:text-cyan-400 hover:border-cyan-500 transition-colors font-mono text-sm" on:click=close_menu>
                                "Call 904-206-7198"
                            </a>
                        </div>
                    </div>
                </Show>
            </header>
            
            <main class="flex-1 w-full mx-auto container px-4 sm:px-6 lg:px-8 py-12 relative z-10">
                {children()}
            </main>
            
            <footer class="border-t border-slate-800/60 py-8 mt-auto relative z-10">
                <div class="container mx-auto px-4 flex flex-col items-center justify-center gap-2 text-sm text-slate-500">
                    <p>"© " {chrono::Local::now().format("%Y").to_string()} " Artorias Tech Lab. All rights reserved."</p>
                    <a href="tel:9042067198" class="hover:text-cyan-400 transition-colors">"904-206-7198"</a>
                </div>
            </footer>
        </div>
    }
}
