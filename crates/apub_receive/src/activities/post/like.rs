use crate::activities::post::like_or_dislike_post;
use activitystreams::activity::kind::LikeType;
use lemmy_apub::{check_is_apub_id_valid, fetcher::person::get_or_fetch_and_upsert_person};
use lemmy_apub_lib::{verify_domains_match, ActivityCommonFields, ActivityHandlerNew, PublicUrl};
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;
use url::Url;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LikePost {
  to: PublicUrl,
  pub(in crate::activities::post) object: Url,
  cc: [Url; 1],
  #[serde(rename = "type")]
  kind: LikeType,
  #[serde(flatten)]
  common: ActivityCommonFields,
}

#[async_trait::async_trait(?Send)]
impl ActivityHandlerNew for LikePost {
  async fn verify(&self, _context: &LemmyContext, _: &mut i32) -> Result<(), LemmyError> {
    verify_domains_match(&self.common.actor, self.common.id_unchecked())?;
    check_is_apub_id_valid(&self.common.actor, false)
  }

  async fn receive(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    let actor =
      get_or_fetch_and_upsert_person(&self.common.actor, context, request_counter).await?;
    like_or_dislike_post(1, &actor, &self.object, context, request_counter).await
  }

  fn common(&self) -> &ActivityCommonFields {
    &self.common
  }
}