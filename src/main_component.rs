use yew::prelude::*;

pub struct MainComponent {}

impl Component for MainComponent {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MainComponent {}
    }

    fn update(&mut self, _: <Self as yew::html::Component>::Message) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
          <main class="index">

            <section class="is-bold hero is-primary">
              <header>
                <div class="hero-body">
                  <div class="container has-text-centered">
                    <h1 class="title">{"zyx的博客"}</h1>
                  </div>
                </div>
              </header>
            </section>

            <div class="container">

              <section class="articles">
                <div class="columns is-desktop">
                  <div class="column is-10-desktop is-offset-1-desktop">
                    <article itemtype="http://schema.org/CreativeWork">
                      <div class="card article">
                        <div class="card-content">
                          <header>
                            <div class="has-text-centered">
                              <a href="">
                                <p class="title article-title">{"标题"}</p>
                              </a>
                              <div class="tags has-addons level-item">
                                <span class="tag is-rounded">{"2020-02-16"}</span>

                                <span class="tag is-rounded is-primary">
                                  {"zyx"}
                                </span>

                                <span class="tag is-rounded">
                                  <svg class="i-clock" viewBox="0 0 32 32" width="16" height="16" fill="none"
                                    stroke="currentcolor" stroke-linecap="round" stroke-linejoin="round" stroke-width="6.25%">
                                    <circle cx="16" cy="16" r="14" />
                                    <path d="M16 8 L16 16 20 20" />
                                  </svg>
                                  <span>   {"2 minute read"}</span>
                                </span>
                              </div>
                            </div>
                          </header>

                          <div itemprop="summary" class="content article-body">
                            <p>
                              {"测试测试测试测试测试测试测试测试测试测试测试测试测试测试.
                          测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试
                          测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试.
                          测试测试测试测试测试测试测试测试测试测试测试测试测试测试
                          测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试.
                          测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试
                          测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试
                          测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试
                          测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试
                          测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试测试
                          测试测试测试测试测试测试测试测试测试测试."}
                            </p>

                            <nav class="readmore">
                              <a itemprop="url" href="">{"Read
                                More  »"}
                              </a>
                            </nav>
                          </div>
                        </div>
                      </div>
                    </article>

                    <article itemtype="http://schema.org/CreativeWork">
                      <div class="card article">
                        <div class="card-content">
                          <header>
                            <div class="has-text-centered">
                              <a href="">
                                <p class="title article-title">
                                  {"一个标题"}
                                </p>
                              </a>
                              <div class="tags has-addons level-item">
                                <span class="tag is-rounded">{"2020-02-16"}</span>

                                <span class="tag is-rounded is-primary">
                                  {"zyx"}
                                </span>

                                <span class="tag is-rounded">
                                  <svg class="i-clock" viewBox="0 0 32 32" width="16" height="16" fill="none"
                                    stroke="currentcolor" stroke-linecap="round" stroke-linejoin="round" stroke-width="6.25%">
                                    <circle cx="16" cy="16" r="14" />
                                    <path d="M16 8 L16 16 20 20" />
                                  </svg>
                                  <span>{"  2 minute read"}</span>
                                </span>
                              </div>
                            </div>
                          </header>

                          <div itemprop="summary" class="content article-body">
                            <p>
                              {"haowenzhanghaowenzhanghaowenzhanghaowenzhang.
                          haowenzhanghaowenzhanghaowenzhanghaowenzhanghaowenzhang.
                          haowenzhanghaowenzhanghaowenzhanghaowenzhanghaowenzha"}
                            </p>

                            <nav class="readmore">
                              <a itemprop="url" href="https://festive-morse-47d46c.netlify.com/some-other-article/">{"Read
                                More  »"}
                              </a>
                            </nav>
                          </div>
                        </div>
                      </div>
                    </article>
                  </div>
                </div>
              </section>
            </div>
          </main>
        }
    }
}
