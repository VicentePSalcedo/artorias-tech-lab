use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::layout::AppLayout;
use crate::pages::index::HomePage;
use crate::pages::services::ServicesPage;
use crate::pages::founder::FounderPage;
use crate::pages::products::index::ProductsIndex;
use crate::pages::products::renivel::RenivelPage;
use crate::pages::contact::ContactPage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="icon" type="image/png" href="/icon-original.png"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
                <script>
                    "document.addEventListener('DOMContentLoaded', () => {
                        const cards = document.querySelectorAll('.bento-scroll-card');
                        const visibleCards = new Set();
                        
                        const observer = new IntersectionObserver((entries) => {
                            entries.forEach(entry => {
                                if (entry.isIntersecting) {
                                    visibleCards.add(entry.target);
                                } else {
                                    visibleCards.delete(entry.target);
                                    entry.target.style.setProperty('--scroll-progress', '0');
                                }
                            });
                        }, {
                            threshold: 0,
                            rootMargin: '100px 0px 100px 0px'
                        });

                        cards.forEach(card => observer.observe(card));

                        let isScrolling = false;
                        
                        function updateProgress() {
                            const viewportHeight = window.innerHeight;
                            visibleCards.forEach(card => {
                                const rect = card.getBoundingClientRect();
                                const startScroll = viewportHeight;
                                const endScroll = viewportHeight * 0.25;
                                
                                let progress = (startScroll - rect.top) / (startScroll - endScroll);
                                progress = Math.max(0, Math.min(1, progress));
                                
                                card.style.setProperty('--scroll-progress', progress.toFixed(3));
                            });
                        }

                        setTimeout(updateProgress, 100);

                        window.addEventListener('scroll', () => {
                            if (!isScrolling) {
                                window.requestAnimationFrame(() => {
                                    updateProgress();
                                    isScrolling = false;
                                });
                                isScrolling = true;
                            }
                        });
                    });"
                </script>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/artorias-tech-lab.css"/>
        <Title text="Artorias Tech Lab"/>

        <Router>
            <AppLayout>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("founder") view=FounderPage/>
                    <Route path=StaticSegment("services") view=ServicesPage/>
                    <Route path=StaticSegment("contact") view=ContactPage/>
                    <Route path=StaticSegment("products") view=ProductsIndex/>
                    <Route path=(StaticSegment("products"), StaticSegment("renivel")) view=RenivelPage/>
                </Routes>
            </AppLayout>
        </Router>
    }
}
