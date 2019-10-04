class Posts::Show < BrowserAction
  get "/posts/:post_id" do
    post = PostQuery.new.preload_categories.find(post_id)
    render ShowPage, post: post
  end
end
