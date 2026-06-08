use leptos::prelude::*;

#[component]
pub fn BentoGrid(children: Children) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 auto-rows-[minmax(180px,auto)]">
            {children()}
        </div>
    }
}

#[component]
pub fn BentoBlock(
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("relative overflow-hidden rounded-2xl border border-slate-800/60 bg-slate-900/50 p-6 backdrop-blur-sm transition-all duration-300 bento-scroll-card group {}", class)>
            <div class="absolute inset-0 bg-gradient-to-br from-cyan-400/5 to-emerald-400/5 opacity-0 transition-opacity duration-500 group-hover:opacity-100"></div>
            <div class="relative z-10 h-full flex flex-col">
                {children()}
            </div>
        </div>
    }
}
