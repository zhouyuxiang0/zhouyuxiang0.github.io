use yew::prelude::*;

pub struct Header {
    link: ComponentLink<Self>,
    opened: bool,
}

pub enum Msg {
    Click,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Header {
            link,
            opened: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                self.opened = !self.opened;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
          <header>
            <nav class="navbar">
              <div class="container">
                <div class="navbar-brand">
                  <a class="navbar-item" href="/">
                    <span>{"主页"}</span>
                  </a>
                  {
                    if self.opened {
                      html! {
                        <span class="navbar-burger burger is-active" data-target="navbarMenu" onclick=self.link.callback(|_| Msg::Click)>
                          <span></span>
                          <span></span>
                          <span></span>
                        </span>
                      }
                    } else {
                      html!{
                        <span class="navbar-burger burger" data-target="navbarMenu" onclick=self.link.callback(|_| Msg::Click)>
                          <span></span>
                          <span></span>
                          <span></span>
                        </span>
                      }
                    }
                  }
                </div>
                {
                  if self.opened {
                    html!{
                      <div id="navbarMenu" class="navbar-menu is-active">
                        <div class="navbar-end">
                          <a itemprop="url" class="navbar-item " href="">
                            <span itemprop="name">{"分类"} </span>
                          </a>

                          <a itemprop="url" class="navbar-item " href="">
                            <span itemprop="name">{"标签"} </span>
                          </a>

                          <a itemprop="url" class="navbar-item " href="">
                            <span itemprop="name">{"作者"} </span>
                          </a>

                          // <div class="navbar-item search-container js-only">
                          //   <input class="input" id="search" type="search" placeholder="Search" />

                          //   <div class="search-results box">
                          //     <div class="search-results__items"></div>
                          //   </div>
                          // </div>
                        </div>
                      </div>
                    }
                  } else {
                    html!{
                      <div id="navbarMenu" class="navbar-menu">
                        <div class="navbar-end">
                          <a itemprop="url" class="navbar-item " href="">
                            <span itemprop="name">{"分类"} </span>
                          </a>

                          <a itemprop="url" class="navbar-item " href="">
                            <span itemprop="name">{"标签"} </span>
                          </a>

                          <a itemprop="url" class="navbar-item " href="">
                            <span itemprop="name">{"作者"} </span>
                          </a>

                          // <div class="navbar-item search-container js-only">
                          //   <input class="input" id="search" type="search" placeholder="Search" />

                          //   <div class="search-results box">
                          //     <div class="search-results__items"></div>
                          //   </div>
                          // </div>
                        </div>
                      </div>
                    }
                  }
                }
              </div>
            </nav>
          </header>
        }
    }
}
