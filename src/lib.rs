use anyhow::Result;
use aws_credential_types::{
    provider::{ProvideCredentials, SharedCredentialsProvider},
    Credentials,
};
use aws_sdk_sns::client::Client;
use aws_types::region::Region;

pub struct Texter {
    sns_client: Client,
}

impl Texter {
    /// new
    /// Make sure these key, secret, and region info has valid permissions for the SNS actions that is about to be performed
    pub fn new(key: &str, secret: &str, region: &str) -> Texter {
        let mut builder = aws_config::SdkConfig::builder();

        builder
            .set_credentials_provider(Some(SharedCredentialsProvider::new(Texter::make_provider(
                key, secret,
            ))))
            .set_region(Texter::get_region(region));

        let config = builder.build();

        let sns_client = aws_sdk_sns::Client::new(&config);

        Texter { sns_client }
    }

    /// publish_text
    ///
    /// Sends message to the given contact_number (please include country code)
    /// Returns message_id and sequence_number for the published text
    pub async fn publish_text(
        &self,
        contact_number: &str,
        message: &str,
    ) -> Result<(Option<String>, Option<String>)> {
        let output = self
            .sns_client
            .publish()
            .set_message(Some(message.to_string()))
            .set_phone_number(Some(contact_number.to_string()))
            .send()
            .await?;

        Ok((output.message_id, output.sequence_number))
    }

    /// publish_topic
    ///
    /// Sends message to the given topic (valid topic arn)
    /// Returns message_id and sequence_number for the published topic
    pub async fn publish_topic(
        &self,
        topic_arn: &str,
        message: &str,
    ) -> Result<(Option<String>, Option<String>)> {
        let output = self
            .sns_client
            .publish()
            .set_topic_arn(Some(topic_arn.to_string()))
            .set_message(Some(message.to_string()))
            .send()
            .await?;

        Ok((output.message_id, output.sequence_number))
    }

    fn make_provider(key: &str, secret: &str) -> impl ProvideCredentials {
        Credentials::new(key, secret, None, None, "aws")
    }

    fn get_region(region: &str) -> Option<aws_types::region::Region> {
        Some(Region::new(region.to_string()))
    }
}
