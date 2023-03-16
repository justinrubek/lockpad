use rsa::{pkcs1::EncodeRsaPublicKey, pkcs8::EncodePrivateKey};
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tracing::info;

#[derive(clap::Args, Debug)]
pub(crate) struct KeyCommand {
    #[clap(subcommand)]
    pub command: KeyCommands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum KeyCommands {
    /// generate a new keypair
    Generate,
    /// Test a keypair
    Verify {
        #[arg(short, long)]
        public: PathBuf,
        #[arg(short, long)]
        secret: PathBuf,
    },
}

impl KeyCommand {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            KeyCommands::Generate => {
                info!("Generating keypair");

                let mut secret_file = tokio::fs::File::create("secret-rsa.pem").await?;
                let mut public_file = tokio::fs::File::create("public-rsa.pem").await?;

                create_rsa_keypair(&mut secret_file, &mut public_file).await?;
            }
            KeyCommands::Verify { public, secret } => {
                let public = tokio::fs::read(public).await?;
                let secret = tokio::fs::read(secret).await?;

                info!("Public key: {:?}", public);

                let claims = lockpad_auth::Claims::new(String::from("test"));
                let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(&secret)?;
                let token = claims.encode(&encoding_key).await?;
                info!(?token);

                let decoding_key = jsonwebtoken::DecodingKey::from_rsa_pem(&public)?;
                let decoded_claims = lockpad_auth::Claims::decode(&token, &decoding_key).await?;

                if claims.sub != decoded_claims.sub {
                    return Err("Claims do not match".into());
                }

                info!("Verified keypair");
            }
        }

        Ok(())
    }
}

async fn create_rsa_keypair(
    secret_file: &mut tokio::fs::File,
    public_file: &mut tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let private_key = rsa::RsaPrivateKey::new(&mut rng, 2048)?;
    let public_key = private_key.to_public_key();
    info!("Public key: {:?}", public_key);

    let secret_pem = private_key.to_pkcs8_pem(Default::default())?;
    let public_pem = public_key.to_pkcs1_pem(Default::default())?;

    info!("writing keys");
    secret_file.write_all(secret_pem.as_bytes()).await?;
    public_file.write_all(public_pem.as_bytes()).await?;

    Ok(())
}
