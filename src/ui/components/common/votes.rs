use crate::ui::components::common::icon::{
  Icon,
  IconType::{Downvote, Upvote},
};
use lemmy_client::lemmy_api_common::{comment::CommentResponse, post::PostResponse};
use leptos::{
  server_fn::{client::Client, codec::PostUrl, request::ClientReq, ServerFn},
  *,
};
use leptos_router::ActionForm;
use serde::{de::DeserializeOwned, Deserialize};

#[server(prefix = "/serverfn")]
pub async fn vote_post_fn(id: i32, score: i16) -> Result<PostResponse, ServerFnError> {
  use crate::server::get_client_and_session;
  use lemmy_client::{
    lemmy_api_common::{lemmy_db_schema::newtypes::PostId, post::CreatePostLike},
    LemmyRequest,
  };

  let body = CreatePostLike {
    post_id: PostId(id),
    score,
  };

  let (client, session) = get_client_and_session().await;

  client?
    .like_post(LemmyRequest {
      body,
      jwt: session?.get::<String>("jwt")?,
    })
    .await
    .map_err(Into::into)
}

#[server(prefix = "/serverfn")]
pub async fn vote_comment_fn(id: i32, score: i16) -> Result<CommentResponse, ServerFnError> {
  use crate::server::get_client_and_session;
  use lemmy_client::{
    lemmy_api_common::{comment::CreateCommentLike, lemmy_db_schema::newtypes::CommentId},
    LemmyRequest,
  };

  let body = CreateCommentLike {
    comment_id: CommentId(id),
    score,
  };
  let (client, session) = get_client_and_session().await;

  client?
    .like_comment(LemmyRequest {
      body,
      jwt: session?.get::<String>("jwt")?,
    })
    .await
    .map_err(Into::into)
}

// These traits and impls are to keep unwanted actions from being passed to the component
trait VoteServerFn: ServerFn + Clone + 'static {}
trait VoteResponse: 'static + for<'de> Deserialize<'de> {}

impl VoteServerFn for VotePostFn {}
impl VoteResponse for PostResponse {}

impl VoteServerFn for VoteCommentFn {}
impl VoteResponse for CommentResponse {}

#[component]
fn Votes<VoteFn, Res>(
  #[prop(into)] id: TextProp,
  #[prop(into, default = MaybeSignal::Static(0))] vote: MaybeSignal<i16>,
  #[prop(into, default = MaybeSignal::Static(0))] score: MaybeSignal<i64>,
  #[prop()] action: Action<VoteFn, Result<VoteFn::Output, ServerFnError<VoteFn::Error>>>,
) -> impl IntoView
where
  VoteFn:
    VoteServerFn + DeserializeOwned + ServerFn<InputEncoding = PostUrl, Output = Res> + 'static,
  <<VoteFn::Client as Client<VoteFn::Error>>::Request as ClientReq<VoteFn::Error>>::FormData:,
  Res: VoteResponse,
  <<<VoteFn as ServerFn>::Client as Client<<VoteFn as ServerFn>::Error>>::Request as ClientReq<
    <VoteFn as ServerFn>::Error,
  >>::FormData: From<web_sys::FormData>,
{
  let is_upvoted = Signal::derive(move || vote.get() == 1);
  let is_downvoted = Signal::derive(move || vote.get() == -1);
  let id2 = id.clone();

  view! {
    <ActionForm action=action>
      <input type="hidden" name="id" value=id/>
      <input
        type="hidden"
        name="score"
        value=move || with!(| is_upvoted | if * is_upvoted { 0 } else { 0 })
      />
      <button
        type="submit"
        class=move || with!(| is_upvoted | if * is_upvoted { Some("text-accent") } else { None })
        title="Up vote"
      >
        <Icon icon=Upvote/>
      </button>
    </ActionForm>
    <span class="block text-sm">{score}</span>
    <ActionForm action=action>
      <input type="hidden" name="id" value=id2/>
      <input
        type="hidden"
        name="score"
        value=move || with!(| is_downvoted | if * is_downvoted { 0 } else { - 1 })
      />
      <button
        type="submit"
        class=move || {
            with!(| is_downvoted | if * is_downvoted { Some("text-accent") } else { None })
        }
        title="Down vote"
      >
        <Icon icon=Downvote/>
      </button>
    </ActionForm>
  }
}

#[component]
pub fn PostVotes(
  #[prop(into)] id: TextProp,
  #[prop(into, default = MaybeSignal::Static(0))] vote: MaybeSignal<i16>,
  #[prop(into, default = MaybeSignal::Static(0))] score: MaybeSignal<i64>,
) -> impl IntoView {
  let action = Action::<VotePostFn, _>::server();

  view! { <Votes id=id vote=vote score=score action=action/> }
}

#[component]
pub fn CommentVotes(
  #[prop(into)] id: TextProp,
  #[prop(into, default = MaybeSignal::Static(0))] vote: MaybeSignal<i16>,
  #[prop(into, default = MaybeSignal::Static(0))] score: MaybeSignal<i64>,
) -> impl IntoView {
  let action = Action::<VoteCommentFn, _>::server();

  view! { <Votes id=id vote=vote score=score action=action/> }
}
