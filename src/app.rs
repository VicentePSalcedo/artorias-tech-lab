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
                        const visibleCards = new Set();
                        const observedCards = new Set();
                        let isScrolling = false;
                        
                        function updateProgress() {
                            const viewportHeight = window.innerHeight;
                            const scrollY = window.scrollY || window.pageYOffset;
                            const totalHeight = document.documentElement.scrollHeight;
                            const isAtBottom = (viewportHeight + scrollY) >= (totalHeight - 80);
                            
                            visibleCards.forEach(card => {
                                if (isAtBottom) {
                                    card.style.setProperty('--scroll-progress', '1.000');
                                    return;
                                }
                                
                                const rect = card.getBoundingClientRect();
                                const startScroll = viewportHeight;
                                const endScroll = viewportHeight * 0.25;
                                
                                let progress = (startScroll - rect.top) / (startScroll - endScroll);
                                progress = Math.max(0, Math.min(1, progress));
                                
                                card.style.setProperty('--scroll-progress', progress.toFixed(3));
                            });
                        }

                        const intersectionObserver = new IntersectionObserver((entries) => {
                            let statusChanged = false;
                            entries.forEach(entry => {
                                if (entry.isIntersecting) {
                                    visibleCards.add(entry.target);
                                    statusChanged = true;
                                } else {
                                    visibleCards.delete(entry.target);
                                    entry.target.style.setProperty('--scroll-progress', '0');
                                }
                            });
                            if (statusChanged) {
                                updateProgress();
                            }
                        }, {
                            threshold: 0,
                            rootMargin: '100px 0px 100px 0px'
                        });

                        function scanAndObserve() {
                            const cards = document.querySelectorAll('.bento-scroll-card');
                            cards.forEach(card => {
                                if (!observedCards.has(card)) {
                                    observedCards.add(card);
                                    intersectionObserver.observe(card);
                                }
                            });
                            updateProgress();
                        }

                        const mutationObserver = new MutationObserver((mutations) => {
                            let hasNewCards = false;
                            mutations.forEach(mutation => {
                                mutation.addedNodes.forEach(node => {
                                    if (node.nodeType === Node.ELEMENT_NODE) {
                                        if (node.classList && node.classList.contains('bento-scroll-card')) {
                                            hasNewCards = true;
                                        } else if (node.querySelector && node.querySelector('.bento-scroll-card')) {
                                            hasNewCards = true;
                                        }
                                    }
                                });
                            });
                            if (hasNewCards) {
                                scanAndObserve();
                            }
                        });

                        mutationObserver.observe(document.body, {
                            childList: true,
                            subtree: true
                        });

                        scanAndObserve();

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
