use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Inscriptions {
    pub inscriptions: Vec<InscriptionItem>,
}

#[derive(Debug, Deserialize)]
pub struct InscriptionItem {
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct Inscription {
    pub number: i64,
    pub offset: i64,
    pub genesis_fee: i64,
    pub genesis_height: i64,
    pub genesis_transaction: String,
    pub inscription_id: String,
    pub output: String,
    pub location: String,
    pub address: String,
    pub genesis_address: String,
    pub content_type: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionInscription {
    pub inscription_id: String,
    pub inscription_number: i64,
    pub content_type: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub transaction: String,
    pub inputs: Vec<(String, u64)>,
    pub outputs: Vec<(String, u64)>,
    pub output_addresses: Vec<String>,
    pub inscriptions: Vec<TransactionInscription>,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub hash: String,
    pub height: i32,
    pub previous_blockhash: String,
    pub size: i32,
    pub target: String,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub weight: i32,
}

#[derive(Clone)]
pub struct Ordinals {
    base_url: String,
}

impl Ordinals {
    pub(crate) fn new(base_url: String) -> Self {
        Ordinals { base_url }
    }

    #[allow(unused)]
    pub(crate) async fn get_status(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}status", self.base_url);
        let response = reqwest::get(url).await?;

        if response.status().is_success() {
            let content = response.text().await?;
            let status = content.trim().to_string();
            Ok(status)
        } else {
            Err(format!("Failed with status code: {}", response.status()).into())
        }
    }

    pub(crate) async fn get_block_height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let url = format!("{}blockheight", self.base_url);
        let response = reqwest::get(url).await?;

        if response.status().is_success() {
            let content = response.text().await?;
            let block_height = content.trim().parse::<i32>()?;
            Ok(block_height)
        } else {
            Err(format!("Failed with status code: {}", response.status()).into())
        }
    }

    pub(crate) async fn get_block(
        &mut self,
        block_height: i32,
    ) -> Result<Block, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}block/{}", self.base_url, block_height);
        let response = reqwest::Client::new()
            .get(&url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let block = response.json::<Block>().await?;
            Ok(block)
        } else {
            Err(format!("Failed with status code: {}", response.status()).into())
        }
    }

    #[allow(unused)]
    pub(crate) async fn get_inscriptions(
        &mut self,
        inscription_num: i64,
    ) -> Result<Inscriptions, Box<dyn std::error::Error>> {
        let url = format!("{}inscriptions/{}", self.base_url, inscription_num);
        let response = reqwest::Client::new()
            .get(&url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let inscriptions = response.json::<Inscriptions>().await?;
            Ok(inscriptions)
        } else {
            Err(format!("Failed with status code: {}", response.status()).into())
        }
    }

    #[allow(unused)]
    pub(crate) async fn get_inscription(
        &mut self,
        inscription_id: String,
    ) -> Result<Inscription, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}inscription/{}", self.base_url, inscription_id.as_str());
        println!("get_inscription: {}", url);

        let response = reqwest::Client::new()
            .get(&url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let inscription = response.json::<Inscription>().await?;
            Ok(inscription)
        } else {
            Err(format!("Failed with status code: {}", response.status()).into())
        }
    }

    pub(crate) async fn get_inscription_content(
        &mut self,
        inscription_id: String,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}content/{}", self.base_url, inscription_id.as_str());
        println!("{:?}", url);
        let response = reqwest::get(url).await?;

        if response.status().is_success() {
            let content = response.text().await?;
            Ok(content)
        } else {
            Err(format!("Failed with status code: {}", response.status()).into())
        }
    }
}
