use leptos::prelude::*;

#[component]
pub fn TerminalMock(
    #[prop(into)] command: String,
    #[prop(into)] output: String,
) -> impl IntoView {
    view! {
        <div class="w-full max-w-2xl rounded-xl border border-slate-800/80 bg-[#0d1117] overflow-hidden shadow-2xl font-mono text-sm">
            <div class="flex items-center gap-2 border-b border-slate-800/80 bg-slate-900/80 px-4 py-2">
                <div class="flex gap-1.5">
                    <div class="h-3 w-3 rounded-full bg-rose-500/80"></div>
                    <div class="h-3 w-3 rounded-full bg-amber-500/80"></div>
                    <div class="h-3 w-3 rounded-full bg-emerald-500/80"></div>
                </div>
                <div class="ml-2 text-xs text-slate-500 font-medium">"bash"</div>
            </div>
            <div class="p-4 overflow-x-auto">
                <div class="flex items-center text-emerald-400">
                    <span class="mr-2">"$"</span>
                    <span class="text-slate-200">{command}</span>
                </div>
                <div class="mt-2 text-slate-400 whitespace-pre-wrap">
                    {output}
                </div>
            </div>
        </div>
    }
}
