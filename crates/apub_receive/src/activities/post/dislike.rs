use crate::activities::{post::like_or_dislike_post, LemmyActivity};
use activitystreams::activity::kind::DislikeType;
use lemmy_apub::check_is_apub_id_valid;
use lemmy_apub_lib::{verify_domains_match, ActivityHandler, PublicUrl};
use lemmy_db_schema::source::person::Person;
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;
use url::Url;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DislikePost {
  to: PublicUrl,
  pub(in crate::activities::post) object: Url,
  cc: [Url; 1],
  #[serde(rename = "type")]
  kind: DislikeType,
}

#[async_trait::async_trait(?Send)]
impl ActivityHandler for LemmyActivity<DislikePost> {
  type Actor = Person;

  async fn verify(&self, _context: &LemmyContext) -> Result<(), LemmyError> {
    verify_domains_match(&self.actor, self.id_unchecked())?;
    check_is_apub_id_valid(&self.actor, false)
  }

  async fn receive(
    &self,
    actor: Self::Actor,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    like_or_dislike_post(-1, &actor, &self.inner.object, context, request_counter).await
  }
}