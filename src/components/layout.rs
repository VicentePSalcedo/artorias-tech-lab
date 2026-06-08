use leptos::prelude::*;

#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-slate-950 text-slate-200 selection:bg-cyan-400/30 bg-grid-pattern relative overflow-x-hidden">
            // Subtle Background Ambiance Blobs
            <div class="absolute inset-0 pointer-events-none overflow-hidden z-0">
                <div class="absolute top-[-10%] left-[-10%] w-[60vw] h-[60vw] max-w-[650px] max-h-[650px] rounded-full bg-cyan-500/18 blur-[120px] ambient-blob-1"></div>
                <div class="absolute top-[35%] right-[-15%] w-[70vw] h-[70vw] max-w-[750px] max-h-[750px] rounded-full bg-emerald-500/12 blur-[150px] ambient-blob-2"></div>
                <div class="absolute bottom-[-5%] left-[5%] w-[50vw] h-[50vw] max-w-[550px] max-h-[550px] rounded-full bg-indigo-500/10 blur-[110px] ambient-blob-1"></div>
            </div>

            <header class="sticky top-0 z-50 w-full border-b border-slate-800/60 bg-slate-950/70 backdrop-blur-md">
                <div class="container mx-auto flex h-16 items-center justify-between px-4 sm:px-6 lg:px-8">
                    <a href="/" class="flex items-center gap-3 font-bold text-xl tracking-tight text-slate-100 hover:text-cyan-400 transition-colors">
                        <img src="/icon-original.png" alt="Artorias Tech Lab Logo" class="h-6 w-6 object-contain" />
                        <div class="flex items-center gap-2">
                            <span class="animate-shimmer">"Artorias"</span>
                            "Tech Lab"
                        </div>
                    </a>
                    <nav class="hidden md:flex items-center gap-6 text-sm font-medium">
                        <a href="/" class="hover:text-cyan-400 transition-colors">"Home"</a>
                        <a href="/products" class="hover:text-cyan-400 transition-colors">"Products"</a>
                        <a href="/services" class="hover:text-cyan-400 transition-colors">"Client Services"</a>
                        <a href="/founder" class="hover:text-cyan-400 transition-colors">"Founder"</a>
                        <a href="/contact" class="hover:text-cyan-400 transition-colors">"Contact"</a>
                        <a href="tel:9042067198" class="ml-4 px-3 py-1 rounded-full bg-slate-900 border border-slate-800 text-slate-300 hover:text-cyan-400 hover:border-cyan-500 transition-colors">"904-206-7198"</a>
                    </nav>
                </div>
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
