use crate::ui::components::common::{
  icon::{
    Icon,
    IconType::{Block, Crosspost, Downvote, Report, Save, Upvote, VerticalDots},
  },
  votes::PostVotes,
};
use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::{newtypes::*, source::post::Post}, //{PersonId, PostId},
  lemmy_db_views::structs::*,                         //PostView,
  person::*,                                          //{BlockPerson, BlockPersonResponse},
  post::*, //{CreatePostLike, CreatePostReport, PostReportResponse, PostResponse, SavePost},
};
use leptos::*;
use leptos_router::*;

// #[server(VotePostFn, "/serverfn")]
// pub async fn vote_post_fn(post_id: i32, score: i16) -> Result<PostResponse, ServerFnError> {
//   use actix_web::web;
//   use leptos_actix::extract;

//   let form = CreatePostLike {
//     post_id: PostId(post_id),
//     score,
//   };
//   let client = extract::<web::Data<awc::Client>>().await?;
//   client.like_post(form).await.map_err(Into::into)
// }

// #[server(SavePostFn, "/serverfn")]
// pub async fn save_post_fn(post_id: i32, save: bool) -> Result<PostResponse, ServerFnError> {
//   use actix_web::web;
//   use leptos_actix::extract;

//   let form = SavePost {
//     post_id: PostId(post_id),
//     save,
//   };
//   let client = extract::<web::Data<awc::Client>>().await?;
//   client.save_post(form).await.map_err(Into::into)
// }

// #[server(BlockUserFn, "/serverfn")]
// pub async fn block_user_fn(
//   person_id: i32,
//   block: bool,
// ) -> Result<BlockPersonResponse, ServerFnError> {
//   use actix_web::web;
//   use leptos_actix::extract;

//   let form = BlockPerson {
//     person_id: PersonId(person_id),
//     block,
//   };
//   let client = extract::<web::Data<awc::Client>>().await?;
//   client.block_user(form).await.map_err(Into::into)
// }

// #[server(ReportPostFn, "/serverfn")]
// pub async fn report_post_fn(
//   post_id: i32,
//   reason: String,
// ) -> Result<PostReportResponse, ServerFnError> {
//   use actix_web::web;
//   use leptos_actix::extract;

//   let form = CreatePostReport {
//     post_id: PostId(post_id),
//     reason,
//   };
//   let client = extract::<web::Data<awc::Client>>().await?;
//   client.report_post(form).await.map_err(Into::into)
// }

#[component]
pub fn PostListing(post_view: PostView) -> impl IntoView {
  let PostView {
    post,
    creator,
    my_vote,
    counts,
    community,
    unread_comments,
    ..
  } = post_view;

  // let save_post_action = create_server_action::<SavePostFn>();

  // create_effect(move |_| {
  //   error.set(None);
  //   match save_post_action.value().get() {
  //     None => {}
  //     Some(Ok(o)) => {
  //       post_view.set(o.post_view);
  //     }
  //     Some(Err(e)) => {
  //       error.set(Some(e.to_string()));
  //     }
  //   }
  // });

  // let block_user_action = create_server_action::<BlockUserFn>();

  // create_effect(move |_| {
  //   error.set(None);
  //   match block_user_action.value().get() {
  //     None => {}
  //     Some(Ok(_o)) => {}
  //     Some(Err(e)) => {
  //       error.set(Some(e.to_string()));
  //     }
  //   }
  // });

  // let report_post_action = create_server_action::<BlockUserFn>();

  // create_effect(move |_| {
  //   error.set(None);
  //   match report_post_action.value().get() {
  //     None => {}
  //     Some(Ok(_o)) => {}
  //     Some(Err(e)) => {
  //       error.set(Some(e.to_string()));
  //     }
  //   }
  // });

  view! {
    <tr>
      <td class="flex flex-col text-center">
        <PostVotes id=post.id.0 score=counts.score vote=my_vote.unwrap_or_default()/>
      </td>
      // <td>

      {if let Some(url) = post.url {
          view! {
            <span>
              <a href=url.to_string()>{post.thumbnail_url.map(|tn| tn.to_string())}</a>
            </span>
          }
      } else {
          view! { <span>{post.thumbnail_url.map(|tn| tn.to_string())}</span> }
      }}

      // </td>
      <td>
        <A href=format!("/post/{}", post.id.0) class="block">
          <span class="text-lg">{post.name}</span>
        </A>
        <span class="block">
          <A href=format!("/u/{}", creator.name) class="text-sm inline-block">
            {creator.name}
          </A>
          " to "
          <A class="text-sm inline-block" href=format!("/c/{}", community.name)>
            {community.title}
          </A>
        </span>
        <span class="block">
          <span title=move || format!("{unread_comments} comments")>
            <A
              href=format!("/post/{}?scrollToComments=true", post.id.0)
              class="text-xs inline-block whitespace-nowrap align-top"
            >
              {unread_comments}
            </A>
          </span>
          // <ActionForm action=save_post_action class="inline-block align-top">
          // <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
          // <input type="hidden" name="save" value=move || format!("{}", !post_view.get().saved)/>
          // <button
          // type="submit"
          // title="Save post"
          // class=move || if post_view.get().saved { " text-accent" } else { "" }
          // >
          // <Icon icon=Save/>
          // </button>
          // </ActionForm>
          <span title="Cross post">
            <A href="/create_post" class="inline-block align-top">
              <Icon icon=Crosspost/>
            </A>
          </span>

          <div class="dropdown inline-block align-top">
            <label tabindex="0">
              <Icon icon=VerticalDots/>
            </label>
            <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
              <li>
                // <ActionForm action=report_post_action>
                // <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
                // <input
                // class="input input-bordered"
                // type="text"
                // name="reason"
                // placeholder="reason"
                // />
                // <button title="Report post" type="submit">
                // <Icon icon=Report/>
                // "Report post"
                // </button>
                // </ActionForm>
                placeholder
              </li>
              // <ActionForm action=block_user_action class="inline-block">
              // <input
              // type="hidden"
              // name="person_id"
              // value=format!("{}", post_view.get().creator.id.0)
              // />
              // <input type="hidden" name="block"/>
              // <button title="Block user" type="submit">
              <li>placeholder// <Icon icon=Block/>
              // "Block user"
              // </button>
              // </ActionForm>
              </li>
            </ul>
          </div>
        </span>
      </td>
    </tr>
  }
}
