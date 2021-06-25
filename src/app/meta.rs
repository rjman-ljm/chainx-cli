use structopt::StructOpt;
use anyhow::{anyhow, Result, Context};
use frame_metadata::{
    decode_different::{DecodeDifferent, DecodeDifferentArray},
    RuntimeMetadata, RuntimeMetadataPrefixed,
};

/// Metadata
#[derive(Debug, StructOpt)]
pub enum Meta {
    Get(GetOpt)
}

#[derive(Debug, StructOpt)]
pub struct GetOpt {
    /// the url of the substrate node to query for metadata
    #[structopt(index = 1, default_value = "http://localhost:8087")]
    url: String,
    /// the name of a pallet to display metadata for, otherwise displays all
    #[structopt(index = 2, short = "p")]
    pallet: Option<String>,
}

impl Meta {
    pub async fn run(self) -> Result<()> {
        match self {
            Meta::Get(get_opt) => {
                let metadata = Self::fetch_metadata(&get_opt.url)?;
                Self::display_metadata(metadata, get_opt.pallet)?;
            }
        }

        Ok(())
    }

    fn fetch_metadata(url: &str) -> Result<RuntimeMetadataPrefixed> {
        let resp = ureq::post(url)
            .set("Content-Type", "application/json")
            .send_json(ureq::json!({
                "jsonrpc": "2.0",
                "method": "state_getMetadata",
                "id": 1
            }))
            .context("error fetching metadata from the substrate node")?;

        let json: serde_json::Value = resp.into_json()?;
        let hex_data = json["result"]
            .as_str()
            .ok_or(anyhow!("metadata result field should be a string"))?;

        let bytes = hex::decode(hex_data.trim_start_matches("0x"))?;
        let decoded = scale::Decode::decode(&mut &bytes[..])?;
        Ok(decoded)
    }

    fn display_metadata(metadata: RuntimeMetadataPrefixed, pallets: Option<String>) -> Result<()> {
        let serialized = if let Some(ref pallet) = pallets {
            match metadata.1 {
                RuntimeMetadata::V12(v12) => {
                    let modules = match v12.modules {
                        DecodeDifferentArray::Decoded(modules) => modules,
                        DecodeDifferentArray::Encode(_) => {
                            return Err(anyhow!("Metadata should be Decoded"))
                        }
                    };
                    let module = modules
                        .iter()
                        .find(|module| module.name == DecodeDifferent::Decoded(pallet.into()))
                        .ok_or_else(|| anyhow!("pallet not found in metadata"))?;
                    serde_json::to_string_pretty(&module)?
                }
                RuntimeMetadata::V13(v13) => {
                    let modules = match v13.modules {
                        DecodeDifferentArray::Decoded(modules) => modules,
                        DecodeDifferentArray::Encode(_) => {
                            return Err(anyhow!("Metadata should be Decoded"))
                        }
                    };
                    let module = modules
                        .iter()
                        .find(|module| module.name == DecodeDifferent::Decoded(pallet.into()))
                        .ok_or_else(|| anyhow!("pallet not found in metadata"))?;
                    serde_json::to_string_pretty(&module)?
                }
                RuntimeMetadata::V14(v14) => {
                    let pallet = v14
                        .pallets
                        .iter()
                        .find(|m| &m.name == pallet)
                        .ok_or_else(|| anyhow!("pallet not found in metadata"))?;
                    serde_json::to_string_pretty(&pallet)?
                }
                _ => return Err(anyhow!("Unsupported metadata version")),
            }
        } else {
            serde_json::to_string_pretty(&metadata)?
        };
        println!("{}", serialized);
        Ok(())
    }
}
    