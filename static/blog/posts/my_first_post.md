# An Example Post

Some intro.

## A subheader

And it's associated subtopics.

- A nice
- Looking
- List

And a checklist...

- [ ] Todo
- [x] Done

How about some code?

  ```python
  # here's some python
  for thing in things:
    print(thing)
  ```

  ```rust
  // and some rust
  #[tokio::main]
  async fn main() {
      pretty_env_logger::init();
      // GET / => 200 OK with body "Hello, root!"
      let hello = warp::path::end().map(|| "Hello.");
      // GET /blog
      let blog = warp::path!("blog").map(|| "Welcome to my blog.");
      // GET /blog/[article_title]
      let posts = warp::path!("blog" / String).map(|post| get_post(post));
  
      // API
      let routes = warp::get().and(hello.or(blog).or(posts));
  
      log::info!("Starting server...");
      warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
  }
  ```

### Subsubheader

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Malesuada fames ac turpis egestas sed tempus urna et. Convallis tellus id interdum velit laoreet id. Amet nulla facilisi morbi tempus iaculis urna. Erat imperdiet sed euismod nisi. Id cursus metus aliquam eleifend mi in nulla posuere. Commodo elit at imperdiet dui. Libero id faucibus nisl tincidunt eget nullam non nisi est. Scelerisque viverra mauris in aliquam sem fringilla ut morbi. Imperdiet proin fermentum leo vel orci. Vulputate eu scelerisque felis imperdiet proin fermentum leo vel.

Cursus turpis massa tincidunt dui ut ornare lectus. Aliquet porttitor lacus luctus accumsan tortor posuere ac ut consequat. Cursus metus aliquam eleifend mi in nulla. Velit laoreet id donec ultrices tincidunt. Sem fringilla ut morbi tincidunt augue interdum velit. Non curabitur gravida arcu ac tortor dignissim convallis. Fermentum et sollicitudin ac orci. Malesuada proin libero nunc consequat interdum varius. Turpis massa sed elementum tempus. Ipsum dolor sit amet consectetur adipiscing elit duis. At tellus at urna condimentum. Amet venenatis urna cursus eget nunc scelerisque viverra mauris in. Gravida in fermentum et sollicitudin ac.

Sollicitudin aliquam ultrices sagittis orci a scelerisque. Nam aliquam sem et tortor consequat id porta nibh venenatis. Hendrerit gravida rutrum quisque non tellus orci. Ultricies tristique nulla aliquet enim tortor at auctor. Phasellus vestibulum lorem sed risus ultricies tristique. Dictum non consectetur a erat nam. Congue quisque egestas diam in arcu cursus euismod. Sapien faucibus et molestie ac feugiat. Metus dictum at tempor commodo ullamcorper a. Ultrices eros in cursus turpis. Aliquet bibendum enim facilisis gravida neque. Tincidunt tortor aliquam nulla facilisi cras fermentum odio. Urna nunc id cursus metus aliquam eleifend mi in nulla. Ut sem viverra aliquet eget sit amet tellus cras.

Cursus vitae congue mauris rhoncus aenean vel. Dolor morbi non arcu risus quis varius quam quisque. Ac orci phasellus egestas tellus rutrum. Lacus sed turpis tincidunt id aliquet risus feugiat in. Massa eget egestas purus viverra accumsan in nisl nisi scelerisque. Neque sodales ut etiam sit amet nisl purus in mollis. Lorem sed risus ultricies tristique. Viverra aliquet eget sit amet tellus cras adipiscing enim. Nulla facilisi morbi tempus iaculis urna id volutpat. Imperdiet sed euismod nisi porta lorem mollis aliquam. Consectetur a erat nam at lectus urna. Id nibh tortor id aliquet. Metus vulputate eu scelerisque felis imperdiet proin fermentum leo.

Proin libero nunc consequat interdum varius sit amet mattis vulputate. Sit amet dictum sit amet justo donec enim diam. Ullamcorper a lacus vestibulum sed arcu non odio. Non consectetur a erat nam at lectus urna duis. Tincidunt tortor aliquam nulla facilisi cras fermentum odio. At erat pellentesque adipiscing commodo elit. Sit amet volutpat consequat mauris nunc congue nisi vitae. Egestas tellus rutrum tellus pellentesque eu tincidunt tortor. Eleifend quam adipiscing vitae proin sagittis nisl rhoncus. Ut consequat semper viverra nam libero. Maecenas accumsan lacus vel facilisis volutpat. Sit amet risus nullam eget felis eget nunc. Integer feugiat scelerisque varius morbi enim nunc. Augue lacus viverra vitae congue eu consequat. Massa tincidunt nunc pulvinar sapien et ligula. Urna cursus eget nunc scelerisque viverra. Ut porttitor leo a diam sollicitudin tempor id eu nisl. Nec ultrices dui sapien eget mi proin. Diam volutpat commodo sed egestas egestas fringilla.

A [hyperlink](https://duckduckgo.com).

Here's giles.

![giles](/static/giles.png)

## Conclusion

I am correct.
