use yew::prelude::*;
use yew::virtual_dom::VNode;

pub enum Tag {
  Rust,
  Other
}

pub struct Blog {
  title: String,
  date: String,
  tag: Tag,
  author: String,
  intor: String,
  detail: String
}

impl Blog {
  fn new(title: String, date: String, tag: Tag, author: String, intor: String, detail: String) -> Self {
    Blog {
      title,
      date,
      tag,
      author,
      intor,
      detail
    }
  }
}

pub fn wapper() -> VNode {
  html!{
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
  }
}