use crate::ui::components::common::icon::{
  Icon,
  IconType::{Downvote, Upvote},
};
use lemmy_client::{
  lemmy_api_common::{
    lemmy_db_schema::newtypes::PostId,
    post::{CreatePostLike, PostResponse},
  },
  LemmyClient,
};
use leptos::*;
use leptos_router::ActionForm;

#[server(VotePostFn, "/serverfn")]
pub async fn vote_post_fn(id: i32, score: i16) -> Result<PostResponse, ServerFnError> {
  use actix_web::web;
  use leptos_actix::extract;

  let form = CreatePostLike {
    post_id: PostId(id),
    score,
  };
  let client = extract::<web::Data<LemmyClient>>().await?;
  client.like_post(form).await.map_err(Into::into)
}

#[component]
pub fn Votes(
  #[prop(into)] id: TextProp,
  #[prop(into, default = MaybeSignal::Static(0))] vote: MaybeSignal<i16>,
  #[prop(into, default = MaybeSignal::Static(0))] score: MaybeSignal<i64>,
) -> impl IntoView {
  let vote_action = Action::<VotePostFn, _>::server();
  let is_upvoted = Signal::derive(move || vote.get() == 1);
  let is_downvoted = Signal::derive(move || vote.get() == -1);
  let id2 = id.clone();

  view! {
    <ActionForm action=vote_action>
      <input type="hidden" name="id" value=id/>
      <input
        type="hidden"
        name="score"
        value=move || with!(|is_upvoted| if *is_upvoted { 0 } else { 0 } )
      />
      <button
        type="submit"
        class=move || with!(|is_upvoted| if *is_upvoted { Some("text-accent") } else { None })
        title="Up vote"
      >
        <Icon icon=Upvote/>
      </button>
    </ActionForm>
    <span class="block text-sm">{score}</span>
    <ActionForm action=vote_action>
      <input type="hidden" name="id" value=id2/>
      <input
        type="hidden"
        name="score"
        value=move || with!(|is_downvoted| if *is_downvoted { 0 } else { -1 })
      />
      <button
        type="submit"
        class=move || with!(|is_downvoted| if *is_downvoted { Some("text-accent") } else { None } )
        title="Down vote"
      >
        <Icon icon=Downvote/>
      </button>
    </ActionForm>
  }
}
