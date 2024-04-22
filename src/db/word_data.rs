use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name=crate::db::schema::stardict)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WordData {
    pub id: i32,
    pub word: String,
    pub sw: String,
    pub phonetic: Option<String>,
    pub definition: Option<String>,
    pub translation: Option<String>,
    pub pos: Option<String>,
    pub collins: Option<i32>,
    pub oxford: Option<i32>,
    pub tag: Option<String>,
    pub bnc: Option<i32>,
    pub frq: Option<i32>,
    pub exchange: Option<String>,
    pub detail: Option<String>,
    pub audio: Option<String>,
}

impl WordData {
    pub fn debug(&self) {
        println!("----------------");
        println!("Word: {}", self.word);
        println!("SW: {}", self.sw);
        println!("Phonetic: {:?}", self.phonetic);
        println!("Definition(英文释义): {:?}", self.definition);
        println!("Translation(中文释义): {:?}", self.translation);
        println!("POS(句子中的位置): {:?}", self.pos);
        println!("Collins(柯林斯星级): {:?}", self.collins);
        println!("Oxford(是否是牛津核心): {:?}", self.oxford);
        println!("Tag: {:?}", self.tag);
        println!("BNC(英国国家语料库词频): {:?}", self.bnc);
        println!("FRQ(当代语料库词频): {:?}", self.frq);
        println!("Exchange: {:?}", self.exchange);
        println!("Detail: {:?}", self.detail);
        println!("Audio: {:?}", self.audio);
        println!("----------------");
        println!();
    }
}


// ----------------
// Word: apple
// SW: apple
// Phonetic: Some("'æpl")
// Definition(英文释义): Some("n. fruit with red or yellow or green skin and sweet to tart crisp whitish flesh\nn. native Eurasian tree widely cultivated in many varieties 
// for its firm rounded edible fruits")
// Translation(中文释义): Some("n. 苹果, 家伙\n[医] 苹果")
// POS(句子中的位置): Some("n:100")
// Collins(柯林斯星级): Some(3)
// Oxford(是否是牛津核心): Some(1)
// Tag: Some("zk gk")
// BNC(英国国家语料库词频): Some(2446)
// FRQ(当代语料库词频): Some(2695)
// Exchange: Some("s:apples")
// Detail: None
// Audio: Some("")
// ----------------

// ----------------
// Word: ample
// SW: ample
// Phonetic: Some("'æmpl")
// Definition(英文释义): Some("a. more than enough in size or scope or capacity\ns. affording an abundant supply\ns. fairly large")
// Translation(中文释义): Some("a. 大量的, 充足的, 丰富的")
// POS(句子中的位置): Some("j:100")
// Collins(柯林斯星级): Some(2)
// Oxford(是否是牛津核心): None
// Tag: Some("gk cet6 ky toefl ielts gre")
// BNC(英国国家语料库词频): Some(6351)
// FRQ(当代语料库词频): Some(6289)
// Exchange: Some("r:ampler/t:amplest")
// Detail: None
// Audio: Some("")
// ----------------

// ----------------
// Word: apale
// SW: apale
// Phonetic: None
// Definition(英文释义): None
// Translation(中文释义): Some("[网络] 苹果")
// POS(句子中的位置): None
// Collins(柯林斯星级): None
// Oxford(是否是牛津核心): None
// Tag: None
// BNC(英国国家语料库词频): None
// FRQ(当代语料库词频): None
// Exchange: None
// Detail: None
// Audio: None
// ----------------

// ----------------
// Word: appale
// SW: appale
// Phonetic: None
// Definition(英文释义): None
// Translation(中文释义): Some("[网络] 苹果；布丁喵")
// POS(句子中的位置): None
// Collins(柯林斯星级): None
// Oxford(是否是牛津核心): None
// Tag: None
// BNC(英国国家语料库词频): None
// FRQ(当代语料库词频): None
// Exchange: None
// Detail: None
// Audio: None
// ----------------

// ----------------
// Word: appe
// SW: appe
// Phonetic: Some("")
// Definition(英文释义): Some("")
// Translation(中文释义): Some("abbr. 增强形帧解析引擎（advanced packet parsing engine）")
// POS(句子中的位置): Some("")
// Collins(柯林斯星级): None
// Oxford(是否是牛津核心): None
// Tag: Some("")
// BNC(英国国家语料库词频): Some(0)
// FRQ(当代语料库词频): Some(0)
// Exchange: Some("")
// Detail: None
// Audio: Some("")
// ----------------
