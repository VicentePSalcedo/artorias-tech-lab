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
        <div class=format!("relative overflow-hidden rounded-2xl border border-slate-800/60 bg-slate-900/90 p-6 transition-all duration-300 bento-scroll-card {}", class)>
            <div class="relative z-10 h-full flex flex-col">
                {children()}
            </div>
        </div>
    }
}
